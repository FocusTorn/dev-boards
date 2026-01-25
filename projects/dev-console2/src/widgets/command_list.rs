use ratatui::{
    buffer::Buffer,
    layout::{Rect, Position},
    style::{Color, Modifier, Style},
    widgets::{Block, Widget},
};
use crossterm::event::{MouseEvent, MouseButton, MouseEventKind};

pub struct CommandListWidget<'a> {
    commands: &'a [String],
    selected_index: usize,
    hovered_index: Option<usize>,
}

impl<'a> CommandListWidget<'a> {
    pub fn new(commands: &'a [String], selected_index: usize, hovered_index: Option<usize>) -> Self {
        Self { commands, selected_index, hovered_index }
    }

    /// Handles mouse events and returns either a Click(index) or Hover(index)
    pub fn handle_mouse_event(&self, area: Rect, mouse_event: MouseEvent) -> Option<CommandListInteraction> {
        let mouse_pos = Position::new(mouse_event.column, mouse_event.row);
        
        if !area.contains(mouse_pos) {
            return None;
        }

        let inner_area = Block::bordered().inner(area);
        let rel_y = mouse_event.row.saturating_sub(inner_area.y) as usize;
        
        if rel_y < self.commands.len() && inner_area.contains(mouse_pos) {
            match mouse_event.kind {
                MouseEventKind::Down(MouseButton::Left) => Some(CommandListInteraction::Click(rel_y)),
                MouseEventKind::Moved | MouseEventKind::Drag(MouseButton::Left) => Some(CommandListInteraction::Hover(rel_y)),
                _ => None,
            }
        } else {
            None
        }
    }
}

pub enum CommandListInteraction {
    Click(usize),
    Hover(usize),
}

impl<'a> Widget for CommandListWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title(" Commands ");
        let inner_area = block.inner(area);
        block.render(area, buf);

        for (idx, cmd) in self.commands.iter().enumerate() {
            if idx >= inner_area.height as usize { break; }
            
            let item_y = inner_area.y + idx as u16;
            let item_area = Rect::new(inner_area.x, item_y, inner_area.width, 1);
            
            let is_selected = idx == self.selected_index;
            let is_hovered = self.hovered_index == Some(idx);
            
            // Requirement: "highlight follows the mouse... return to the last selected item"
            // "no grey remnant, and same coloration for the highlighted item"
            // The effective highlighted row is the hover, or the selection if nothing is hovered.
            let is_highlighted = if self.hovered_index.is_some() {
                is_hovered
            } else {
                is_selected
            };

            let style = if is_highlighted {
                Style::default()
                    .fg(Color::Cyan)
                    .bg(Color::Rgb(0, 40, 40)) // Very dim cyan bg
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            if is_highlighted {
                // Fill background border to border
                for x in inner_area.left()..inner_area.right() {
                    buf[(x, item_y)].set_style(style);
                }
            }

            buf.set_string(item_area.x + 1, item_y, format!(" {}", cmd), style);
        }
    }
}
