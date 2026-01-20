// Standalone Settings2 implementation using pure ratatui layout framework
// This demonstrates how to render the settings interface without HWND complexity

use crate::settings::Settings;
use crate::field_editor::{FieldEditorState, SettingsFields};
use crate::profile_state::ProfileState;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Clear, Padding},
};


/// Render Settings2 tab using pure ratatui layouts (no HWND system)
/// 
/// This function demonstrates a clean alternative to the complex HWND-based approach used in the original Settings tab.
/// It uses only ratatui's Layout system to create a responsive, maintainable UI structure.
/// 
/// # Layout Structure
/// 
/// The layout follows this hierarchy:
/// ┌─ Terminal Area ─────────────────────────────────────┐
/// │  ┌─ Left Inset (2 cols) ─┐  ┌─ Content Area ─────┐    │
/// │  │                        │  │ ┌─ Header ───────┐ │    │
/// │  │                        │  │ │ Settings2      │ │    │
/// │  │                        │  │ └────────────────┘ │    │
/// │  │                        │  │ ┌─ Main Content ─┐ │    │
/// │  │                        │  │ │ Profiles|Config│ │    │
/// │  │                        │  │ └────────────────┘ │    │
/// │  │                        │  └────────────────────┘    │
/// │  └─ Right Inset (2 cols) ─┘                           │
/// └────────────────────────────────────────────────────────────┘
/// 
/// # Arguments
/// * `f` - The ratatui frame to render on
/// * `area` - The available terminal area to render within
/// * `settings` - Current application settings
/// * `fields` - Settings field definitions and accessors
/// * `editor_state` - Current field editor state (selected, editing, etc.)
/// * `profile_state` - Profile management state
pub fn render_settings2_standalone(
    f: &mut Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
    profile_state: &ProfileState,
) {
    
    //> Terminal size validation
    //> NOTES 
     // This prevents rendering issues on very small terminals where the layout would break
    //<
    if area.width < 80 || area.height < 21 {
        render_too_small_warning(f, area);
        return;
    }
    //<
    
    
    // ===============================================================================
    // STEP 1: Create the main layout with 2-column inset on both sides
    // ===============================================================================
    // This creates the "inset by two columns on left and right" effect as requested
    // The layout uses three horizontal constraints:
    // 1. Left inset: Fixed 2 columns
    // 2. Main content: Remaining space (Min(0) means "take whatever is left")
    // 3. Right inset: Fixed 2 columns
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2),           // Left inset - creates visual padding
            Constraint::Min(0),             // Main content area - flexible width
            Constraint::Length(2),           // Right inset - creates visual padding
        ])
        .split(area);
    
    let content_area = main_chunks[1]; // This is our main canvas for all content
    
    
    let main_content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),    // Profiles section - fixed width ensures consistent layout
            Constraint::Min(0),        // Configuration section - responsive to remaining space
        ])
        .split(content_area);
    
    let profiles_area = main_content_chunks[0];     // Left side - profile management
    let config_area = main_content_chunks[1];       // Right side - all configuration fields
    
    
    render_profiles_section(f, profiles_area, profile_state);
    
    render_configuration_section(f, config_area, settings, fields, editor_state);
    
    
    
}






















/// Render the profiles section
fn render_profiles_section(f: &mut Frame, area: Rect, profile_state: &ProfileState) {
    let profiles = profile_state.profiles.lock().unwrap();
    let selected_index = profile_state.selected_index.lock().unwrap();
    let active_profile_name = profile_state.active_profile_name.lock().unwrap();
    
    // Create profile list items
    let list_items: Vec<ListItem> = profiles.iter()
        .enumerate()
        .map(|(_i, name)| {
            let is_active = active_profile_name.as_ref().map_or(false, |active| active == name);
            let prefix = if is_active { "● " } else { "○ " };
            
            ListItem::new(Line::from(vec![
                Span::styled(
                    prefix,
                    Style::default()
                        .fg(if is_active { Color::Green } else { Color::Gray }),
                ),
                Span::styled(
                    name,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(if is_active { Modifier::BOLD } else { Modifier::empty() }),
                ),
            ]))
        })
        .collect();
    
    // Create list state
    let mut list_state = ListState::default();
    if let Some(idx) = *selected_index {
        if idx < list_items.len() {
            list_state.select(Some(idx));
        }
    }
    
    let title = format!(" Profiles ({}) ", profiles.len());
    
    let list = List::new(list_items)
        .block(
            Block::default()
                .title(Span::styled(title, Style::default().fg(Color::Cyan)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(102, 102, 102)))
        )
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(255, 215, 0)) // Gold
                .add_modifier(Modifier::BOLD)
        );
    
    f.render_stateful_widget(list, area, &mut list_state);
}






