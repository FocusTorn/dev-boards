# Advanced Progress Tracking with Time Estimates

## Overview

This document describes the design and implementation plan for advanced progress tracking with time estimates in the dev-console application. This feature will provide users with accurate time estimates based on historical data and current progress rates.

## Current State

The application currently tracks:
- `progress_percent` (f64) - Percentage complete
- `progress_stage` (Arc<str>) - Current stage name
- `current_file` (Arc<str>) - Currently processing file
- Basic time tracking in `CompileState` (start_time, stage start times)

## Proposed Implementation

### 1. Enhanced Progress Tracking Structure

```rust
// src/progress_tracker.rs

use std::time::{Instant, Duration};
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
    pub stage_averages: HashMap<ProgressStage, Vec<Duration>>,
    pub total_averages: Vec<Duration>,
    pub last_updated: std::time::SystemTime,
}

/// Progress estimate calculation methods
pub enum EstimateMethod {
    /// Use current progress rate (items/time)
    CurrentRate,
    /// Use historical averages for this stage
    HistoricalAverage,
    /// Weighted combination of current rate and historical data
    Weighted { current_weight: f64, historical_weight: f64 },
}
```

### 2. Time Estimation Logic

```rust
impl ProgressTracker {
    /// Create a new progress tracker
    pub fn new(total_items: Option<usize>) -> Self {
        Self {
            start_time: Instant::now(),
            current_stage: ProgressStage::Initializing,
            current_stage_start: Instant::now(),
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
        
        // Calculate progress percentage
        if let Some(total) = self.total_items {
            self.progress_percent = (items_processed as f64 / total as f64) * 100.0;
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
    
    /// Calculate estimate based on current progress rate
    fn calculate_current_rate_estimate(&self) -> Option<Duration> {
        if self.items_processed == 0 || self.elapsed_time.as_secs_f64() == 0.0 {
            return None;
        }
        
        let rate = self.items_processed as f64 / self.elapsed_time.as_secs_f64();
        
        if let Some(total) = self.total_items {
            let remaining_items = total - self.items_processed;
            let estimated_secs = remaining_items as f64 / rate;
            Some(Duration::from_secs_f64(estimated_secs))
        } else {
            None
        }
    }
    
    /// Calculate estimate based on historical data
    fn calculate_historical_estimate(&self) -> Option<Duration> {
        self.historical_data.as_ref().and_then(|hist| {
            hist.stage_averages.get(&self.current_stage)
                .and_then(|durations| {
                    if durations.is_empty() {
                        None
                    } else {
                        let avg: Duration = durations.iter().sum::<Duration>() / durations.len() as u32;
                        Some(avg)
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
                Some(Duration::from_secs_f64(current_secs + historical_secs))
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
```

### 3. Historical Data Storage

```rust
// src/progress_history.rs

use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;

/// Manager for historical progress data
pub struct ProgressHistory {
    data_file: PathBuf,
    history: HashMap<PathBuf, HistoricalData>,
}

impl ProgressHistory {
    /// Load historical data from file
    pub fn load(data_file: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let history = if data_file.exists() {
            let contents = fs::read_to_string(&data_file)?;
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            HashMap::new()
        };
        
        Ok(Self { data_file, history })
    }
    
    /// Save historical data to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let contents = serde_json::to_string_pretty(&self.history)?;
        fs::write(&self.data_file, contents)?;
        Ok(())
    }
    
    /// Record completion of an operation
    pub fn record_completion(
        &mut self,
        file_path: PathBuf,
        stage_times: HashMap<ProgressStage, Duration>,
        total_time: Duration,
    ) {
        let entry = self.history.entry(file_path.clone())
            .or_insert_with(|| HistoricalData {
                file_path: file_path.clone(),
                stage_averages: HashMap::new(),
                total_averages: Vec::new(),
                last_updated: std::time::SystemTime::now(),
            });
        
        // Update stage averages
        for (stage, duration) in stage_times {
            entry.stage_averages
                .entry(stage)
                .or_insert_with(Vec::new)
                .push(duration);
            
            // Keep only last 10 measurements per stage
            if let Some(times) = entry.stage_averages.get_mut(&stage) {
                if times.len() > 10 {
                    times.remove(0);
                }
            }
        }
        
        // Update total averages
        entry.total_averages.push(total_time);
        if entry.total_averages.len() > 20 {
            entry.total_averages.remove(0);
        }
        
        entry.last_updated = std::time::SystemTime::now();
    }
    
    /// Get historical data for a file
    pub fn get_historical_data(&self, file_path: &PathBuf) -> Option<&HistoricalData> {
        self.history.get(file_path)
    }
}
```

