use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::{Path};
use std::fs;
use crate::commands::predictor::CompileStage;

/// Historical performance metrics for a specific Arduino sketch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SketchHistory {
    pub stage_times: HashMap<String, Vec<f64>>, // Stage name -> Last 10 durations
}

/// Manages the persistence and analysis of build duration history.
///>
/// This struct handles loading and saving history from a JSON file and 
/// calculating statistical averages and weights used for progress 
/// prediction.
///<
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryManager {
    pub sketches: HashMap<String, SketchHistory>,
}

/// Aggregated statistical data for a specific sketch.
#[derive(Debug, Clone)]
pub struct StageStats {
    pub weights: HashMap<CompileStage, f64>,
    pub averages: HashMap<CompileStage, f64>,
}

impl HistoryManager {
    /// Loads history from a JSON file.
    pub fn load(path: &Path) -> Self {
        if let Ok(content) = fs::read_to_string(path) { //>
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        } //<
    }

    /// Persists history to a JSON file.
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() { //>
            fs::create_dir_all(parent)?;
        } //<
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Calculates weights and averages for a sketch based on historical data.
    ///>
    /// If no history exists for a specific stage, it falls back to 
    /// defaults from the `WorkloadProfile`.
    ///<
    pub fn get_stats(&self, sketch_id: &str) -> Option<StageStats> {
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
        for (stage, name) in &stages { //>
            if let Some(times) = history.stage_times.get(*name) { //>
                if !times.is_empty() { //>
                    let avg = times.iter().sum::<f64>() / times.len() as f64;
                    averages.insert(*stage, avg);
                    total_avg_time += avg;
                } //<
            } //<
        } //<

        if total_avg_time < 1.0 { return None; }

        // 2. Convert time to weights (0.0 - 1.0)
        let defaults = crate::commands::predictor::WorkloadProfile::default().stage_weights;
        
        for (stage, _) in &stages { //>
            if let Some(avg) = averages.get(stage) { //>
                weights.insert(*stage, avg / total_avg_time);
            } else if let Some(def_w) = defaults.get(stage) {
                weights.insert(*stage, *def_w);
                averages.insert(*stage, *def_w * 60.0); // Assume 60s total for fallback
            } //<
        } //<

        // Normalize weights to sum to 1.0
        let sum: f64 = weights.values().sum();
        if sum > 0.0 { //>
            for w in weights.values_mut() { *w /= sum; }
        } //<

        Some(StageStats { weights, averages })
    }

    /// Records the results of a single compilation or upload run.
    ///>
    /// Stores the last 10 durations for each stage to enable rolling 
    /// average calculation.
    ///<
    pub fn record_run(&mut self, sketch_id: &str, actual_times: HashMap<CompileStage, f64>) {
        let entry = self.sketches.entry(sketch_id.to_string())
            .or_insert_with(|| SketchHistory { stage_times: HashMap::new() });

        for (stage, time) in actual_times { //>
            let name = format!("{:?}", stage);
            let times = entry.stage_times.entry(name).or_insert_with(Vec::new);
            times.push(time);
            if times.len() > 10 { //>
                times.remove(0);
            } //<
        } //<
    }
}