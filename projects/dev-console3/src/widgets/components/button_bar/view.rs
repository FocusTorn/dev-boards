use crate::widgets::components::button_bar::state::{ButtonBar, ButtonBarStyle};
use ratatui::prelude::*;

pub fn render(bar: &ButtonBar, f: &mut Frame, area: Rect) {
    let aligned_area = bar.get_aligned_area(area);
    if aligned_area.height == 0 || aligned_area.width == 0 { return; }

    let buf = f.buffer_mut();
    let mut spans = Vec::new();
    let active_color = bar.config.colors.as_ref().and_then(|c| c.active.as_deref()).map(parse_color).unwrap_or(Color::Cyan);
    let negate_color = bar.config.colors.as_ref().and_then(|c| c.negate.as_deref()).map(parse_color).unwrap_or(Color::White);
    let inactive_style = Style::default().fg(Color::White);
    let active_style = Style::default().fg(active_color).add_modifier(Modifier::BOLD);
    let negate_style = Style::default().fg(negate_color);

    for (idx, item) in bar.items.iter().enumerate() {
        if idx > 0 {
            spans.push(Span::styled("─", inactive_style));
        }

        if item.active {
            match bar.config.style {
                ButtonBarStyle::Boxed | ButtonBarStyle::BoxStatic => {
                    spans.push(Span::styled("[", inactive_style));
                    spans.push(Span::styled(format!(" {} ", item.name), active_style));
                    spans.push(Span::styled("]", inactive_style));
                }
                _ => {
                    spans.push(Span::styled(format!(" {} ", item.name), active_style));
                }
            }
        } else {
            let item_style = if bar.config.style == ButtonBarStyle::BoxStatic || bar.config.style == ButtonBarStyle::TextStatic {
                negate_style
            } else {
                inactive_style
            };
            let content = if bar.config.style == ButtonBarStyle::BoxStatic {
                format!("[ {} ]", item.name)
            } else {
                format!(" {} ", item.name)
            };
            spans.push(Span::styled(content, item_style));
        }
    }

    Line::from(spans).render(aligned_area, buf);
}

fn parse_color(c: &str) -> Color {
    match c.to_lowercase().as_str() {
        "black" => Color::Black, "red" => Color::Red, "green" => Color::Green,
        "yellow" => Color::Yellow, "blue" => Color::Blue, "magenta" => Color::Magenta,
        "cyan" => Color::Cyan, "gray" | "grey" => Color::Gray, "darkgray" | "darkgrey" => Color::DarkGray,
        "lightred" => Color::LightRed, "lightgreen" => Color::LightGreen, "lightyellow" => Color::LightYellow,
        "lightblue" => Color::LightBlue, "lightmagenta" => Color::LightMagenta, "lightcyan" => Color::LightCyan,
        "white" => Color::White, _ => Color::White,
    }
}
