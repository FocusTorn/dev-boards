// Progress command execution (Rust-based, direct arduino-cli call)

use crate::dashboard::DashboardState;
use crate::settings::Settings;
use crate::commands::utils::remove_ansi_escapes;
use crate::commands::compile_state::{CompileState, CompileStage};
use crate::commands::compile_parser::{detect_stage_change, parse_compilation_info};
use crate::commands::process_handler::ProcessHandler;
use crate::process_manager::ProcessManager;
use crate::path_utils::{find_project_root, find_arduino_cli, get_library_path};
use crate::progress_tracker::{ProgressStage, EstimateMethod};
use crate::progress_history::ProgressHistory;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::fs::{File, OpenOptions};

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
    
    // Spawn process using process handler
    let mut process_handler = match ProcessHandler::spawn(cmd, process_manager.clone()) {
        Ok(handler) => handler,
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
    
    // Start stderr reader in separate thread
    process_handler.start_stderr_reader(dashboard.clone(), log_file.clone());
    
    // Read stdout and parse
    let mut compile_state = CompileState::new();
    let mut pending_lines: Vec<String> = Vec::new(); // Buffer for lines when lock is busy
    
    if let Some(stdout) = process_handler.take_stdout() {
        let reader = BufReader::new(stdout);
        
        for line_result in reader.lines() {
            let line = match line_result {
                Ok(l) => l,
                Err(_) => break,
            };
            
            // Preserve ANSI codes for colorization - only clean for parsing
            let cleaned = remove_ansi_escapes(&line);
            let trimmed = cleaned.trim();
            
            if trimmed.is_empty() {
                continue;
            }
            
            let trimmed_line = line.trim();
            // Log to file immediately (no lock needed)
            log_output(&log_file, trimmed_line);
            
            // Try to add line to dashboard - use try_lock to avoid blocking UI thread
            let current_progress = if let Ok(mut state) = dashboard.try_lock() {
                // Got the lock - add pending lines first, then this one
                for pending_line in pending_lines.drain(..) {
                    state.add_output_line(pending_line);
                }
                state.add_output_line(trimmed_line.to_string());
                // Get current progress while we have the lock
                state.progress_percent
            } else {
                // Lock is busy (UI thread is rendering) - queue this line for later
                // This prevents blocking the UI thread during rapid output bursts
                pending_lines.push(trimmed_line.to_string());
                // Use calculated progress as fallback
                compile_state.calculate_progress()
            };
            // Auto-scroll is handled during rendering with correct visible_height
            
            // Parse line for compilation state using parser module
            let (stage_changed, should_continue) = detect_stage_change(&line, &mut compile_state, current_progress);
            if !should_continue {
                // Error detected - already added to output
                continue;
            }
            
            // Parse compilation info (files, commands, etc.)
            parse_compilation_info(&line, &mut compile_state);
            
            // Calculate progress BEFORE locking (expensive operations outside lock)
            let stage_progress = compile_state.calculate_progress();
            
            // Update dashboard state with progress tracking - SINGLE LOCK for all updates
            // Only update if stage changed OR progress changed significantly to reduce lock contention
            let should_update = stage_changed || {
                // Check if progress would change significantly (do this calculation outside lock)
                let current_tracker_progress = if compile_state.total_files > 0 {
                    // Estimate based on files
                    (compile_state.files_compiled as f64 / compile_state.total_files as f64) * 60.0 + 5.0
                } else {
                    stage_progress
                };
                // Only update if progress would change by more than 0.5%
                (current_tracker_progress - compile_state.last_logged_progress).abs() > 0.5
            };
            
            if should_update {
                // Try to get lock, but don't block - if busy, skip this update
                // Progress will be updated on next successful lock acquisition
                let mut state = match dashboard.try_lock() {
                    Ok(s) => s,
                    Err(_) => {
                        // Lock busy - skip this update, will catch up later
                        continue;
                    }
                };
                
                // Flush any pending lines while we have the lock
                if !pending_lines.is_empty() {
                    for pending_line in pending_lines.drain(..) {
                        state.add_output_line(pending_line);
                    }
                }
                
                // Update stage if changed
                if stage_changed {
                    let new_stage = match compile_state.stage {
                        CompileStage::Initializing => ProgressStage::Initializing,
                        CompileStage::Compiling => ProgressStage::Compiling,
                        CompileStage::Linking => ProgressStage::Linking,
                        CompileStage::Generating => ProgressStage::Generating,
                        CompileStage::Complete => ProgressStage::Complete,
                    };
                    state.transition_progress_stage(new_stage);
                }
                
                match compile_state.stage {
                    CompileStage::Initializing => state.set_progress_stage("Initializing"),
                    CompileStage::Compiling => state.set_progress_stage("Compiling"),
                    CompileStage::Linking => state.set_progress_stage("Linking"),
                    CompileStage::Generating => state.set_progress_stage("Generating"),
                    CompileStage::Complete => state.set_progress_stage("Complete"),
                }
                
                state.set_current_file(&compile_state.current_file);
                
                // Update progress tracker - ensure cumulative progress across stages
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
                        state.progress_percent = tracker.progress_percent;
                    } else {
                        // Fallback: use stage-based progress but ensure it's cumulative
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
                    state.progress_percent = stage_progress.max(state.progress_percent);
                }
                
                // Log progress update if it changed significantly
                let new_progress = state.progress_percent;
                if (new_progress - old_progress).abs() > 0.01 {
                    // Progress changed significantly, log it
                    compile_state.last_logged_progress = new_progress;
                    log_output(&log_file, "");
                    log_output(&log_file, &format!("{{commanded progress bar percent: {:.2}}}", new_progress));
                    log_output(&log_file, "");
                }
            }
        }
        
        // Flush any remaining pending lines before exiting
        if !pending_lines.is_empty() {
            if let Ok(mut state) = dashboard.lock() {
                for pending_line in pending_lines.drain(..) {
                    state.add_output_line(pending_line);
                }
            }
        }
    }
    
    // Wait for process to finish
    let exit_status = process_handler.wait(process_manager);
    
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
