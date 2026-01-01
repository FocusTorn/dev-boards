// Advanced progress tracking with time estimates

use std::time::{Duration, Instant};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

/// Enhanced progress tracking with time estimates
#[derive(Debug, Clone)]
pub struct ProgressTracker {
    // Current operation tracking
    pub start_time: Instant,
    pub current_stage: ProgressStage,
    pub current_stage_start: Instant,
    
    // Progress metrics
    pub progress_percent: f64,
    pub items_processed: usize,
    pub total_items: Option<usize>,
    
    // Time estimates
    pub elapsed_time: Duration,
    pub estimated_remaining: Option<Duration>,
    pub estimated_total: Option<Duration>,
    
    // Stage-specific tracking
    pub stage_times: HashMap<ProgressStage, StageTiming>,
    
    // Historical data for estimates
    pub historical_data: Option<HistoricalData>,
}

/// Progress stages for different operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProgressStage {
    Initializing,
    Compiling,
    Linking,
    Generating,
    Uploading,
    Verifying,
    Complete,
}

impl ProgressStage {
    /// Get display name for the stage
    pub fn display_name(&self) -> &'static str {
        match self {
            ProgressStage::Initializing => "Initializing",
            ProgressStage::Compiling => "Compiling",
            ProgressStage::Linking => "Linking",
            ProgressStage::Generating => "Generating",
            ProgressStage::Uploading => "Uploading",
            ProgressStage::Verifying => "Verifying",
            ProgressStage::Complete => "Complete",
        }
    }
}

/// Timing information for a specific stage
#[derive(Debug, Clone)]
pub struct StageTiming {
    pub start_time: Instant,
    pub elapsed: Duration,
    pub estimated_remaining: Option<Duration>,
    pub items_in_stage: usize,
    pub total_items_in_stage: Option<usize>,
}

/// Historical performance data for better estimates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalData {
    pub file_path: PathBuf,
    pub stage_averages: HashMap<String, Vec<f64>>, // Duration in seconds
    pub total_averages: Vec<f64>, // Total duration in seconds
    pub last_updated: u64, // Unix timestamp
}

/// Progress estimate calculation methods
#[derive(Debug, Clone, Copy)]
pub enum EstimateMethod {
    /// Use current progress rate (items/time)
    CurrentRate,
    /// Use historical averages for this stage
    HistoricalAverage,
    /// Weighted combination of current rate and historical data
    Weighted { current_weight: f64, historical_weight: f64 },
}

impl ProgressTracker {
    /// Create a new progress tracker
    pub fn new(total_items: Option<usize>) -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            current_stage: ProgressStage::Initializing,
            current_stage_start: now,
            progress_percent: 0.0,
            items_processed: 0,
            total_items,
            elapsed_time: Duration::ZERO,
            estimated_remaining: None,
            estimated_total: None,
            stage_times: HashMap::new(),
            historical_data: None,
        }
    }
    
    /// Update progress and recalculate estimates
    pub fn update_progress(&mut self, items_processed: usize, method: EstimateMethod) {
        self.items_processed = items_processed;
        self.elapsed_time = self.start_time.elapsed();
        
        // Calculate progress percentage - prioritize total_items if available
        if let Some(total) = self.total_items {
            if total > 0 {
                self.progress_percent = (items_processed as f64 / total as f64 * 100.0).min(100.0);
            } else {
                // If total is 0, keep current progress or use a small increment
                self.progress_percent = self.progress_percent.min(99.9);
            }
        } else {
            // If no total_items, progress_percent should be set externally
            // Don't modify it here if we don't have total_items
        }
        
        // Calculate time estimates based on method
        self.estimated_remaining = match method {
            EstimateMethod::CurrentRate => self.calculate_current_rate_estimate(),
            EstimateMethod::HistoricalAverage => self.calculate_historical_estimate(),
            EstimateMethod::Weighted { current_weight, historical_weight } => {
                self.calculate_weighted_estimate(current_weight, historical_weight)
            },
        };
        
        // Calculate estimated total time
        if let Some(remaining) = self.estimated_remaining {
            self.estimated_total = Some(self.elapsed_time + remaining);
        }
    }
    
    /// Set progress percentage directly (for percentage-based tracking)
    pub fn set_progress_percent(&mut self, percent: f64) {
        self.progress_percent = percent.min(100.0).max(0.0);
    }
    
    /// Calculate estimate based on current progress rate
    fn calculate_current_rate_estimate(&self) -> Option<Duration> {
        if self.items_processed == 0 || self.elapsed_time.as_secs_f64() == 0.0 {
            return None;
        }
        
        let rate = self.items_processed as f64 / self.elapsed_time.as_secs_f64();
        
        if let Some(total) = self.total_items {
            if total > self.items_processed {
                let remaining_items = total - self.items_processed;
                let estimated_secs = remaining_items as f64 / rate;
                Some(Duration::from_secs_f64(estimated_secs.max(0.0)))
            } else {
                Some(Duration::ZERO)
            }
        } else {
            None
        }
    }
    
    /// Calculate estimate based on historical data
    fn calculate_historical_estimate(&self) -> Option<Duration> {
        self.historical_data.as_ref().and_then(|hist| {
            let stage_key = format!("{:?}", self.current_stage);
            hist.stage_averages.get(&stage_key)
                .and_then(|durations| {
                    if durations.is_empty() {
                        None
                    } else {
                        let avg: f64 = durations.iter().sum::<f64>() / durations.len() as f64;
                        Some(Duration::from_secs_f64(avg))
                    }
                })
        })
    }
    
    /// Calculate weighted estimate combining current rate and historical data
    fn calculate_weighted_estimate(&self, current_weight: f64, historical_weight: f64) -> Option<Duration> {
        let current_estimate = self.calculate_current_rate_estimate();
        let historical_estimate = self.calculate_historical_estimate();
        
        match (current_estimate, historical_estimate) {
            (Some(current), Some(historical)) => {
                let current_secs = current.as_secs_f64() * current_weight;
                let historical_secs = historical.as_secs_f64() * historical_weight;
                Some(Duration::from_secs_f64((current_secs + historical_secs).max(0.0)))
            }
            (Some(current), None) => Some(current),
            (None, Some(historical)) => Some(historical),
            (None, None) => None,
        }
    }
    
    /// Transition to a new stage
    pub fn transition_stage(&mut self, new_stage: ProgressStage) {
        // Record timing for previous stage
        let stage_elapsed = self.current_stage_start.elapsed();
        self.stage_times.insert(
            self.current_stage,
            StageTiming {
                start_time: self.current_stage_start,
                elapsed: stage_elapsed,
                estimated_remaining: None,
                items_in_stage: 0,
                total_items_in_stage: None,
            }
        );
        
        // Start new stage
        self.current_stage = new_stage;
        self.current_stage_start = Instant::now();
    }
    
    /// Format elapsed time for display
    pub fn format_elapsed(&self) -> String {
        format_duration(self.elapsed_time)
    }
    
    /// Format estimated remaining time for display
    pub fn format_estimated_remaining(&self) -> Option<String> {
        self.estimated_remaining.map(format_duration)
    }
    
    /// Format estimated total time for display
    pub fn format_estimated_total(&self) -> Option<String> {
        self.estimated_total.map(format_duration)
    }
    
    /// Get current stage display name
    pub fn current_stage_name(&self) -> &'static str {
        self.current_stage.display_name()
    }
}

/// Format duration as human-readable string
fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}