### 4. Integration with DashboardState

```rust
// Update src/dashboard.rs

pub struct DashboardState {
    // ... existing fields ...
    
    // Enhanced progress tracking
    pub progress_tracker: Option<ProgressTracker>,
    pub progress_history: Option<ProgressHistory>,
}

impl DashboardState {
    /// Initialize progress tracking for a new operation
    pub fn start_progress_tracking(&mut self, total_items: Option<usize>, historical_data: Option<HistoricalData>) {
        let mut tracker = ProgressTracker::new(total_items);
        tracker.historical_data = historical_data;
        self.progress_tracker = Some(tracker);
    }
    
    /// Update progress with time estimates
    pub fn update_progress_with_estimate(&mut self, items_processed: usize, method: EstimateMethod) {
        if let Some(ref mut tracker) = self.progress_tracker {
            tracker.update_progress(items_processed, method);
            self.progress_percent = tracker.progress_percent;
        }
    }
    
    /// Get formatted progress string with time estimates
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
}
```

### 5. UI Display Enhancement

```rust
// Update src/render/dashboard.rs

fn render_progress_with_time_estimate(
    f: &mut Frame,
    area: Rect,
    dashboard_state: &DashboardState,
) {
    if let Some(ref tracker) = dashboard_state.progress_tracker {
        let progress_text = format!(
            "{}: {:.1}% | Elapsed: {} | ETA: {}",
            tracker.current_stage,
            tracker.progress_percent,
            tracker.format_elapsed(),
            tracker.format_estimated_remaining()
                .unwrap_or_else(|| "calculating...".to_string())
        );
        
        // Render progress bar with time estimates
        // ...
    }
}
```

## Implementation Plan

### Phase 1: Core Progress Tracker (Week 1)
1. Create `progress_tracker.rs` module
2. Implement basic time tracking and estimation
3. Add current rate calculation
4. Integrate with `DashboardState`

### Phase 2: Historical Data (Week 2)
1. Create `progress_history.rs` module
2. Implement JSON-based storage
3. Add data recording after operations
4. Load historical data on startup

### Phase 3: UI Integration (Week 3)
1. Update dashboard rendering to show time estimates
2. Add progress bar with time display
3. Format time estimates nicely
4. Handle edge cases (no data, very fast operations)

### Phase 4: Enhanced Estimates (Week 4)
1. Implement weighted estimation method
2. Add per-stage time tracking
3. Improve accuracy with historical data
4. Add configuration for estimation preferences

## Benefits

1. **User Experience**: Users know how long operations will take
2. **Better Planning**: Developers can plan work around build times
3. **Performance Monitoring**: Track performance over time
4. **Debugging**: Identify slow stages or operations
5. **Historical Insights**: Learn from past performance

## Configuration Options

```yaml
# config.yaml additions
progress_tracking:
  enabled: true
  estimation_method: "weighted"  # current_rate, historical, weighted
  current_weight: 0.7
  historical_weight: 0.3
  history_file: ".dev-console/progress_history.json"
  max_history_entries: 20
  show_time_estimates: true
```

## Example Display

```
Compiling: 45.2% | Elapsed: 2m 15s | ETA: 2m 45s
[████████████████░░░░░░░░░░░░░░░░] 45.2%
Current file: main.cpp
Files compiled: 12/27
```

## Future Enhancements

1. **Per-file time estimates**: Track time per file for better granularity
2. **Performance trends**: Show if builds are getting faster/slower
3. **Stage breakdown**: Show time spent in each stage
4. **Comparison mode**: Compare current build to average
5. **Notifications**: Alert when operations take longer than expected
