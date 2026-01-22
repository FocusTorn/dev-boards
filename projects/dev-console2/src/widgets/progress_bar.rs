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

    pub fn current_file(mut self, file: String) -> Self {
        self.file_text = file;
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

        let line1 = format!(
            "{}: {:.1}%{}{}{}{}",
            self.stage_text,
            self.progress_percentage,
            if self.elapsed_text.is_empty() { "" } else { " | Elapsed: " },
            self.elapsed_text,
            if self.eta_text.is_empty() { "" } else { " | ETA: " },
            self.eta_text
        );

        let percent_text = format!("{:.1}%", self.progress_percentage);
        let percent_text_width = percent_text.len();
        let progress_bar_width = (content_area.width as usize).saturating_sub(percent_text_width + 3);
        
        let filled_width = ((progress_bar_width as f64 * self.progress_percentage / 100.0) as usize).min(progress_bar_width);
        let empty_width = progress_bar_width.saturating_sub(filled_width);
        
        let line2 = format!("[{}{}] {}", "█".repeat(filled_width), "░".repeat(empty_width), percent_text);

        let mut lines = vec![
            Line::from(Span::styled(line1, Style::default().fg(Color::Cyan))),
            Line::from(Span::styled(line2, Style::default().fg(Color::Green))),
        ];

        if !self.file_text.is_empty() && content_area.height > 2 {
            lines.push(Line::from(Span::styled(format!("File: {}", self.file_text), Style::default())));
        }

        Paragraph::new(lines).render(content_area, buf);
    }
}
