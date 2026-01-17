// Dashboard state management module

use crate::constants::MAX_OUTPUT_LINES;
use crate::progress_tracker::{ProgressTracker, EstimateMethod};
use std::sync::Arc;

/// Dashboard state structure
#[derive(Debug, Clone)]
pub struct DashboardState {
    pub commands: Vec<String>,
    pub selected_command: usize,
    pub status_text: Arc<str>,  // Use Arc<str> for string interning
    pub output_lines: Vec<String>,
    pub output_scroll: usize,
    /// Auto-scroll enabled flag - when true, new lines automatically scroll to bottom
    pub auto_scroll_enabled: bool,
    // Progress tracking
    pub is_running: bool,
    pub progress_percent: f64,
    pub progress_stage: Arc<str>,  // Use Arc<str> for string interning
    pub current_file: Arc<str>,  // Use Arc<str> for string interning
    // Batch update tracking (for future use)
    #[allow(dead_code)]
    pending_updates: Vec<DashboardUpdate>,
    // Advanced progress tracking with time estimates
    pub progress_tracker: Option<ProgressTracker>,
}

/// Sentinel value to indicate "scroll to bottom" - renderer will calculate actual position
pub const SCROLL_TO_BOTTOM: usize = usize::MAX;

/// Types of dashboard updates that can be batched (for future use)
#[derive(Debug, Clone)]
#[allow(dead_code)]
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
                "Monitor-Serial".to_string(),
                "Monitor-MQTT".to_string(),
                "Clean".to_string(),
                "All".to_string(),
                "Help".to_string(),
            ], //<
            selected_command: 0,
            status_text: common::READY.clone(),
            output_lines: Vec::new(),
            output_scroll: 0,
            auto_scroll_enabled: true,  // Auto-scroll enabled by default
            is_running: false,
            progress_percent: 0.0,
            progress_stage: Arc::from(""),
            current_file: Arc::from(""),
            pending_updates: Vec::new(),
            progress_tracker: None,
        }
    } //<
    
    /// Scroll output up - disables auto-scroll when user manually scrolls
    pub fn scroll_output_up(&mut self, amount: usize) {
        // User manually scrolled - disable auto-scroll
        self.auto_scroll_enabled = false;
        if self.output_scroll > 0 && self.output_scroll != SCROLL_TO_BOTTOM {
            self.output_scroll = self.output_scroll.saturating_sub(amount);
        }
    }
    
    /// Scroll output down - disables auto-scroll when user manually scrolls
    /// Note: This method doesn't have visible_height, so it uses a conservative estimate
    /// The actual scroll position will be clamped during rendering
    pub fn scroll_output_down(&mut self, amount: usize) {
        // User manually scrolled - disable auto-scroll
        self.auto_scroll_enabled = false;
        // Use a conservative estimate - actual max_scroll will be calculated during render
        let total_lines = self.output_lines.len();
        if total_lines > 0 {
            // Estimate max_scroll conservatively (assume at least 1 line visible)
            let estimated_max = total_lines.saturating_sub(1);
            let current_scroll = if self.output_scroll == SCROLL_TO_BOTTOM {
                0  // Start from 0 if we were at bottom (unlikely when scrolling down)
            } else {
                self.output_scroll
            };
            
            if current_scroll < estimated_max {
                self.output_scroll = (current_scroll + amount).min(estimated_max);
                
                // If we reached the end of the buffer, re-enable auto-scroll
                // This allows the user to "snap back" to following the logs
                if self.output_scroll >= estimated_max {
                    self.auto_scroll_enabled = true;
                    self.output_scroll = SCROLL_TO_BOTTOM;
                }
            }
        }
    }
    
    /// Scroll to bottom of output (called by renderer with correct visible_height)
    pub fn scroll_to_bottom(&mut self, visible_height: usize) {
        if self.output_lines.is_empty() {
            self.output_scroll = 0;
            return;
        }
        let total_lines = self.output_lines.len();
        // Calculate maximum scroll position
        let max_scroll = if total_lines > visible_height {
            total_lines - visible_height
        } else {
            0
        };
        self.output_scroll = max_scroll;
    }
    
    /// Re-enable auto-scroll (e.g., when user scrolls to bottom manually)
    #[allow(dead_code)]
    pub fn enable_auto_scroll(&mut self) {
        self.auto_scroll_enabled = true;
    }
    
    /// Add a line to output, enforcing size limit
    /// If auto-scroll is enabled, marks scroll position for "scroll to bottom" during render
    pub fn add_output_line(&mut self, line: String) {
        self.output_lines.push(line);
        
        // Enforce size limit by removing oldest lines
        if self.output_lines.len() > MAX_OUTPUT_LINES {
            let remove_count = self.output_lines.len() - MAX_OUTPUT_LINES;
            self.output_lines.drain(0..remove_count);
            
            // Adjust scroll position if needed (but preserve SCROLL_TO_BOTTOM sentinel)
            if self.output_scroll != SCROLL_TO_BOTTOM {
                if self.output_scroll >= remove_count {
                    self.output_scroll -= remove_count;
                } else {
                    self.output_scroll = 0;
                }
            }
        }
        
        // If auto-scroll is enabled, mark for scrolling to bottom during render
        // The renderer will calculate the correct position with visible_height
        if self.auto_scroll_enabled {
            self.output_scroll = SCROLL_TO_BOTTOM;
        }
    }
    
    /// Queue an update to be applied in batch (for future use)
    #[allow(dead_code)]
    pub fn queue_update(&mut self, update: DashboardUpdate) {
        self.pending_updates.push(update);
    }
    
    /// Apply all pending updates in a single lock operation (for future use)
    #[allow(dead_code)]
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
    
    /// Initialize progress tracking for a new operation
    pub fn start_progress_tracking(&mut self, total_items: Option<usize>, historical_data: Option<crate::progress_tracker::HistoricalData>) {
        let mut tracker = ProgressTracker::new(total_items);
        tracker.historical_data = historical_data;
        self.progress_tracker = Some(tracker);
    }
    
    /// Update progress with time estimates (for future use)
    #[allow(dead_code)]
    pub fn update_progress_with_estimate(&mut self, items_processed: usize, method: EstimateMethod) {
        if let Some(ref mut tracker) = self.progress_tracker {
            tracker.update_progress(items_processed, method);
            self.progress_percent = tracker.progress_percent;
        }
    }
    
    /// Get formatted progress string with time estimates (for future use)
    #[allow(dead_code)]
    pub fn get_progress_display(&self) -> String {
        if let Some(ref tracker) = self.progress_tracker {
            let elapsed = tracker.format_elapsed();
            let remaining = tracker.format_estimated_remaining()
                .map(|r| format!(" (ETA: {})", r))
                .unwrap_or_default();
            
            format!("{}% - Elapsed: {}{}", 
                tracker.progress_percent as u32,
                elapsed,
                remaining
            )
        } else {
            format!("{}%", self.progress_percent as u32)
        }
    }
    
    /// Transition progress tracker to a new stage
    pub fn transition_progress_stage(&mut self, new_stage: crate::progress_tracker::ProgressStage) {
        let stage_name = if let Some(ref mut tracker) = self.progress_tracker {
            tracker.transition_stage(new_stage);
            tracker.current_stage_name()
        } else {
            return;
        };
        self.set_progress_stage(stage_name);
    }
}
