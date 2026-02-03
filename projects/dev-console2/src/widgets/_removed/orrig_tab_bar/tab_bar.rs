use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Position, Rect, Spacing},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};
use serde::Deserialize;
use strum_macros::{Display, EnumString};

/// Visual themes for the tab bar.
///>
/// Controls the decorative characters and spacing used to render individual
/// tabs, allowing for various degrees of visual prominence.
///<
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
///>
/// Used to calculate the bounding box of the tab bar relative to a larger
/// anchor region, supporting both edge-aligned and centered layouts.
///<
#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum TabBarAlignment {
    Left,
    Center,
    Right,
    Top,
    Bottom,
}
/// Metadata for a single interactive tab.
///>
/// Represents the state and identity of a tab, used by the widget to
/// determine rendering styles and hit-detection results.
///<
#[derive(Debug, Clone)]
pub struct TabBarItem {
    pub id: String,
    pub name: String,
    pub active: bool,
}

/// A highly configurable tab bar widget for Ratatui.
///>
/// Supports multiple visual styles (Tabs, Boxes, Plain Text), dynamic
/// alignment, and integrated mouse hit detection. Designed to be used
/// both as a standalone widget and as a layout-defining component.
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
    /// Creates a new tab bar with default settings.
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

    /// Sets a minimum width for each tab to ensure visual consistency.
    pub fn min_tab_width(mut self, width: u16) -> Self {
        self.min_tab_width = width;
        self
    }

    /// Enables or disables tooltip support for individual tabs.
    pub fn tab_tooltips(mut self, enabled: bool) -> Self {
        self.tab_tooltips = enabled;
        self
    }

    /// Sets the visual style (theme) for the tab bar.
    pub fn style(mut self, style: TabBarStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the base foreground color for inactive tabs and decorations.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets an optional override color for the active tab's text.
    pub fn active_color(mut self, color: Option<Color>) -> Self {
        self.active_color = color;
        self
    }

    /// Sets an optional override color for static/negated tab elements.
    pub fn negate_color(mut self, color: Option<Color>) -> Self {
        self.negate_color = color;
        self
    }

    /// Returns the vertical space required to render the widget.
    ///>
    /// The "Tab" style requires two lines to accommodate the decorative top
    /// border, while other styles only consume a single line.
    ///<
    pub fn desired_height(&self) -> u16 {
        if self.style == TabBarStyle::Tab {
            2
        } else {
            1
        }
    }

    /// Returns the amount of vertical space consumed by decorations.
    ///>
    /// Used by layout engines to subtract padding from content areas.
    ///<
    pub fn consumed_height(&self) -> u16 {
        self.desired_height().saturating_sub(1)
    }

    /// Static helper for calculating consumed height from configuration.
    ///>
    /// Allows the layout engine to reserve space without instantiating the
    /// full widget, supporting optimized layout caching.
    ///<
    pub fn config_consumed_height(config: &crate::config::Config, id: &str) -> u16 {
        if let Some(tab_config) = config.tab_bars.iter().find(|t| t.id == id) {
            let desired: u16 = if tab_config.style == Some(TabBarStyle::Tab) {
                2
            } else {
                1
            };
            desired.saturating_sub(1)
        } else {
            0
        }
    }

    /// Splits a rectangle into a header area and a remaining body area.
    pub fn split_layout(&self, area: Rect) -> [Rect; 2] {
        Layout::vertical([
            Constraint::Length(self.desired_height()),
            Constraint::Min(0),
        ])
        .spacing(Spacing::Overlap(1))
        .areas(area)
    }
    /// Resolves widget configuration from the global application state.
    ///>
    /// Maps YAML-defined tab bar settings to a functional widget instance,
    /// including alignment, colors, and offsets.
    ///<
    pub fn from_config(
        config: &'a crate::config::Config,
        tabs: &'a [TabBarItem],
        id: &str,
    ) -> Option<(Self, TabBarAlignment, TabBarAlignment, i16, i16)> {
        let tab_config = config.tab_bars.iter().find(|t| t.id == id)?;
        //>
        let style = tab_config.style.unwrap_or(TabBarStyle::Text);
        let horizontal = tab_config
            .alignment
            .horizontal
            .unwrap_or(TabBarAlignment::Center);
        let vertical = tab_config
            .alignment
            .vertical
            .unwrap_or(TabBarAlignment::Top);
        let color = tab_config
            .color
            .as_deref()
            .map(parse_color)
            .unwrap_or(Color::White);
        let active_color = tab_config
            .colors
            .as_ref()
            .and_then(|c| c.active.as_deref())
            .map(parse_color);
        let negate_color = tab_config
            .colors
            .as_ref()
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
        //<
    }

    /// Calculates the horizontal span required for a single tab.
    pub fn get_item_width(&self, item: &TabBarItem) -> u16 {
        let base_width = match self.style {
            TabBarStyle::Tab | TabBarStyle::Boxed | TabBarStyle::BoxStatic => {
                if item.active || self.style == TabBarStyle::BoxStatic {
                    item.name.len() as u16 + 4
                } else {
                    item.name.len() as u16 + 2
                }
            }
            _ => item.name.len() as u16 + 2,
        };
        base_width.max(self.min_tab_width)
    }

    /// Estimates the total width of the tab bar including separators.
    pub fn estimate_width(&self) -> u16 {
        if self.items.is_empty() {
            return 0;
        }
        let mut width = 0;
        for (idx, item) in self.items.iter().enumerate() {
            if idx > 0 {
                width += 1; // Separator width
            }
            width += self.get_item_width(item);
        }
        width
    }

    /// Generates the main line of text for the tab bar.
    ///>
    /// Iterates through all items, applying styles and decorative characters
    /// (e.g., brackets, connectors) based on the active theme and state.
    ///<
    fn build_tab_line(&self) -> Line<'a> {
        let mut spans = Vec::new();
        //>
        let active_text_color = self.active_color.unwrap_or(self.color);
        let active_style = Style::default()
            .fg(active_text_color)
            .add_modifier(Modifier::BOLD);
        let negate_style = Style::default().fg(self.negate_color.unwrap_or(Color::White));
        let inactive_style = Style::default().fg(Color::White);

        for (idx, item) in self.items.iter().enumerate() {
            if idx > 0 {
                spans.push(Span::styled("─", inactive_style));
            }

            let item_width = self.get_item_width(item);
            if item.active {
                match self.style {
                    TabBarStyle::Tab => {
                        let content = format!(" {} ", item.name);
                        let content_len = content.chars().count() as u16 + 2;
                        let total_pad = item_width.saturating_sub(content_len);
                        let left_pad = total_pad / 2;
                        let right_pad = total_pad - left_pad;

                        if left_pad > 0 {
                            spans.push(Span::raw(" ".repeat(left_pad as usize)));
                        }
                        spans.push(Span::styled("╯", inactive_style));
                        spans.push(Span::styled(content, active_style));
                        spans.push(Span::styled("╰", inactive_style));
                        if right_pad > 0 {
                            spans.push(Span::raw(" ".repeat(right_pad as usize)));
                        }
                    }
                    TabBarStyle::Boxed | TabBarStyle::BoxStatic => {
                        let content = format!(" {} ", item.name);
                        let content_len = content.chars().count() as u16 + 2;
                        let total_pad = item_width.saturating_sub(content_len);
                        let left_pad = total_pad / 2;
                        let right_pad = total_pad - left_pad;

                        if left_pad > 0 {
                            spans.push(Span::raw(" ".repeat(left_pad as usize)));
                        }
                        spans.push(Span::styled("[", inactive_style));
                        spans.push(Span::styled(content, active_style));
                        spans.push(Span::styled("]", inactive_style));
                        if right_pad > 0 {
                            spans.push(Span::raw(" ".repeat(right_pad as usize)));
                        }
                    }
                    _ => {
                        let content = format!(" {} ", item.name);
                        let content_len = content.chars().count() as u16;
                        let total_pad = item_width.saturating_sub(content_len);
                        let left_pad = total_pad / 2;
                        let right_pad = total_pad - left_pad;

                        if left_pad > 0 {
                            spans.push(Span::raw(" ".repeat(left_pad as usize)));
                        }
                        spans.push(Span::styled(content, active_style));
                        if right_pad > 0 {
                            spans.push(Span::raw(" ".repeat(right_pad as usize)));
                        }
                    }
                }
            } else {
                let item_style = if self.style == TabBarStyle::BoxStatic
                    || self.style == TabBarStyle::TextStatic
                {
                    negate_style
                } else {
                    inactive_style
                };
                let content = if self.style == TabBarStyle::BoxStatic {
                    format!("[ {} ]", item.name)
                } else {
                    format!(" {} ", item.name)
                };

                let content_len = content.chars().count() as u16;
                let total_pad = item_width.saturating_sub(content_len);
                let left_pad = total_pad / 2;
                let right_pad = total_pad - left_pad;

                if left_pad > 0 {
                    spans.push(Span::raw(" ".repeat(left_pad as usize)));
                }
                spans.push(Span::styled(content, item_style));
                if right_pad > 0 {
                    spans.push(Span::raw(" ".repeat(right_pad as usize)));
                }
            }
        }
        Line::from(spans)
        //<
    }

    /// Generates the decorative top border for "Tab" style headers.
    fn build_top_line(&self) -> Line<'a> {
        if self.style != TabBarStyle::Tab {
            return Line::default();
        }

        if let Some(active_idx) = self.items.iter().position(|i| i.active) {
            //>
            let inactive_style = Style::default().fg(Color::White);
            let mut pre_width = 0;
            for i in 0..active_idx {
                pre_width += self.get_item_width(&self.items[i]);
                pre_width += 1;
            }

            let item_width = self.get_item_width(&self.items[active_idx]);
            let active_item = &self.items[active_idx];
            let mut spans = Vec::new();

            if pre_width > 0 {
                spans.push(Span::raw(" ".repeat(pre_width as usize)));
            }

            let content_len = active_item.name.len() as u16 + 4;
            let total_pad = item_width.saturating_sub(content_len);
            let left_pad = total_pad / 2;
            let right_pad = total_pad - left_pad;

            spans.push(Span::raw(" ".repeat(left_pad as usize)));
            spans.push(Span::styled("╭", inactive_style));
            spans.push(Span::styled(
                "─".repeat(active_item.name.len() + 2),
                inactive_style,
            ));
            spans.push(Span::styled("╮", inactive_style));
            spans.push(Span::raw(" ".repeat(right_pad as usize)));
            Line::from(spans)
            //<
        } else {
            Line::default()
        }
    }

    /// Calculates the final bounding box based on alignment rules.
    pub fn get_aligned_area(
        &self,
        area: Rect,
        horizontal: TabBarAlignment,
        vertical: TabBarAlignment,
        offset_x: i16,
        offset_y: i16,
    ) -> Rect {
        //>
        let width = self.estimate_width();
        let height = self.desired_height();

        let x = match horizontal {
            TabBarAlignment::Left => area.x + 1,
            TabBarAlignment::Center => area.x + (area.width.saturating_sub(width)) / 2,
            TabBarAlignment::Right => area.x + area.width.saturating_sub(width).saturating_sub(1),
            _ => area.x + 1,
        };

        let y = match vertical {
            TabBarAlignment::Top => area.y,
            TabBarAlignment::Bottom => area.y + area.height.saturating_sub(height),
            _ => area.y,
        };

        let final_x = if offset_x >= 0 {
            x.saturating_add(offset_x as u16)
        } else {
            x.saturating_sub(offset_x.abs() as u16)
        };

        let final_y = if offset_y >= 0 {
            y.saturating_add(offset_y as u16)
        } else {
            y.saturating_sub(offset_y.abs() as u16)
        };

        let clipped_x = final_x.max(area.x).min(area.right().saturating_sub(1));
        let clipped_y = final_y.max(area.y).min(area.bottom().saturating_sub(1));
        let clipped_width = width.min(area.right().saturating_sub(clipped_x));
        let clipped_height = height.min(area.bottom().saturating_sub(clipped_y));

        Rect {
            x: clipped_x,
            y: clipped_y,
            width: clipped_width,
            height: clipped_height,
        }
        //<
    }
        /// Renders the tab bar with specific alignment and offsets.
        pub fn render_aligned(
            self,
            area: Rect,
            horizontal: TabBarAlignment,
            vertical: TabBarAlignment,
            offset_x: i16,
            offset_y: i16,
            buf: &mut Buffer,
        ) {
            //>
            let tab_area = self.get_aligned_area(area, horizontal, vertical, offset_x, offset_y);
            self.render(tab_area, buf);
            //<
        }
    
        /// Handles mouse clicks to determine if a tab was selected.
        ///>
        /// Delegates hit detection to individual tab geometry and returns the
        /// index of the clicked tab if the event occurred within the widget.
        ///<
        pub fn handle_mouse_event(
            &self,
            area: Rect,
            horizontal: TabBarAlignment,
            vertical: TabBarAlignment,
            offset_x: i16,
            offset_y: i16,
            mouse_event: MouseEvent,
        ) -> Option<usize> {
            if !matches!(mouse_event.kind, MouseEventKind::Down(MouseButton::Left)) {
                return None;
            }
    
            let aligned_area = self.get_aligned_area(area, horizontal, vertical, offset_x, offset_y);
            let mouse_pos = Position::new(mouse_event.column, mouse_event.row);
    
            if !aligned_area.contains(mouse_pos) {
                return None;
            }
    
            let rel_x = mouse_pos.x.saturating_sub(aligned_area.x);
            let mut current_x = 0;
    
            for (idx, item) in self.items.iter().enumerate() {
                let item_width = self.get_item_width(item);
                if rel_x >= current_x && rel_x < current_x + item_width {
                    return Some(idx);
                }
                current_x += item_width;
                current_x += 1; // Separator
            }
            None
        }
    
        /// High-level helper for rendering a tabbed container with borders.
        ///>
        /// Automatically handles header/footer placement based on tab alignment
        /// and returns the inner area available for content rendering.
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
    
            //>
            for id in tab_ids {
                if let Some((tab_bar, horizontal, vertical, off_x, off_y)) =
                    Self::from_config(config, tabs, id)
                {
                    if vertical == TabBarAlignment::Top {
                        let [header, body] = tab_bar.split_layout(current_body_area);
                        current_body_area = body;
                        active_decals.push((tab_bar, horizontal, vertical, off_x, off_y, header));
                    } else if vertical == TabBarAlignment::Bottom {
                        let height = tab_bar.consumed_height();
                        let [body, footer] =
                            Layout::vertical([Constraint::Min(0), Constraint::Length(height)])
                                .areas(current_body_area);
                        current_body_area = body;
                        active_decals.push((tab_bar, horizontal, vertical, off_x, off_y, footer));
                    } else {
                        active_decals.push((
                            tab_bar,
                            horizontal,
                            vertical,
                            off_x,
                            off_y,
                            current_body_area,
                        ));
                    }
                }
            }
    
            Block::bordered().render(current_body_area, buf);
    
            for (widget, horizontal, vertical, off_x, off_y, header_area) in active_decals {
                widget.render_aligned(header_area, horizontal, vertical, off_x, off_y, buf);
            }
    
            Block::bordered().inner(current_body_area)
            //<
        }
    }
    
    /// Helper for parsing color names from configuration strings.
    ///>
    /// Supports standard ANSI color names and specific indexed colors for
    /// legacy compatibility with the original dev-console TUI.
    ///<
    fn parse_color(c: &str) -> Color {
        //>
        match c.to_lowercase().as_str() {
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
        }
        //<
    }
    
    impl<'a> Widget for TabBarWidget<'a> {
        /// Renders the tab bar into the provided buffer.
        ///>
        /// Selects the appropriate rendering strategy (single or multi-line) based
        /// on the active style and available vertical space.
        ///<
        fn render(self, area: Rect, buf: &mut Buffer) {
            //>
            if self.style == TabBarStyle::Tab && area.height >= 2 {
                buf.set_line(area.x, area.y, &self.build_top_line(), area.width);
                buf.set_line(area.x, area.y + 1, &self.build_tab_line(), area.width);
            } else {
                buf.set_line(area.x, area.y, &self.build_tab_line(), area.width);
            }
            //<
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        use crossterm::event::KeyModifiers;
        fn buffer_content(buffer: &Buffer) -> String {
        let mut result = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                result.push_str(buffer[(x, y)].symbol());
            }
            result.push('\n');
        }
        result
    }
    #[test]
    fn test_tab_bar_render_styles() {
        let items = vec![
            TabBarItem {
                id: "t1".to_string(),
                name: "Tab 1".to_string(),
                active: true,
            },
            TabBarItem {
                id: "t2".to_string(),
                name: "Tab 2".to_string(),
                active: false,
            },
        ];
        let area = Rect::new(0, 0, 30, 3);
        let styles = vec![
            TabBarStyle::Tab,
            TabBarStyle::Text,
            TabBarStyle::Boxed,
            TabBarStyle::BoxStatic,
            TabBarStyle::TextStatic,
        ];
        for style in styles {
            let mut buffer = Buffer::empty(area);
            let widget = TabBarWidget::new(&items).style(style);
            widget.render(area, &mut buffer);
            let s = buffer_content(&buffer);
            assert!(s.contains("Tab 1"));
        }
    }
    #[test]
    fn test_tab_bar_alignment_and_offset() {
        let items = vec![TabBarItem {
            id: "t1".to_string(),
            name: "T".to_string(),
            active: true,
        }];
        let widget = TabBarWidget::new(&items);
        let area = Rect::new(0, 0, 50, 10);
        let aligned =
            widget.get_aligned_area(area, TabBarAlignment::Center, TabBarAlignment::Top, 5, 2);
        assert_eq!(aligned.y, 2);
        assert!(aligned.x > 0);
    }
    #[test]
    fn test_tab_bar_mouse_interaction() {
        let items = vec![
            TabBarItem {
                id: "t1".to_string(),
                name: "T1".to_string(),
                active: true,
            },
            TabBarItem {
                id: "t2".to_string(),
                name: "T2".to_string(),
                active: false,
            },
        ];
        let widget = TabBarWidget::new(&items);
        let area = Rect::new(0, 0, 50, 5);
        let event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 2,
            row: 0,
            modifiers: KeyModifiers::empty(),
        };
        let hit = widget.handle_mouse_event(
            area,
            TabBarAlignment::Left,
            TabBarAlignment::Top,
            0,
            0,
            event,
        );
        assert_eq!(hit, Some(0));
    }
    #[test]
    fn test_parse_color() {
        assert_eq!(parse_color("red"), Color::Red);
        assert_eq!(parse_color("blue"), Color::Blue);
        assert_eq!(parse_color("dimgray"), Color::Indexed(240));
        assert_eq!(parse_color("unknown"), Color::White);
    }
    #[test]
    fn test_tab_bar_remaining_styles_and_alignment() {
        let items = vec![TabBarItem {
            id: "t1".to_string(),
            name: "T".to_string(),
            active: true,
        }];
        let area = Rect::new(0, 0, 20, 5);
        let mut buffer = Buffer::empty(area);
        let widget = TabBarWidget::new(&items).style(TabBarStyle::Boxed);
        widget.render_aligned(
            area,
            TabBarAlignment::Right,
            TabBarAlignment::Bottom,
            0,
            0,
            &mut buffer,
        );
        let widget = TabBarWidget::new(&items).style(TabBarStyle::BoxStatic);
        widget.render(area, &mut buffer);
        let widget = TabBarWidget::new(&items).style(TabBarStyle::TextStatic);
        let aligned =
            widget.get_aligned_area(area, TabBarAlignment::Center, TabBarAlignment::Top, -5, -2);
        assert!(aligned.x < area.width);
        widget.render(area, &mut buffer);
    }
    #[test]
    fn test_tab_bar_mouse_no_hit() {
        let items = vec![TabBarItem {
            id: "t1".to_string(),
            name: "T".to_string(),
            active: true,
        }];
        let widget = TabBarWidget::new(&items);
        let area = Rect::new(0, 0, 20, 5);
        let event = MouseEvent {
            kind: MouseEventKind::Moved,
            column: 1,
            row: 0,
            modifiers: KeyModifiers::empty(),
        };
        assert!(widget
            .handle_mouse_event(
                area,
                TabBarAlignment::Left,
                TabBarAlignment::Top,
                0,
                0,
                event
            )
            .is_none());
        let event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 50,
            row: 50,
            modifiers: KeyModifiers::empty(),
        };
        assert!(widget
            .handle_mouse_event(
                area,
                TabBarAlignment::Left,
                TabBarAlignment::Top,
                0,
                0,
                event
            )
            .is_none());
    }
    #[test]
    fn test_tab_bar_from_config() {
        let mut config = crate::config::Config::default();
        config.tab_bars.push(crate::config::TabBarConfig {
            id: "tb1".to_string(),
            style: Some(TabBarStyle::Boxed),
            alignment: crate::config::Alignment {
                vertical: Some(TabBarAlignment::Bottom),
                horizontal: Some(TabBarAlignment::Right),
                ..Default::default()
            },
            ..Default::default()
        });
        let items = vec![TabBarItem {
            id: "t1".to_string(),
            name: "T".to_string(),
            active: true,
        }];
        let (widget, horiz, vert, _, _) =
            TabBarWidget::from_config(&config, &items, "tb1").unwrap();
        assert_eq!(widget.style, TabBarStyle::Boxed);
        assert_eq!(horiz, TabBarAlignment::Right);
        assert_eq!(vert, TabBarAlignment::Bottom);
        assert!(TabBarWidget::from_config(&config, &items, "nonexistent").is_none());
    }
    #[test]
    fn test_render_composite() {
        let mut config = crate::config::Config::default();
        config.tab_bars.push(crate::config::TabBarConfig {
            id: "tb1".to_string(),
            alignment: crate::config::Alignment {
                vertical: Some(TabBarAlignment::Top),
                horizontal: Some(TabBarAlignment::Center),
                ..Default::default()
            },
            ..Default::default()
        });
        config.tab_bars.push(crate::config::TabBarConfig {
            id: "tb2".to_string(),
            alignment: crate::config::Alignment {
                vertical: Some(TabBarAlignment::Bottom),
                horizontal: Some(TabBarAlignment::Left),
                ..Default::default()
            },
            ..Default::default()
        });
        let items = vec![TabBarItem {
            id: "t1".to_string(),
            name: "T".to_string(),
            active: true,
        }];
        let area = Rect::new(0, 0, 50, 20);
        let mut buffer = Buffer::empty(area);
        let inner = TabBarWidget::render_composite(
            &config,
            &items,
            &["tb1", "tb2", "nonexistent"],
            area,
            &mut buffer,
        );
        assert!(inner.height < area.height);
    }
}
