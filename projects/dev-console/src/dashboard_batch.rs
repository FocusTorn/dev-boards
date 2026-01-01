// Batch dashboard state updates module
// Reduces lock contention on Arc<Mutex<DashboardState>> by batching updates

use crate::dashboard::DashboardState;
use std::sync::{Arc, Mutex};

/// Batch of dashboard state updates (for future use)
#[allow(dead_code)]
pub struct DashboardUpdateBatch {
    status_text: Option<Arc<str>>,
    output_lines: Vec<String>,
    progress_percent: Option<f64>,
    progress_stage: Option<Arc<str>>,
    current_file: Option<Arc<str>>,
    is_running: Option<bool>,
}

impl DashboardUpdateBatch {
    /// Create a new batch (for future use)
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            status_text: None,
            output_lines: Vec::new(),
            progress_percent: None,
            progress_stage: None,
            current_file: None,
            is_running: None,
        }
    }
    
    /// Set status text (uses string interning, for future use)
    #[allow(dead_code)]
    pub fn set_status_text(&mut self, text: &str) {
        use crate::string_intern::intern_string;
        self.status_text = Some(intern_string(text));
    }
    
    /// Add output line (for future use)
    #[allow(dead_code)]
    pub fn add_output_line(&mut self, line: String) {
        self.output_lines.push(line);
    }
    
    /// Add multiple output lines (for future use)
    #[allow(dead_code)]
    pub fn add_output_lines(&mut self, lines: Vec<String>) {
        self.output_lines.extend(lines);
    }
    
    /// Set progress percent (for future use)
    #[allow(dead_code)]
    pub fn set_progress_percent(&mut self, percent: f64) {
        self.progress_percent = Some(percent);
    }
    
    /// Set progress stage (uses string interning, for future use)
    #[allow(dead_code)]
    pub fn set_progress_stage(&mut self, stage: &str) {
        use crate::string_intern::intern_string;
        self.progress_stage = Some(intern_string(stage));
    }
    
    /// Set current file (uses string interning, for future use)
    #[allow(dead_code)]
    pub fn set_current_file(&mut self, file: &str) {
        use crate::string_intern::intern_string;
        self.current_file = Some(intern_string(file));
    }
    
    /// Set is_running flag (for future use)
    #[allow(dead_code)]
    pub fn set_is_running(&mut self, running: bool) {
        self.is_running = Some(running);
    }
    
    /// Apply all batched updates to dashboard state (single lock acquisition, for future use)
    #[allow(dead_code)]
    pub fn apply(&self, dashboard: Arc<Mutex<DashboardState>>) {
        let mut state = dashboard.lock().unwrap();
        
        // Apply status text
        if let Some(ref text) = self.status_text {
            state.status_text = text.clone();
        }
        
        // Apply output lines
        for line in &self.output_lines {
            state.add_output_line(line.clone());
        }
        
        // Apply progress updates
        if let Some(percent) = self.progress_percent {
            state.progress_percent = percent;
        }
        
        if let Some(ref stage) = self.progress_stage {
            state.progress_stage = stage.clone();
        }
        
        if let Some(ref file) = self.current_file {
            state.current_file = file.clone();
        }
        
        if let Some(running) = self.is_running {
            state.is_running = running;
        }
    }
    
    /// Check if batch is empty (for future use)
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.status_text.is_none()
            && self.output_lines.is_empty()
            && self.progress_percent.is_none()
            && self.progress_stage.is_none()
            && self.current_file.is_none()
            && self.is_running.is_none()
    }
}

impl Default for DashboardUpdateBatch {
    fn default() -> Self {
        Self::new()
    }
}
