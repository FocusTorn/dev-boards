use crate::widgets::{InteractiveWidget, WidgetOutcome};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Widget},
};

/// A generic modal container that centers its content and applies dimming.
#[derive(Debug)]
pub struct Popup<T> {
    pub content: T,
    pub title: String,
    pub width_percent: u16,
    pub height_percent: u16,
}

impl<T> Popup<T> {
    pub fn new(content: T, title: String) -> Self {
        Self {
            content,
            title,
            width_percent: 60,
            height_percent: 40,
        }
    }

    pub fn with_size(mut self, width: u16, height: u16) -> Self {
        self.width_percent = width;
        self.height_percent = height;
        self
    }

    /// Helper to calculate the centered rect.
    pub fn calculate_area(&self, area: Rect) -> Rect {
        let popup_width = area.width * self.width_percent / 100;
        let popup_height = area.height * self.height_percent / 100;

        Rect {
            x: area.x + (area.width.saturating_sub(popup_width)) / 2,
            y: area.y + (area.height.saturating_sub(popup_height)) / 2,
            width: popup_width,
            height: popup_height,
        }
    }

    pub fn handle_key<O>(&mut self, key: crossterm::event::KeyEvent) -> WidgetOutcome<O>
    where
        T: InteractiveWidget<Outcome = O>,
    {
        self.content.handle_key(key)
    }

    pub fn handle_mouse<O>(
        &mut self,
        mouse: crossterm::event::MouseEvent,
        parent_area: Rect,
    ) -> WidgetOutcome<O>
    where
        T: InteractiveWidget<Outcome = O>,
    {
        let popup_area = self.calculate_area(parent_area);
        if popup_area.contains(ratatui::layout::Position::new(mouse.column, mouse.row)) {
            // Adjust coordinates to be relative to the inner content area?
            // Actually, most widgets use absolute coordinates from Rect, so we pass absolute mouse but with the relevant area.
            let content_area = Rect {
                x: popup_area.x + 1,
                y: popup_area.y + 1,
                width: popup_area.width.saturating_sub(2),
                height: popup_area.height.saturating_sub(2),
            };
            self.content.handle_mouse(mouse, content_area)
        } else {
            // Clicked outside modal
            if let crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left) =
                mouse.kind
            {
                WidgetOutcome::Canceled
            } else {
                WidgetOutcome::None
            }
        }
    }
}

impl<T> Widget for Popup<T>
where
    for<'a> &'a T: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self).render(area, buf);
    }
}

impl<T> Widget for &Popup<T>
where
    for<'a> &'a T: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = self.calculate_area(area);

        // 1. Clear the area
        Clear.render(popup_area, buf);

        // 2. Create the block (Chrome)
        let block = Block::default()
            .title(format!(" {} ", self.title))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .style(Style::default().bg(Color::Indexed(234))); // Slightly lighter than DIM_BG (232) to pop

        // 3. Render the block
        block.render(popup_area, buf);

        // 4. Render the inner content
        let content_area = Rect {
            x: popup_area.x + 1,
            y: popup_area.y + 1,
            width: popup_area.width.saturating_sub(2),
            height: popup_area.height.saturating_sub(2),
        };
        (&self.content).render(content_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::widgets::Paragraph;

    #[test]
    fn test_popup_area_calculation() {
        let parent = Rect::new(0, 0, 100, 100);
        let popup = Popup::new(Paragraph::new("test"), "Title".to_string()).with_size(50, 50);

        let area = popup.calculate_area(parent);
        assert_eq!(area.width, 50);
        assert_eq!(area.height, 50);
        assert_eq!(area.x, 25);
        assert_eq!(area.y, 25);
    }

    #[derive(Debug)]
    struct MockContent;
    impl Widget for &MockContent {
        fn render(self, _: Rect, _: &mut Buffer) {}
    }
    impl Widget for MockContent {
        fn render(self, _: Rect, _: &mut Buffer) {}
    }
    impl InteractiveWidget for MockContent {
        type Outcome = ();
        fn handle_key(&mut self, _: crossterm::event::KeyEvent) -> WidgetOutcome<()> {
            WidgetOutcome::Confirmed(())
        }
        fn handle_mouse(&mut self, _: crossterm::event::MouseEvent, _: Rect) -> WidgetOutcome<()> {
            WidgetOutcome::None
        }
    }

    #[test]
    fn test_popup_handle_key_delegation() {
        let mut popup = Popup::new(MockContent, "Title".to_string());
        let key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Enter,
            crossterm::event::KeyModifiers::empty(),
        );
        let outcome = popup.handle_key(key);
        assert_eq!(outcome, WidgetOutcome::Confirmed(()));
    }
}
