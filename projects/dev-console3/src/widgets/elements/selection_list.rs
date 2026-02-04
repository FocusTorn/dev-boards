use ratatui::prelude::*;
use ratatui::widgets::Widget;

pub struct SelectionList<'a> {
    pub items: &'a [String],
    pub selected_index: usize,
    pub hovered_index: Option<usize>,
    pub highlight_style: Style,
    pub normal_style: Style,
}

impl<'a> SelectionList<'a> {
    pub fn new(items: &'a [String], selected_index: usize) -> Self {
        Self {
            items,
            selected_index,
            hovered_index: None,
            highlight_style: Style::default().fg(Color::Cyan).bg(Color::Rgb(0, 40, 40)),
            normal_style: Style::default().fg(Color::DarkGray),
        }
    }

    pub fn hovered(mut self, index: Option<usize>) -> Self {
        self.hovered_index = index;
        self
    }
}

impl<'a> Widget for SelectionList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (idx, item) in self.items.iter().enumerate() {
            if idx >= area.height as usize {
                break;
            }

            let item_y = area.y + idx as u16;
            let is_highlighted = if let Some(hovered) = self.hovered_index {
                idx == hovered
            } else {
                idx == self.selected_index
            };

            let style = if is_highlighted {
                self.highlight_style
            } else {
                self.normal_style
            };

            if is_highlighted {
                for x in area.left()..area.right() {
                    if let Some(cell) = buf.cell_mut((x, item_y)) {
                        cell.set_style(style);
                    }
                }
            }

            buf.set_string(area.x + 1, item_y, format!(" {}", item), style);
        }
    }
}
