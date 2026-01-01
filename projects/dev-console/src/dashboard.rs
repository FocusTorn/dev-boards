// Dashboard state management module

use crate::constants::MAX_OUTPUT_LINES;
use std::sync::Arc;

/// Dashboard state structure
#[derive(Debug, Clone)]
pub struct DashboardState {
    pub commands: Vec<String>,
    pub selected_command: usize,
    pub status_text: Arc<str>,  // Use Arc<str> for string interning
    pub output_lines: Vec<String>,
    pub output_scroll: usize,
    // Progress tracking
    pub is_running: bool,
    pub progress_percent: f64,
    pub progress_stage: Arc<str>,  // Use Arc<str> for string interning
    pub current_file: Arc<str>,  // Use Arc<str> for string interning
    // Batch update tracking
    pending_updates: Vec<DashboardUpdate>,
}

/// Types of dashboard updates that can be batched
#[derive(Debug, Clone)]
pub enum DashboardUpdate {
    StatusText(Arc<str>),
    ProgressPercent(f64),
    ProgressStage(Arc<str>),
    CurrentFile(Arc<str>),
    OutputLine(String),
    IsRunning(bool),
}

impl DashboardState {
    /// Create a new dashboard state
    pub fn new() -> Self { //>
        use crate::string_intern::common;
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
            status_text: common::READY.clone(),
            output_lines: Vec::new(),
            output_scroll: 0,
            is_running: false,
            progress_percent: 0.0,
            progress_stage: Arc::from(""),
            current_file: Arc::from(""),
            pending_updates: Vec::new(),
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
    
    /// Add a line to output, enforcing size limit
    pub fn add_output_line(&mut self, line: String) {
        self.output_lines.push(line);
        
        // Enforce size limit by removing oldest lines
        if self.output_lines.len() > MAX_OUTPUT_LINES {
            let remove_count = self.output_lines.len() - MAX_OUTPUT_LINES;
            self.output_lines.drain(0..remove_count);
            
            // Adjust scroll position if needed
            if self.output_scroll >= remove_count {
                self.output_scroll -= remove_count;
            } else {
                self.output_scroll = 0;
            }
        }
        
        // Auto-scroll to bottom if near the end
        if self.output_lines.len() > 1 {
            self.output_scroll = self.output_lines.len().saturating_sub(1);
        }
    }
    
    /// Queue an update to be applied in batch
    pub fn queue_update(&mut self, update: DashboardUpdate) {
        self.pending_updates.push(update);
    }
    
    /// Apply all pending updates in a single lock operation
    pub fn apply_pending_updates(&mut self) {
        let updates: Vec<_> = self.pending_updates.drain(..).collect();
        for update in updates {
            match update {
                DashboardUpdate::StatusText(text) => {
                    self.status_text = text;
                }
                DashboardUpdate::ProgressPercent(percent) => {
                    self.progress_percent = percent;
                }
                DashboardUpdate::ProgressStage(stage) => {
                    self.progress_stage = stage;
                }
                DashboardUpdate::CurrentFile(file) => {
                    self.current_file = file;
                }
                DashboardUpdate::OutputLine(line) => {
                    self.add_output_line(line);
                }
                DashboardUpdate::IsRunning(running) => {
                    self.is_running = running;
                }
            }
        }
    }
    
    /// Set status text using string interning
    pub fn set_status_text(&mut self, text: &str) {
        use crate::string_intern::intern_string;
        self.status_text = intern_string(text);
    }
    
    /// Set progress stage using string interning
    pub fn set_progress_stage(&mut self, stage: &str) {
        use crate::string_intern::intern_string;
        self.progress_stage = intern_string(stage);
    }
    
    /// Set current file using string interning
    pub fn set_current_file(&mut self, file: &str) {
        use crate::string_intern::intern_string;
        self.current_file = intern_string(file);
    }
}
