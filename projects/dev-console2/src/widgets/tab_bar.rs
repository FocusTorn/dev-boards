

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Spacing},
    style::{Style, Color, Modifier},
    symbols::merge::MergeStrategy,
    widgets::{Block, Widget},
    text::{Line, Span}
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TabBarStyle {
    Tab,
    Text,
    Boxed,
    BoxStatic,
    TextStatic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TabBarAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub struct TabBarItem {
    pub name: String,
    pub active: bool,
}

pub struct TabBarWidget<'a> {
    pub items: &'a [TabBarItem],
    pub style: TabBarStyle,
    pub color: Color,
}

impl<'a> TabBarWidget<'a> {
    pub fn new(items: &'a [TabBarItem]) -> Self { //>
        Self {
            items,
            style: TabBarStyle::Text,
            color: Color::White,
        }
    } //<

    pub fn style(mut self, style: TabBarStyle) -> Self { //>
        self.style = style;
        self
    } //<

    pub fn color(mut self, color: Color) -> Self { //>
        self.color = color;
        self
    } //<

    /// Public helper: Returns the height required by the current style
    pub fn desired_height(&self) -> u16 { //>
        if self.style == TabBarStyle::Tab { 2 } else { 1 }
    } //<

    /// Public helper: Splits an area into [header, body] with the correct 1-line overlap
    pub fn split_layout(&self, area: Rect) -> [Rect; 2] { //>
        Layout::vertical([
            Constraint::Length(self.desired_height()),
            Constraint::Min(0),
        ])
        .spacing(Spacing::Overlap(1))
        .areas(area)
    } //<

    /// Public helper: Consolidates config lookup, style mapping, and alignment mapping
    pub fn from_config( config: &'a crate::config::Config, tabs: &'a [TabBarItem], id: &str, ) -> Option<(Self, TabBarAlignment)> { //>
        let tab_config = config.tab_bars.iter().find(|t| t.id == id)?;

        let style = match tab_config.style.as_deref() {
            Some("tabbed") => TabBarStyle::Tab,
            Some("boxed") => TabBarStyle::Boxed,
            _ => TabBarStyle::Text,
        };

        let alignment = match tab_config.alignment.horizontal.as_deref() {
            Some("center") => TabBarAlignment::Center,
            Some("right") => TabBarAlignment::Right,
            _ => TabBarAlignment::Left,
        };

        let color = match tab_config.color.as_deref() {
            Some("cyan") => Color::Cyan,
            _ => Color::White,
        };

        Some((Self::new(tabs).style(style).color(color), alignment))
    } //<

    pub fn estimate_width(&self) -> u16 { //>
        if self.items.is_empty() { return 0; }
        let mut width = 0;
        for (idx, item) in self.items.iter().enumerate() {
            if idx > 0 { width += 1; } // Separator "─"
            width += match (item.active, self.style) {
                (true, TabBarStyle::Tab) | (true, TabBarStyle::Boxed) => item.name.len() as u16 + 4,
                _ => item.name.len() as u16 + 2,
            };
        }
        width
    } //<

    fn build_tab_line(&self) -> Line<'a> { //>
        let mut spans = Vec::new();
        let active_style = Style::default().fg(self.color).add_modifier(Modifier::BOLD);
        let inactive_style = Style::default().fg(Color::White);

        for (idx, item) in self.items.iter().enumerate() {
            if idx > 0 {
                spans.push(Span::styled("─", inactive_style));
            }
            if item.active {
                match self.style {
                    TabBarStyle::Tab => {
                        spans.push(Span::styled("╯ ", inactive_style));
                        spans.push(Span::styled(item.name.clone(), active_style));
                        spans.push(Span::styled(" ╰", inactive_style));
                    }
                    TabBarStyle::Boxed => {
                        spans.push(Span::styled("[ ", inactive_style));
                        spans.push(Span::styled(item.name.clone(), active_style));
                        spans.push(Span::styled(" ]", inactive_style));
                    }
                    _ => spans.push(Span::styled(format!(" {} ", item.name), active_style)),
                }
            } else {
                spans.push(Span::styled(format!(" {} ", item.name), inactive_style));
            }
        }
        Line::from(spans)
    } //<

    fn build_top_line(&self) -> Line<'a> { //>
        if self.style != TabBarStyle::Tab { return Line::default(); }
        if let Some(active_idx) = self.items.iter().position(|i| i.active) {
            let mut pre_width = 0;
            for i in 0..active_idx {
                pre_width += self.items[i].name.len() as u16 + 2 + 1;
            }
            let active_width = self.items[active_idx].name.len() as u16 + 2;
            let top_bar = format!("╭{}╮", "─".repeat(active_width as usize));
            return Line::from(vec![
                Span::raw(" ".repeat(pre_width as usize)),
                Span::styled(top_bar, Style::default().fg(Color::White)),
            ]);
        }
        Line::default()
    } //<

    pub fn render_aligned(self, area: Rect, alignment: TabBarAlignment, buf: &mut Buffer) { //>
        let width = self.estimate_width();

        let x = match alignment {
            TabBarAlignment::Left => area.x + 1,
            TabBarAlignment::Center => area.x + (area.width.saturating_sub(width)) / 2,
            TabBarAlignment::Right => area.x + area.width.saturating_sub(width).saturating_sub(1),
        };

        // Use the area.y passed in; split_layout handles providing the correct Y
        let tab_area = Rect {
            x,
            y: area.y,
            width: width.min(area.width),
            height: self.desired_height().min(area.height),
        };

        self.render(tab_area, buf);
    } //<

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
        for id in tab_ids {
            if let Some((tab_bar, alignment)) = Self::from_config(config, tabs, id) {
                let [header, body] = tab_bar.split_layout(current_body_area);
                current_body_area = body;
                active_decals.push((tab_bar, alignment, header));
            }
        }

        // 2. Render the primary Block (the "Parent")
        // The block is rendered into the final body area
        Block::bordered()
            .merge_borders(MergeStrategy::Exact)
            .render(current_body_area, buf);

        // 3. Render the tab bar decals over the borders
        for (widget, alignment, header_area) in active_decals {
            widget.render_aligned(header_area, alignment, buf);
        }

        // 4. Return the inner area (padding handled by block automatically)
        Block::bordered().inner(current_body_area)
    }



}

impl<'a> Widget for TabBarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.style == TabBarStyle::Tab && area.height >= 2 {
            buf.set_line(area.x, area.y, &self.build_top_line(), area.width);
            buf.set_line(area.x, area.y + 1, &self.build_tab_line(), area.width);
        } else {
            buf.set_line(area.x, area.y, &self.build_tab_line(), area.width);
        }
    }
}
