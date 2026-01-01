// Settings panel rendering

use crate::settings::Settings;
use crate::field_editor::{FieldEditorState, SettingsFields};
use crate::constants::*;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// Render settings panel
pub fn render_settings(
    f: &mut Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
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
    
    // Split into top section (Sketch Directory, Sketch Name) and bottom section (Device/Connection)
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Top section: 2 boxes (3 lines each)
            Constraint::Min(0),   // Bottom section: Device/Connection
        ])
        .split(content_area);
    
    // Render top section: Sketch Directory and Sketch Name
    let top_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Sketch Directory
            Constraint::Length(3), // Sketch Name
        ])
        .split(main_chunks[0]);
    
    render_full_width_field(f, top_chunks[0], settings, fields, editor_state, 0, "Sketch Directory");
    render_full_width_field(f, top_chunks[1], settings, fields, editor_state, 1, "Sketch Name");
    
    // Render bottom section: Device and Connection columns
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Device column
            Constraint::Percentage(50), // Connection column
        ])
        .split(main_chunks[1]);
    
    // Device section: Environment, Board Model, FQBN
    let device_height = render_section(f, bottom_chunks[0], settings, fields, editor_state, "Device", &[2, 3, 4], None);
    
    // Connection section: Port, Baud Rate - match Device height
    render_section(f, bottom_chunks[1], settings, fields, editor_state, "Connection", &[5, 6], Some(device_height));
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
    
    // Border color: #666666 (RGB 102, 102, 102) for box characters by default
    // Title color: white for text
    let border_color = if is_editing {
        Color::Rgb(255, 215, 0) // Gold when editing
    } else if is_selected {
        Color::Cyan // Cyan when selected
    } else {
        Color::Rgb(102, 102, 102) // #666666 by default
    };
    
    let title_color = Color::White;
    
    let text_color = if is_editing {
        Color::Rgb(255, 215, 0) // Gold when editing
    } else if is_selected {
        Color::Cyan // Cyan when selected
    } else {
        Color::White // White by default
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
) -> u16 {
    // Ensure area is valid
    if area.width == 0 || area.height == 0 {
        return 0;
    }
    
    // Border color: #666666 (RGB 102, 102, 102) for box characters
    let border_color = Color::Rgb(102, 102, 102);
    
    // Title color: white for text
    let title_color = Color::White;
    
    // Calculate field height (3 lines per field)
    let field_height = FIELD_HEIGHT;
    let spacing = FIELD_SPACING; // Spacing between fields
    let total_fields = field_indices.len();
    
    // Calculate exact height needed: (field_height * total_fields) + (spacing * (total_fields - 1)) + borders (2)
    let mut needed_height = (field_height * total_fields as u16) + (spacing * total_fields.saturating_sub(1) as u16) + 2;
    
    // If target_height is provided (for matching heights), use it
    // Otherwise, if this is Device section, add 1 extra row
    let final_height = if let Some(target) = target_height {
        target
    } else if section_title == "Device" {
        needed_height += 1; // Device box needs 1 extra row
        needed_height as u16
    } else {
        needed_height as u16
    };
    
    // Use the calculated height, not the full area
    let section_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: final_height.min(area.height),
    };
    
    // Outer section block with white title and gray border
    let section_block = Block::default()
        .title(Span::styled(format!(" {} ", section_title), Style::default().fg(title_color)))
        .borders(Borders::ALL)
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0))
        .border_style(Style::default().fg(border_color));
    
    // Inner area for nested fields
    let inner_area = section_block.inner(section_area);
    
    let mut y_offset = 1; // Start after top border
    
    for &field_index in field_indices {
        if field_index >= fields.count() {
            break;
        }
        
        let field_area = Rect {
            x: inner_area.x + 1,
            y: inner_area.y + y_offset,
            width: inner_area.width.saturating_sub(2),
            height: field_height as u16,
        };
        
        render_nested_field(f, field_area, settings, fields, editor_state, field_index);
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
) {
    // Ensure area is valid
    if area.width == 0 || area.height == 0 {
        return;
    }
    
    let label = fields.get_label(field_index);
    let value = fields.get_value(settings, field_index);
    let is_selected = matches!(editor_state, FieldEditorState::Selected { field_index: idx } if *idx == field_index);
    let is_editing = matches!(editor_state, FieldEditorState::Editing { field_index: idx, .. } if *idx == field_index);
    let is_selecting = matches!(editor_state, FieldEditorState::Selecting { field_index: idx, .. } if *idx == field_index);
    
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
    
    // Border color: #666666 (RGB 102, 102, 102) for box characters by default
    // Title color: white for text
    let border_color = if is_editing || is_selecting {
        Color::Rgb(255, 215, 0) // Gold when editing or selecting
    } else if is_selected {
        Color::Cyan // Cyan when selected
    } else {
        Color::Rgb(102, 102, 102) // #666666 by default
    };
    
    let title_color = Color::White;
    
    let text_color = if is_editing || is_selecting {
        Color::Rgb(255, 215, 0) // Gold when editing or selecting
    } else if is_selected {
        Color::Cyan // Cyan when selected
    } else {
        Color::White // White by default
    };
    
    let block = Block::default()
        .title(Span::styled(format!(" {} ", label), Style::default().fg(title_color)))
        .borders(Borders::ALL)
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0))
        .border_style(Style::default().fg(border_color));
    
    let para = Paragraph::new(display_value)
        .style(Style::default().fg(text_color))
        .block(block);
    
    f.render_widget(para, area);
    
    // Note: Dropdown overlay is rendered in main loop after all widgets
    // to ensure it appears on top
}
