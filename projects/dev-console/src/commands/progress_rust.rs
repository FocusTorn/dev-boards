// Progress command execution (Rust-based, direct arduino-cli call)

use crate::dashboard::DashboardState;
use crate::settings::Settings;
use crate::commands::utils::remove_ansi_escapes;
use crate::process_manager::ProcessManager;
use crate::path_utils::{find_project_root, find_arduino_cli, get_library_path};
use crate::progress_tracker::{ProgressStage, EstimateMethod};
use crate::progress_history::ProgressHistory;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::{File, OpenOptions};
use regex::Regex;
use lazy_static::lazy_static;

// Compilation state tracking
#[derive(Debug, Clone, Copy, PartialEq)]
enum CompileStage {
    Initializing,
    Compiling,
    Linking,
    Generating,
    Complete,
}

struct CompileState {
    stage: CompileStage,
    current_file: String,
    files_compiled: usize,
    total_files: usize,
    compile_lines_seen: std::collections::HashSet<String>,
    compiled_lines_seen: std::collections::HashSet<String>,
    start_time: std::time::Instant,
    compile_stage_start: Option<std::time::Instant>,
    link_stage_start: Option<std::time::Instant>,
    generate_stage_start: Option<std::time::Instant>,
    previous_stage_progress: f64, // Track progress when transitioning stages
}

impl CompileState {
    fn new() -> Self {
        Self {
            stage: CompileStage::Initializing,
            current_file: String::new(),
            files_compiled: 0,
            total_files: 0,
            compile_lines_seen: std::collections::HashSet::new(),
            compiled_lines_seen: std::collections::HashSet::new(),
            start_time: std::time::Instant::now(),
            compile_stage_start: None,
            link_stage_start: None,
            generate_stage_start: None,
            previous_stage_progress: 0.0,
        }
    }
    
    fn calculate_progress(&self) -> f64 {
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

lazy_static! {
    static ref RE_COMPILE_COMMAND: Regex = Regex::new(
        r"@([^\s]+\.(cpp|c|ino|S))|([^\s/\\]+\.(cpp|c|ino|S))"
    ).unwrap();
    static ref RE_COMPILE_LINE: Regex = Regex::new(
        r"(?i)compiling\s+([^\s]+\.(cpp|c|ino|S))"
    ).unwrap();
    static ref RE_COMPILED_FILE: Regex = Regex::new(
        r"(?i)\.(cpp|c|ino|S)\.o|gcc-ar|compiled\s+[^\s]+\.(cpp|c|ino|S)|using previously compiled file"
    ).unwrap();
}

/// Execute progress command using Rust (direct arduino-cli call)
pub fn execute_progress_rust(
    dashboard: Arc<Mutex<DashboardState>>,
    settings: Settings,
    process_manager: Arc<ProcessManager>,
) {
    // Build arduino-cli command
    let sketch_dir = PathBuf::from(&settings.sketch_directory);
    let sketch_file = sketch_dir.join(&settings.sketch_name);
    let build_path = sketch_dir.join("build");
    
    // Find project root (workspace root)
    let project_root = find_project_root(&sketch_dir);
    
    // Create log file for this compilation session
    let log_file_path = project_root.join(".dev-console").join("compile_output.log");
    // Create .dev-console directory if it doesn't exist
    if let Some(parent) = log_file_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    // Open log file in write mode (truncate/clear on each new compilation)
    let log_file = Arc::new(Mutex::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&log_file_path)
            .unwrap_or_else(|e| {
                eprintln!("Warning: Could not open log file {:?}: {}", log_file_path, e);
                // Create a dummy file that will fail silently on write
                std::fs::File::create("/dev/null").unwrap()
            })
    ));
    
    // Write session start marker to log
    {
        let mut log = log_file.lock().unwrap();
        let _ = writeln!(log, "\n=== Compilation Session Started ===");
        let _ = writeln!(log, "Timestamp: {:?}", std::time::SystemTime::now());
        let _ = writeln!(log, "Sketch: {:?}", sketch_file);
    }
    
