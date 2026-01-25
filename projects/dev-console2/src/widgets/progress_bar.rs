use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Clone, Debug)]
pub struct ProgressBarWidget<'a> {
    title: Line<'a>,
    progress_percentage: f64,
    stage_text: String,
    elapsed_text: String,
    eta_text: String,
    file_text: String,
}

impl<'a> ProgressBarWidget<'a> {
    pub fn new(title: String, progress: f64, stage: String) -> Self {
        Self {
            title: Line::from(title),
            progress_percentage: progress,
            stage_text: stage,
            elapsed_text: String::new(),
            eta_text: String::new(),
            file_text: String::new(),
        }
    }

    pub fn elapsed(mut self, elapsed: String) -> Self {
        self.elapsed_text = elapsed;
        self
    }

    pub fn eta(mut self, eta: String) -> Self {
        self.eta_text = eta;
        self
    }

}

impl<'a> Widget for ProgressBarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title);
        
        let inner_area = block.inner(area);
        block.render(area, buf);

        let content_area = inner_area.inner(Margin {
            vertical: 0,
            horizontal: 1,
        });

        if content_area.height < 1 {
            return;
        }

        // Row 1: Structured Status Info
        let line1 = format!(
            "Progress: {:>5.1}% | Elapsed: {:>5} | ETA: {:>5} | Stage: {}",
            self.progress_percentage,
            if self.elapsed_text.is_empty() { "00:00" } else { &self.elapsed_text },
            if self.eta_text.is_empty() { "--:--" } else { &self.eta_text },
            self.stage_text
        );

        // Row 2: Pure Progress Bar (Percentage moved to structured header)
        let bar_width = (content_area.width as usize).saturating_sub(2);
        let filled_width = ((bar_width as f64 * self.progress_percentage / 100.0).round() as usize).min(bar_width);
        let empty_width = bar_width.saturating_sub(filled_width);
        
        let bar_text = format!("[{}{}]", "█".repeat(filled_width), " ".repeat(empty_width));

        let mut lines = vec![
            Line::from(Span::styled(line1, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
            Line::from(Span::styled(bar_text, Style::default().fg(Color::Green))),
        ];

        if !self.file_text.is_empty() && content_area.height > 2 {
            lines.push(Line::from(Span::styled(format!("File: {}", self.file_text), Style::default())));
        }

        Paragraph::new(lines).render(content_area, buf);
    }
}
