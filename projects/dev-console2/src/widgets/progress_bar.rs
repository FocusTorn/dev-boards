use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Widget},
};

/// A widget for displaying progress of a background task.
#[derive(Clone, Debug)]
pub struct ProgressBarWidget<'a> {
    title: Line<'a>,
    pub progress_percentage: f64,
    pub stage_text: String,
    pub elapsed_text: String,
    pub eta_text: String,
    pub file_text: String,
    pub border_style: Style,
    pub title_style: Style,
}

impl<'a> ProgressBarWidget<'a> {
    /// Creates a new progress bar with the provided title, percentage, and stage.
    pub fn new(title: String, progress: f64, stage: String) -> Self {
        Self {
            title: Line::from(title),
            progress_percentage: progress,
            stage_text: stage,
            elapsed_text: String::new(),
            eta_text: String::new(),
            file_text: String::new(),
            border_style: Style::default(),
            title_style: Style::default(),
        }
    }

    /// Sets the style for the widget's surrounding block borders.
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    /// Sets the style for the title text.
    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }

    /// Sets the elapsed time string (e.g., "01:23").
    pub fn elapsed(mut self, elapsed: String) -> Self {
        self.elapsed_text = elapsed;
        self
    }

    /// Sets the estimated time remaining string (e.g., "00:45").
    pub fn eta(mut self, eta: String) -> Self {
        self.eta_text = eta;
        self
    }
}

impl<'a> Widget for ProgressBarWidget<'a> {
    /// Renders the progress bar with integrated metrics and visual bar.
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(self.border_style)
            .title(self.title.style(self.title_style));
        let inner_area = block.inner(area);
        block.render(area, buf);

        let content_area = inner_area.inner(Margin {
            vertical: 0,
            horizontal: 1,
        });

        if content_area.height < 1 {
            return;
        }

        let line1 = format!(
            "Progress: {:>5.1}% | Elapsed: {:>5} | ETA: {:>5} | Stage: {}",
            self.progress_percentage,
            if self.elapsed_text.is_empty() {
                "00:00"
            } else {
                &self.elapsed_text
            },
            if self.eta_text.is_empty() {
                "--:--"
            } else {
                &self.eta_text
            },
            self.stage_text
        );

        let bar_width = (content_area.width as usize).saturating_sub(2);
        let filled_width = ((bar_width as f64 * self.progress_percentage / 100.0).round() as usize).min(bar_width);
        let empty_width = bar_width.saturating_sub(filled_width);
        let bar_text = format!("[{}{}]", "█".repeat(filled_width), " ".repeat(empty_width));

        let mut lines = vec![
            Line::from(Span::styled(
                line1,
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(bar_text, Style::default().fg(Color::Green))),
        ];

        if !self.file_text.is_empty() && content_area.height > 2 {
            lines.push(Line::from(Span::styled(
                format!("File: {}", self.file_text),
                Style::default(),
            )));
        }

        Paragraph::new(lines).render(content_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn buffer_content(buffer: &Buffer) -> String {
        let mut result = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                result.push_str(buffer[(x, y)].symbol());
            }
            result.push('\n');
        }
        result
    }

    #[test]
    fn test_progress_bar_render() {
        let backend = TestBackend::new(100, 5);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                let widget = ProgressBarWidget::new("Build".to_string(), 50.0, "Compiling".to_string())
                    .elapsed("00:10".to_string())
                    .eta("00:10".to_string());
                f.render_widget(widget, f.area());
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        let s = buffer_content(buffer);
        assert!(s.contains("50.0%"));
        assert!(s.contains("Compiling"));
        assert!(s.contains("█"));
    }

    #[test]
    fn test_progress_bar_styles_and_file() {
        let area = Rect::new(0, 0, 50, 5);
        let mut buffer = Buffer::empty(area);
        let widget = ProgressBarWidget::new("T".to_string(), 10.0, "S".to_string())
            .border_style(Style::default().fg(Color::Red))
            .title_style(Style::default().fg(Color::Yellow));

        // Manual set file_text since there is no setter yet
        let mut widget = widget;
        widget.file_text = "test.ino".to_string();

        widget.render(area, &mut buffer);
        let s = buffer_content(&buffer);
        assert!(s.contains("File: test.ino"));
    }
}
