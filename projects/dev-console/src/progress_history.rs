// Historical progress data storage and retrieval

use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;
use crate::progress_tracker::{ProgressStage, HistoricalData};

/// Manager for historical progress data
pub struct ProgressHistory {
    data_file: PathBuf,
    history: HashMap<String, HistoricalData>, // Key: normalized path string
}

impl ProgressHistory {
    /// Load historical data from file
    pub fn load(data_file: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let history = if data_file.exists() {
            match fs::read_to_string(&data_file) {
                Ok(contents) => {
                    serde_json::from_str(&contents).unwrap_or_default()
                }
                Err(_) => {
                    // If file exists but can't be read, start with empty history
                    HashMap::new()
                }
            }
        } else {
            HashMap::new()
        };
        
        Ok(Self { data_file, history })
    }
    
    /// Create a new empty progress history
    pub fn new(data_file: PathBuf) -> Self {
        Self {
            data_file,
            history: HashMap::new(),
        }
    }
    
    /// Save historical data to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = self.data_file.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let contents = serde_json::to_string_pretty(&self.history)?;
        fs::write(&self.data_file, contents)?;
        Ok(())
    }
    
    /// Record completion of an operation
    pub fn record_completion(
        &mut self,
        file_path: PathBuf,
        stage_times: HashMap<ProgressStage, std::time::Duration>,
        total_time: std::time::Duration,
    ) {
        let key = normalize_path(&file_path);
        let entry = self.history.entry(key.clone())
            .or_insert_with(|| HistoricalData {
                file_path: file_path.clone(),
                stage_averages: HashMap::new(),
                total_averages: Vec::new(),
                last_updated: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            });
        
        // Update stage averages
        for (stage, duration) in stage_times {
            let stage_key = format!("{:?}", stage);
            entry.stage_averages
                .entry(stage_key)
                .or_insert_with(Vec::new)
                .push(duration.as_secs_f64());
            
            // Keep only last 10 measurements per stage
            if let Some(times) = entry.stage_averages.get_mut(&format!("{:?}", stage)) {
                if times.len() > 10 {
                    times.remove(0);
                }
            }
        }
        
        // Update total averages
        entry.total_averages.push(total_time.as_secs_f64());
        if entry.total_averages.len() > 20 {
            entry.total_averages.remove(0);
        }
        
        entry.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }
    
    /// Get historical data for a file
    pub fn get_historical_data(&self, file_path: &PathBuf) -> Option<&HistoricalData> {
        let key = normalize_path(file_path);
        self.history.get(&key)
    }
    
    /// Get average total time for a file
    pub fn get_average_total_time(&self, file_path: &PathBuf) -> Option<std::time::Duration> {
        self.get_historical_data(file_path)
            .and_then(|hist| {
                if hist.total_averages.is_empty() {
                    None
                } else {
                    let avg: f64 = hist.total_averages.iter().sum::<f64>() / hist.total_averages.len() as f64;
                    Some(std::time::Duration::from_secs_f64(avg))
                }
            })
    }
}

/// Normalize path for consistent key generation
fn normalize_path(path: &PathBuf) -> String {
    path.to_string_lossy().to_string()
        .replace('\\', "/")
        .to_lowercase()
}
