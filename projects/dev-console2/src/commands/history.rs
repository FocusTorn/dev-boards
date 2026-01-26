/// Persistent storage and analysis of build performance metrics.
///>
/// The `history` module enables "Magic Progress Bars" by recording the actual 
/// time spent in each build stage for every unique firmware project. It uses 
/// this data to calculate weighted averages, allowing the TUI to provide 
/// highly accurate ETAs even for diverse codebases.
///<
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::{Path};
use std::fs;
use crate::commands::predictor::CompileStage;

/// Historical metrics for a specific firmware sketch.
///>
/// Mapping of stage names to a rolling buffer of the last 10 durations (in seconds).
/// Tracks the number of source files detected in the most recent successful build.
///<
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SketchHistory {
    pub stage_times: HashMap<String, Vec<f64>>,
    pub total_files: Option<usize>,
}

/// Root manager for the collection of all sketch histories.
///>
/// This struct manages the high-level map of sketches and provides the 
/// interface for loading from and saving to disk.
///<
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryManager {
    pub sketches: HashMap<String, SketchHistory>,
}

/// Calculated statistics used to drive the real-time progress predictor.
///>
/// Provides the relative weights and expected durations for each build stage, 
/// derived from historical performance data.
///<
#[derive(Debug, Clone)]
pub struct StageStats {
    pub weights: HashMap<CompileStage, f64>,
    pub averages: HashMap<CompileStage, f64>,
}

impl HistoryManager {
    /// Loads history from a JSON file on disk.
///>
    /// Attempts to read the specified file and deserialize its content into a 
    /// `HistoryManager`. Returns a default (empty) manager if the file 
    /// does not exist or is malformed.
    ///<
    pub fn load(path: &Path) -> Self {
        if let Ok(content) = fs::read_to_string(path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    /// Persists current history data to a JSON file.
///>
    /// Ensures that the parent directory exists before writing the 
    /// pretty-printed JSON representation of the current metrics.
    ///<
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Generates accurate progress weights and ETAs for a specific sketch.
///>
    /// This method analyzes the last 10 runs to find the average time spent in 
    /// each stage. If history is incomplete, it gracefully blends historical 
    /// data with the default workload profile.
    ///<
    pub fn get_stats(&self, sketch_id: &str) -> Option<(StageStats, Option<usize>)> {
        let history = self.sketches.get(sketch_id)?;
        let mut weights = HashMap::new();
        let mut averages = HashMap::new();
        let mut total_avg_time = 0.0;

        let stages = [
            (CompileStage::Initializing, "Initializing"),
            (CompileStage::DetectingLibraries, "DetectingLibraries"),
            (CompileStage::Compiling, "Compiling"),
            (CompileStage::Linking, "Linking"),
            (CompileStage::Generating, "Generating"),
            (CompileStage::Resetting, "Resetting"),
            (CompileStage::Uploading, "Uploading"),
            (CompileStage::Verifying, "Verifying"),
        ];

        // 1. Calculate average time for each stage
        for (stage, name) in &stages {
            if let Some(times) = history.stage_times.get(*name) {
                if !times.is_empty() {
                    let avg = times.iter().sum::<f64>() / times.len() as f64;
                    averages.insert(*stage, avg);
                    total_avg_time += avg;
                }
            }
        }

        if total_avg_time < 1.0 { return None; }

        // 2. Convert time to weights (0.0 - 1.0)
        // If some stages are missing (e.g. upload stats not recorded yet),
        // we'll mix historical averages with defaults.
        let defaults = crate::commands::predictor::WorkloadProfile::default().stage_weights;
        
        for (stage, _) in &stages {
            if let Some(avg) = averages.get(stage) {
                weights.insert(*stage, avg / total_avg_time);
            } else if let Some(def_w) = defaults.get(stage) {
                // If we have no history for this stage, use the default weight
                // Note: This won't sum to exactly 1.0 anymore, so we normalize below
                weights.insert(*stage, *def_w);
                averages.insert(*stage, *def_w * 60.0); // Assume 60s total for fallback
            }
        }

        // Normalize weights to sum to 1.0
        let sum: f64 = weights.values().sum();
        if sum > 0.0 {
            for w in weights.values_mut() { *w /= sum; }
        }

        Some((StageStats { weights, averages }, history.total_files))
    }

    /// Updates the history with new performance data from a successful build.
///>
    /// Appends the actual durations recorded for each stage into the 
    /// sketch's history buffer. Maintains a rolling window of the last 10 
    /// entries to stay relevant to recent code changes.
    ///<
    pub fn record_run(&mut self, sketch_id: &str, actual_times: HashMap<CompileStage, f64>, total_files: usize) {
        let entry = self.sketches.entry(sketch_id.to_string())
            .or_insert_with(|| SketchHistory { stage_times: HashMap::new(), total_files: None });

        if total_files > 0 {
            entry.total_files = Some(total_files);
        }

        for (stage, time) in actual_times {
            let name = format!("{:?}", stage);
            let times = entry.stage_times.entry(name).or_insert_with(Vec::new);
            times.push(time);
            if times.len() > 10 {
                times.remove(0);
            }
        }
    }
}
