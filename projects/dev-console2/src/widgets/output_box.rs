use ratatui::{
    layout::{Rect, Margin},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget},
    buffer::Buffer,
};

pub struct OutputBoxWidget<'a> {
    output_lines: &'a [String],
    scroll_state: &'a mut ScrollbarState,
    scroll_offset: u16,
    is_focused: bool,
}

impl<'a> OutputBoxWidget<'a> {
    pub fn new(output_lines: &'a [String], scroll_state: &'a mut ScrollbarState, scroll_offset: u16, is_focused: bool) -> Self {
        Self { output_lines, scroll_state, scroll_offset, is_focused }
    }
}

impl<'a> Widget for OutputBoxWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_color = if self.is_focused {
            Color::Cyan
        } else {
            Color::White
        };

        let block = Block::bordered()
            .title(" Output ")
            .border_style(Style::default().fg(border_color))
            .title_style(Style::default().fg(border_color));

        let inner_area = block.inner(area);
        block.render(area, buf);

        // Determine if scrollbar should be shown
        let show_scrollbar = self.output_lines.len() > inner_area.height as usize;

        // Calculate text area with margin of 1 on left and right
        let mut text_area = inner_area.inner(Margin {
            vertical: 0,
            horizontal: 1,
        });

        // If scrollbar is shown, reduce text width to leave a margin between text and scrollbar
        if show_scrollbar {
            text_area.width = text_area.width.saturating_sub(1);
        }

        // Calculate visible lines based on scroll offset
        let visible_height = text_area.height as usize;
        
        let visible_lines: Vec<Line> = if self.output_lines.is_empty() {
            vec![Line::from(Span::styled("No output yet.", Style::default().fg(Color::DarkGray)))]
        } else {
            let start_idx = (self.scroll_offset as usize).min(self.output_lines.len().saturating_sub(1));
            let end_idx = (start_idx + visible_height).min(self.output_lines.len());
            
            self.output_lines[start_idx..end_idx]
                .iter()
                .map(|line| Line::from(line.as_str()))
                .collect()
        };
        
        let paragraph = Paragraph::new(visible_lines);
            
        paragraph.render(text_area, buf);

        if show_scrollbar {
            // Update and render scrollbar
            // Positioned 1 col to the left of the border (which is the right-most col of inner_area)
            *self.scroll_state = ScrollbarState::new(self.output_lines.len())
                .position(self.scroll_offset as usize);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
            
            StatefulWidget::render(
                scrollbar,
                inner_area,
                buf,
                self.scroll_state,
            );
        }
    }
}
