// use ratatui::{
//     buffer::Buffer,
//     layout::Rect,
//     style::{Color, Modifier, Style},
//     text::{Line, Span},
//     widgets::Widget,
// };

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum TabBarStyle {
//     Tab,
//     Text,
//     Boxed,
//     BoxStatic,
//     TextStatic,
// }

// // This is not used by the widget itself but is useful for the app's logic.
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum TabBarAlignment {
//     Left,
//     Center,
//     Right,
// }

// #[derive(Debug, Clone)]
// pub struct TabBarItem {
//     pub name: String,
//     pub active: bool,
// }

// /// A "decal" widget that renders only the unique parts of a tab bar.
// /// It is intended to be rendered on top of an existing block border.
// pub struct TabBarWidget<'a> {
//     pub items: &'a [TabBarItem],
//     pub style: TabBarStyle,
//     pub color: Color,
// }

// impl<'a> TabBarWidget<'a> {
//     pub fn new(items: &'a [TabBarItem]) -> Self {
//         Self {
//             items,
//             style: TabBarStyle::Text,
//             color: Color::White,
//         }
//     }

//     pub fn style(mut self, style: TabBarStyle) -> Self {
//         self.style = style;
//         self
//     }

//     pub fn color(mut self, color: Color) -> Self {
//         self.color = color;
//         self
//     }

//     /// Estimates the width of the minimal tab bar element (tabs + internal separators).
//     pub fn estimate_width(&self) -> u16 {
//         let mut width = 0;
//         if self.items.is_empty() {
//             return 0;
//         }

//         for (idx, item) in self.items.iter().enumerate() {
//             if idx > 0 {
//                 width += 1; // Separator: "─"
//             }

//             if item.active {
//                 width += match self.style {
//                     TabBarStyle::Tab => item.name.len() as u16 + 4,   // "╯ NAME ╰"
//                     TabBarStyle::Boxed => item.name.len() as u16 + 4, // "[ NAME ]"
//                     _ => item.name.len() as u16,
//                 };
//             } else {
//                 width += item.name.len() as u16 + 2; // " NAME "
//             }
//         }
//         width
//     }



//     fn build_tab_line(&self) -> Line<'a> { //> Builds the minimal tab line (e.g., "╯ Dashboard ╰─ Tab 2 ").
//         let mut spans = Vec::new();
//         let active_style = Style::default().fg(self.color).add_modifier(Modifier::BOLD);
//         let inactive_style = Style::default().fg(Color::White);

//         for (idx, item) in self.items.iter().enumerate() {
//             if idx > 0 {
//                 spans.push(Span::styled("─", inactive_style));
//             }

//             if item.active {
//                 match self.style {
//                     TabBarStyle::Tab => {
//                         spans.push(Span::styled("╯ ", inactive_style));
//                         spans.push(Span::styled(item.name.clone(), active_style));
//                         spans.push(Span::styled(" ╰", inactive_style));
//                     }
//                     TabBarStyle::Boxed => {
//                         spans.push(Span::styled("[ ", inactive_style));
//                         spans.push(Span::styled(item.name.clone(), active_style));
//                         spans.push(Span::styled(" ]", inactive_style));
//                     }
//                     _ => spans.push(Span::styled(item.name.clone(), active_style)),
//                 }
//             } else {
//                 spans.push(Span::styled(format!(" {} ", item.name), inactive_style));
//             }
//         }
//         Line::from(spans)
//     } //<

    
    
//     /// Builds the minimal top line (e.g., "  ╭───────╮  ").
//     fn build_top_line(&self) -> Line<'a> {
//         if self.style != TabBarStyle::Tab {
//             return Line::default();
//         }

//         if let Some(active_tab) = self.items.iter().find(|i| i.active) {
//             let mut pre_width = 0;
            
            
//             for item in self.items.iter() {
//                 if item.active {
//                     break;
//                 }
//                 pre_width += item.name.len() as u16 + 2 + 1; // " NAME " + "─"
//             }

//             let active_width = active_tab.name.len() as u16 + 2; // " NAME "
            
            
//             let top_bar = format!("╭{}╮", "─".repeat(active_width as usize));

//             let mut spans = vec![
//                 Span::raw(" ".repeat(pre_width as usize)),
//                 Span::styled(top_bar, Style::default().fg(Color::White)),
//             ];
            
//             return Line::from(spans);
//         }

//         Line::default()
//     }
    
    
    
    
    
    
    
//     /// Helper to determine height based on style
//     pub fn desired_height(&self) -> u16 {
//         if self.style == TabBarStyle::Tab { 2 } else { 1 }
//     }

//     /// Renders the tab bar aligned within an area, intended to overlap a block's top border
//     pub fn render_aligned(self, area: Rect, alignment: TabBarAlignment, buf: &mut Buffer) {
//         let width = self.estimate_width();
//         let height = self.desired_height();
        
//         let x = match alignment {
//             TabBarAlignment::Left => area.x + 1,
//             TabBarAlignment::Center => area.x + (area.width.saturating_sub(width)) / 2,
//             TabBarAlignment::Right => area.x + area.width.saturating_sub(width).saturating_sub(1),
//         };

//         // We only render into the specific calculated slice
//         let tab_area = Rect {
//             x,
//             y: area.y,
//             width: width.min(area.width),
//             height: height.min(area.height),
//         };

//         self.render(tab_area, buf);
//     }
    
    
    
    
    
    
// }

// impl<'a> Widget for TabBarWidget<'a> {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         if self.style == TabBarStyle::Tab {
//             if area.height < 2 { return; }
//             let top_line = self.build_top_line();
//             let tab_line = self.build_tab_line();
//             buf.set_line(area.x, area.y, &top_line, area.width);
//             buf.set_line(area.x, area.y + 1, &tab_line, area.width);
//         } else {
//             let tab_line = self.build_tab_line();
//             buf.set_line(area.x, area.y, &tab_line, area.width);
//         }
//     }
// }








use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Spacing},
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

    /// Public helper: Returns the height required by the current style
    pub fn desired_height(&self) -> u16 {
        if self.style == TabBarStyle::Tab { 2 } else { 1 }
    }

    /// Public helper: Splits an area into [header, body] with the correct 1-line overlap
    pub fn split_layout(&self, area: Rect) -> [Rect; 2] {
        Layout::vertical([
            Constraint::Length(self.desired_height()),
            Constraint::Min(0),
        ])
        .spacing(Spacing::Overlap(1))
        .areas(area)
    }

    /// Public helper: Consolidates config lookup, style mapping, and alignment mapping
    pub fn from_config(
        config: &'a crate::config::Config,
        tabs: &'a [TabBarItem],
        id: &str,
    ) -> Option<(Self, TabBarAlignment)> {
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
    }

    pub fn estimate_width(&self) -> u16 {
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
    }

    fn build_tab_line(&self) -> Line<'a> {
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
    }

    fn build_top_line(&self) -> Line<'a> {
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
    }

    /// Renders the widget with alignment logic applied to a specific buffer
    // pub fn render_aligned(self, area: Rect, alignment: TabBarAlignment, buf: &mut Buffer) {
    //     let width = self.estimate_width();
    //     let x = match alignment {
    //         TabBarAlignment::Left => area.x + 1,
    //         TabBarAlignment::Center => area.x + (area.width.saturating_sub(width)) / 2,
    //         TabBarAlignment::Right => area.x + area.width.saturating_sub(width).saturating_sub(1),
    //     };
    //     let tab_area = Rect { x, y: area.y, width: width.min(area.width), height: self.desired_height().min(area.height) };
    //     self.render(tab_area, buf);
    // }
    
    pub fn render_aligned(self, area: Rect, alignment: TabBarAlignment, buf: &mut Buffer) {
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
