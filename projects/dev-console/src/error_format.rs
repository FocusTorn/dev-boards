// Error formatting utility module
// Standardizes error message formatting across the application

use crate::dashboard::DashboardState;
use std::sync::{Arc, Mutex};

/// Format an error message consistently (for future use)
#[allow(dead_code)]
pub fn format_error(message: &str) -> String {
    format!("Error: {}", message)
}

/// Format a warning message consistently (for future use)
#[allow(dead_code)]
pub fn format_warning(message: &str) -> String {
    format!("Warning: {}", message)
}

/// Format an info message consistently (for future use)
#[allow(dead_code)]
pub fn format_info(message: &str) -> String {
    format!("Info: {}", message)
}

/// Format a success message consistently (for future use)
#[allow(dead_code)]
pub fn format_success(message: &str) -> String {
    format!("Success: {}", message)
}

/// Report an error to the dashboard state (for future use)
#[allow(dead_code)]
pub fn report_error(
    dashboard: Arc<Mutex<DashboardState>>,
    message: &str,
) {
    use crate::string_intern::intern_string;
    let formatted = format_error(message);
    let mut state = dashboard.lock().unwrap();
    state.status_text = intern_string(&formatted);
    state.add_output_line(formatted);
}

/// Report a warning to the dashboard state (for future use)
#[allow(dead_code)]
pub fn report_warning(
    dashboard: Arc<Mutex<DashboardState>>,
    message: &str,
) {
    let formatted = format_warning(message);
    let mut state = dashboard.lock().unwrap();
    state.add_output_line(formatted);
}

/// Report an info message to the dashboard state (for future use)
#[allow(dead_code)]
pub fn report_info(
    dashboard: Arc<Mutex<DashboardState>>,
    message: &str,
) {
    let formatted = format_info(message);
    let mut state = dashboard.lock().unwrap();
    state.add_output_line(formatted);
}

/// Report a success message to the dashboard state (for future use)
#[allow(dead_code)]
pub fn report_success(
    dashboard: Arc<Mutex<DashboardState>>,
    message: &str,
) {
    use crate::string_intern::intern_string;
    let formatted = format_success(message);
    let mut state = dashboard.lock().unwrap();
    state.status_text = intern_string(&formatted);
    state.add_output_line(formatted);
}

/// Report an error with detailed context (for future use)
#[allow(dead_code)]
pub fn report_error_with_context(
    dashboard: Arc<Mutex<DashboardState>>,
    message: &str,
    context: &str,
) {
    use crate::string_intern::intern_string;
    let formatted = format!("{} ({})", format_error(message), context);
    let mut state = dashboard.lock().unwrap();
    state.status_text = intern_string(&formatted);
    state.add_output_line(formatted);
}
