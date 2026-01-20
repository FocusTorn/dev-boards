// Custom Tab Bar Widget
// A flexible tab bar component that implements the Widget trait

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TabBarStyle {
    /// Curved brackets around active tab: ╭─────╮
    Tab,
    /// Plain text with separators: ─ TAB ─
    Text,
    /// Square brackets around active tab: [ TAB ]
    Boxed,
    /// Static boxed style: all tabs in brackets [ TAB ]─[ TAB ]
    BoxStatic,
    /// Static text style: all tabs as plain text ─ TAB ─ TAB
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

/// Custom Tab Bar Widget implementing the Widget trait
pub struct TabBarWidget<'a> {
    pub items: &'a [TabBarItem],
    pub style: TabBarStyle,
    pub alignment: TabBarAlignment,
    pub color: Color,
    pub area: Option<Rect>, // Parent constraint/bounding box area
}

impl<'a> TabBarWidget<'a> {
    pub fn new(items: &'a [TabBarItem]) -> Self {
        Self {
            items,
            style: TabBarStyle::Text,
            alignment: TabBarAlignment::Left,
            color: Color::White,
            area: None,
        }
    }

    pub fn style(mut self, style: TabBarStyle) -> Self {
        self.style = style;
        self
    }

    pub fn alignment(mut self, alignment: TabBarAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn area(mut self, area: Rect) -> Self {
        self.area = Some(area);
        self
    }

    /// Calculate the width needed for the tab bar
    fn estimate_width(&self) -> u16 {
        let mut width = 0u16;

        // Leading separator
        let first_is_active = self.items.first().map(|item| item.active && self.style == TabBarStyle::Tab).unwrap_or(false);
        let leading = match self.style {
            TabBarStyle::Tab => {
                if first_is_active {
                    "──" // No space, connects directly to ╯
                } else {
                    "── " // Space after for inactive tabs
                }
            }
            TabBarStyle::Text | TabBarStyle::Boxed | TabBarStyle::BoxStatic | TabBarStyle::TextStatic => {
                "── " // Text, Boxed, and static styles always have space after leading separator
            }
        };
        width += leading.chars().count() as u16;

        // Tab items and separators
        let mut prev_was_active = false;
        for (idx, item) in self.items.iter().enumerate() {
            if idx > 0 {
                // Skip separator if previous tab was active
                if !prev_was_active || self.style == TabBarStyle::BoxStatic || self.style == TabBarStyle::TextStatic {
                    let separator = match self.style {
                        TabBarStyle::Tab => {
                            if item.active {
                                " ─" // Space-dash, creates gap before ╯
                            } else {
                                " ─ " // Space before and after for inactive tabs
                            }
                        }
                        TabBarStyle::Boxed => {
                            if item.active {
                                " ─" // Space-dash, creates gap before [
                            } else {
                                " ─ " // Space before and after for inactive tabs
                            }
                        }
                        TabBarStyle::Text | TabBarStyle::TextStatic => {
                            " ─ " // Text and TextStatic styles always use consistent separators
                        }
                        TabBarStyle::BoxStatic => {
                            "─" // Just dash, connects to [ for static boxed style
                        }
                    };
                    width += separator.chars().count() as u16;
                }
            }

            // Tab text width
            match self.style {
                TabBarStyle::Tab => {
                    if item.active {
                        let text = format!("╯ {} ╰", item.name);
                        width += text.chars().count() as u16;
                    } else {
                        width += item.name.chars().count() as u16;
                    }
                }
                TabBarStyle::Boxed => {
                    if item.active {
                        let text = format!("[ {} ]", item.name);
                        width += text.chars().count() as u16;
                    } else {
                        width += item.name.chars().count() as u16;
                    }
                }
                TabBarStyle::BoxStatic => {
                    let text = format!("[ {} ]", item.name);
                    width += text.chars().count() as u16;
                }
                TabBarStyle::Text | TabBarStyle::TextStatic => {
                    width += item.name.chars().count() as u16;
                }
            }

            if self.style == TabBarStyle::Tab || self.style == TabBarStyle::Boxed {
                prev_was_active = item.active;
            }
        }

        // Trailing separator
        let last_is_active = match self.style {
            TabBarStyle::BoxStatic | TabBarStyle::TextStatic => false,
            _ => self.items.last().map(|item| item.active).unwrap_or(false),
        };
        let trailing = if last_is_active {
            "──" // No space needed if last tab is active
        } else {
            " ──" // Add space before trailing separator if last tab is inactive
        };
        width += trailing.chars().count() as u16;

        width
    }

    /// Build the tab line as spans
    fn build_tab_line(&self, max_width: u16) -> Line<'a> {
        let mut spans: Vec<Span<'a>> = Vec::new();
        let mut current_width = 0;

        let styled = |text: &str, style: Option<Style>| -> Span<'a> {
            Span::styled(text.to_string(), style.unwrap_or_else(|| Style::default().fg(self.color)))
        };

