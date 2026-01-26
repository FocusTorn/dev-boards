/// Entry point for the dev-console-v2 application.
///>
/// This function initializes the environment, sets up the terminal into raw mode,
/// and runs the main event loop. It ensures that the terminal is gracefully
/// restored even if the application encounters an error.
///<
use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};


mod app;
mod config;
mod terminal;
mod widgets;
mod commands;

use app::{App, Message};

fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;

    // Install panic hook to restore terminal on crash
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
        if app.should_redraw || app.is_task_running() || app.is_animating() || app.is_toast_animating() {
            terminal.draw(|f| app.view(f))?;
            app.should_redraw = false;
        }
    }

    // Restore terminal
    terminal::restore_terminal()?;
    Ok(())
}

/// Translates raw crossterm events into high-level application Messages.
///>
/// This provides a layer of abstraction between the terminal's input stream
/// and the application's internal logic, allowing for cleaner state transitions.
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