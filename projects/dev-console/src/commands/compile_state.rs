// Compilation state tracking and progress calculation

use std::time::Instant;

/// Compilation stage enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompileStage {
    Initializing,
    Compiling,
    Linking,
    Generating,
    Complete,
}

/// Compilation state tracking structure
pub struct CompileState {
    pub stage: CompileStage,
    pub current_file: String,
    pub files_compiled: usize,
    pub total_files: usize,
    pub compile_lines_seen: std::collections::HashSet<String>,
    pub compiled_lines_seen: std::collections::HashSet<String>,
    pub start_time: Instant,
    pub compile_stage_start: Option<Instant>,
    pub link_stage_start: Option<Instant>,
    pub generate_stage_start: Option<Instant>,
    pub previous_stage_progress: f64, // Track progress when transitioning stages
    pub last_logged_progress: f64, // Track last logged progress to avoid unnecessary updates
}

impl CompileState {
    pub fn new() -> Self {
        Self {
            stage: CompileStage::Initializing,
            current_file: String::new(),
            files_compiled: 0,
            total_files: 0,
            compile_lines_seen: std::collections::HashSet::new(),
            compiled_lines_seen: std::collections::HashSet::new(),
            start_time: Instant::now(),
            compile_stage_start: None,
            link_stage_start: None,
            generate_stage_start: None,
            previous_stage_progress: 0.0,
            last_logged_progress: 0.0,
        }
    }
    
    /// Calculate progress percentage based on current stage and state
    pub fn calculate_progress(&self) -> f64 {
        match self.stage {
            CompileStage::Initializing => {
                let elapsed = self.start_time.elapsed().as_secs_f64();
                (elapsed / 2.0).min(5.0).max(1.0)
            }
            CompileStage::Compiling => {
                let compile_elapsed = self.compile_stage_start
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                
                // Start from previous stage progress (or 5% minimum) to avoid jumps
                let start_progress = self.previous_stage_progress.max(5.0);
                let max_progress = 65.0; // Compiling stage max
                
                if self.total_files > 0 {
                    let file_progress = self.files_compiled as f64 / self.total_files as f64;
                    // Calculate progress within the Compiling range (start_progress to max_progress)
                    let range = max_progress - start_progress;
                    let file_based = start_progress + (file_progress * range);
                    let time_based = start_progress + (compile_elapsed * 2.0).min(range);
                    (file_based * 0.9 + time_based * 0.1).min(max_progress)
                } else {
                    start_progress + (compile_elapsed * 2.0).min(max_progress - start_progress)
                }
            }
            CompileStage::Linking => {
                let link_elapsed = self.link_stage_start
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                // Start from previous stage progress (or 65% minimum) to avoid jumps
                // More gradual progress: previous to 90% (up to 25% range)
                let start_progress = self.previous_stage_progress.max(65.0);
                let max_progress = 90.0; // Linking stage max
                let range = max_progress - start_progress;
                start_progress + (link_elapsed * 5.0).min(range)
            }
            CompileStage::Generating => {
                let gen_elapsed = self.generate_stage_start
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                // Generating typically takes ~5 seconds out of ~45 seconds total (11% of time)
                // Start from previous stage progress (or 90% minimum) to avoid jumps
                // Allocate up to 5% additional progress for generating stage
                let start_progress = self.previous_stage_progress.max(90.0);
                start_progress + (gen_elapsed * 1.0).min(5.0).min(95.0 - start_progress)
            }
            CompileStage::Complete => 100.0,
        }
    }
}
