// Compilation state tracking and progress calculation

pub use crate::commands::predictor::CompileStage;
use std::time::Instant;

/// Tracks the lifecycle and progress of a single compilation or upload run.
///>
/// This structure maintains the state-machine for compilation stages,
/// calculates weighted progress percentages based on historical and real-time
/// data, and detects anomalies like missing markers or stuck stages.
///<
pub struct CompileState {
    pub stage: CompileStage,
    pub stage_weights: std::collections::HashMap<CompileStage, f64>,
    pub expected_durations: std::collections::HashMap<CompileStage, f64>,
    pub current_file: String,
    pub files_compiled: usize,
    pub total_files: usize,
    pub compile_lines_seen: std::collections::HashSet<String>,
    pub compiled_lines_seen: std::collections::HashSet<String>,
    pub start_time: Instant,
    pub detect_libs_stage_start: Option<Instant>,
    pub compile_stage_start: Option<Instant>,
    pub link_stage_start: Option<Instant>,
    pub generate_stage_start: Option<Instant>,
    pub previous_stage_progress: f64, // Track progress when transitioning stages
    pub max_progress: f64,            // Track maximum progress reached to prevent backwards jumps
    pub last_marker_time: Instant,    // Track when the last stage marker was seen
    pub has_warned_current_stage: bool, // Prevent multiple warnings for the same stuck stage
    pub stage_progress: f64,          // 0.0 - 100.0 within the CURRENT stage
    pub stage_durations: std::collections::HashMap<crate::commands::predictor::CompileStage, f64>,
}

