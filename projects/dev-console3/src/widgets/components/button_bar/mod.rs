pub mod state;
pub mod view;

pub use state::{ButtonBar, ButtonBarConfig, ButtonBarStyle, ButtonItem};
use crate::widgets::traits::{Component, WidgetOutcome};
use ratatui::prelude::*;
use color_eyre::Result;
use crossterm::event::{MouseEvent, MouseEventKind, MouseButton};

impl Component for ButtonBar {
    type Outcome = String;

    fn handle_mouse(&mut self, mouse: MouseEvent, area: Rect) -> Result<WidgetOutcome<Self::Outcome>> {
        if !matches!(mouse.kind, MouseEventKind::Down(MouseButton::Left)) {
            return Ok(WidgetOutcome::None);
        }

        let aligned_area = self.get_aligned_area(area);
        if !aligned_area.contains(Position::new(mouse.column, mouse.row)) {
            return Ok(WidgetOutcome::None);
        }

        let rel_x = mouse.column.saturating_sub(aligned_area.x);
        let mut current_x = 0;
        for item in &self.items {
            let item_width = self.get_item_width(item);
            if rel_x >= current_x && rel_x < current_x + item_width {
                return Ok(WidgetOutcome::Confirmed(item.id.clone()));
            }
            current_x += item_width + 1;
        }

        Ok(WidgetOutcome::None)
    }

    fn view(&mut self, f: &mut Frame, area: Rect) {
        view::render(self, f, area);
    }
}