/// Managed state for an active compilation or upload task.
///>
/// `CompileState` is the source of truth for build progress. It combines 
/// historical averages, real-time file counts, and time-based heuristic curves 
/// to ensure the UI progress bar moves smoothly and predictably, even when 
/// underlying tools provide erratic output.
///<
use std::time::Instant;
pub use crate::commands::predictor::CompileStage;

/// Tracks metrics and timestamps for a single compilation session.
///>
/// This struct maintains all the counters, timers, and sets needed to
/// calculate smooth progress percentages across multiple build stages.
///<
pub struct CompileState {
    /// The current high-level phase of the build (e.g., Compiling, Linking).
    pub stage: CompileStage,
    /// Weights assigned to each stage to determine their share of the 100% progress bar.
    pub stage_weights: std::collections::HashMap<CompileStage, f64>,
    /// Historical durations for each stage to calculate time-based progress.
    pub expected_durations: std::collections::HashMap<CompileStage, f64>,
    /// Name of the file currently being processed by the compiler.
    pub current_file: String,
    /// Number of files successfully compiled so far.
    pub files_compiled: usize,
    /// Number of unique compilation commands detected in this session.
    pub total_files: usize,
    /// Unique set of compilation lines seen (to avoid double-counting).
    pub compile_lines_seen: std::collections::HashSet<String>,
    /// Unique set of successful compilation markers seen.
    pub compiled_lines_seen: std::collections::HashSet<String>,
    /// Timestamp when the entire process started.
    pub start_time: Instant,
    /// Timestamp when the library detection phase started.
    pub detect_libs_stage_start: Option<Instant>,
    /// Timestamp when the compilation phase started.
    pub compile_stage_start: Option<Instant>,
    /// Timestamp when the linking phase started.
    pub link_stage_start: Option<Instant>,
    /// Timestamp when the binary generation phase started.
    pub generate_stage_start: Option<Instant>,
    /// Progress percentage at the moment the last stage completed.
    pub previous_stage_progress: f64,
    /// Highest progress percentage reached so far (to prevent bar regression).
    pub max_progress: f64,
    /// Timestamp of the last detected stage marker or progress event.
    pub last_marker_time: Instant,
    /// Flag to ensure the 'missing marker' warning is only sent once per stage.
    pub has_warned_current_stage: bool,
    /// Real-time progress (0-100) emitted by tools like `esptool`.
    pub stage_progress: f64,
    /// Map of actual durations recorded for each stage in this session.
    pub stage_durations: std::collections::HashMap<crate::commands::predictor::CompileStage, f64>,
    /// Historical file count for this sketch, used to weight compilation progress.
    pub estimated_total_files: Option<usize>,
}

impl CompileState {
    /// Creates a new state tracker with provided weights and historical averages.
///>
    /// Initializes all timers and sets the starting stage to `Initializing`.
    ///<
    pub fn new(weights: std::collections::HashMap<CompileStage, f64>, expected_durations: std::collections::HashMap<CompileStage, f64>, estimated_total_files: Option<usize>) -> Self {
        Self {
            stage: CompileStage::Initializing,
            stage_weights: weights,
            expected_durations,
            current_file: String::new(),
            files_compiled: 0,
            total_files: 0,
            compile_lines_seen: std::collections::HashSet::new(),
            compiled_lines_seen: std::collections::HashSet::new(),
            start_time: Instant::now(),
            detect_libs_stage_start: None,
            compile_stage_start: None,
            link_stage_start: None,
            generate_stage_start: None,
            previous_stage_progress: 0.0,
            max_progress: 0.0,
            last_marker_time: Instant::now(),
            has_warned_current_stage: false,
            stage_progress: 0.0,
            stage_durations: std::collections::HashMap::new(),
            estimated_total_files,
        }
    }