/// Render the main configuration section
fn render_configuration_section(
    f: &mut Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
) {
    
    
    
    
    
    
    
    // Split configuration into Sketch (top) and bottom sections
    
    let config_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),    // Sketch section (2 fields x 3 lines)
            Constraint::Min(0),       // Bottom sections
        ])
        .split(area);
    
        
        
    
    render_sketch_section(f, config_chunks[0], settings, fields, editor_state);
    
    
    
    
    
    
    
    
    // Bottom section: Device | Connection | MQTT
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Device
            Constraint::Percentage(25), // Connection
            Constraint::Percentage(50), // MQTT
        ])
        .split(config_chunks[1]);
    
    // Render device section
    render_device_section(f, bottom_chunks[0], settings, fields, editor_state);
    
    // Render connection section
    render_connection_section(f, bottom_chunks[1], settings, fields, editor_state);
    
    // Render MQTT section (split into two columns)
    render_mqtt_section(f, bottom_chunks[2], settings, fields, editor_state);
}






/// Render sketch section (Sketch Directory and Sketch Name)
fn render_sketch_section(
    f: &mut Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
) {
    
    
    
    
     let section_block = Block::default()
        .title(Span::styled(" Sketch Target ", Style::default().fg(Color::Cyan)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(102, 102, 102)))
        .padding(Padding::new(1, 1, 0, 0)); // left, right, top, bottom

    // AUTOMATIC CALCULATION: handles borders and padding for you
    let inner_area = section_block.inner(area); 
    
    let sketch_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Sketch Directory
            Constraint::Length(3), // Sketch Name
        ])
        .split(inner_area);

    // Render components
    render_field(f, sketch_chunks[0], settings, fields, editor_state, 0, "Sketch Directory");
    render_field(f, sketch_chunks[1], settings, fields, editor_state, 1, "Sketch Name");
    
    // Render the block frame last to ensure it stays on top/visible
    f.render_widget(section_block, area);
    
    
    
}







/// Render device section
fn render_device_section(
    f: &mut Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
) {
    
    let device_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Environment
            Constraint::Length(3), // Board Model
            Constraint::Length(3), // FQBN
        ])
        .split(area);
    
        
        
        
    render_field(f, device_chunks[0], settings, fields, editor_state, 2, "Environment");
    render_field(f, device_chunks[1], settings, fields, editor_state, 3, "Board Model");
    render_field(f, device_chunks[2], settings, fields, editor_state, 4, "FQBN");
    
    
    
    // Add section border
    let section_block = Block::default()
        .title(Span::styled(" Device ", Style::default().fg(Color::Cyan)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(102, 102, 102)));
    f.render_widget(section_block, area);    
        
        
        
    
}




/// Render connection section
fn render_connection_section(
    f: &mut Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
) {
    let conn_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Port
            Constraint::Length(3), // Baud Rate
        ])
        .split(area.inner(Margin::new(0, 1))); // Account for section border
    
    render_field(f, conn_chunks[0], settings, fields, editor_state, 5, "Port");
    render_field(f, conn_chunks[1], settings, fields, editor_state, 6, "Baud Rate");
    
    // Add section border
    let section_block = Block::default()
        .title(Span::styled(" Connection ", Style::default().fg(Color::Cyan)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(102, 102, 102)));
    f.render_widget(section_block, area);
}

/// Render MQTT section (split into Credentials and Topics)
fn render_mqtt_section(
    f: &mut Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
) {
    let mqtt_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Credentials
            Constraint::Percentage(50), // Topics
        ])
        .split(area.inner(Margin::new(0, 1))); // Account for section border
    
    // MQTT Credentials
    let cred_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Host
            Constraint::Length(3), // Port
            Constraint::Length(3), // Username
            Constraint::Length(3), // Password
        ])
        .split(mqtt_chunks[0]);
    
    render_field(f, cred_chunks[0], settings, fields, editor_state, 7, "Host");
    render_field(f, cred_chunks[1], settings, fields, editor_state, 8, "Port");
    render_field(f, cred_chunks[2], settings, fields, editor_state, 9, "Username");
    render_field(f, cred_chunks[3], settings, fields, editor_state, 10, "Password");
    
    // MQTT Topics
    let topics_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Command
            Constraint::Length(3), // State
            Constraint::Length(3), // Status
        ])
        .split(mqtt_chunks[1]);
    
    render_field(f, topics_chunks[0], settings, fields, editor_state, 11, "Command");
    render_field(f, topics_chunks[1], settings, fields, editor_state, 12, "State");
    render_field(f, topics_chunks[2], settings, fields, editor_state, 13, "Status");
    
    // Add section border
    let section_block = Block::default()
        .title(Span::styled(" MQTT ", Style::default().fg(Color::Cyan)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(102, 102, 102)));
    f.render_widget(section_block, area);
}