        // Leading separator
        let first_is_active = self.items.first().map(|item| item.active && self.style == TabBarStyle::Tab).unwrap_or(false);
        let leading_sep = match self.style {
            TabBarStyle::Tab => {
                if first_is_active { "──" } else { "── " }
            }
            _ => "── ",
        };
        spans.push(styled(leading_sep, None));
        current_width += leading_sep.chars().count() as u16;

        let mut prev_was_active = false;
        for (idx, item) in self.items.iter().enumerate() {
            if idx > 0 {
                if !prev_was_active || self.style == TabBarStyle::BoxStatic || self.style == TabBarStyle::TextStatic {
                    let separator = match self.style {
                        TabBarStyle::Tab if item.active => " ─",
                        TabBarStyle::Tab => " ─ ",
                        TabBarStyle::Boxed if item.active => " ─",
                        TabBarStyle::Boxed => " ─ ",
                        TabBarStyle::BoxStatic => "─",
                        _ => " ─ ",
                    };
                    if current_width + separator.chars().count() as u16 > max_width { break; }
                    spans.push(styled(separator, None));
                    current_width += separator.chars().count() as u16;
                }
            }

            let tab_text = match self.style {
                TabBarStyle::Tab if item.active => format!("╯ {} ╰", item.name),
                TabBarStyle::Boxed if item.active => format!("[ {} ]", item.name),
                TabBarStyle::BoxStatic => format!("[ {} ]", item.name),
                _ => item.name.clone(),
            };

            if current_width + tab_text.chars().count() as u16 > max_width { break; }
            let style = if item.active { Some(Style::default().fg(Color::Yellow)) } else { None };
            spans.push(styled(&tab_text, style));
            current_width += tab_text.chars().count() as u16;

            if self.style == TabBarStyle::Tab || self.style == TabBarStyle::Boxed {
                prev_was_active = item.active;
            }
        }

        let last_is_active = self.items.last().map_or(false, |item| item.active);
        let trailing_sep = match self.style {
            TabBarStyle::Tab | TabBarStyle::Boxed if last_is_active => "──",
            TabBarStyle::BoxStatic | TabBarStyle::TextStatic => " ──",
            _ => " ──",
        };
        if current_width + trailing_sep.chars().count() as u16 <= max_width {
            spans.push(styled(trailing_sep, None));
        }

        Line::from(spans)
    }

    fn calculate_area(&self, parent_area: Rect) -> Rect {
        if let Some(area) = self.area {
            area
        } else {
            let tab_bar_width = self.estimate_width();
            Rect {
                x: parent_area.x,
                y: parent_area.y,
                width: tab_bar_width.min(parent_area.width),
                height: 1,
            }
        }
    }
}

impl<'a> Widget for TabBarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let tab_area = self.calculate_area(area);
        if tab_area.width == 0 || tab_area.height == 0 {
            return;
        }

        let line = self.build_tab_line(tab_area.width);

        let start_x = match self.alignment {
            TabBarAlignment::Left => tab_area.x,
            TabBarAlignment::Center => {
                let line_width = line.width() as u16;
                if line_width < tab_area.width {
                    tab_area.x + (tab_area.width - line_width) / 2
                } else {
                    tab_area.x
                }
            }
            TabBarAlignment::Right => {
                let line_width = line.width() as u16;
                if line_width < tab_area.width {
                    tab_area.x + tab_area.width - line_width
                } else {
                    tab_area.x
                }
            }
        };

        buf.set_line(start_x, tab_area.y, &line, tab_area.width);
    }
}