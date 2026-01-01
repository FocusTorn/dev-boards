// Dashboard panel rendering

use crate::dashboard::DashboardState;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

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
    let max_scroll = total_lines.saturating_sub(visible_height.max(1));
    dashboard_state.output_scroll = dashboard_state.output_scroll.min(max_scroll);
    
    // Get visible lines
    let start_line = dashboard_state.output_scroll;
    let end_line = (start_line + visible_height).min(total_lines);
    
    let visible_lines: Vec<Line> = if dashboard_state.output_lines.is_empty() {
        vec![Line::from(Span::styled(
            "No output yet. Select a command to run.",
            Style::default().fg(Color::Rgb(128, 128, 128)),
        ))]
    } else {
        dashboard_state.output_lines[start_line..end_line]
            .iter()
            .map(|line| Line::from(line.clone()))
            .collect()
    };
    
    let output_para = Paragraph::new(visible_lines)
        .block(output_block.clone())
        .style(Style::default().fg(Color::White));
    
    f.render_widget(output_para, output_area);
    
    // Render scrollbar if there are more lines than visible
    if total_lines > visible_height {
        let mut scrollbar_state = ScrollbarState::new(total_lines)
            .position(dashboard_state.output_scroll);
        
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .thumb_symbol("█")
            .track_symbol(Some("│"));
        
        f.render_stateful_widget(scrollbar, output_area, &mut scrollbar_state);
    }
}
