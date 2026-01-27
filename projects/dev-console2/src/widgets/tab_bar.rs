use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Spacing, Position},
    style::{Style, Color, Modifier},
    widgets::{Block, Widget},
    text::{Line, Span}
};
use crossterm::event::{MouseEvent, MouseButton, MouseEventKind};

use strum_macros::{EnumString, Display};
use serde::Deserialize;

/// Available visual representations for the tab bar.
#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum TabBarStyle {
    #[strum(serialize = "tab", serialize = "tabbed")]
    Tab,
    Text,
    Boxed,
    #[strum(serialize = "box_static")]
    BoxStatic,
    #[strum(serialize = "text_static")]
    TextStatic,
}

/// Positional alignment for the tab bar within its parent area.
#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum TabBarAlignment {
    Left,
    Center,
    Right,
    Top,
    Bottom,
}

/// Data model for a single tab entry.
#[derive(Debug, Clone)]
pub struct TabBarItem {
    pub id: String,
    pub name: String,
    pub active: bool,
}

/// A highly configurable tab navigation widget.
///>
/// Supports multiple visual styles (tabs, boxed, text), custom alignment, 
/// and decorative "decal" rendering over block borders. Provides integrated 
/// mouse detection for tab switching.
///<
pub struct TabBarWidget<'a> {
    pub items: &'a [TabBarItem],
    pub style: TabBarStyle,
    pub color: Color,
    pub active_color: Option<Color>,
    pub negate_color: Option<Color>,
    pub min_tab_width: u16,
    pub tab_tooltips: bool,
}

impl<'a> TabBarWidget<'a> {
    /// Creates a new tab bar with the provided items.
    pub fn new(items: &'a [TabBarItem]) -> Self {
        Self {
            items,
            style: TabBarStyle::Text,
            color: Color::White,
            active_color: None,
            negate_color: None,
            min_tab_width: 0,
            tab_tooltips: false,
        }
    }

    /// Sets the minimum width for each tab item.
    pub fn min_tab_width(mut self, width: u16) -> Self {
        self.min_tab_width = width;
        self
    }

    /// Enables or disables tab tooltips (context-sensitive help).
    pub fn tab_tooltips(mut self, enabled: bool) -> Self {
        self.tab_tooltips = enabled;
        self
    }

