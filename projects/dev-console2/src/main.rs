use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};

mod app;
mod commands;
mod config;
mod terminal;
mod widgets;

use app::{App, Message};

/// Entry point for the dev-console-v2 application.
///>
/// Initializes error handling, terminal state, and the main event loop.
/// The application follows the Elm Architecture (Model-Update-View), with
/// background tasks translated into internal messages.
///<
fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;
    terminal::install_panic_hook();

    // Create application state first (so config errors print to stderr before terminal takeover)
    let mut app = App::new()?;

    // Initialize terminal
    let mut terminal = terminal::init_terminal()?;

    // Initial draw to set view_area and show initial state
    terminal.draw(|f| app.view(f))?;
    app.should_redraw = false;

    // Main application loop
    while app.running {
        // Handle events and map to Message
        if let Some(msg) = handle_event()? {
            app.update(msg);
        }

        // Handle any updates from background commands (translated to Messages)
        app.poll_system_events();

        // Advance animations
        app.tick();

        // Render if state changed OR if we are animating (to keep movement smooth)
        if app.should_redraw
            || app.is_task_running()
            || app.is_animating()
            || app.is_toast_animating()
        {
            terminal.draw(|f| app.view(f))?;
            app.should_redraw = false;
        }
    }

    // Restore terminal
    terminal::restore_terminal()?;
    Ok(())
}

/// Translates raw terminal events into internal application Messages.
///>
/// Polls for keyboard, mouse, and resize events with a short timeout to
/// maintain UI responsiveness and allow background processing.
///<
fn handle_event() -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    return Ok(Some(Message::Key(key)));
                }
            }
            Event::Mouse(mouse) => {
                return Ok(Some(Message::Mouse(mouse)));
            }
            Event::Resize(w, h) => {
                return Ok(Some(Message::Resize(w, h)));
            }
            _ => {}
        }
    }
    Ok(None)
}
