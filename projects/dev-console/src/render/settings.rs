// Settings panel rendering

use crate::settings::Settings;
use crate::field_editor::{FieldEditorState, SettingsFields};
use crate::profile_state::ProfileState;
use crate::constants::*;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use tui_components::{RectRegistry, DimmingContext, hex_color};

/// Helper function to register or update a HWND element
fn register_or_update(registry: &mut RectRegistry, hwnd_name: &str, rect: Rect) {
    if let Some(handle) = registry.get_handle(hwnd_name) {
        registry.update(handle, rect);
    } else {
        registry.register(Some(hwnd_name), rect);
    }
}

/// Render settings panel
pub fn render_settings(
    f: &mut Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
    profile_state: &ProfileState,
    registry: &mut RectRegistry,
    dimming: &DimmingContext,
) {
    // Check if terminal is too small (minimum size requirements)
    let min_width_pixels = MIN_WIDTH_PIXELS;
    let min_height_pixels = MIN_HEIGHT_PIXELS;
    
    if area.width < min_width_pixels || area.height < min_height_pixels {
        // Terminal is too small - show warning message
        let warning_text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "⚠ Terminal Too Small",
                Style::default().fg(Color::Rgb(255, 215, 0)).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!("Minimum size required: {}x{}", min_width_pixels, min_height_pixels)),
            Line::from(format!("Current size: {}x{}", area.width, area.height)),
            Line::from(""),
            Line::from("Please resize your terminal to at least 80 columns by 21 rows."),
            Line::from(""),
            Line::from(Span::styled(
                "The form will appear automatically when the terminal is large enough.",
                Style::default().fg(Color::Cyan),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Press [q] to quit",
                Style::default().fg(Color::White),
            )),
        ];
        
        let warning_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 215, 0)))
            .title("Warning");
        
        let warning_para = Paragraph::new(warning_text)
            .block(warning_block)
            .alignment(Alignment::Center);
        
        // Center the warning message
        let warning_width = 60;
        let warning_height = 11;
        let warning_x = area.x + (area.width.saturating_sub(warning_width)) / 2;
        let warning_y = area.y + (area.height.saturating_sub(warning_height)) / 2;
        
        let warning_area = Rect {
            x: warning_x,
            y: warning_y,
            width: warning_width.min(area.width),
            height: warning_height.min(area.height),
        };
        
        f.render_widget(warning_para, warning_area);
        return;
    }
    
    // Calculate content size: 50% of available space, but at least 80 pixels wide and 25 pixels tall
    let content_width = (area.width * CONTENT_WIDTH_PERCENT / 100).max(min_width_pixels).min(area.width);
    let content_height = (area.height * CONTENT_HEIGHT_PERCENT / 100).max(min_height_pixels).min(area.height);
    // Center the content (no blank lines above/below)
    let content_x = area.x + (area.width.saturating_sub(content_width)) / 2;
    let content_y = area.y + (area.height.saturating_sub(content_height)) / 2;
    
    // Ensure content area doesn't exceed bounds
    let content_area = Rect {
        x: content_x.min(area.x + area.width),
        y: content_y.min(area.y + area.height),
        width: content_width.min(area.width.saturating_sub(content_x.saturating_sub(area.x))),
        height: content_height.min(area.height.saturating_sub(content_y.saturating_sub(area.y))),
    };
    
    // Ensure content_area is valid before splitting
    if content_area.width == 0 || content_area.height == 0 {
        return;
    }
    
    // Calculate profile box width based on longest profile name + 2 spaces
    // Also account for title and minimum width for instructions
    let profile_state_guard = profile_state.profiles.lock().unwrap();
    let max_profile_name_len = profile_state_guard.iter()
        .map(|name| name.len())
        .max()
        .unwrap_or(0);
    drop(profile_state_guard);
    
    // Calculate profile box width based on longest profile name
    // Account for: "> " (highlight, 2 chars) + "  " (padding, 2 chars) + name + 1 space + borders (2)
    // Total: 2 + 2 + name_len + 1 + 2 = name_len + 7
    let profile_box_width = max_profile_name_len + 7;
    
    // Split into left (Profiles) and right (Configuration) sections - NO CENTERING, like dashboard
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(profile_box_width as u16), // Profile box (fixed width)
            Constraint::Min(0),                            // Configuration section (remaining)
        ])
        .split(area);
    
    let profile_area = columns[0];
    let config_area = columns[1];
    
    // Split configuration: Sketch boxes (full width) at top, then 3-column bottom (Device | Connection | MQTT)
    let config_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Sketch boxes (2 x 3 lines)
            Constraint::Min(0),    // Bottom sections
        ])
        .split(config_area);
    
    // Sketch boxes - FULL WIDTH like dashboard status box
    let sketch_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Sketch Directory
            Constraint::Length(3), // Sketch Name
        ])
        .split(config_chunks[0]);
    
    register_or_update(registry, HWND_SETTINGS_FIELD_SKETCH_DIR, sketch_chunks[0]);
    render_full_width_field(f, sketch_chunks[0], settings, fields, editor_state, 0, "Sketch Directory", dimming);
    
    register_or_update(registry, HWND_SETTINGS_FIELD_SKETCH_NAME, sketch_chunks[1]);
    render_full_width_field(f, sketch_chunks[1], settings, fields, editor_state, 1, "Sketch Name", dimming);
    
    // Bottom section: 3 columns - Device | Connection | MQTT (2 sub-columns)
    let bottom_columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Device
            Constraint::Percentage(25), // Connection  
            Constraint::Percentage(50), // MQTT (will be split into 2 sub-columns)
        ])
        .split(config_chunks[1]);
    
    // Device section: Environment, Board Model, FQBN - FULL HEIGHT
    render_section(f, bottom_columns[0], settings, fields, editor_state, "Device", &[2, 3, 4], None, registry, dimming);
    register_or_update(registry, HWND_SETTINGS_SECTION_DEVICE, bottom_columns[0]);
    
    // Connection section: Port, Baud Rate - FULL HEIGHT
    render_section(f, bottom_columns[1], settings, fields, editor_state, "Connection", &[5, 6], None, registry, dimming);
    register_or_update(registry, HWND_SETTINGS_SECTION_CONNECTION, bottom_columns[1]);

    // MQTT section: 2 sub-columns (Credentials | Topics)
    let mqtt_columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Credentials: Host, Port, Username, Password
            Constraint::Percentage(50), // Topics: Command, State, Status
        ])
        .split(bottom_columns[2]);
    
    // MQTT Credentials column - FULL HEIGHT
    render_section(f, mqtt_columns[0], settings, fields, editor_state, "MQTT", &[7, 8, 9, 10], None, registry, dimming);
    
    // MQTT Topics column - FULL HEIGHT
    render_section(f, mqtt_columns[1], settings, fields, editor_state, "Topics", &[11, 12, 13], None, registry, dimming);
    
    // Register combined MQTT section (full height)
    register_or_update(registry, HWND_SETTINGS_SECTION_MQTT, bottom_columns[2]);
    
    // Render profile box - FULL HEIGHT on the left (top-aligned, like dashboard commands)
    register_or_update(registry, HWND_PROFILE_BOX, profile_area);
    render_profile_box(f, profile_area, profile_state, registry, dimming);
}

