use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget, Wrap},
    buffer::Buffer,
};

pub struct StatusBoxWidget<'a> {
    status_text: &'a str,
    is_focused: bool,
}

impl<'a> StatusBoxWidget<'a> {
    pub fn new(status_text: &'a str, is_focused: bool) -> Self {
        Self { status_text, is_focused }
    }
}

impl<'a> Widget for StatusBoxWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let is_error = self.status_text.starts_with("[Error]");

        let border_color = if is_error {
            Color::Red
        } else if self.is_focused {
            Color::Cyan
        } else {
            Color::White
        };

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
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        };
        let paragraph = Paragraph::new(self.status_text)
            .style(style)
            .wrap(Wrap { trim: true });

        paragraph.render(text_area, buf);
    }
}