// Dashboard panel rendering

use crate::dashboard::{DashboardState, SCROLL_TO_BOTTOM};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

/// Parse a line with ANSI color codes and convert to ratatui Line
fn parse_ansi_line(line: &str) -> Line<'static> {
    // Simple ANSI code parser - preserves color codes
    // If line contains ANSI codes, parse them; otherwise use plain text
    if line.contains('\x1b') || line.contains('\u{001b}') {
        // Line contains ANSI escape sequences - parse them
        parse_ansi_to_spans(line)
    } else {
        // No ANSI codes - use plain text (convert to owned String for 'static)
        Line::from(Span::raw(line.to_string()))
    }
}

/// Parse ANSI escape sequences and convert to ratatui Spans
fn parse_ansi_to_spans(text: &str) -> Line<'static> {
    use regex::Regex;
    use lazy_static::lazy_static;
    
    lazy_static! {
        // Match ANSI escape sequences: \x1b[ followed by codes and ending with m
        static ref ANSI_REGEX: Regex = Regex::new(r"\x1b\[([0-9;]*)([a-zA-Z])").unwrap();
    }
    
    let mut spans = Vec::new();
    let mut last_end = 0;
    let mut current_style = Style::default();
    
    for cap in ANSI_REGEX.captures_iter(text) {
        let full_match = cap.get(0).unwrap();
        let codes = cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let command = cap.get(2).map(|m| m.as_str()).unwrap_or("");
        
        // Add text before this ANSI code
        if full_match.start() > last_end {
            let text_before = &text[last_end..full_match.start()];
            if !text_before.is_empty() {
                spans.push(Span::styled(text_before.to_string(), current_style));
            }
        }
        
        // Parse ANSI codes and update style
        if command == "m" {
            current_style = parse_ansi_codes_to_style(codes, current_style);
        }
        
        last_end = full_match.end();
    }
    
    // Add remaining text (convert to owned String for 'static)
    if last_end < text.len() {
        let remaining = &text[last_end..];
        if !remaining.is_empty() {
            spans.push(Span::styled(remaining.to_string(), current_style));
        }
    }
    
    // If no spans were created (no ANSI codes matched), return plain text
    if spans.is_empty() {
        Line::from(Span::raw(text.to_string()))
    } else {
        Line::from(spans)
    }
}

/// Parse ANSI color codes and convert to ratatui Style
fn parse_ansi_codes_to_style(codes: &str, mut current_style: Style) -> Style {
    if codes.is_empty() {
        // Reset
        return Style::default();
    }
    
    let code_parts: Vec<&str> = codes.split(';').collect();
    let mut i = 0;
    
    while i < code_parts.len() {
        let code = code_parts[i].parse::<u8>().unwrap_or(0);
        
        match code {
            0 => {
                // Reset
                current_style = Style::default();
            }
            1 => {
                // Bold
                current_style = current_style.add_modifier(Modifier::BOLD);
            }
            30..=37 => {
                // Foreground color (standard)
                current_style = current_style.fg(parse_ansi_color(code - 30));
            }
            38 => {
                // Extended foreground color
                if i + 1 < code_parts.len() {
                    let color_type = code_parts[i + 1].parse::<u8>().unwrap_or(0);
                    if color_type == 5 && i + 2 < code_parts.len() {
                        // 256-color mode
                        let color_code = code_parts[i + 2].parse::<u16>().unwrap_or(0);
                        current_style = current_style.fg(parse_256_color(color_code));
                        i += 2;
                    } else if color_type == 2 && i + 4 < code_parts.len() {
                        // RGB mode
                        let r = code_parts[i + 2].parse::<u8>().unwrap_or(0);
                        let g = code_parts[i + 3].parse::<u8>().unwrap_or(0);
                        let b = code_parts[i + 4].parse::<u8>().unwrap_or(0);
                        current_style = current_style.fg(Color::Rgb(r, g, b));
                        i += 4;
                    }
                }
            }
            39 => {
                // Default foreground
                current_style = current_style.fg(Color::Reset);
            }
            90..=97 => {
                // Bright foreground color
                current_style = current_style.fg(parse_ansi_color(code - 90 + 8));
            }
            _ => {}
        }
        
        i += 1;
    }
    
    current_style
}