    /// Updates internal progress based on percentage-style logs from external tools.
///>
    /// Some tools (like esptool) emit multiple 0-100 segments.
    /// We only move the bar forward or reset if a new segment is clearly detected.
    ///<
    pub fn update_stage_progress(&mut self, progress: f64) {
        if progress < self.stage_progress && self.stage_progress > 90.0 {
            self.stage_progress = progress;
        } else if progress > self.stage_progress {
            self.stage_progress = progress.clamp(0.0, 100.0);
        }
    }
    
    /// Transitions the state to a new build stage.
///>
    /// This records the duration of the finished stage and identifies any 
    /// intermediate stages that were skipped by the build tool.
    ///<
    pub fn transition_to(&mut self, next_stage: CompileStage) -> Vec<CompileStage> {
        let mut skipped = Vec::new();
        let current_rank = self.stage.rank();
        let next_rank = next_stage.rank();

        // Record the actual time spent in the stage we are leaving
        let duration = self.last_marker_time.elapsed().as_secs_f64();
        self.stage_durations.insert(self.stage, duration);

        // Identify stages that were implicitly completed
        if next_rank > current_rank + 1 {
            for rank in (current_rank + 1)..next_rank {
                if let Some(skipped_stage) = CompileStage::from_rank(rank) {
                    skipped.push(skipped_stage);
                }
            }
        }

        // Lock progress floor to the start of the next stage
        let (next_start, _) = self.get_stage_range(next_stage);
        self.previous_stage_progress = next_start;

        self.stage = next_stage;
        self.stage_progress = 0.0;
        self.has_warned_current_stage = false;
        skipped
    }

    /// Resolves the percentage window (start/end) allocated to a specific stage.
///>
    /// Iterates through the known stages in order to determine where the 
    /// current stage sits on the overall 0-100% timeline.
    ///<
    fn get_stage_range(&self, stage: CompileStage) -> (f64, f64) {
        let stages_in_order = [
            CompileStage::Initializing,
            CompileStage::DetectingLibraries,
            CompileStage::Compiling,
            CompileStage::Linking,
            CompileStage::Generating,
            CompileStage::Resetting,
            CompileStage::Uploading,
            CompileStage::Verifying,
            CompileStage::Complete,
        ];

        let mut start_pct = 0.0;
        for s in &stages_in_order {
            let weight = *self.stage_weights.get(s).unwrap_or(&0.0);
            if *s == stage {
                return (start_pct * 100.0, (start_pct + weight) * 100.0);
            }
            start_pct += weight;
        }
        (0.0, 100.0)
    }