    // Load historical data if available
    let history_file = project_root.join(".dev-console").join("progress_history.json");
    let mut history = ProgressHistory::load(history_file.clone())
        .unwrap_or_else(|_| ProgressHistory::new(history_file));
    
    // Get historical data for this sketch
    let historical_data = history.get_historical_data(&sketch_dir)
        .map(|h| crate::progress_tracker::HistoricalData {
            file_path: h.file_path.clone(),
            stage_averages: h.stage_averages.clone(),
            total_averages: h.total_averages.clone(),
            last_updated: h.last_updated,
        });
    
    // Calculate library path
    let library_path = get_library_path(&project_root, &settings.board_model);
    
    // Find arduino-cli
    let arduino_cli = find_arduino_cli(&settings.env, &project_root);
    
    // Build command arguments - MUST include --libraries like Python version
    let mut cmd = Command::new(&arduino_cli);
    cmd.arg("compile");
    cmd.arg("--fqbn").arg(&settings.fqbn);
    cmd.arg("--libraries").arg(&library_path);
    cmd.arg("--build-path").arg(&build_path);
    cmd.arg("--verbose");
    cmd.arg(&sketch_file);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.current_dir(&sketch_dir);
    
    // Helper function to write to log file
    let log_output = |log_file: &Arc<Mutex<File>>, line: &str| {
        if let Ok(mut log) = log_file.lock() {
            let _ = writeln!(log, "{}", line);
        }
    };
    
    // Add initial message
    {
        let mut state = dashboard.lock().unwrap();
        let lines = vec![
            format!("Executing: {:?} compile --fqbn {} --libraries {:?} --verbose {:?}", 
                arduino_cli, settings.fqbn, library_path, sketch_file),
            format!("Build path: {:?}", build_path),
            format!("Library path: {:?}", library_path),
            format!("Library path exists: {}", library_path.exists()),
            format!("Arduino CLI path: {:?}", arduino_cli),
            format!("Arduino CLI exists: {}", arduino_cli.exists()),
        ];
        for line in &lines {
            state.add_output_line(line.clone());
            log_output(&log_file, line);
        }
        state.is_running = true;
        state.set_progress_stage("Initializing");
        state.progress_percent = 0.0;
        
        // Log initial progress
        log_output(&log_file, "");
        log_output(&log_file, "{{commanded progress bar percent: 0.0}}");
        log_output(&log_file, "");
        
        // Initialize progress tracking with time estimates
        state.start_progress_tracking(None, historical_data);
        if let Some(ref mut tracker) = state.progress_tracker {
            tracker.current_stage = ProgressStage::Initializing;
        }
    }
    
    // Check if arduino-cli exists (unless it's in PATH)
    if !arduino_cli.exists() && arduino_cli.to_string_lossy() != "arduino-cli" {
        let mut state = dashboard.lock().unwrap();
        state.is_running = false;
        let error_msg1 = format!("Error: arduino-cli not found at: {:?}", arduino_cli);
        let error_msg2 = "Please ensure arduino-cli.exe is installed in the Arduino directory at the workspace root.".to_string();
        state.set_status_text(&error_msg1);
        state.add_output_line(error_msg1.clone());
        state.add_output_line(error_msg2.clone());
        log_output(&log_file, &error_msg1);
        log_output(&log_file, &error_msg2);
        return;
    }
    
    // Spawn process
    let mut child = match cmd.spawn() {
        Ok(child) => {
            // Register process with process manager for cleanup tracking
            process_manager.register(&child);
            child
        }
        Err(e) => {
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.set_status_text(&format!("Error: Failed to start arduino-cli: {}", e));
            state.add_output_line(format!("Error: Failed to start arduino-cli: {}", e));
            state.add_output_line(format!("Tried path: {:?}", arduino_cli));
            if !arduino_cli.exists() && arduino_cli.to_string_lossy() != "arduino-cli" {
                state.add_output_line("The arduino-cli executable was not found at the expected location.".to_string());
            }
            return;
        }
    };
    