impl CompileState {
    /// Initializes a new state tracker with provided weights and durations.
    pub fn new(
        weights: std::collections::HashMap<CompileStage, f64>,
        expected_durations: std::collections::HashMap<CompileStage, f64>,
    ) -> Self {
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
        }
    }

    /// Updates internal progress within the current stage.
    ///>
    /// Clamps progress to prevent backward jumps unless a clear segment
    /// reset is detected (common during multi-stage uploads).
    ///<
    pub fn update_stage_progress(&mut self, progress: f64) {
        // esptool does multiple segments.
        // If we see a significant drop (e.g. 100% -> 5%), it's a new segment.
        if progress < self.stage_progress && self.stage_progress > 90.0 {
            //>
            // New segment started. We'll "nudge" the floor up but allow 0-100 again.
            self.stage_progress = progress;
        } else if progress > self.stage_progress {
            self.stage_progress = progress.clamp(0.0, 100.0);
        } //<
    }

    /// Transitions the state machine to a new stage.
    ///>
    /// Returns a list of any stages that were skipped based on the jump in
    /// rank. This ensures the progress bar remains monotonic even if markers
    /// are missing from the input stream.
    ///<
    pub fn transition_to(&mut self, next_stage: CompileStage) -> Vec<CompileStage> {
        let mut skipped = Vec::new();
        let current_rank = self.stage.rank();
        let next_rank = next_stage.rank();

        // Record duration of the stage we just FINISHED
        let duration = self.last_marker_time.elapsed().as_secs_f64();
        self.stage_durations.insert(self.stage, duration);

        if next_rank > current_rank + 1 {
            //>
            // We skipped one or more stages
            for rank in (current_rank + 1)..next_rank {
                //>
                if let Some(skipped_stage) = CompileStage::from_rank(rank) {
                    //>
                    skipped.push(skipped_stage);
                } //<
            } //<
        } //<

        // Before moving, snap previous_stage_progress to the START of the next stage
        // to prevent jumps in calculate_progress
        let (next_start, _) = self.get_stage_range(next_stage);
        self.previous_stage_progress = next_start;

        self.stage = next_stage;
        self.stage_progress = 0.0;
        self.has_warned_current_stage = false;
        skipped
    }

    /// Determines the 0-100 range occupied by a stage based on its weight.
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
            //>
            let weight = *self.stage_weights.get(s).unwrap_or(&0.0);
            if *s == stage {
                //>
                return (start_pct * 100.0, (start_pct + weight) * 100.0);
            } //<
            start_pct += weight;
        } //<
        (0.0, 100.0)
    }

    /// Calculates the global progress percentage (0-100).
    ///>
    /// Combines stage weights, file-based counts, and time-based estimates
    /// to provide a smooth, monotonic progress indicator.
    ///<
    pub fn calculate_progress(&mut self) -> f64 {
        let (start_range, end_range) = self.get_stage_range(self.stage);
        let range_width = end_range - start_range;

        // Use max_progress as floor to ensure no jumps backwards
        let progress_floor: f64 = start_range.max(self.max_progress);

        let current = match self.stage {
            //>
            CompileStage::Initializing => {
                let elapsed = self.start_time.elapsed().as_secs_f64();
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&5.0);
                start_range + (elapsed / typical * range_width).min(range_width * 0.9)
            }
            CompileStage::DetectingLibraries => {
                let elapsed = self
                    .detect_libs_stage_start
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&10.0);
                start_range + (elapsed / typical * range_width).min(range_width * 0.9)
            }
            CompileStage::Compiling => {
                let compile_elapsed = self
                    .compile_stage_start
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);

                if self.total_files > 0 {
                    //>
                    let file_progress = self.files_compiled as f64 / self.total_files as f64;
                    let file_based = start_range + (file_progress * range_width);
                    let typical = *self.expected_durations.get(&self.stage).unwrap_or(&45.0);
                    let time_based =
                        start_range + (compile_elapsed / typical * range_width).min(range_width);
                    (file_based * 0.95 + time_based * 0.05).min(end_range)
                } else {
                    let typical = *self.expected_durations.get(&self.stage).unwrap_or(&45.0);
                    // Use stage_progress if it was set externally, otherwise use time
                    let base = if self.stage_progress > 0.0 {
                        start_range + (self.stage_progress / 100.0 * range_width)
                    } else {
                        start_range + (compile_elapsed / typical * range_width)
                    };
                    base.min(start_range + range_width * 0.9)
                } //<
            }
            CompileStage::Linking => {
                let link_elapsed = self
                    .link_stage_start
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&15.0);
                progress_floor
                    + (link_elapsed / typical * range_width)
                        .min(end_range - progress_floor)
                        .min(range_width * 0.9)
            }
            CompileStage::Generating => {
                let gen_elapsed = self
                    .generate_stage_start
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&5.0);
                progress_floor
                    + (gen_elapsed / typical * (end_range - progress_floor))
                        .min(end_range - progress_floor)
                        .min(range_width * 0.9)
            }
            CompileStage::Resetting => {
                let elapsed = self.last_marker_time.elapsed().as_secs_f64();
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&2.0);
                progress_floor
                    + (elapsed / typical * (end_range - progress_floor))
                        .min(end_range - progress_floor)
                        .min(range_width * 0.9)
            }
            CompileStage::Uploading => {
                let elapsed = self.last_marker_time.elapsed().as_secs_f64();
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&30.0);

                if self.stage_progress > 0.0 {
                    //>
                    start_range + (self.stage_progress / 100.0 * range_width)
                } else {
                    progress_floor
                        + (elapsed / typical * (end_range - progress_floor))
                            .min(end_range - progress_floor)
                            .min(range_width * 0.9)
                } //<
            }
            CompileStage::Verifying => {
                let elapsed = self.last_marker_time.elapsed().as_secs_f64();
                let typical = *self.expected_durations.get(&self.stage).unwrap_or(&2.0);
                progress_floor
                    + (elapsed / typical * (end_range - progress_floor))
                        .min(end_range - progress_floor)
                        .min(range_width * 0.9)
            }
            CompileStage::Complete => 100.0,
        }; //<

        if current > self.max_progress {
            //>
            self.max_progress = current;
        } //<
        self.max_progress
    }

    /// Monitors for parser stalls where markers are expected but missing.
    ///>
    /// If more than 30 seconds pass without a stage transition, this
    /// returns a warning message to be displayed in the TUI output.
    ///<
    pub fn check_for_missing_markers(&mut self) -> Option<String> {
        if self.has_warned_current_stage {
            //>
            return None;
        } //<

        let elapsed_since_marker = self.last_marker_time.elapsed().as_secs();

        if elapsed_since_marker > 30 && self.stage != CompileStage::Complete {
            //>
            self.has_warned_current_stage = true;
            Some(format!(
                "[WARNING] No stage markers seen for {}s. Current stage: {:?}",
                elapsed_since_marker, self.stage
            ))
        } else {
            None
        } //<
    }
}