/// Render a full-width field (for Sketch Directory and Sketch Name)
fn render_full_width_field(
    f: &mut Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
    field_index: usize,
    title: &str,
    dimming: &DimmingContext,
) {
    // Ensure area is valid
    if area.width == 0 || area.height == 0 {
        return;
    }
    
    let is_selected = matches!(editor_state, FieldEditorState::Selected { field_index: idx } if *idx == field_index);
    let is_editing = matches!(editor_state, FieldEditorState::Editing { field_index: idx, .. } if *idx == field_index);
    let value = fields.get_value(settings, field_index);
    
    // Get inner area for text (accounting for borders)
    let inner_area = Block::default().borders(Borders::ALL).inner(area);
    let text_width = inner_area.width as usize;
    
    let (display_value, _scroll_offset) = if is_editing {
        if let FieldEditorState::Editing { input, .. } = editor_state {
            let scroll = input.visual_scroll(text_width);
            let value_str = input.value();
            // Get the visible portion of the text
            let chars: Vec<char> = value_str.chars().collect();
            let visible_start = scroll.min(chars.len());
            let visible_end = (visible_start + text_width).min(chars.len());
            let visible_text: String = chars[visible_start..visible_end].iter().collect();
            (visible_text, scroll)
        } else {
            (value, 0)
        }
    } else {
        (value, 0)
    };
    
    // Border color: #666666 (RGB 102, 102, 102) for box characters
    let border_color = if dimming.modal_visible {
        hex_color(0x222222)
    } else if is_editing {
        Color::Cyan // Cyan when editing
    } else if is_selected {
        Color::White // White when selected but not editing
    } else {
        Color::Rgb(102, 102, 102) // Gray when inactive
    };
    
    // Title color: white for text
    let title_color = if dimming.modal_visible { hex_color(0x444444) } else { Color::White };
    
    // Value color: cyan when editing, white when not
    let text_color = if dimming.modal_visible {
        hex_color(0x444444)
    } else if is_editing {
        Color::Cyan
    } else {
        Color::White
    };
    
    let block = Block::default()
        .title(Span::styled(format!(" {} ", title), Style::default().fg(title_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0));
    
    let para = Paragraph::new(display_value)
        .style(Style::default().fg(text_color))
        .block(block);
    
    f.render_widget(para, area);
}

/// Render a section with nested fields (Device or Connection)
/// Returns the height used for the section
fn render_section(
    f: &mut Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
    section_title: &str,
    field_indices: &[usize],
    target_height: Option<u16>,
    registry: &mut RectRegistry,
    dimming: &DimmingContext,
) -> u16 {
    // Ensure area is valid
    if area.width == 0 || area.height == 0 {
        return 0;
    }

    // Border color: #666666 (RGB 102, 102, 102) for box characters
    let border_color = if dimming.modal_visible { hex_color(0x222222) } else { Color::Rgb(102, 102, 102) };
    
    // Section title style
    let section_title_style = Style::default().fg(if dimming.modal_visible { hex_color(0x444444) } else { Color::Cyan });
    
    // Calculate field height (3 lines per field)
    let field_height = FIELD_HEIGHT;
    let spacing = FIELD_SPACING; // Spacing between fields
    
    // Use the full area height (sections now span to bottom)
    let section_area = area;
    
    // Outer section block with white title and gray border
    let section_block = Block::default()
        .title(Span::styled(format!(" {} ", section_title), section_title_style))
        .borders(Borders::ALL)
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0))
        .border_style(Style::default().fg(border_color));
    
    // Inner area for nested fields
    let inner_area = section_block.inner(section_area);
    
    let mut y_offset = 0; // Start at top of inner area
    
    // Map field indices to HWND constants
    let field_hwnds = [
        HWND_SETTINGS_FIELD_ENV,
        HWND_SETTINGS_FIELD_BOARD_MODEL,
        HWND_SETTINGS_FIELD_FQBN,
        HWND_SETTINGS_FIELD_PORT,
        HWND_SETTINGS_FIELD_BAUDRATE,
        HWND_SETTINGS_FIELD_MQTT_HOST,
        HWND_SETTINGS_FIELD_MQTT_PORT,
        HWND_SETTINGS_FIELD_MQTT_USERNAME,
        HWND_SETTINGS_FIELD_MQTT_PASSWORD,
        HWND_SETTINGS_FIELD_MQTT_TOPIC_COMMAND,
        HWND_SETTINGS_FIELD_MQTT_TOPIC_STATE,
        HWND_SETTINGS_FIELD_MQTT_TOPIC_STATUS,
    ];
    
    for &field_index in field_indices {
        if field_index >= fields.count() {
            break;
        }
        
        let field_area = Rect {
            x: inner_area.x,
            y: inner_area.y + y_offset,
            width: inner_area.width,
            height: field_height as u16,
        };
        
        // Register field with HWND (field_index 2-13 map to hwnds 0-11)
        if field_index >= 2 && field_index <= 13 {
            let hwnd_index = field_index - 2;
            if hwnd_index < field_hwnds.len() {
                register_or_update(registry, field_hwnds[hwnd_index], field_area);
            }
        }
        
        render_nested_field(f, field_area, settings, fields, editor_state, field_index, dimming);
        y_offset += field_height as u16 + spacing as u16; // Add spacing between fields
    }
    
    // Render the section block with calculated height
    f.render_widget(section_block, section_area);
    
    // Return the height used
    section_area.height
}