    /// Sets the visual style of the tab bar.
    pub fn style(mut self, style: TabBarStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the base color for tab decorators.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets the text color for the active tab.
    pub fn active_color(mut self, color: Option<Color>) -> Self {
        self.active_color = color;
        self
    }

    /// Sets the text color for inactive tabs in static styles.
    pub fn negate_color(mut self, color: Option<Color>) -> Self {
        self.negate_color = color;
        self
    }

    /// Returns the physical height required by the current style.
    pub fn desired_height(&self) -> u16 {
        if self.style == TabBarStyle::Tab { 2 } else { 1 }
    }

    /// Returns the vertical space this tab bar consumes from a parent layout.
    pub fn consumed_height(&self) -> u16 {
        self.desired_height().saturating_sub(1)
    }

    /// Static helper to calculate consumed height from raw configuration.
    pub fn config_consumed_height(config: &crate::config::Config, id: &str) -> u16 {
        if let Some(tab_config) = config.tab_bars.iter().find(|t| t.id == id) { //>
            let desired: u16 = if tab_config.style == Some(TabBarStyle::Tab) { 2 } else { 1 };
            desired.saturating_sub(1)
        } else { 0 } //<
    }

    /// Splits an area into header and body regions with correct overlap.
    pub fn split_layout(&self, area: Rect) -> [Rect; 2] {
        Layout::vertical([
            Constraint::Length(self.desired_height()),
            Constraint::Min(0),
        ])
        .spacing(Spacing::Overlap(1))
        .areas(area)
    }

    /// Factory method to construct a themed widget from application config.
    ///>
    /// Resolves style, alignment, colors, and offsets from the workspace 
    /// configuration file.
    ///<
    pub fn from_config( config: &'a crate::config::Config, tabs: &'a [TabBarItem], id: &str, ) -> Option<(Self, TabBarAlignment, TabBarAlignment, i16, i16)> {
        let tab_config = config.tab_bars.iter().find(|t| t.id == id)?;

        let style = match tab_config.style {
            Some(s) => s,
            None => TabBarStyle::Text,
        };

        let horizontal = match tab_config.alignment.horizontal {
            Some(a) => a,
            None => TabBarAlignment::Center,
        };

        let vertical = match tab_config.alignment.vertical {
            Some(a) => a,
            None => TabBarAlignment::Top,
        };

        let color = tab_config.color.as_deref()
            .map(parse_color)
            .unwrap_or(Color::White);

        let active_color = tab_config.colors.as_ref()
            .and_then(|c| c.active.as_deref())
            .map(parse_color);

        let negate_color = tab_config.colors.as_ref()
            .and_then(|c| c.negate.as_deref())
            .map(parse_color);

        Some((
            Self::new(tabs)
                .style(style)
                .color(color)
                .active_color(active_color)
                .negate_color(negate_color)
                .min_tab_width(tab_config.min_tab_width)
                .tab_tooltips(tab_config.tab_tooltips),
            horizontal,
            vertical,
            tab_config.alignment.offset_x,
            tab_config.alignment.offset_y,
        ))
    }

    /// Calculates the required width for a single tab item.
    pub fn get_item_width(&self, item: &TabBarItem) -> u16 {
        let base_width = match self.style { //>
            TabBarStyle::Tab | TabBarStyle::Boxed | TabBarStyle::BoxStatic => {
                if item.active || self.style == TabBarStyle::BoxStatic { //>
                    item.name.len() as u16 + 4
                } else {
                    item.name.len() as u16 + 2
                } //<
            }
            _ => item.name.len() as u16 + 2,
        }; //<
        base_width.max(self.min_tab_width)
    }

    /// Estimates the total width required to render all tabs and separators.
    pub fn estimate_width(&self) -> u16 {
        if self.items.is_empty() { return 0; }
        let mut width = 0;
        for (idx, item) in self.items.iter().enumerate() { //>
            if idx > 0 { width += 1; } // Separator "─"
            width += self.get_item_width(item);
        } //<
        width
    }

    /// Constructs the main text line containing tab names and decorators.
    fn build_tab_line(&self) -> Line<'a> {
        let mut spans = Vec::new();
        let active_text_color = self.active_color.unwrap_or(self.color);
        let active_style = Style::default().fg(active_text_color).add_modifier(Modifier::BOLD);
        let negate_style = Style::default().fg(self.negate_color.unwrap_or(Color::White));
        let inactive_style = Style::default().fg(Color::White); // The "Border/Decorator" color

        for (idx, item) in self.items.iter().enumerate() { //>
            if idx > 0 { //>
                spans.push(Span::styled("─", inactive_style));
            } //<

            let item_width = self.get_item_width(item);
            
            if item.active { //>
                match self.style { //>
                    TabBarStyle::Tab => {
                        let content = format!(" {} ", item.name);
                        let content_len = content.chars().count() as u16 + 2;
                        let total_pad = item_width.saturating_sub(content_len);
                        let left_pad = total_pad / 2;
                        let right_pad = total_pad - left_pad;

                        if left_pad > 0 { spans.push(Span::raw(" ".repeat(left_pad as usize))); }
                        spans.push(Span::styled("╯", inactive_style));
                        spans.push(Span::styled(content, active_style));
                        spans.push(Span::styled("╰", inactive_style));
                        if right_pad > 0 { spans.push(Span::raw(" ".repeat(right_pad as usize))); }
                    }
                    TabBarStyle::Boxed | TabBarStyle::BoxStatic => {
                        let content = format!(" {} ", item.name);
                        let content_len = content.chars().count() as u16 + 2;
                        let total_pad = item_width.saturating_sub(content_len);
                        let left_pad = total_pad / 2;
                        let right_pad = total_pad - left_pad;

                        if left_pad > 0 { spans.push(Span::raw(" ".repeat(left_pad as usize))); }
                        spans.push(Span::styled("[", inactive_style));
                        spans.push(Span::styled(content, active_style));
                        spans.push(Span::styled("]", inactive_style));
                        if right_pad > 0 { spans.push(Span::raw(" ".repeat(right_pad as usize))); }
                    }
                    _ => {
                        let content = format!(" {} ", item.name);
                        let content_len = content.chars().count() as u16;
                        let total_pad = item_width.saturating_sub(content_len);
                        let left_pad = total_pad / 2;
                        let right_pad = total_pad - left_pad;

                        if left_pad > 0 { spans.push(Span::raw(" ".repeat(left_pad as usize))); }
                        spans.push(Span::styled(content, active_style));
                        if right_pad > 0 { spans.push(Span::raw(" ".repeat(right_pad as usize))); }
                    }
                } //<
            } else {
                let item_style = if self.style == TabBarStyle::BoxStatic || self.style == TabBarStyle::TextStatic { //>
                    negate_style
                } else {
                    inactive_style
                }; //<

                let content = if self.style == TabBarStyle::BoxStatic { //>
                    format!("[ {} ]", item.name)
                } else {
                    format!(" {} ", item.name)
                }; //<

                let content_len = content.chars().count() as u16;
                let total_pad = item_width.saturating_sub(content_len);
                let left_pad = total_pad / 2;
                let right_pad = total_pad - left_pad;

                if left_pad > 0 { spans.push(Span::raw(" ".repeat(left_pad as usize))); }
                spans.push(Span::styled(content, item_style));
                if right_pad > 0 { spans.push(Span::raw(" ".repeat(right_pad as usize))); }
            } //<
        } //<
        Line::from(spans)
    }

