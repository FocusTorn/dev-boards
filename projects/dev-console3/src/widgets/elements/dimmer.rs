use ratatui::prelude::*;
use ratatui::widgets::Widget;

pub struct Dimmer;

impl Widget for Dimmer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_style(Style::default().fg(Color::DarkGray).bg(Color::Black));
                }
            }
        }
    }
}