    /// Calculates the overall project progress (0-100%).
///>
    /// This is the core logic for the "Magic Progress Bar." It uses a hybrid 
    /// approach:
    /// - **File Count:** Used during compilation if history or discovery data exists.
    /// - **Time Curves:** Asymptotic curves ensure the bar moves even if no 
    ///   logs are received, slowing down as it nears the "typical" duration.
    /// - **External Pct:** Direct ingestion of percentages from flash tools.
    ///<
    pub fn calculate_progress(&mut self) -> f64 {
        let (start_range, end_range) = self.get_stage_range(self.stage);
        let range_width = end_range - start_range;
        
        let progress_floor: f64 = start_range.max(self.max_progress);

        // Soft asymptotic curve: ensures progress never truly hits 100% of a 
        // stage based on time alone, preventing "jumping" when the real 
        // marker finally arrives.
        let calc_time_progress = |elapsed: f64, typical: f64| -> f64 {
            let ratio = elapsed / typical;
            if ratio < 0.8 {
                ratio 
            } else {
                0.8 + 0.2 * (1.0 - (-1.0 * (ratio - 0.8)).exp())
            }
        };

        let current = match self.stage {
            CompileStage::Initializing => {
                let elapsed = self.start_time.elapsed().as_secs_f64();
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&5.0);
                start_range + calc_time_progress(elapsed, typical) * range_width
            }
            CompileStage::DetectingLibraries => {
                let elapsed = self.detect_libs_stage_start
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&10.0);
                start_range + calc_time_progress(elapsed, typical) * range_width
            }
            CompileStage::Compiling => {
                let compile_elapsed = self.compile_stage_start
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                
                if let Some(est_total) = self.estimated_total_files {
                    if est_total > 0 {
                        // We have history: use file ratio with asymptotic overshoot handling
                        let mut file_ratio = self.files_compiled as f64 / est_total as f64;
                        
                        if file_ratio > 0.95 {
                             let overshoot = file_ratio - 0.95;
                             file_ratio = 0.95 + 0.05 * (1.0 - (-5.0 * overshoot).exp());
                        }

                        let time_factor = (compile_elapsed / 300.0).min(0.05);
                        let combined = (file_ratio + time_factor).min(0.99); 
                        
                        start_range + (combined * range_width)
                    } else {
                         let typical = *self.expected_durations.get(&self.stage).unwrap_or(&45.0);
                         start_range + calc_time_progress(compile_elapsed, typical) * range_width
                    }
                } else if self.total_files > 0 {
                    // No history: weight time heavily (80%) and files lightly (20%) 
                    // since total_files is still being discovered.
                    let typical = *self.expected_durations.get(&self.stage).unwrap_or(&45.0);
                    let time_prog = calc_time_progress(compile_elapsed, typical);
                    let file_ratio = self.files_compiled as f64 / self.total_files as f64;
                    
                    let combined = (time_prog * 0.8) + (file_ratio * 0.2);
                    start_range + combined * range_width
                } else {
                    let typical = *self.expected_durations.get(&self.stage).unwrap_or(&45.0);
                    if self.stage_progress > 0.0 {
                        start_range + (self.stage_progress / 100.0 * range_width)
                    } else {
                        start_range + calc_time_progress(compile_elapsed, typical) * range_width
                    }
                }
            }
            CompileStage::Linking => {
                let link_elapsed = self.link_stage_start
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&15.0);
                progress_floor + calc_time_progress(link_elapsed, typical) * (end_range - progress_floor)
            }
            CompileStage::Generating => {
                let gen_elapsed = self.generate_stage_start
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&5.0);
                progress_floor + calc_time_progress(gen_elapsed, typical) * (end_range - progress_floor)
            }
            CompileStage::Resetting => {
                let elapsed = self.last_marker_time.elapsed().as_secs_f64();
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&2.0);
                progress_floor + calc_time_progress(elapsed, typical) * (end_range - progress_floor)
            }
            CompileStage::Uploading => {
                let elapsed = self.last_marker_time.elapsed().as_secs_f64();
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&30.0);
                
                if self.stage_progress > 0.0 {
                    start_range + (self.stage_progress / 100.0 * range_width)
                } else {
                    progress_floor + calc_time_progress(elapsed, typical) * (end_range - progress_floor)
                }
            }
            CompileStage::Verifying => {
                let elapsed = self.last_marker_time.elapsed().as_secs_f64();
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&2.0);
                progress_floor + calc_time_progress(elapsed, typical) * (end_range - progress_floor)
            }
            CompileStage::Complete => 100.0,
        };

        if current > self.max_progress {
            self.max_progress = current;
        }
        self.max_progress
    }

    /// Watchdog to detect if the build tool has stalled or the parser missed a marker.
///>
    /// If we've been in a stage for more than 30 seconds without a transition
    /// and we aren't in the final stage, it might indicate a parser failure.
    ///<
    pub fn check_for_missing_markers(&mut self) -> Option<String> {
        if self.has_warned_current_stage {
            return None;
        }

        let elapsed_since_marker = self.last_marker_time.elapsed().as_secs();
        
        if elapsed_since_marker > 30 && self.stage != CompileStage::Complete {
            self.has_warned_current_stage = true;
            Some(format!(
                "[WARNING] No stage markers seen for {}s. Current stage: {:?}", 
                elapsed_since_marker, self.stage
            ))
        } else {
            None
        }
    }
}
