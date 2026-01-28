use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect, Position},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};
use crate::app::theme::Theme;
use crate::widgets::smooth_scrollbar::{ScrollBar, ScrollLengths};

/// A widget for displaying the application's output log with scrolling and input support.
pub struct OutputBoxWidget<'a> {
    lines: &'a [Line<'a>],
    scroll: u16,
    autoscroll: bool,
    theme: &'a Theme,
    input_active: bool,
    input_value: &'a str,
    input_cursor: usize,
}

impl<'a> OutputBoxWidget<'a> {
    pub fn new(lines: &'a [Line<'a>], scroll: u16, theme: &'a Theme) -> Self {
        Self {
            lines,
            scroll,
            autoscroll: true,
            theme,
            input_active: false,
            input_value: "",
            input_cursor: 0,
        }
    }

    pub fn autoscroll(mut self, autoscroll: bool) -> Self {
        self.autoscroll = autoscroll;
        self
    }

    pub fn input(mut self, active: bool, value: &'a str, cursor: usize) -> Self {
        self.input_active = active;
        self.input_value = value;
        self.input_cursor = cursor;
        self
    }
}

impl<'a> Widget for OutputBoxWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let output_block = Block::bordered()
            .title(Span::styled(" Output ", self.theme.style("output_title")))
            .border_style(self.theme.style("output_border"));
        let inner_output_area = output_block.inner(area);
        output_block.render(area, buf);

        let mut actual_text_area = inner_output_area;

        // Render Input Box if active
        if self.input_active {
            let [text_part, input_part] = Layout::vertical([
                Constraint::Min(0),
                Constraint::Length(3),
            ]).areas(inner_output_area);
            
            actual_text_area = text_part;
            
            let input_block = Block::bordered()
                .title(Span::styled(" Send Command ", self.theme.style("input_title")))
                .border_style(self.theme.style("input_border"));
            let input_inner = input_block.inner(input_part);
            input_block.render(input_part, buf);
            
            buf.set_string(
                input_inner.x,
                input_inner.y,
                self.input_value,
                self.theme.style("input_text"),
            );
            
            // Note: The cursor position is handled by the Frame in view.rs, 
            // but we can store it or let the caller handle it.
        }

        // Handle Scrollbar calculations
        let total_lines = self.lines.len();
        let total_count = if self.autoscroll { total_lines + 1 } else { total_lines };
        let show_scrollbar = total_count > actual_text_area.height as usize;

        let mut text_area = actual_text_area.inner(Margin { vertical: 0, horizontal: 1 });
        if show_scrollbar { text_area.width = text_area.width.saturating_sub(1); }

        let display_lines = if self.lines.is_empty() {
            vec![Line::from(Span::styled("No output yet.", Style::default().fg(Color::DarkGray)))]
        } else {
            self.lines.to_vec()
        };

        Paragraph::new(display_lines)
            .scroll((self.scroll, 0))
            .render(text_area, buf);

        if show_scrollbar {
            let scrollbar_area = Rect {
                x: inner_output_area.right().saturating_sub(1),
                y: inner_output_area.top(),
                width: 1,
                height: inner_output_area.height,
            };

            let scrollbar = ScrollBar::vertical(ScrollLengths {
                content_len: total_count,
                viewport_len: text_area.height as usize,
            }).offset(self.scroll as usize);
            
            scrollbar.render(scrollbar_area, buf);
        }
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
    fn test_output_box_render_empty() {
        let theme = Theme::default();
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        
        terminal.draw(|f| {
            let widget = OutputBoxWidget::new(&[], 0, &theme);
            f.render_widget(widget, f.area());
        }).unwrap();

        let s = buffer_to_string(terminal.backend().buffer());
        assert!(s.contains(" Output "));
        assert!(s.contains("No output yet."));
    }

    #[test]
    fn test_output_box_render_with_lines() {
        let theme = Theme::default();
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        let lines = vec![Line::from("Line 1"), Line::from("Line 2")];
        
        terminal.draw(|f| {
            let widget = OutputBoxWidget::new(&lines, 0, &theme);
            f.render_widget(widget, f.area());
        }).unwrap();

        let s = buffer_to_string(terminal.backend().buffer());
        assert!(s.contains("Line 1"));
        assert!(s.contains("Line 2"));
    }

    #[test]
    fn test_output_box_input_active() {
        let theme = Theme::default();
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        
        terminal.draw(|f| {
            let widget = OutputBoxWidget::new(&[], 0, &theme)
                .input(true, "cmd", 3);
            f.render_widget(widget, f.area());
        }).unwrap();

        let s = buffer_to_string(terminal.backend().buffer());
        assert!(s.contains("Send Command"));
        assert!(s.contains("cmd"));
    }
}
