// Dashboard state management module

/// Dashboard state structure
#[derive(Debug, Clone)]
pub struct DashboardState {
    pub commands: Vec<String>,
    pub selected_command: usize,
    pub status_text: String,
    pub output_lines: Vec<String>,
    pub output_scroll: usize,
    // Progress tracking
    pub is_running: bool,
    pub progress_percent: f64,
    pub progress_stage: String,
    pub current_file: String,
}

impl DashboardState {
    /// Create a new dashboard state
    pub fn new() -> Self { //>
        Self {
            commands: vec![ //>
                // "Build".to_string(),         // orriginal python pmake command
                // "Compile-py".to_string(),       // orriginal python pmake command
                // "Progress-py".to_string(),      // orriginal python pmake command
                "Compile".to_string(),
                // "Upload".to_string(),        // orriginal python pmake command
                // "Upload_custom".to_string(), // orriginal python pmake command
                "Upload".to_string(),
                "Monitor".to_string(),
                "Clean".to_string(),
                "All".to_string(),
                "Help".to_string(),
            ], //<
            selected_command: 0,
            status_text: "Ready".to_string(),
            output_lines: Vec::new(),
            output_scroll: 0,
            is_running: false,
            progress_percent: 0.0,
            progress_stage: String::new(),
            current_file: String::new(),
        }
    } //<
    
    /// Scroll output up
    pub fn scroll_output_up(&mut self, amount: usize) {
        if self.output_scroll > 0 {
            self.output_scroll = self.output_scroll.saturating_sub(amount);
        }
    }
    
    /// Scroll output down
    pub fn scroll_output_down(&mut self, amount: usize) {
        let max_scroll = self.output_lines.len().saturating_sub(1);
        if self.output_scroll < max_scroll {
            self.output_scroll = (self.output_scroll + amount).min(max_scroll);
        }
    }
}
