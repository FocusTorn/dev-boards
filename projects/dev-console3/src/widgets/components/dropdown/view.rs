use crate::widgets::components::dropdown::state::OverlayDropdown;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Scrollbar, ScrollbarOrientation, Paragraph};

pub fn render(dropdown: &mut OverlayDropdown, f: &mut Frame, area: Rect) {
    if !dropdown.is_open {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(format!(" {} ", dropdown.title));
        f.render_widget(block, area);
        f.render_widget(
            Paragraph::new(dropdown.items[dropdown.selected].as_str())
                .style(Style::default().fg(Color::Cyan)),
            area.inner(Margin { horizontal: 1, vertical: 1 })
        );
        return;
    }

    let terminal_height = f.area().height;
    let (total_area, is_down) = dropdown.calculate_layout(area, terminal_height);
    
    f.render_widget(Clear, total_area);

    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(format!(" {} ", dropdown.title));
    f.render_widget(main_block, total_area);

    let list_items: Vec<ListItem> = dropdown.items.iter()
        .map(|i| ListItem::new(i.as_str()).style(Style::default().fg(Color::White)))
        .collect();

    let list = List::new(list_items)
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .highlight_symbol(">> ");

    if is_down {
        let header_area = Rect::new(total_area.x, total_area.y, total_area.width, area.height);
        let list_area = Rect::new(total_area.x, total_area.y + area.height, total_area.width, total_area.height - area.height);
        
        f.render_widget(Paragraph::new(dropdown.items[dropdown.selected].as_str()).style(Style::default().fg(Color::White)), header_area.inner(Margin { horizontal: 1, vertical: 1 }));
        f.render_stateful_widget(list, list_area.inner(Margin { horizontal: 1, vertical: 0 }), &mut dropdown.list_state);
        
        f.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            list_area,
            &mut dropdown.scroll_state,
        );
    } else {
        let list_area = Rect::new(total_area.x, total_area.y, total_area.width, total_area.height - area.height);
        let header_area = Rect::new(total_area.x, total_area.y + list_area.height, total_area.width, area.height);

        f.render_widget(Paragraph::new(dropdown.items[dropdown.selected].as_str()).style(Style::default().fg(Color::White)), header_area.inner(Margin { horizontal: 1, vertical: 1 }));
        f.render_stateful_widget(list, list_area.inner(Margin { horizontal: 1, vertical: 0 }), &mut dropdown.list_state);

        f.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            list_area,
            &mut dropdown.scroll_state,
        );
    }
}