    // Store PID for unregistering when process completes
    let pid = child.id();
    
    // Read stderr in separate thread
    let dashboard_stderr = dashboard.clone();
    let log_file_stderr = log_file.clone();
    if let Some(stderr) = child.stderr.take() {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    // Preserve ANSI codes for colorization - only trim whitespace
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        let mut state = dashboard_stderr.lock().unwrap();
                        state.add_output_line(trimmed.to_string());
                        // Log to file
                        if let Ok(mut log) = log_file_stderr.lock() {
                            let _ = writeln!(log, "{}", trimmed);
                        }
                        // Auto-scroll is handled during rendering with correct visible_height
                    }
                }
            }
        });
    }
    
    // Read stdout and parse
    let mut compile_state = CompileState::new();
    
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        
        for line_result in reader.lines() {
            let line = match line_result {
                Ok(l) => l,
                Err(_) => break,
            };
            
            // Preserve ANSI codes for colorization - only clean for parsing
            let cleaned = remove_ansi_escapes(&line);
            let line_lower = cleaned.to_lowercase();
            let trimmed = cleaned.trim();
            
            if trimmed.is_empty() {
                continue;
            }
            
            // Add to output (preserve ANSI codes from original line)
            {
                let trimmed_line = line.trim();
                let mut state = dashboard.lock().unwrap();
                // Store original line with ANSI codes for colorized display
                state.add_output_line(trimmed_line.to_string());
                // Log to file
                log_output(&log_file, trimmed_line);
                // Auto-scroll is handled during rendering with correct visible_height
            }
            
            // Parse line for compilation state
            // Detect errors
            if line_lower.contains("error") || line_lower.contains("fatal") {
                // Error detected - already added to output
                continue;
            }
            
            // Detect stages and update progress tracker
            // Get current progress from dashboard state before transitioning
            let current_progress = {
                let state = dashboard.lock().unwrap();
                state.progress_percent
            };
            
            let stage_changed = if line_lower.contains("detecting libraries") || line_lower.contains("detecting library") {
                // Save actual current progress from dashboard state before transitioning
                compile_state.previous_stage_progress = current_progress;
                compile_state.stage = CompileStage::Compiling;
                if compile_state.compile_stage_start.is_none() {
                    compile_state.compile_stage_start = Some(std::time::Instant::now());
                }
                true
            } else if line_lower.contains("generating function prototypes") || line_lower.contains("generating prototypes") {
                compile_state.stage = CompileStage::Compiling;
                false
            } else if line_lower.contains("linking everything together") || (line_lower.contains("linking") && line_lower.contains("together")) {
                // Save actual current progress from dashboard state before transitioning
                compile_state.previous_stage_progress = current_progress;
                compile_state.stage = CompileStage::Linking;
                compile_state.current_file.clear();
                if compile_state.link_stage_start.is_none() {
                    compile_state.link_stage_start = Some(std::time::Instant::now());
                }
                true
            } else if line_lower.contains("esptool") && line_lower.contains("elf2image") && 
                      line_lower.contains(".ino.elf") && line_lower.contains(".ino.bin") &&
                      !line_lower.contains("bootloader") {
                // Only trigger Generating for final .ino.elf to .ino.bin conversion (after linking)
                // This is the actual final image generation, not bootloader creation
                // Must check that we're past Linking stage to avoid triggering too early
                if compile_state.stage == CompileStage::Linking || compile_state.link_stage_start.is_some() {
                    // Save actual current progress from dashboard state before transitioning to avoid jumps
                    compile_state.previous_stage_progress = current_progress;
                    compile_state.stage = CompileStage::Generating;
                    compile_state.current_file.clear();
                    if compile_state.generate_stage_start.is_none() {
                        compile_state.generate_stage_start = Some(std::time::Instant::now());
                    }
                    true
                } else {
                    false
                }
            } else if line_lower.contains("sketch uses") || line_lower.contains("global variables use") {
                compile_state.stage = CompileStage::Complete;
                compile_state.current_file.clear();
                true
            } else {
                false
            };
            
            // Update progress tracker stage if changed
            if stage_changed {
                let new_stage = match compile_state.stage {
                    CompileStage::Initializing => ProgressStage::Initializing,
                    CompileStage::Compiling => ProgressStage::Compiling,
                    CompileStage::Linking => ProgressStage::Linking,
                    CompileStage::Generating => ProgressStage::Generating,
                    CompileStage::Complete => ProgressStage::Complete,
                };
                let mut state = dashboard.lock().unwrap();
                state.transition_progress_stage(new_stage);
            }
            
            // Detect compilation commands/files
            if line.contains("xtensa-esp32s3-elf-g++") || line.contains("xtensa-esp32s3-elf-gcc") {
                if line.contains("-c") {
                    // Save actual current progress from dashboard state before transitioning
                    if compile_state.stage != CompileStage::Compiling {
                        let state = dashboard.lock().unwrap();
                        compile_state.previous_stage_progress = state.progress_percent;
                        drop(state); // Release lock before continuing
                    }
                    compile_state.stage = CompileStage::Compiling;
                    if compile_state.compile_stage_start.is_none() {
                        compile_state.compile_stage_start = Some(std::time::Instant::now());
                    }
                    
                    if let Some(captures) = RE_COMPILE_COMMAND.captures(&line) {
                        if let Some(file_match) = captures.get(1).or_else(|| captures.get(3)) {
                            let file_path = file_match.as_str();
                            compile_state.current_file = file_path.to_string();
                            if !compile_state.compile_lines_seen.contains(trimmed) {
                                compile_state.compile_lines_seen.insert(trimmed.to_string());
                                compile_state.total_files = compile_state.compile_lines_seen.len();
                            }
                        }
                    }
                }
            } else if let Some(captures) = RE_COMPILE_LINE.captures(&line_lower) {
                if let Some(file_match) = captures.get(1) {
                    let file_path = file_match.as_str();
                    compile_state.current_file = file_path.to_string();
                    // Save actual current progress from dashboard state before transitioning
                    if compile_state.stage != CompileStage::Compiling {
                        let state = dashboard.lock().unwrap();
                        compile_state.previous_stage_progress = state.progress_percent;
                        drop(state); // Release lock before continuing
                    }
                    compile_state.stage = CompileStage::Compiling;
                    if compile_state.compile_stage_start.is_none() {
                        compile_state.compile_stage_start = Some(std::time::Instant::now());
                    }
                    if !compile_state.compile_lines_seen.contains(trimmed) {
                        compile_state.compile_lines_seen.insert(trimmed.to_string());
                        compile_state.total_files = compile_state.compile_lines_seen.len();
                    }
                }
            } else if RE_COMPILED_FILE.is_match(&line_lower) {
                if !compile_state.compiled_lines_seen.contains(trimmed) {
                    compile_state.compiled_lines_seen.insert(trimmed.to_string());
                    compile_state.files_compiled = compile_state.compiled_lines_seen.len();
                }
            }
            
            // Update dashboard state with progress tracking
            {
                let mut state = dashboard.lock().unwrap();
                
                match compile_state.stage {
                    CompileStage::Initializing => state.set_progress_stage("Initializing"),
                    CompileStage::Compiling => state.set_progress_stage("Compiling"),
                    CompileStage::Linking => state.set_progress_stage("Linking"),
                    CompileStage::Generating => state.set_progress_stage("Generating"),
                    CompileStage::Complete => state.set_progress_stage("Complete"),
                }
                
                state.set_current_file(&compile_state.current_file);
                
                // Update progress tracker - ensure cumulative progress across stages
                // calculate_progress() returns stage-specific ranges, so we need to ensure
                // progress doesn't reset when transitioning stages
                let stage_progress = compile_state.calculate_progress();
                let old_progress = state.progress_percent;
                
                if let Some(ref mut tracker) = state.progress_tracker {
                    // Use weighted estimation (70% current rate, 30% historical)
                    let method = EstimateMethod::Weighted {
                        current_weight: 0.7,
                        historical_weight: 0.3,
                    };
                    
                    // Update based on files compiled (more accurate than percentage)
                    if compile_state.total_files > 0 {
                        // Set total_items BEFORE updating progress
                        tracker.total_items = Some(compile_state.total_files);
                        tracker.update_progress(compile_state.files_compiled, method);
                        // Sync tracker's progress_percent back to state (this is the source of truth)
                        // This ensures cumulative progress based on files, not stage-specific ranges
                        state.progress_percent = tracker.progress_percent;
                    } else {
                        // Fallback: use stage-based progress but ensure it's cumulative
                        // Don't reset progress when transitioning stages - only increase it
                        // Use the maximum of current tracker progress and stage progress to avoid jumps
                        let new_progress = stage_progress.max(tracker.progress_percent);
                        if new_progress > tracker.progress_percent {
                            tracker.set_progress_percent(new_progress);
                        }
                        // Still update time estimates
                        tracker.update_progress((tracker.progress_percent * 100.0) as usize, method);
                        state.progress_percent = tracker.progress_percent;
                    }
                } else {
                    // Fallback if no tracker - use stage-based progress, but don't decrease
                    // Ensure progress only increases, never decreases
                    state.progress_percent = stage_progress.max(state.progress_percent);
                }
                
                // Log progress update if it changed
                let new_progress = state.progress_percent;
                if (new_progress - old_progress).abs() > 0.01 {
                    // Progress changed significantly, log it
                    log_output(&log_file, "");
                    log_output(&log_file, &format!("{{commanded progress bar percent: {:.2}}}", new_progress));
                    log_output(&log_file, "");
                }
            }
        }
    }
    
    // Wait for process to finish
    let exit_status = child.wait();
    
    // Unregister process from process manager (completed normally)
    process_manager.unregister(pid);
    
    // Record completion and timing data
    let (total_time, stage_times) = {
        let state = dashboard.lock().unwrap();
        if let Some(ref tracker) = state.progress_tracker {
            let total = tracker.elapsed_time;
            let mut stages = std::collections::HashMap::new();
            for (stage, timing) in &tracker.stage_times {
                stages.insert(*stage, timing.elapsed);
            }
            (total, stages)
        } else {
            (std::time::Duration::ZERO, std::collections::HashMap::new())
        }
    };
    
    {
        let mut state = dashboard.lock().unwrap();
        state.is_running = false;
        
        match exit_status {
            Ok(status) => {
                if status.success() {
                    state.progress_percent = 100.0;
                    state.set_progress_stage("Complete");
                    
                    // Transition to Complete stage in tracker
                    if let Some(ref mut tracker) = state.progress_tracker {
                        tracker.transition_stage(ProgressStage::Complete);
                        tracker.progress_percent = 100.0;
                    }
                    
                    state.set_status_text("Compilation completed successfully");
                    // Log final progress update
                    log_output(&log_file, "");
                    log_output(&log_file, "{{commanded progress bar percent: 100.0}}");
                    log_output(&log_file, "");
                    
                    // Record successful completion to history
                    if !stage_times.is_empty() {
                        let _ = history.record_completion(sketch_dir.clone(), stage_times, total_time);
                        let _ = history.save();
                    }
                } else {
                    let error_msg = format!("Compilation failed with exit code: {:?}", status.code());
                    state.set_status_text(&error_msg);
                    state.add_output_line(error_msg.clone());
                    log_output(&log_file, &error_msg);
                }
            }
            Err(e) => {
                let error_msg = format!("Error waiting for process: {}", e);
                state.set_status_text(&error_msg);
                state.add_output_line(error_msg.clone());
                log_output(&log_file, &error_msg);
            }
        }
    }
}
