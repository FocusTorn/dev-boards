use crate::widgets::components::tabbed_bar::state::TabbedBar;
use crate::config::TabBarAlignment;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Widget};
use ratatui::layout::Spacing;

pub fn render_integrated(bar: &TabbedBar, area: Rect, buf: &mut Buffer, block: Block) -> Rect {
    let consumed = 1; // Always 1 for Tab style
    let vertical = bar.config.alignment.vertical.unwrap_or(TabBarAlignment::Top);

    let (tab_target_area, body_area) = if vertical == TabBarAlignment::Top {
        let [header, body] = Layout::vertical([
            Constraint::Length(consumed + 1),
            Constraint::Min(0),
        ]).spacing(Spacing::Overlap(1)).areas(area);
        (header, body)
    } else {
        let [body, footer] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(consumed + 1),
        ]).spacing(Spacing::Overlap(1)).areas(area);
        (footer, body)
    };

    block.render(body_area, buf);
    render_into(bar, tab_target_area, buf);
    
    body_area.inner(Margin { horizontal: 1, vertical: 1 })
}

pub fn render_into(bar: &TabbedBar, area: Rect, buf: &mut Buffer) {
    let aligned_area = bar.get_aligned_area(area);
    if aligned_area.height == 0 || aligned_area.width == 0 { return; }

    let active_text_color = bar.config.colors.as_ref()
        .and_then(|c| c.active.as_deref())
        .map(parse_color)
        .or_else(|| bar.config.color.as_deref().map(parse_color))
        .unwrap_or(Color::Cyan);
    
    let active_style = Style::default().fg(active_text_color).add_modifier(Modifier::BOLD);
    let inactive_style = Style::default().fg(Color::White);

    if aligned_area.height >= 2 {
        // Build Top Line
        if let Some(active_idx) = bar.items.iter().position(|i| i.active) {
            let mut pre_width = 0;
            for i in 0..active_idx {
                pre_width += bar.get_item_width(&bar.items[i]) + 1;
            }
            let item_width = bar.get_item_width(&bar.items[active_idx]);
            let active_item = &bar.items[active_idx];
            let mut spans = Vec::new();
            if pre_width > 0 { spans.push(Span::raw(" ".repeat(pre_width as usize))); }
            
            let content_len = active_item.name.len() as u16 + 4;
            let total_pad = item_width.saturating_sub(content_len);
            let left_pad = total_pad / 2;
            
            spans.push(Span::raw(" ".repeat(left_pad as usize)));
            spans.push(Span::styled("╭", inactive_style));
            spans.push(Span::styled("─".repeat(active_item.name.len() + 2), inactive_style));
            spans.push(Span::styled("╮", inactive_style));
            
            Line::from(spans).render(aligned_area, buf);
        }

        // Build Tab Line
        let mut spans = Vec::new();
        for (idx, item) in bar.items.iter().enumerate() {
            if idx > 0 { spans.push(Span::styled("─", inactive_style)); }
            let item_width = bar.get_item_width(item);
            if item.active {
                let content = format!(" {} ", item.name);
                let content_len = content.chars().count() as u16 + 2;
                let total_pad = item_width.saturating_sub(content_len);
                let left_pad = total_pad / 2;
                let right_pad = total_pad - left_pad;
                if left_pad > 0 { spans.push(Span::raw(" ".repeat(left_pad as usize))); }
                spans.push(Span::styled("╯", inactive_style));
                spans.push(Span::styled(content, active_style));
                spans.push(Span::styled("╰", inactive_style));
                if right_pad > 0 { spans.push(Span::raw(" ".repeat(right_pad as usize))); }
            } else {
                let content = format!(" {} ", item.name);
                let total_pad = item_width.saturating_sub(content.len() as u16);
                let left_pad = total_pad / 2;
                let right_pad = total_pad - left_pad;
                if left_pad > 0 { spans.push(Span::raw(" ".repeat(left_pad as usize))); }
                spans.push(Span::styled(content, inactive_style));
                if right_pad > 0 { spans.push(Span::raw(" ".repeat(right_pad as usize))); }
            }
        }
        let tab_line_area = Rect { y: aligned_area.y + 1, height: 1, ..aligned_area };
        Line::from(spans).render(tab_line_area, buf);
    } else {
        let mut spans = Vec::new();
        for (idx, item) in bar.items.iter().enumerate() {
            if idx > 0 { spans.push(Span::styled("─", inactive_style)); }
            let content = format!(" {} ", item.name);
            let style = if item.active { active_style } else { inactive_style };
            spans.push(Span::styled(content, style));
        }
        Line::from(spans).render(aligned_area, buf);
    }
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