/// Render a single field
fn render_field(
    f: &mut Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
    field_index: usize,
    label: &str,
) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    
    let is_selected = matches!(editor_state, FieldEditorState::Selected { field_index: idx } if *idx == field_index);
    let is_editing = matches!(editor_state, FieldEditorState::Editing { field_index: idx, .. } if *idx == field_index);
    let value = fields.get_value(settings, field_index);
    
    // Get display value (handle scrolling for editing)
    let display_value = if is_editing {
        if let FieldEditorState::Editing { input, .. } = editor_state {
            input.value()
        } else {
            &value
        }
    } else {
        &value
    };
    
    // Determine colors
    let border_color = if is_editing {
        Color::Cyan
    } else if is_selected {
        Color::White
    } else {
        Color::Rgb(102, 102, 102)
    };
    
    let title_color = Color::Rgb(153, 153, 153); // Gray
    let value_color = if is_editing { Color::Cyan } else { Color::White };
    
    let block = Block::default()
        .title(Span::styled(format!(" {} ", label), Style::default().fg(title_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0));
    
    let paragraph = Paragraph::new(display_value)
        .style(Style::default().fg(value_color))
        .block(block);
    
    f.render_widget(paragraph, area);
}

/// Render dropdown overlay for field selection
fn render_dropdown_overlay(
    f: &mut Frame,
    area: Rect,
    _field_index: usize,
    selected_index: usize,
    options: &[String],
) {
    // Calculate dropdown position and size
    let dropdown_width = 30.min(area.width.saturating_sub(4));
    let dropdown_height = (options.len() as u16).min(10).min(area.height.saturating_sub(4));
    
    // Center the dropdown
    let dropdown_x = area.x + (area.width.saturating_sub(dropdown_width)) / 2;
    let dropdown_y = area.y + (area.height.saturating_sub(dropdown_height)) / 2;
    
    let dropdown_area = Rect {
        x: dropdown_x,
        y: dropdown_y,
        width: dropdown_width,
        height: dropdown_height,
    };
    
    // Create dropdown items
    let list_items: Vec<ListItem> = options.iter()
        .enumerate()
        .map(|(i, option)| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    if i == selected_index { "> " } else { "  " },
                    Style::default().fg(Color::Cyan),
                ),
                Span::styled(
                    option,
                    Style::default().fg(Color::White),
                ),
            ]))
        })
        .collect();
    
    // Create list state
    let mut list_state = ListState::default();
    if selected_index < list_items.len() {
        list_state.select(Some(selected_index));
    }
    
    let list = List::new(list_items)
        .block(
            Block::default()
                .title(Span::styled(" Select Option ", Style::default().fg(Color::Cyan)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        );
    
    // Clear the area and render dropdown
    f.render_widget(Clear, dropdown_area);
    f.render_stateful_widget(list, dropdown_area, &mut list_state);
}

/// Render warning when terminal is too small
fn render_too_small_warning(f: &mut Frame, area: Rect) { //>
    let warning_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "⚠ Terminal Too Small",
            Style::default().fg(Color::Rgb(255, 215, 0)).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Minimum size required: 80x21"),
        Line::from(format!("Current size: {}x{}", area.width, area.height)),
        Line::from(""),
        Line::from("Please resize your terminal."),
        Line::from(""),
    ];
    
    let warning_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(255, 215, 0)))
        .title("Warning");
    
    let warning_paragraph = Paragraph::new(warning_text)
        .block(warning_block)
        .alignment(Alignment::Center);
    
    // Center the warning
    let warning_area = Rect {
        x: area.x + (area.width.saturating_sub(60)) / 2,
        y: area.y + (area.height.saturating_sub(8)) / 2,
        width: 60.min(area.width),
        height: 8.min(area.height),
    };
    
    f.render_widget(warning_paragraph, warning_area);
} //<