    /// Constructs the decorative top line for the `Tab` visual style.
    fn build_top_line(&self) -> Line<'a> {
        if self.style != TabBarStyle::Tab { return Line::default(); }
        if let Some(active_idx) = self.items.iter().position(|i| i.active) { //>
            let inactive_style = Style::default().fg(Color::White); // Keep structural line white
            let mut pre_width = 0;
            for i in 0..active_idx { //>
                pre_width += self.get_item_width(&self.items[i]);
                pre_width += 1; // Separator
            } //<
            
            let item_width = self.get_item_width(&self.items[active_idx]);
            let active_item = &self.items[active_idx];
            
            let mut spans = Vec::new();
            if pre_width > 0 { //>
                spans.push(Span::raw(" ".repeat(pre_width as usize)));
            } //<
            
            let content_len = active_item.name.len() as u16 + 4; // "╭ Name ╮"
            let total_pad = item_width.saturating_sub(content_len);
            let left_pad = total_pad / 2;
            let right_pad = total_pad - left_pad;

            spans.push(Span::raw(" ".repeat(left_pad as usize)));
            spans.push(Span::styled("╭", inactive_style));
            spans.push(Span::styled("─".repeat(active_item.name.len() + 2), inactive_style));
            spans.push(Span::styled("╮", inactive_style));
            spans.push(Span::raw(" ".repeat(right_pad as usize)));
            
            Line::from(spans)
        } else {
            Line::default()
        } //<
    }

    /// Calculates the screen coordinates for the aligned tab bar.
    pub fn get_aligned_area(&self, area: Rect, horizontal: TabBarAlignment, vertical: TabBarAlignment, offset_x: i16, offset_y: i16) -> Rect {
        let width = self.estimate_width();
        let height = self.desired_height();

        let x = match horizontal { //>
            TabBarAlignment::Left => area.x + 1,
            TabBarAlignment::Center => area.x + (area.width.saturating_sub(width)) / 2,
            TabBarAlignment::Right => area.x + area.width.saturating_sub(width).saturating_sub(1),
            _ => area.x + 1,
        }; //<

        let y = match vertical { //>
            TabBarAlignment::Top => area.y,
            TabBarAlignment::Bottom => area.y + area.height.saturating_sub(height),
            _ => area.y,
        }; //<

        // Apply offsets
        let final_x = if offset_x >= 0 { //>
            x.saturating_add(offset_x as u16)
        } else {
            x.saturating_sub(offset_x.abs() as u16)
        }; //<

        let final_y = if offset_y >= 0 { //>
            y.saturating_add(offset_y as u16)
        } else {
            y.saturating_sub(offset_y.abs() as u16)
        }; //<

        Rect {
            x: final_x,
            y: final_y,
            width: width.min(area.width.saturating_sub(final_x.saturating_sub(area.x))),
            height: height.min(area.height.saturating_sub(final_y.saturating_sub(area.y))),
        }
    }

    /// Renders the widget with specified alignment and offsets.
    pub fn render_aligned(self, area: Rect, horizontal: TabBarAlignment, vertical: TabBarAlignment, offset_x: i16, offset_y: i16, buf: &mut Buffer) {
        let tab_area = self.get_aligned_area(area, horizontal, vertical, offset_x, offset_y);
        self.render(tab_area, buf);
    }

    /// Identifies if a mouse click occurred over a specific tab item.
    pub fn handle_mouse_event(&self, area: Rect, horizontal: TabBarAlignment, vertical: TabBarAlignment, offset_x: i16, offset_y: i16, mouse_event: MouseEvent) -> Option<usize> {
        // Only respond to left clicks
        if !matches!(mouse_event.kind, MouseEventKind::Down(MouseButton::Left)) { //>
            return None;
        } //<

        let aligned_area = self.get_aligned_area(area, horizontal, vertical, offset_x, offset_y);
        let mouse_pos = Position::new(mouse_event.column, mouse_event.row);

        if !aligned_area.contains(mouse_pos) { //>
            return None;
        } //<

        // Relative x position within the aligned area
        let rel_x = mouse_pos.x.saturating_sub(aligned_area.x);
        
        let mut current_x = 0;
        for (idx, item) in self.items.iter().enumerate() { //>
            let item_width = self.get_item_width(item);

            if rel_x >= current_x && rel_x < current_x + item_width { //>
                return Some(idx);
            } //<

            current_x += item_width;
            current_x += 1; // Separator "─"
        } //<

        None
    }

    /// Orchestrates the rendering of multiple tab bars over a shared parent block.
    ///>
    /// Handles layout partitioning for stacked tab bars and ensures they 
    /// are rendered correctly over the parent borders.
    ///<
    pub fn render_composite(
        config: &'a crate::config::Config,
        tabs: &'a [TabBarItem],
        tab_ids: &[&str],
        area: Rect,
        buf: &mut Buffer,
    ) -> Rect {
        let mut current_body_area = area;
        let mut active_decals = Vec::new();

        // 1. Calculate the stacked layout for all tab bars
        for id in tab_ids { //>
            if let Some((tab_bar, horizontal, vertical, off_x, off_y)) = Self::from_config(config, tabs, id) { //>
                if vertical == TabBarAlignment::Top { //>
                    let [header, body] = tab_bar.split_layout(current_body_area);
                    current_body_area = body;
                    active_decals.push((tab_bar, horizontal, vertical, off_x, off_y, header));
                } else if vertical == TabBarAlignment::Bottom {
                    // For bottom alignment, split the bottom off
                    let height = tab_bar.consumed_height();
                    let [body, footer] = Layout::vertical([
                        Constraint::Min(0),
                        Constraint::Length(height),
                    ]).areas(current_body_area);
                    current_body_area = body;
                    active_decals.push((tab_bar, horizontal, vertical, off_x, off_y, footer));
                } else {
                    active_decals.push((tab_bar, horizontal, vertical, off_x, off_y, current_body_area));
                } //<
            } //<
        } //<

        // 2. Render the primary Block (the "Parent")
        Block::bordered()
            .render(current_body_area, buf);

        // 3. Render the tab bar decals over the borders
        for (widget, horizontal, vertical, off_x, off_y, header_area) in active_decals { //>
            widget.render_aligned(header_area, horizontal, vertical, off_x, off_y, buf);
        } //<

        // 4. Return the inner area
        Block::bordered().inner(current_body_area)
    }
}

/// Helper function to parse string color names into `ratatui::style::Color`.
fn parse_color(c: &str) -> Color {
    match c.to_lowercase().as_str() { //>
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" | "grey" => Color::Gray,
        "darkgray" | "darkgrey" => Color::DarkGray,
        "lightred" => Color::LightRed,
        "lightgreen" => Color::LightGreen,
        "lightyellow" => Color::LightYellow,
        "lightblue" => Color::LightBlue,
        "lightmagenta" => Color::LightMagenta,
        "lightcyan" => Color::LightCyan,
        "white" => Color::White,
        "dimgrey" | "dimgray" => Color::Indexed(240),
        _ => Color::White,
    } //<
}

impl<'a> Widget for TabBarWidget<'a> {
    /// Renders the tab bar line(s) into the buffer.
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.style == TabBarStyle::Tab && area.height >= 2 { //>
            buf.set_line(area.x, area.y, &self.build_top_line(), area.width);
            buf.set_line(area.x, area.y + 1, &self.build_tab_line(), area.width);
        } else {
            buf.set_line(area.x, area.y, &self.build_tab_line(), area.width);
        } //<
    }
}