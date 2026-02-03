use std::{io, panic};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{
            DisableMouseCapture, EnableMouseCapture, KeyboardEnhancementFlags,
            PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
        },
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};

/// Configures the terminal for raw mode and enters the alternate screen.
///>
/// Enables mouse capture and the Kitty Keyboard Protocol enhancement flags
/// for improved modifier key detection. Returns a `Terminal` instance ready
/// for rendering.
///<
pub fn init_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    io::stdout().execute(EnableMouseCapture)?;

    // Enable Kitty Keyboard Protocol if supported
    // This allows for better detection of modifiers (Shift, Alt, Ctrl) especially on Windows Terminal
    let _ = io::stdout().execute(PushKeyboardEnhancementFlags(
        KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
            | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
            | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
            | KeyboardEnhancementFlags::REPORT_EVENT_TYPES,
    ));

    let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    Ok(terminal)
}

/// Teardown terminal configuration and restore original state.
///>
/// Disables mouse capture, leaves the alternate screen, and disables raw
/// mode to return the terminal to the user in a clean state.
///<
pub fn restore_terminal() -> io::Result<()> {
    let _ = io::stdout().execute(PopKeyboardEnhancementFlags);
    io::stdout().execute(DisableMouseCapture)?;
    io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/// Installs a custom panic hook to prevent terminal corruption.
///>
/// If the application crashes, this hook ensures the terminal is restored to
/// its original state before the error message is printed, preventing the
/// "frozen terminal" state common in failed TUI apps.
///<
#[allow(dead_code)]
pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = io::stdout().execute(PopKeyboardEnhancementFlags);
        let _ = io::stdout().execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
        original_hook(panic_info);
    }));
}
