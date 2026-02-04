use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

pub struct StatusBar {
    pub status: String,
    pub help_hints: String,
}

impl StatusBar {
    pub fn new(status: String, help_hints: String) -> Self {
        Self { status, help_hints }
    }
}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::White))
            .render(area, buf);

        let text_area = Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: 1,
        };

        if text_area.height > 0 && text_area.width > 0 {
            Paragraph::new(Line::from(vec![
                Span::styled(format!(" {} ", self.status), Style::default().fg(Color::White)),
            ]))
            .render(text_area, buf);
        }

        let help_area = Rect {
            x: area.x + area.width.saturating_sub(self.help_hints.len() as u16 + 1),
            y: text_area.y,
            width: self.help_hints.len() as u16 + 1,
            height: 1,
        };
        Paragraph::new(Line::from(vec![
            Span::styled(self.help_hints, Style::default().fg(Color::Cyan)),
        ]))
        .alignment(Alignment::Right)
        .render(help_area, buf);
    }
}