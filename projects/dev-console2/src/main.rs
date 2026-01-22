use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect, Position},
    widgets::Block,
};


mod app;
mod config;
mod terminal;
mod widgets;
mod commands;

use app::{App, Message, ActiveBox};

fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;

    // Create application state first (so config errors print to stderr before terminal takeover)
    let mut app = App::new()?;

    // Initialize terminal
    let mut terminal = terminal::init_terminal()?;

    // Main application loop
    while app.running {
        // Render the current view
        terminal.draw(|f| app.view(f))?;

        // Get the current frame area for event handling
        let frame_area = terminal.get_frame().area(); // Changed .size() to .area()

        // Handle events and map to Message
        let mut current_msg = handle_event(&mut app, frame_area)?;

        // Process updates (allow message chaining)
        while let Some(msg) = current_msg {
            current_msg = app.update(msg);
        }

        // Handle any updates from background commands
        app.handle_command_updates();
    }

    // Restore terminal
    terminal::restore_terminal()?;
    Ok(())
}

/// Convert crossterm events to application messages
fn handle_event(app: &mut App, frame_area: Rect) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        match event::read()? {
            Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    return Ok(map_key_to_message(app, key));
                }
            }
            Event::Mouse(mouse_event) => {
                let mouse_pos = Position::new(mouse_event.column, mouse_event.row);

                let status_area = app.get_status_area(frame_area);

                // Re-calculate output area for hit testing
                let vertical_layout = Layout::vertical([
                    Constraint::Length(1), // Title Bar
                    Constraint::Min(0),    // Main Content
                    Constraint::Length(1), // Bindings
                    Constraint::Length(2), // Status Bar (bottom)
                ]);
                let [_, main_area, _, _] = vertical_layout.areas(frame_area);

                let mut effective_inner_area = main_area;
                if let Some(tab_config) = app.config.tab_bars.iter().find(|t| t.id == "MainContentTabBar") {
                    let desired_height = if tab_config.style.as_deref() == Some("tabbed") { 2u16 } else { 1u16 };
                    if main_area.height >= desired_height {
                        effective_inner_area.y += desired_height.saturating_sub(1);
                        effective_inner_area.height = effective_inner_area.height.saturating_sub(desired_height.saturating_sub(1));
                    }
                }
                effective_inner_area = Block::bordered().inner(effective_inner_area);

                let [_, right_col_area] = Layout::horizontal([
                    Constraint::Length(25),
                    Constraint::Min(0),
                ])
                .areas(effective_inner_area);

                let [_, output_area] = Layout::vertical([
                    Constraint::Length(4),
                    Constraint::Min(0),
                ])
                .areas(right_col_area);

                match mouse_event.kind {
                    MouseEventKind::Down(_) => {
                        if status_area.contains(mouse_pos) {
                            if mouse_event.modifiers.contains(KeyModifiers::CONTROL) {
                                return Ok(Some(Message::CopyStatusText(app.status_text.clone())));
                            } else {
                                return Ok(Some(Message::FocusBox(ActiveBox::Status)));
                            }
                        } else if output_area.contains(mouse_pos) {
                            if mouse_event.modifiers.contains(KeyModifiers::CONTROL) {
                                let content = app.output_lines.join("\n");
                                return Ok(Some(Message::CopyOutputText(content)));
                            } else {
                                return Ok(Some(Message::FocusBox(ActiveBox::Output)));
                            }
                        } else {
                            return Ok(Some(Message::UnfocusBox));
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        if output_area.contains(mouse_pos) || app.active_box == ActiveBox::Output {
                             return Ok(Some(Message::ScrollOutputUp));
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if output_area.contains(mouse_pos) || app.active_box == ActiveBox::Output {
                            return Ok(Some(Message::ScrollOutputDown));
                        }
                    }
                    _ => {} // Ignore other mouse events
                }
            }
            _ => {} // Ignore other events
        }
    }
    Ok(None)
}

/// Map key events to application messages
fn map_key_to_message(app: &mut App, key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Esc => Some(Message::UnfocusBox),
        KeyCode::Up => {
            match app.active_box {
                ActiveBox::None => Some(Message::SelectPreviousCommand),
                ActiveBox::Output => Some(Message::ScrollOutputUp),
                _ => None,
            }
        }
        KeyCode::Down => {
            match app.active_box {
                ActiveBox::None => Some(Message::SelectNextCommand),
                ActiveBox::Output => Some(Message::ScrollOutputDown),
                _ => None,
            }
        }
        KeyCode::Left => Some(Message::SelectPreviousProfile),
        KeyCode::Right => Some(Message::SelectNextProfile),
        KeyCode::PageUp => Some(Message::ScrollOutputUp),
        KeyCode::PageDown => Some(Message::ScrollOutputDown),
        KeyCode::Enter => Some(Message::ExecuteCommand),
        _ => None,
    }
}