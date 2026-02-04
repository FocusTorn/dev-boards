pub mod state;
pub mod view;
#[cfg(test)]
mod tests;

pub use state::OverlayDropdown;
use crate::widgets::traits::{Component, WidgetOutcome};
use ratatui::prelude::*;
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind, MouseButton};

impl Component for OverlayDropdown {
    type Outcome = String;

    fn handle_key(&mut self, key: KeyEvent) -> Result<WidgetOutcome<Self::Outcome>> {
        if !self.is_open {
            if key.code == KeyCode::Enter || key.code == KeyCode::Down {
                self.is_open = true;
                return Ok(WidgetOutcome::Consumed);
            }
            return Ok(WidgetOutcome::None);
        }

        match key.code {
            KeyCode::Up => {
                let i = if self.selected == 0 {
                    self.items.len() - 1
                } else {
                    self.selected - 1
                };
                self.selected = i;
                self.list_state.select(Some(i));
                self.scroll_state = self.scroll_state.position(i.saturating_sub(self.max_shown / 2));
                Ok(WidgetOutcome::Changed(self.items[i].clone()))
            }
            KeyCode::Down => {
                let i = if self.selected >= self.items.len() - 1 {
                    0
                } else {
                    self.selected + 1
                };
                self.selected = i;
                self.list_state.select(Some(i));
                self.scroll_state = self.scroll_state.position(i.saturating_sub(self.max_shown / 2));
                Ok(WidgetOutcome::Changed(self.items[i].clone()))
            }
            KeyCode::Enter => {
                self.is_open = false;
                Ok(WidgetOutcome::Confirmed(self.items[self.selected].clone()))
            }
            KeyCode::Esc => {
                self.is_open = false;
                Ok(WidgetOutcome::Canceled)
            }
            _ => Ok(WidgetOutcome::None),
        }
    }

    fn handle_mouse(&mut self, mouse: MouseEvent, area: Rect) -> Result<WidgetOutcome<Self::Outcome>> {
        if !matches!(mouse.kind, MouseEventKind::Down(MouseButton::Left)) {
            return Ok(WidgetOutcome::None);
        }

        let mouse_pos = Position::new(mouse.column, mouse.row);

        if !self.is_open {
            // Check if click is within the anchor area
            if area.contains(mouse_pos) {
                self.is_open = true;
                return Ok(WidgetOutcome::Consumed);
            }
            return Ok(WidgetOutcome::None);
        }

        // Dropdown is OPEN
        // We use a large virtual height for the click-test to ensure we detect the expansion
        let (total_area, _) = self.calculate_layout(area, 500); 

        if !total_area.contains(mouse_pos) {
            self.is_open = false;
            return Ok(WidgetOutcome::Canceled);
        }

        // Logic for clicking items
        self.is_open = false;
        Ok(WidgetOutcome::Confirmed(self.items[self.selected].clone()))
    }

    fn view(&mut self, f: &mut Frame, area: Rect) {
        view::render(self, f, area);
    }
}