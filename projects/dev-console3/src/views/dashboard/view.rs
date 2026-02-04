use crate::views::dashboard::state::Dashboard;
use crate::widgets::elements::selection_list::SelectionList;
use crate::widgets::traits::Component;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render(dashboard: &mut Dashboard, f: &mut Frame, area: Rect) {
    // 1. Split into Left and Right Columns
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(30),
            Constraint::Min(0),
        ])
        .split(area);

    // 2. Left Column: Profile (Top) and Commands (Bottom)
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Reduced height for Profile
            Constraint::Min(0),
        ])
        .split(main_chunks[0]);

    // 3. Right Column: Status (Top) and Output (Bottom)
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(0),
        ])
        .split(main_chunks[1]);

    // --- RENDER LEFT COLUMN ---

    // The "Profile" section is now JUST the dropdown, anchored correctly
    let dropdown_area = Rect::new(left_chunks[0].x, left_chunks[0].y, left_chunks[0].width, 3);

    // Commands Panel
    let cmd_block = Block::default()
        .borders(Borders::ALL)
        .title(" Commands ")
        .border_style(Style::default().fg(Color::DarkGray));
    let cmd_inner = cmd_block.inner(left_chunks[1]);
    f.render_widget(cmd_block, left_chunks[1]);

    let cmd_list = SelectionList::new(&dashboard.commands, dashboard.selected_command);
    f.render_widget(cmd_list, cmd_inner);

    // --- RENDER RIGHT COLUMN ---

    let status_text = if let Some(config) = &dashboard.profile_config {
        format!(" {} profiles loaded.", config.sketches.len())
    } else {
        " 0 profiles loaded.".to_string()
    };
    let status_block = Block::default()
        .borders(Borders::ALL)
        .title(" Status ")
        .border_style(Style::default().fg(Color::DarkGray));
    f.render_widget(Paragraph::new(status_text).block(status_block), right_chunks[0]);

    let output_block = Block::default()
        .borders(Borders::ALL)
        .title(" Output ")
        .border_style(Style::default().fg(Color::DarkGray));
    f.render_widget(output_block, right_chunks[1]);

    // --- OVERLAYS ---
    
    // Dim the background if dropdown is open
    if dashboard.profile_dropdown.is_open {
        f.render_widget(crate::widgets::elements::dimmer::Dimmer, area);
    }
    
    // Render the dropdown at its anchored position
    dashboard.profile_dropdown.view(f, dropdown_area);
}