use crate::views::profiles::state::Profiles;
use crate::widgets::elements::selection_list::SelectionList;
use crate::widgets::traits::Component;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render(profiles: &mut Profiles, f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(25),
            Constraint::Min(0),
        ])
        .split(area);

    // 1. Sidebar (Categories)
    let sidebar_block = Block::default()
        .borders(Borders::ALL)
        .title(" Categories ")
        .border_style(Style::default().fg(Color::DarkGray));
    let sidebar_inner = sidebar_block.inner(chunks[0]);
    f.render_widget(sidebar_block, chunks[0]);

    let category_list = SelectionList::new(&profiles.categories, profiles.selected_category);
    f.render_widget(category_list, sidebar_inner);

    // 2. Content Area
    let content_inner = chunks[1].inner(Margin { horizontal: 2, vertical: 1 });
    
    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Category Header
            Constraint::Min(0),    // Settings
        ])
        .split(content_inner);

    let active_category = &profiles.categories[profiles.selected_category];
    f.render_widget(
        Paragraph::new(active_category.to_uppercase())
            .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::White)),
        header_chunks[0]
    );

    // Render "Device" settings as an example
    if active_category == "Device" {
        render_device_settings(profiles, f, header_chunks[1]);
    } else {
        f.render_widget(
            Paragraph::new(format!("{} settings migration in progress...", active_category))
                .alignment(Alignment::Center),
            header_chunks[1]
        );
    }
}

fn render_device_settings(profiles: &mut Profiles, f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Field 1 (Port)
            Constraint::Length(5), // Field 2 (Baud)
            Constraint::Min(0),
        ])
        .split(area);

    // Field 1: Serial Port (Dropdown)
    let port_area = Rect::new(chunks[0].x, chunks[0].y + 1, 40, 3);
    
    // Field 2: Baud Rate (Dropdown)
    let baud_area = Rect::new(chunks[1].x, chunks[1].y + 1, 40, 3);

    // Handle Dimming if any dropdown is open
    if profiles.port_dropdown.is_open || profiles.baud_dropdown.is_open {
        f.render_widget(crate::widgets::elements::dimmer::Dimmer, area);
    }

    // Render Dropdowns
    profiles.port_dropdown.view(f, port_area);
    profiles.baud_dropdown.view(f, baud_area);
}
