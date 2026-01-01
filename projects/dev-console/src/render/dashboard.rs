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
        // Show progress bar with time estimates if available
        let progress_text = if let Some(ref tracker) = dashboard_state.progress_tracker {
            let elapsed = tracker.format_elapsed();
            let eta = tracker.format_estimated_remaining()
                .map(|r| format!(" | ETA: {}", r))
                .unwrap_or_default();
            
            if dashboard_state.current_file.is_empty() {
                format!("{}: {:.1}% | Elapsed: {}{}", 
                    tracker.current_stage_name(), 
                    tracker.progress_percent, 
                    elapsed,
                    eta
                )
            } else {
                format!("{}: {:.1}% | Elapsed: {}{} | {}", 
                    tracker.current_stage_name(), 
                    tracker.progress_percent, 
                    elapsed,
                    eta,
                    dashboard_state.current_file.as_ref()
                )
            }
        } else {
            // Fallback to basic progress display
            if dashboard_state.current_file.is_empty() {
                format!("{}: {:.1}%", dashboard_state.progress_stage.as_ref(), dashboard_state.progress_percent)
            } else {
                format!("{}: {:.1}% - {}", dashboard_state.progress_stage.as_ref(), dashboard_state.progress_percent, dashboard_state.current_file.as_ref())
            }
        };
        
        // Create progress bar
        let progress_width = status_inner.width as usize;
        let filled_width = ((progress_width as f64 * dashboard_state.progress_percent / 100.0) as usize).min(progress_width);
        let empty_width = progress_width.saturating_sub(filled_width);
        
        let progress_bar = format!(
            "{}{}",
            "█".repeat(filled_width),
            "░".repeat(empty_width)
        );
        
        let progress_lines = vec![
            Line::from(Span::styled(
                progress_text,
                Style::default().fg(Color::Cyan),
            )),
            Line::from(Span::styled(
                progress_bar,
                Style::default().fg(Color::Green),
            )),
        ];
        
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
