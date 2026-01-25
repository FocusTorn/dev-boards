use std::time::{Duration, Instant};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CompileStage {
    Initializing,
    DetectingLibraries,
    Compiling,
    Linking,
    Generating,
    Resetting,
    Uploading,
    Verifying,
    Complete,
}

impl CompileStage {
    pub fn rank(&self) -> usize {
        match self {
            CompileStage::Initializing => 0,
            CompileStage::DetectingLibraries => 1,
            CompileStage::Compiling => 2,
            CompileStage::Linking => 3,
            CompileStage::Generating => 4,
            CompileStage::Resetting => 5,
            CompileStage::Uploading => 6,
            CompileStage::Verifying => 7,
            CompileStage::Complete => 8,
        }
    }

    pub fn from_rank(rank: usize) -> Option<Self> {
        match rank {
            0 => Some(CompileStage::Initializing),
            1 => Some(CompileStage::DetectingLibraries),
            2 => Some(CompileStage::Compiling),
            3 => Some(CompileStage::Linking),
            4 => Some(CompileStage::Generating),
            5 => Some(CompileStage::Resetting),
            6 => Some(CompileStage::Uploading),
            7 => Some(CompileStage::Verifying),
            8 => Some(CompileStage::Complete),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct WorkloadProfile {
    // Percentage of total time each stage typically takes (0.0 - 1.0)
    pub stage_weights: HashMap<CompileStage, f64>,
}

impl Default for WorkloadProfile {
    fn default() -> Self {
        let mut weights = HashMap::new();
        weights.insert(CompileStage::Initializing, 0.05);
        weights.insert(CompileStage::DetectingLibraries, 0.15); 
        weights.insert(CompileStage::Compiling, 0.40); // Was 0.50
        weights.insert(CompileStage::Linking, 0.30);   // Was 0.20
        weights.insert(CompileStage::Generating, 0.10);
        Self { stage_weights: weights }
    }
}

use crate::commands::history::StageStats;

#[derive(Debug)]
pub struct ProgressPredictor {
    profile: WorkloadProfile,
    expected_durations: HashMap<CompileStage, f64>,
    stage_starts: HashMap<CompileStage, Instant>,
    current_stage: CompileStage,
    start_time: Instant,
    smoothed_performance_ratio: f64,
}

impl ProgressPredictor {
    pub fn new() -> Self {
        Self::with_stats(None)
    }

    pub fn with_stats(stats: Option<StageStats>) -> Self {
        let now = Instant::now();
        let mut stage_starts = HashMap::new();
        stage_starts.insert(CompileStage::Initializing, now);
        
        let (profile, expected_durations) = if let Some(s) = stats {
            (WorkloadProfile { stage_weights: s.weights }, s.averages)
        } else {
            let mut defaults = HashMap::new();
            defaults.insert(CompileStage::Initializing, 5.0);
            defaults.insert(CompileStage::DetectingLibraries, 10.0);
            defaults.insert(CompileStage::Compiling, 45.0);
            defaults.insert(CompileStage::Linking, 15.0);
            defaults.insert(CompileStage::Generating, 5.0);
            (WorkloadProfile::default(), defaults)
        };

        Self {
            profile,
            expected_durations,
            stage_starts,
            current_stage: CompileStage::Initializing,
            start_time: now,
            smoothed_performance_ratio: 1.0,
        }
    }

    pub fn enter_stage(&mut self, stage: CompileStage) {
        if self.current_stage != stage {
            self.current_stage = stage;
            self.stage_starts.insert(stage, Instant::now());
        }
    }

    pub fn get_stats(&self) -> StageStats {
        StageStats {
            weights: self.profile.stage_weights.clone(),
            averages: self.expected_durations.clone(),
        }
    }

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
            let weight = *self.profile.stage_weights.get(s).unwrap_or(&0.0);
            if *s == stage {
                return (start_pct, start_pct + weight);
            }
            start_pct += weight;
        }
        (0.0, 1.0)
    }

    pub fn predict_remaining(&mut self, total_progress: f64) -> Option<Duration> {
        let elapsed_total = self.start_time.elapsed().as_secs_f64();
        let stage_start = self.stage_starts.get(&self.current_stage)?;
        let elapsed_in_stage = stage_start.elapsed().as_secs_f64();
        let (stage_start_pct, stage_end_pct) = self.get_stage_range(self.current_stage);
        let stage_weight = stage_end_pct - stage_start_pct;

        // 1. Calculate relative progress WITHIN the current stage
        // Fix: Clamp total_progress to at least the start of the current stage to avoid negative progress holes
        let effective_total_pct = (total_progress / 100.0).max(stage_start_pct);
        
        let mut relative_progress = if stage_weight > 0.0 {
            (effective_total_pct - stage_start_pct) / stage_weight
        } else {
            0.0
        };

        // 2. Synthetic Heartbeat if no output is arriving
        if relative_progress < 0.01 {
            let typical_duration = *self.expected_durations.get(&self.current_stage).unwrap_or(&15.0);
            relative_progress = (elapsed_in_stage / typical_duration).min(0.90);
        }
        relative_progress = relative_progress.min(1.0).max(0.001);

        // 3. Performance Ratio calculation
        // Fix: Use a 'Warm-up' period. Don't adjust the ratio for the first 3 seconds
        // to let the velocity stabilize.
        let ratio = if elapsed_total > 3.0 {
            let typical_total_so_far = effective_total_pct * 60.0;
            let instant_performance_ratio = if typical_total_so_far > 0.5 {
                elapsed_total / typical_total_so_far
            } else {
                1.0
            };

            // Smooth the ratio to prevent jumps (Alpha 0.05 = very high inertia)
            let alpha = 0.05;
            self.smoothed_performance_ratio = (self.smoothed_performance_ratio * (1.0 - alpha)) + (instant_performance_ratio * alpha);
            self.smoothed_performance_ratio.clamp(0.3, 5.0)
        } else {
            1.0 // Use baseline during warm-up
        };

        // 4. Calculate remaining time
        if elapsed_in_stage < 0.2 {
            return None;
        }

        let velocity_in_stage = relative_progress / elapsed_in_stage;
        let remaining_in_stage = (1.0 - relative_progress) / velocity_in_stage;

        // Future stages projection
        let mut future_weight_sum = 0.0;
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

        let mut found_current = false;
        for s in &stages_in_order {
            if found_current {
                future_weight_sum += self.profile.stage_weights.get(s).unwrap_or(&0.0);
            }
            if *s == self.current_stage {
                found_current = true;
            }
        }

        // Future prediction = (Future Weights as Seconds) * Smoothed Ratio
        let future_time_projection = (future_weight_sum * 60.0) * ratio;

        Some(Duration::from_secs_f64(remaining_in_stage + future_time_projection))
    }
}
