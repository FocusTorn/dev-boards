/// Terminal lifecycle management and raw mode orchestration.
/// 
/// This module handles the transition between the standard shell and the 
/// interactive TUI environment. It manages raw mode, mouse capture, and 
/// keyboard protocol enhancements to ensure a high-fidelity user experience.
use std::{io, panic};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture, PushKeyboardEnhancementFlags, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};

/// Prepares the terminal for high-performance TUI interaction.
/// 
/// This function:
/// 1. Enables 'Raw Mode' to intercept all keystrokes immediately.
/// 2. Switches to the 'Alternate Screen' to preserve the user's original shell history.
/// 3. Enables 'Mouse Capture' for interactive UI elements.
/// 4. Pushes 'Keyboard Enhancement Flags' (Kitty Protocol) to detect complex key combinations (e.g., Ctrl+Shift).
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
            | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
    ));

    let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    Ok(terminal)
}

/// Gracefully restores the terminal to its original state.
/// 
/// This MUST be called before the application exits to ensure the user's shell
/// returns to normal operation (e.g., re-enabling line buffering and cursor visibility).
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
//<
pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = io::stdout().execute(PopKeyboardEnhancementFlags);
        let _ = io::stdout().execute(DisableMouseCapture);
        let _ = io::stdout().execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
        original_hook(panic_info);
    }));
}


 

