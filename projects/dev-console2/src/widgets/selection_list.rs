use ratatui::{
    buffer::Buffer,
    layout::{Rect, Position},
    style::{Color, Style},
    widgets::{Widget},
};
use crossterm::event::{MouseEvent, MouseButton, MouseEventKind, KeyModifiers};

/// Semantic result of a mouse interaction with the selection list.
pub enum SelectionListInteraction {
    Click(usize),
    Hover(usize),
}

/// A general-purpose list widget for selecting items.
/// 
/// Unlike CommandListWidget, this does not render its own borders or title.
pub struct SelectionListWidget<'a> {
    items: &'a [String],
    selected_index: usize,
    hovered_index: Option<usize>,
    highlight_style: Style,
    normal_style: Style,
}

impl<'a> SelectionListWidget<'a> {
    pub fn new(items: &'a [String], selected_index: usize, hovered_index: Option<usize>) -> Self {
        Self {
            items,
            selected_index,
            hovered_index,
            highlight_style: Style::default().fg(Color::Cyan).bg(Color::Rgb(0, 40, 40)),
            normal_style: Style::default().fg(Color::DarkGray),
        }
    }

    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    pub fn normal_style(mut self, style: Style) -> Self {
        self.normal_style = style;
        self
    }

    pub fn handle_mouse_event(&self, area: Rect, mouse_event: MouseEvent) -> Option<SelectionListInteraction> {
        let mouse_pos = Position::new(mouse_event.column, mouse_event.row);
        
        if !area.contains(mouse_pos) {
            return None;
        }

        let rel_y = mouse_event.row.saturating_sub(area.y) as usize;
        
        if rel_y < self.items.len() {
            match mouse_event.kind {
                MouseEventKind::Down(MouseButton::Left) => Some(SelectionListInteraction::Click(rel_y)),
                MouseEventKind::Moved | MouseEventKind::Drag(MouseButton::Left) => Some(SelectionListInteraction::Hover(rel_y)),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl<'a> Widget for SelectionListWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (idx, item) in self.items.iter().enumerate() {
            if idx >= area.height as usize { break; }
            
            let item_y = area.y + idx as u16;
            let is_selected = idx == self.selected_index;
            let is_hovered = self.hovered_index == Some(idx);
            
            let is_highlighted = if self.hovered_index.is_some() {
                is_hovered
            } else {
                is_selected
            };

            let style = if is_highlighted {
                self.highlight_style
            } else {
                self.normal_style
            };

            if is_highlighted {
                for x in area.left()..area.right() {
                    buf[(x, item_y)].set_style(style);
                }
            }

            buf.set_string(area.x + 1, item_y, format!(" {}", item), style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_selection_list_render_no_borders() {
        let backend = TestBackend::new(20, 5);
        let mut terminal = Terminal::new(backend).unwrap();
        let items = vec!["Item 1".to_string(), "Item 2".to_string()];
        
        terminal.draw(|f| {
            let widget = SelectionListWidget::new(&items, 0, None);
            f.render_widget(widget, f.area());
        }).unwrap();

        let buffer = terminal.backend().buffer();
        // Verify no "┌" or "Commands" title
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                let symbol = buffer[(x, y)].symbol();
                assert_ne!(symbol, "┌");
                assert_ne!(symbol, "┐");
            }
        }
        
        let s = format!("{:?}", buffer); // Simplified check
        assert!(s.contains("Item 1"));
        assert!(s.contains("Item 2"));
    }

    #[test]
    fn test_selection_list_mouse_interaction() {
        let items = vec!["Item 1".to_string()];
        let widget = SelectionListWidget::new(&items, 0, None);
        let area = Rect::new(0, 0, 20, 5);
        
        // Click at row 0
        let event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 5,
            row: 0,
            modifiers: KeyModifiers::empty(),
        };
        let interaction = widget.handle_mouse_event(area, event).unwrap();
        if let SelectionListInteraction::Click(idx) = interaction {
            assert_eq!(idx, 0);
        } else { panic!("Expected Click"); }
    }
}
