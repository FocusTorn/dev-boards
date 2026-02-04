use crate::widgets::components::toast::state::{ToastManager, ToastPosition};
use ratatui::prelude::*;
use ratatui::widgets::{Clear, Paragraph};

pub fn render(manager: &mut ToastManager, f: &mut Frame, area: Rect) {
    if manager.toasts.is_empty() {
        return;
    }

    let mut max_width = 0usize;
    let mut toast_data = Vec::new();

    for toast in &manager.toasts {
        let content = format!("{} {}", toast.level.icon(), toast.message);
        max_width = max_width.max(content.len());
        toast_data.push((content, toast.level.color(), toast.opacity));
    }

    max_width += 3;
    let toast_height = 1u16;
    let max_width_u16 = max_width as u16;
    let mut y_offset = 0u16;

    for (content, fg_color, opacity) in toast_data.iter().rev() {
        let (toast_x, toast_y) = match manager.config.position {
            ToastPosition::BottomCenter => (
                area.x + (area.width.saturating_sub(max_width_u16)) / 2,
                area.y + area.height.saturating_sub(1 + toast_height + y_offset),
            ),
            _ => (area.x + 1, area.y + 1 + y_offset),
        };

        let toast_area = Rect {
            x: toast_x,
            y: toast_y,
            width: max_width_u16,
            height: toast_height,
        };

        f.render_widget(Clear, toast_area);

        let current_fg = if *opacity < 1.0 {
            Color::DarkGray
        } else {
            *fg_color
        };

        let paragraph = Paragraph::new(content.as_str())
            .alignment(Alignment::Center)
            .style(Style::default().fg(current_fg).bg(Color::Rgb(10, 10, 10)).add_modifier(Modifier::BOLD));
        f.render_widget(paragraph, toast_area);

        y_offset += toast_height;
    }
}
