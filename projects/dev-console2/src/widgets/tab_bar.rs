use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TabBarStyle {
    Tab,
    Text,
    Boxed,
    BoxStatic,
    TextStatic,
}

// This is not used by the widget itself but is useful for the app's logic.
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

/// A "decal" widget that renders only the unique parts of a tab bar.
/// It is intended to be rendered on top of an existing block border.
pub struct TabBarWidget<'a> {
    pub items: &'a [TabBarItem],
    pub style: TabBarStyle,
    pub color: Color,
}

impl<'a> TabBarWidget<'a> {
    pub fn new(items: &'a [TabBarItem]) -> Self {
        Self {
            items,
            style: TabBarStyle::Text,
            color: Color::White,
        }
    }

    pub fn style(mut self, style: TabBarStyle) -> Self {
        self.style = style;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Estimates the width of the minimal tab bar element (tabs + internal separators).
    pub fn estimate_width(&self) -> u16 {
        let mut width = 0;
        if self.items.is_empty() {
            return 0;
        }

        for (idx, item) in self.items.iter().enumerate() {
            if idx > 0 {
                width += 1; // Separator: "─"
            }

            if item.active {
                width += match self.style {
                    TabBarStyle::Tab => item.name.len() as u16 + 4,   // "╯ NAME ╰"
                    TabBarStyle::Boxed => item.name.len() as u16 + 4, // "[ NAME ]"
                    _ => item.name.len() as u16,
                };
            } else {
                width += item.name.len() as u16 + 2; // " NAME "
            }
        }
        width
    }



    fn build_tab_line(&self) -> Line<'a> { //> Builds the minimal tab line (e.g., "╯ Dashboard ╰─ Tab 2 ").
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
                    _ => spans.push(Span::styled(item.name.clone(), active_style)),
                }
            } else {
                spans.push(Span::styled(format!(" {} ", item.name), inactive_style));
            }
        }
        Line::from(spans)
    } //<

    
    
    /// Builds the minimal top line (e.g., "  ╭───────╮  ").
    fn build_top_line(&self) -> Line<'a> {
        if self.style != TabBarStyle::Tab {
            return Line::default();
        }

        if let Some(active_tab) = self.items.iter().find(|i| i.active) {
            let mut pre_width = 0;
            
            
            for item in self.items.iter() {
                if item.active {
                    break;
                }
                pre_width += item.name.len() as u16 + 2 + 1; // " NAME " + "─"
            }

            let active_width = active_tab.name.len() as u16 + 2; // " NAME "
            
            
            let top_bar = format!("╭{}╮", "─".repeat(active_width as usize));

            let mut spans = vec![
                Span::raw(" ".repeat(pre_width as usize)),
                Span::styled(top_bar, Style::default().fg(Color::White)),
            ];
            
            return Line::from(spans);
        }

        Line::default()
    }
}

impl<'a> Widget for TabBarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.style == TabBarStyle::Tab {
            if area.height < 2 { return; }
            let top_line = self.build_top_line();
            let tab_line = self.build_tab_line();
            buf.set_line(area.x, area.y, &top_line, area.width);
            buf.set_line(area.x, area.y + 1, &tab_line, area.width);
        } else {
            let tab_line = self.build_tab_line();
            buf.set_line(area.x, area.y, &tab_line, area.width);
        }
    }
}