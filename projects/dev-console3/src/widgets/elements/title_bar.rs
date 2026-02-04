use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Widget};

pub struct TitleBar {
    pub title: String,
}

impl TitleBar {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

impl Widget for TitleBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_text = format!(" {} ", self.title);
        let line = if (area.width as usize) <= title_text.len() + 2 {
            Line::from(Span::styled(&self.title, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)))
        } else {
            let dash_count = (area.width as usize).saturating_sub(title_text.len() + 2);
            let left = dash_count / 2;
            let right = dash_count - left;
            Line::from(vec![
                Span::styled("═".repeat(left), Style::default().fg(Color::White)),
                Span::styled(title_text, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled("═".repeat(right), Style::default().fg(Color::White)),
            ])
        };
        Paragraph::new(line).alignment(Alignment::Center).render(area, buf);
    }
}