/// Parse standard ANSI color code (0-15)
fn parse_ansi_color(code: u8) -> Color {
    match code {
        0 => Color::Black,
        1 => Color::Red,
        2 => Color::Green,
        3 => Color::Yellow,
        4 => Color::Blue,
        5 => Color::Magenta,
        6 => Color::Cyan,
        7 => Color::White,
        8 => Color::DarkGray,
        9 => Color::LightRed,
        10 => Color::LightGreen,
        11 => Color::LightYellow,
        12 => Color::LightBlue,
        13 => Color::LightMagenta,
        14 => Color::LightCyan,
        15 => Color::White,
        _ => Color::Reset,
    }
}

/// Parse 256-color code
fn parse_256_color(code: u16) -> Color {
    if code < 16 {
        parse_ansi_color(code as u8)
    } else if code < 232 {
        // 6x6x6 color cube
        let r = ((code - 16) / 36) * 51;
        let g = (((code - 16) / 6) % 6) * 51;
        let b = ((code - 16) % 6) * 51;
        Color::Rgb(r as u8, g as u8, b as u8)
    } else {
        // Grayscale
        let gray = ((code - 232) * 10 + 8) as u8;
        Color::Rgb(gray, gray, gray)
    }
}

/// Render dashboard panel
pub fn render_dashboard(
    f: &mut Frame,
    area: Rect,
    dashboard_state: &mut DashboardState,
) {
    // Ensure area is valid
    if area.width == 0 || area.height == 0 {
        return;
    }
    
    // Calculate commands box width: longest command + 4 spaces
    let max_command_width = dashboard_state.commands
        .iter()
        .map(|cmd| cmd.len())
        .max()
        .unwrap_or(10);
    let commands_box_width = ((max_command_width + 4) as u16).min(area.width);
    
    // Split into two columns
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(commands_box_width), // Column 1: Commands (fixed width)
            Constraint::Min(0),                      // Column 2: Status and Output (remaining)
        ])
        .split(area);
    
    // Column 1: Command list
    let command_items: Vec<ListItem> = dashboard_state.commands
        .iter()
        .enumerate()
        .map(|(idx, cmd)| {
            let style = if idx == dashboard_state.selected_command {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::White)
            };
            ListItem::new(Line::from(Span::styled(cmd.clone(), style)))
        })
        .collect();
    
    let command_list = List::new(command_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(" Commands ", Style::default().fg(Color::White)))
            .border_style(Style::default().fg(Color::Rgb(102, 102, 102)))
            .padding(ratatui::widgets::Padding::new(1, 1, 0, 0)));
    
    f.render_widget(command_list, columns[0]);
    
    // Column 2: Split into status bar and output
    let column2_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Status bar (4 lines: border + text + progress bar + border)
            Constraint::Min(0),     // Output (remaining space)
        ])
        .split(columns[1]);
    
    // Status bar box - show progress bar if running progress command
    let status_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(" Status ", Style::default().fg(Color::White)))
        .border_style(Style::default().fg(Color::Rgb(102, 102, 102)))
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0));
    
    let status_inner = status_block.inner(column2_chunks[0]);
    
    if dashboard_state.is_running && !dashboard_state.progress_stage.is_empty() {
        // Show progress with time estimates in the proposed format
        // Format: 
        // Line 1: Compiling: 45.2% | Elapsed: 2m 15s | ETA: 2m 45s
        // Line 2: [████████████████░░░░░░░░░░░░░░░░] 45.2%
        // Line 3: Current file: main.cpp
        // Line 4: Files compiled: 12/27
        
        let (line1, line2, line3, line4) = if let Some(ref tracker) = dashboard_state.progress_tracker {
            let elapsed = tracker.format_elapsed();
            let eta = tracker.format_estimated_remaining()
                .map(|r| format!(" | ETA: {}", r))
                .unwrap_or_default();
            
            // Line 1: Compiling: 45.2% | Elapsed: 2m 15s | ETA: 2m 45s
            let line1 = format!("{}: {:.1}% | Elapsed: {}{}", 
                tracker.current_stage_name(), 
                tracker.progress_percent, 
                elapsed,
                eta
            );
            
            // Line 2: [████████████████░░░░░░░░░░░░░░░░] 45.2%
            // Calculate width: reserve space for brackets, percentage text, and padding
            let percent_text = format!("{:.1}%", tracker.progress_percent);
            let percent_text_width = percent_text.len();
            let progress_width = (status_inner.width as usize).saturating_sub(percent_text_width + 3); // [ ] + space + percentage
            let filled_width = ((progress_width as f64 * tracker.progress_percent / 100.0) as usize).min(progress_width);
            let empty_width = progress_width.saturating_sub(filled_width);
            let line2 = format!("[{}{}] {}",
                "█".repeat(filled_width),
                "░".repeat(empty_width),
                percent_text
            );
            
            // Line 3: Current file: main.cpp (if available)
            let line3 = if !dashboard_state.current_file.is_empty() {
                format!("Current file: {}", dashboard_state.current_file.as_ref())
            } else {
                String::new()
            };
            
            // Line 4: Files compiled: 12/27 (if available from tracker)
            let line4 = if let Some(total) = tracker.total_items {
                if total > 0 {
                    format!("Files compiled: {}/{}", tracker.items_processed, total)
                } else {
                    String::new()
                }
            } else {
                String::new()
            };
            
            (line1, line2, line3, line4)
        } else {
            // Fallback to basic progress display
            let line1 = format!("{}: {:.1}%", 
                dashboard_state.progress_stage.as_ref(), 
                dashboard_state.progress_percent
            );
            
            let percent_text = format!("{:.1}%", dashboard_state.progress_percent);
            let percent_text_width = percent_text.len();
            let progress_width = (status_inner.width as usize).saturating_sub(percent_text_width + 3);
            let filled_width = ((progress_width as f64 * dashboard_state.progress_percent / 100.0) as usize).min(progress_width);
            let empty_width = progress_width.saturating_sub(filled_width);
            let line2 = format!("[{}{}] {}",
                "█".repeat(filled_width),
                "░".repeat(empty_width),
                percent_text
            );
            
            let line3 = if !dashboard_state.current_file.is_empty() {
                format!("Current file: {}", dashboard_state.current_file.as_ref())
            } else {
                String::new()
            };
            
            (line1, line2, line3, String::new())
        };
        
        // Build progress lines - exactly as proposed
        let mut progress_lines = vec![
            Line::from(Span::styled(
                line1,
                Style::default().fg(Color::Cyan),
            )),
            Line::from(Span::styled(
                line2,
                Style::default().fg(Color::Green),
            )),
        ];
        
        if !line3.is_empty() {
            progress_lines.push(Line::from(Span::styled(
                line3,
                Style::default().fg(Color::White),
            )));
        }
        
        if !line4.is_empty() {
            progress_lines.push(Line::from(Span::styled(
                line4,
                Style::default().fg(Color::White),
            )));
        }
        
        let status_para = Paragraph::new(progress_lines)
            .block(status_block)
            .style(Style::default().fg(Color::White));
        
        f.render_widget(status_para, column2_chunks[0]);
    } else {
        // Show regular status text
        let status_para = Paragraph::new(dashboard_state.status_text.as_ref())
            .block(status_block)
            .style(Style::default().fg(Color::White));
        
        f.render_widget(status_para, column2_chunks[0]);
    }
    
    // Output box with scrolling
    let output_area = column2_chunks[1];
    let output_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(" Output ", Style::default().fg(Color::White)))
        .border_style(Style::default().fg(Color::Rgb(102, 102, 102)))
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0));
    let output_inner = output_block.inner(output_area);
    
    // Calculate visible lines
    let visible_height = output_inner.height as usize;
    let total_lines = dashboard_state.output_lines.len();
    
    // Calculate maximum scroll position (0-based index of first visible line when at bottom)
    // If total_lines <= visible_height, max_scroll is 0 (no scrolling needed)
    let max_scroll = if total_lines > visible_height {
        total_lines - visible_height
    } else {
        0
    };
    
    // Handle auto-scroll: if scroll position is SCROLL_TO_BOTTOM sentinel, scroll to bottom
    // This is much simpler than checking "is_at_bottom" with tolerance
    if dashboard_state.output_scroll == SCROLL_TO_BOTTOM {
        dashboard_state.scroll_to_bottom(visible_height);
    }
    
    // Ensure scroll position is valid (clamp to valid range [0, max_scroll])
    dashboard_state.output_scroll = dashboard_state.output_scroll.min(max_scroll);
    
    // Get visible lines
    let start_line = dashboard_state.output_scroll;
    let end_line = (start_line + visible_height).min(total_lines);
    
    // Parse ANSI color codes and convert to ratatui Spans
    let visible_lines: Vec<Line> = if dashboard_state.output_lines.is_empty() {
        vec![Line::from(Span::styled(
            "No output yet. Select a command to run.",
            Style::default().fg(Color::Rgb(128, 128, 128)),
        ))]
    } else {
        dashboard_state.output_lines[start_line..end_line]
            .iter()
            .map(|line| {
                // Parse ANSI codes in the line and convert to Spans
                parse_ansi_line(line)
            })
            .collect()
    };
    
    // Render the block (borders and title) to the full area
    f.render_widget(output_block.clone(), output_area);
    
    // Create content area that's one column narrower to leave space for scrollbar
    // This ensures content doesn't overlap with the scrollbar
    let content_area = if total_lines > visible_height {
        // Leave one column for scrollbar
        Rect {
            x: output_inner.x,
            y: output_inner.y,
            width: output_inner.width.saturating_sub(1),
            height: output_inner.height,
        }
    } else {
        // No scrollbar, use full width
        output_inner
    };
    
    // Render content without block (block already rendered above)
    let output_para = Paragraph::new(visible_lines)
        .style(Style::default().fg(Color::White));
    
    f.render_widget(output_para, content_area);
    
    // Render scrollbar if there are more lines than visible
    if total_lines > visible_height {
        // Position scrollbar on the right edge of the inner content area
        // The scrollbar should extend the full height from top to bottom of inner area
        // output_inner already accounts for padding, so use its full height
        let scrollbar_area = Rect {
            x: output_inner.x + output_inner.width.saturating_sub(1),
            y: output_inner.y,  // Start at top of inner area (after top padding)
            width: 1,
            height: output_inner.height,  // Full height of inner area (no gap at bottom)
        };
        
        // Scrollbar calculation for ratatui ScrollbarState:
        // According to ratatui docs, ScrollbarState expects:
        // - content_length: Total number of items in the content (total_lines)
        // - viewport_content_length: Number of items visible in viewport (visible_height)
        // - position: The current scroll position (index of first visible item)
        //
        // The scrollbar automatically calculates:
        // - Thumb size = (viewport_length / content_length) * scrollbar_height
        // - Thumb position = (position / (content_length - viewport_length)) * available_track_height
        //
        // CRITICAL: When at bottom, position must be exactly max_scroll to show thumb at bottom
        // The scrollbar calculates: thumb_pos = (position / max_scrollable) * track_height
        // When position = max_scroll, thumb should be at bottom of track
        let content_length = total_lines;
        let viewport_length = visible_height;
        // Position is already set correctly by scroll_to_bottom() when SCROLL_TO_BOTTOM was detected
        // Just ensure it's clamped to valid range
        let position = dashboard_state.output_scroll.min(max_scroll);
        
        // Create scrollbar state - must be created fresh each render to ensure correct calculation
        let mut scrollbar_state = ScrollbarState::new(content_length)
            .viewport_content_length(viewport_length)
            .position(position);
        
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .thumb_symbol("█")
            .track_symbol(Some("│"));
        
        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}