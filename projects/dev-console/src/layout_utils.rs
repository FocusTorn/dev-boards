// Layout calculation utilities

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::Block;
use crate::constants::*;

/// Calculate centered content area within a given rect
#[allow(dead_code)]
pub fn calculate_centered_content_area(area: Rect) -> Option<Rect> {
    // Check if terminal is large enough
    if area.width < MIN_WIDTH_PIXELS || area.height < MIN_HEIGHT_PIXELS {
        return None;
    }
    
    let content_width = (area.width * CONTENT_WIDTH_PERCENT / 100).max(MIN_WIDTH_PIXELS).min(area.width);
    let content_height = (area.height * CONTENT_HEIGHT_PERCENT / 100).max(MIN_HEIGHT_PIXELS).min(area.height);
    let content_x = area.x + (area.width.saturating_sub(content_width)) / 2;
    let content_y = area.y + (area.height.saturating_sub(content_height)) / 2;
    
    Some(Rect {
        x: content_x.min(area.x + area.width),
        y: content_y.min(area.y + area.height),
        width: content_width.min(area.width.saturating_sub(content_x.saturating_sub(area.x))),
        height: content_height.min(area.height.saturating_sub(content_y.saturating_sub(area.y))),
    })
}

/// Calculate field area for dropdown positioning
#[allow(dead_code)]
pub fn calculate_field_area(
    content_area: Rect,
    field_index: usize,
) -> Option<Rect> {
    // Split into top section (Sketch Directory, Sketch Name) and bottom section (Device/Connection)
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Top section: 2 boxes (3 lines each)
            Constraint::Min(0),   // Bottom section: Device/Connection
        ])
        .split(content_area);
    
    if field_index < 2 {
        // Top full-width fields
        let top_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Sketch Directory
                Constraint::Length(3), // Sketch Name
            ])
            .split(main_chunks[0]);
        Some(top_chunks[field_index])
    } else if field_index < 5 {
        // Device column (left)
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Device column
                Constraint::Percentage(50), // Connection column
            ])
            .split(main_chunks[1]);
        
        // Device section: Environment (2), Board Model (3), FQBN (4)
        let section_inner = Block::default().borders(ratatui::widgets::Borders::ALL).inner(bottom_chunks[0]);
        let field_offset = (field_index - 2) as u16 * (FIELD_HEIGHT + FIELD_SPACING);
        Some(Rect {
            x: section_inner.x + 1,
            y: section_inner.y + 1 + field_offset,
            width: section_inner.width.saturating_sub(2),
            height: FIELD_HEIGHT,
        })
    } else {
        // Connection column (right)
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Device column
                Constraint::Percentage(50), // Connection column
            ])
            .split(main_chunks[1]);
        
        // Connection section: Port (5), Baudrate (6)
        let section_inner = Block::default().borders(ratatui::widgets::Borders::ALL).inner(bottom_chunks[1]);
        let field_offset = (field_index - 5) as u16 * (FIELD_HEIGHT + FIELD_SPACING);
        Some(Rect {
            x: section_inner.x + 1,
            y: section_inner.y + 1 + field_offset,
            width: section_inner.width.saturating_sub(2),
            height: FIELD_HEIGHT,
        })
    }
}

/// Calculate dropdown area position
#[allow(dead_code)]
pub fn calculate_dropdown_area(
    field_area: Rect,
    options_count: usize,
    frame_height: u16,
) -> Rect {
    let dropdown_height = (options_count + 2).min(10) as u16;
    let dropdown_area = Rect {
        x: field_area.x,
        y: field_area.y + field_area.height,
        width: field_area.width,
        height: dropdown_height,
    };
    
    // Make sure dropdown fits in the frame
    if dropdown_area.y + dropdown_area.height > frame_height {
        // If doesn't fit below, show above
        Rect {
            x: dropdown_area.x,
            y: field_area.y.saturating_sub(dropdown_area.height),
            width: dropdown_area.width,
            height: dropdown_height,
        }
    } else {
        dropdown_area
    }
}

/// Calculate cursor position for editing fields
#[allow(dead_code)]
pub fn calculate_cursor_position(
    content_area: Rect,
    field_index: usize,
    cursor_pos: usize,
    scroll_offset: usize,
    _inner_width: usize,
) -> Option<(u16, u16)> {
    let content_x = content_area.x;
    let content_y = content_area.y;
    let content_width = content_area.width;
    
    let cursor_pos_in_view = cursor_pos.saturating_sub(scroll_offset);
    
    let (cursor_x, cursor_y) = if field_index < 2 {
        // Top section: Sketch Directory (0) or Sketch Name (1)
        let y_offset = if field_index == 0 { 0 } else { 3 };
        (content_x + 1 + cursor_pos_in_view as u16, content_y + y_offset + 1)
    } else if field_index < 5 {
        // Device section: Environment (2), Board Model (3), FQBN (4)
        let section_y = content_y + 6; // After top section
        let field_offset = ((field_index - 2) * 4) as u16; // 3 lines per field + 1 spacing
        (content_x + 1 + 1 + cursor_pos_in_view as u16, section_y + 1 + 1 + field_offset + 1)
    } else {
        // Connection section: Port (5), Baudrate (6)
        let section_y = content_y + 6; // After top section
        let section_x = content_x + content_width / 2; // Right column
        let field_offset = ((field_index - 5) * 4) as u16; // 3 lines per field + 1 spacing
        (section_x + 1 + 1 + cursor_pos_in_view as u16, section_y + 1 + 1 + field_offset + 1)
    };
    
    Some((cursor_x, cursor_y))
}
