// Application constants

/// Minimum terminal width in pixels
pub const MIN_WIDTH_PIXELS: u16 = 80;

/// Minimum terminal height in pixels
pub const MIN_HEIGHT_PIXELS: u16 = 21;

/// Field height in lines (for settings fields)
pub const FIELD_HEIGHT: u16 = 3;

/// Spacing between fields
pub const FIELD_SPACING: u16 = 1;

/// Content area width percentage (50% of available space)
pub const CONTENT_WIDTH_PERCENT: u16 = 50;

/// Content area height percentage (50% of available space)
pub const CONTENT_HEIGHT_PERCENT: u16 = 50;

/// Maximum output lines to keep in memory
pub const MAX_OUTPUT_LINES: usize = 1000;

/// Toast display duration in seconds
pub const TOAST_DURATION_SECS: f64 = 1.5;

/// Main content box handle name
pub const HWND_MAIN_CONTENT_BOX: &str = "hwndMainContentBox";

/// Main content tab bar handle name
pub const HWND_MAIN_CONTENT_TAB_BAR: &str = "hwndMainContentTabBar";
