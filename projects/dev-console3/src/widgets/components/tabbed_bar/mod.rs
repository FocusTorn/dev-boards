pub mod state;
pub mod view;

pub use state::{TabbedBar, TabbedBarConfig, TabItem};
use crate::widgets::traits::{Component, WidgetOutcome};
use ratatui::prelude::*;
use color_eyre::Result;
use crossterm::event::{MouseEvent, MouseEventKind, MouseButton};

impl Component for TabbedBar {
    type Outcome = String;

    fn handle_mouse(&mut self, mouse: MouseEvent, area: Rect) -> Result<WidgetOutcome<Self::Outcome>> {
        if !matches!(mouse.kind, MouseEventKind::Down(MouseButton::Left)) {
            return Ok(WidgetOutcome::None);
        }

        let aligned_area = self.get_aligned_area(area);
        if !aligned_area.contains(Position::new(mouse.column, mouse.row)) {
            return Ok(WidgetOutcome::None);
        }

        let label_row = aligned_area.y + 1; // Always y+1 for Tab style

        if mouse.row != label_row { return Ok(WidgetOutcome::None); }

        let rel_x = mouse.column.saturating_sub(aligned_area.x);
        let mut current_x = 0;
        let mut clicked_id = None;
        for item in &self.items {
            let item_width = self.get_item_width(item);
            if rel_x >= current_x && rel_x < current_x + item_width {
                clicked_id = Some(item.id.clone());
                break;
            }
            current_x += item_width + 1;
        }

        if let Some(id) = clicked_id {
            self.set_active(&id);
            return Ok(WidgetOutcome::Confirmed(id));
        }

        Ok(WidgetOutcome::None)
    }

    fn view(&mut self, f: &mut Frame, area: Rect) {
        view::render_into(self, area, f.buffer_mut());
    }
}

impl TabbedBar {
    pub fn render_integrated(&self, area: Rect, buf: &mut Buffer, block: ratatui::widgets::Block) -> Rect {
        view::render_integrated(self, area, buf, block)
    }
}