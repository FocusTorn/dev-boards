use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

mod app;
mod config;
mod terminal;
mod widgets;
mod commands;

use app::{App, Message};

fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;

    // Initialize terminal
    let mut terminal = terminal::init_terminal()?;

    // Create application state
    let mut app = App::default();

    // Main application loop
    while app.running {
        // Render the current view
        terminal.draw(|f| app.view(f))?;

        // Handle events and map to Message
        let mut current_msg = handle_event(&app)?;

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
fn handle_event(_app: &App) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return Ok(map_key_to_message(key));
            }
        }
    }
    Ok(None)
}

/// Map key events to application messages
fn map_key_to_message(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Some(Message::Quit),
        KeyCode::Up => Some(Message::SelectPreviousCommand),
        KeyCode::Down => Some(Message::SelectNextCommand),
        KeyCode::Enter => Some(Message::ExecuteCommand),
        _ => None,
    }
}