/// Render a nested field inside a section
fn render_nested_field(
    f: &mut Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
    field_index: usize,
    dimming: &DimmingContext,
) {
    // Ensure area is valid
    if area.width == 0 || area.height == 0 {
        return;
    }
    
    let label = fields.get_label(field_index);
    let value = fields.get_value(settings, field_index);
    let is_selected = matches!(editor_state, FieldEditorState::Selected { field_index: idx } if *idx == field_index);
    let is_editing = matches!(editor_state, FieldEditorState::Editing { field_index: idx, .. } if *idx == field_index);
    let _is_selecting = matches!(editor_state, FieldEditorState::Selecting { field_index: idx, .. } if *idx == field_index);
    
    // Get inner area for text (accounting for borders)
    let inner_area = Block::default().borders(Borders::ALL).inner(area);
    let text_width = inner_area.width as usize;
    
    // Render the field normally first
    let (display_value, _scroll_offset) = if is_editing {
        if let FieldEditorState::Editing { input, .. } = editor_state {
            let scroll = input.visual_scroll(text_width);
            let value_str = input.value();
            // Get the visible portion of the text
            let chars: Vec<char> = value_str.chars().collect();
            let visible_start = scroll.min(chars.len());
            let visible_end = (visible_start + text_width).min(chars.len());
            let visible_text: String = chars[visible_start..visible_end].iter().collect();
            (visible_text, scroll)
        } else {
            (value.clone(), 0)
        }
    } else {
        (value.clone(), 0)
    };
    
    // Label color: grey
    let label_color = if dimming.modal_visible { hex_color(0x444444) } else { Color::Rgb(153, 153, 153) };
    
    // Value color: white, but cyan when editing
    let value_color = if dimming.modal_visible {
        hex_color(0x444444)
    } else if is_editing {
        Color::Cyan
    } else {
        Color::White
    };
    
    // Selection highlight color (for the '>' symbol or bracket)
    let highlight_color = if dimming.modal_visible {
        hex_color(0x222222)
    } else if is_editing {
        Color::Cyan
    } else if is_selected {
        Color::White
    } else {
        Color::White // Match title bar style (bright when undimmed)
    };
    
    let block = Block::default()
        .title(Span::styled(format!(" {} ", label), Style::default().fg(label_color)))
        .borders(Borders::ALL)
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0))
        .border_style(Style::default().fg(highlight_color));
    
    let para = Paragraph::new(display_value)
        .style(Style::default().fg(value_color))
        .block(block);
    
    f.render_widget(para, area);
    
    // Note: Dropdown overlay is rendered in main loop after all widgets
    // to ensure it appears on top
}

