use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget, Wrap},
};

/// A simple bordered box for displaying the application's global status.
///>
/// Dynamically adjusts its border and text color to red if an "[Error]"
/// prefix is detected in the status message, providing immediate visual
/// feedback for failure states.
///<
pub struct StatusBoxWidget<'a> {
    status_text: &'a str,
}

impl<'a> StatusBoxWidget<'a> {
    /// Creates a new status box with the provided text.
    pub fn new(status_text: &'a str) -> Self {
        Self { status_text }
    }
}

impl<'a> Widget for StatusBoxWidget<'a> {
    /// Renders the status box with contextual color coding.
    ///>
    /// Detects error states by string prefix and applies high-visibility
    /// styles to the borders and text content.
    ///<
    fn render(self, area: Rect, buf: &mut Buffer) {
        let is_error = self.status_text.starts_with("[Error]");

        let border_color = if is_error {
            //>
            Color::Red
        } else {
            Color::White
        }; //<

        let block = Block::bordered()
            .title(" Status ")
            .border_style(Style::default().fg(border_color))
            .title_style(Style::default().fg(border_color));

        let inner_area = block.inner(area);
        block.render(area, buf);

        let text_area = inner_area.inner(ratatui::layout::Margin {
            vertical: 0,
            horizontal: 1,
        });

        let style = if is_error {
            //>
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        }; //<
        let paragraph = Paragraph::new(self.status_text)
            .style(style)
            .wrap(Wrap { trim: true });

        paragraph.render(text_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn buffer_to_string(buffer: &Buffer) -> String {
        let mut result = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                result.push_str(&buffer[(x, y)].symbol());
            }
            result.push('\n');
        }
        result
    }

    #[test]
    fn test_status_box_render_normal() {
        let backend = TestBackend::new(20, 3);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let widget = StatusBoxWidget::new("Ready");
                f.render_widget(widget, f.area());
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        let s = buffer_to_string(buffer);
        assert!(s.contains("Status"));
        assert!(s.contains("Ready"));

        // Verify border characters (approximate)
        assert!(s.contains("┌"));
        assert!(s.contains("┐"));
    }

    #[test]
    fn test_status_box_render_error() {
        let backend = TestBackend::new(25, 3);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let widget = StatusBoxWidget::new("[Error] Failed");
                f.render_widget(widget, f.area());
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        let s = buffer_to_string(buffer);
        assert!(s.contains("Status"));
        assert!(s.contains("[Error] Failed"));

        // Verify color (this is harder to check in string, but we can check buffer directly)
        let cell = &buffer[(1, 0)]; // Title " Status " part
        assert_eq!(cell.fg, Color::Red);
    }
}
