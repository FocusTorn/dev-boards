pub mod state;
pub mod view;

pub use state::Profiles;
use crate::widgets::traits::{Component, WidgetOutcome};
use ratatui::prelude::*;
use color_eyre::Result;
use crossterm::event::{KeyEvent, KeyCode};

impl Component for Profiles {
    type Outcome = String;

    fn handle_key(&mut self, key: KeyEvent) -> Result<WidgetOutcome<Self::Outcome>> {
        // 1. Dropdown Priority
        if self.port_dropdown.is_open {
            return self.port_dropdown.handle_key(key);
        }
        if self.baud_dropdown.is_open {
            return self.baud_dropdown.handle_key(key);
        }

        // 2. Local Navigation
        match key.code {
            KeyCode::Up => {
                self.selected_field = self.selected_field.saturating_sub(1);
                Ok(WidgetOutcome::Consumed)
            }
            KeyCode::Down => {
                self.selected_field = (self.selected_field + 1).min(1);
                Ok(WidgetOutcome::Consumed)
            }
            KeyCode::Enter => {
                if self.selected_field == 0 {
                    self.port_dropdown.is_open = true;
                } else {
                    self.baud_dropdown.is_open = true;
                }
                Ok(WidgetOutcome::Consumed)
            }
            _ => Ok(WidgetOutcome::None),
        }
    }

    fn view(&mut self, f: &mut Frame, area: Rect) {
        view::render(self, f, area);
    }
}
