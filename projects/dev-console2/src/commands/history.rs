use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::{Path};
use std::fs;
use crate::commands::predictor::CompileStage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SketchHistory {
    pub stage_times: HashMap<String, Vec<f64>>, // Stage name -> Last 10 durations
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryManager {
    pub sketches: HashMap<String, SketchHistory>,
}

#[derive(Debug, Clone)]
pub struct StageStats {
    pub weights: HashMap<CompileStage, f64>,
    pub averages: HashMap<CompileStage, f64>,
}

impl HistoryManager {
    pub fn load(path: &Path) -> Self {
        if let Ok(content) = fs::read_to_string(path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

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
        for (stage, _) in &stages {
            if let Some(avg) = averages.get(stage) {
                weights.insert(*stage, avg / total_avg_time);
            }
        }

        Some(StageStats { weights, averages })
    }

    pub fn record_run(&mut self, sketch_id: &str, actual_times: HashMap<CompileStage, f64>) {
        let entry = self.sketches.entry(sketch_id.to_string())
            .or_insert_with(|| SketchHistory { stage_times: HashMap::new() });

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
