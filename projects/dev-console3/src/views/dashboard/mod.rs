pub mod state;
pub mod view;
#[cfg(test)]
mod tests;

pub use state::Dashboard;
use crate::widgets::traits::{Component, WidgetOutcome};
use ratatui::prelude::*;
use color_eyre::Result;
use crossterm::event::{KeyEvent, KeyCode, MouseEvent};

impl Component for Dashboard {
    type Outcome = String;

    fn handle_key(&mut self, key: KeyEvent) -> Result<WidgetOutcome<Self::Outcome>> {
        if self.profile_dropdown.is_open {
            return self.profile_dropdown.handle_key(key);
        }

        match key.code {
            KeyCode::Enter => {
                if !self.profile_dropdown.is_open {
                    self.profile_dropdown.is_open = true;
                    return Ok(WidgetOutcome::Consumed);
                }
            }
            KeyCode::Up => {
                if self.focus_commands {
                    self.selected_command = if self.selected_command > 0 { self.selected_command - 1 } else { self.commands.len() - 1 };
                    return Ok(WidgetOutcome::Consumed);
                }
            }
            KeyCode::Down => {
                if self.focus_commands {
                    self.selected_command = (self.selected_command + 1) % self.commands.len();
                    return Ok(WidgetOutcome::Consumed);
                }
            }
            KeyCode::Left | KeyCode::Right => {
                self.focus_commands = !self.focus_commands;
                return Ok(WidgetOutcome::Consumed);
            }
            _ => {}
        }

        Ok(WidgetOutcome::None)
    }

    fn handle_mouse(&mut self, mouse: MouseEvent, area: Rect) -> Result<WidgetOutcome<Self::Outcome>> {
        // Calculate the same layout used in view.rs to find the dropdown anchor
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(30), Constraint::Min(0)])
            .split(area);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(6), Constraint::Min(0)])
            .split(main_chunks[0]);

        let dropdown_area = Rect::new(left_chunks[0].x, left_chunks[0].y, left_chunks[0].width, 3);

        // Delegate to dropdown
        self.profile_dropdown.handle_mouse(mouse, dropdown_area)
    }

    fn view(&mut self, f: &mut Frame, area: Rect) {
        view::render(self, f, area);
    }
}