/// Render the profile box
fn render_profile_box(
    f: &mut Frame,
    area: Rect,
    profile_state: &ProfileState,
    registry: &mut RectRegistry,
    dimming: &DimmingContext,
) {
    // Ensure area is valid
    if area.width == 0 || area.height == 0 {
        return;
    }
    
    // Get profile list and state
    let profiles = profile_state.profiles.lock().unwrap();
    let selected_index = profile_state.selected_index.lock().unwrap();
    let is_active = profile_state.is_active.lock().unwrap();
    
    // Border color: #666666 (RGB 102, 102, 102) for box characters
    let border_color = if dimming.modal_visible {
        hex_color(0x222222)
    } else if *is_active {
        Color::White          // White when focused
        .add_modifier(Modifier::BOLD)
    } else {
        hex_color(0x777777)  // Brighter grey when unfocused (match title bar style)
    };
    
    // Title color: white for text
    let title_color = if dimming.modal_visible { hex_color(0x444444) } else { Color::White };
    
    // Create profile list items
    let list_items: Vec<ListItem> = profiles.iter()
        .map(|name| {
            ListItem::new(Span::styled(
                format!("  {}", name),
                Style::default().fg(Color::White),
            ))
        })
        .collect();
    
    // Create list state with selected index
    let mut list_state = ListState::default();
    if let Some(idx) = *selected_index {
        if idx < list_items.len() {
            list_state.select(Some(idx));
        }
    }
    
    // Create title with active indicator
    let title = if *is_active {
        " Profiles [Active] "
    } else {
        " Profiles "
    };
    
    // Create the list widget
    let list = List::new(list_items)
        .block(
            Block::default()
                .title(Span::styled(title, Style::default().fg(title_color)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .padding(ratatui::widgets::Padding::new(1, 1, 0, 0))
        )
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(255, 215, 0)) // Gold for selected
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("> ");
    
    // Render the list using full height (top-aligned)
    let list_area = area;
    
    // Register profile list
    register_or_update(registry, HWND_PROFILE_LIST, list_area);
    
    f.render_stateful_widget(list, list_area, &mut list_state);
}
