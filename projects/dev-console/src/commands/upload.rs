// Upload command execution (Rust-based, direct arduino-cli call)

use crate::dashboard::DashboardState;
use crate::settings::Settings;
use crate::commands::utils::remove_ansi_escapes;
use crate::process_manager::ProcessManager;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_WRITING_AT: Regex = Regex::new(
        r"(?i)Writing at (0x[0-9a-fA-F]+).*?(\d+\.?\d*)%"
    ).unwrap();
}

/// Execute upload command using Rust (direct arduino-cli call)
pub fn execute_upload_rust(
    dashboard: Arc<Mutex<DashboardState>>,
    settings: Settings,
    process_manager: Arc<ProcessManager>,
) {
    // Build arduino-cli command
    let sketch_dir = PathBuf::from(&settings.sketch_directory);
    let build_path = sketch_dir.join("build");
    
    // Find project root (workspace root) - same logic as Python version
    let project_root = sketch_dir
        .parent()  // esp32-s3__LB-Gold
        .and_then(|p| p.parent())  // projects
        .and_then(|p| p.parent())  // dev-boards (workspace root)
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| sketch_dir.clone());
    
    // Find arduino-cli
    let arduino_cli = if settings.env == "arduino" {
        let workspace_path = project_root.join("Arduino").join("arduino-cli.exe");
        if workspace_path.exists() {
            workspace_path
        } else {
            if which::which("arduino-cli").is_ok() {
                PathBuf::from("arduino-cli")
            } else {
                workspace_path
            }
        }
    } else {
        PathBuf::from("arduino-cli")
    };
    
    // Build command arguments - same as Python upload_custom
    let mut cmd = Command::new(&arduino_cli);
    cmd.arg("upload");
    cmd.arg("-p").arg(&settings.port);
    cmd.arg("--fqbn").arg(&settings.fqbn);
    cmd.arg("--build-path").arg(&build_path);
    cmd.arg(&sketch_dir);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.current_dir(&sketch_dir);
    
    // Add initial message
    {
        let mut state = dashboard.lock().unwrap();
        state.output_lines.push(format!("Uploading to {} on port {}...", settings.board_model, settings.port));
        state.output_lines.push(format!("Executing: {:?} upload -p {} --fqbn {} --build-path {:?} {:?}", 
            arduino_cli, settings.port, settings.fqbn, build_path, sketch_dir));
        state.is_running = true;
        state.progress_stage = "Initializing".to_string();
        state.progress_percent = 0.0;
    }
    
    // Check if arduino-cli exists
    if !arduino_cli.exists() && arduino_cli.to_string_lossy() != "arduino-cli" {
        let mut state = dashboard.lock().unwrap();
        state.is_running = false;
        state.status_text = format!("Error: arduino-cli not found at: {:?}", arduino_cli);
        state.output_lines.push(format!("Error: arduino-cli not found at: {:?}", arduino_cli));
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
            state.status_text = format!("Error: Failed to start arduino-cli: {}", e);
            state.output_lines.push(format!("Error: Failed to start arduino-cli: {}", e));
            return;
        }
    };
    
    // Store PID for unregistering when process completes
    let pid = child.id();
    
    // Read stderr in separate thread
    let dashboard_stderr = dashboard.clone();
    if let Some(stderr) = child.stderr.take() {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let cleaned = remove_ansi_escapes(&line);
                    let trimmed = cleaned.trim();
                    if !trimmed.is_empty() {
                        let mut state = dashboard_stderr.lock().unwrap();
                        state.output_lines.push(trimmed.to_string());
                        if state.output_lines.len() > 1 {
                            state.output_scroll = state.output_lines.len().saturating_sub(1);
                        }
                    }
                }
            }
        });
    }
    
    // Track upload state
    let mut current_address: Option<String> = None;
    let mut flash_count = 0;
    
    // Read stdout and parse upload progress
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        
        for line_result in reader.lines() {
            let line = match line_result {
                Ok(l) => l,
                Err(_) => break,
            };
            
            let cleaned = remove_ansi_escapes(&line);
            let line_lower = cleaned.to_lowercase();
            let trimmed = cleaned.trim();
            
            if trimmed.is_empty() {
                continue;
            }
            
            // Suppress "Hash of data verified" (like Python version)
            if line_lower.contains("hash of data verified") {
                continue;
            }
            
            // Suppress "Compressed" lines after first block (like Python version)
            if line_lower.contains("compressed") && line_lower.contains("bytes to") {
                if flash_count > 0 {
                    continue;
                }
            }
            
            // Handle "Writing at" lines - extract progress
            if trimmed.contains("Writing at") {
                if let Some(captures) = RE_WRITING_AT.captures(&trimmed) {
                    if let (Some(addr_match), Some(percent_match)) = (captures.get(1), captures.get(2)) {
                        let addr = addr_match.as_str().to_string();
                        if let Ok(percent) = percent_match.as_str().parse::<f64>() {
                            current_address = Some(addr.clone());
                            
                            {
                                let mut state = dashboard.lock().unwrap();
                                state.progress_percent = percent;
                                state.progress_stage = format!("Writing at {}", addr);
                                state.current_file = addr;
                                
                                // Add progress line to output
                                state.output_lines.push(trimmed.to_string());
                                if state.output_lines.len() > 1 {
                                    state.output_scroll = state.output_lines.len().saturating_sub(1);
                                }
                            }
                            continue;
                        }
                    }
                }
            }
            
            // Handle "Wrote" lines - flash complete
            if trimmed.contains("Wrote") && trimmed.contains("compressed") {
                flash_count += 1;
                current_address = None;
                
                {
                    let mut state = dashboard.lock().unwrap();
                    state.progress_percent = 100.0;
                    state.progress_stage = "Upload complete".to_string();
                    state.output_lines.push(trimmed.to_string());
                    if state.output_lines.len() > 1 {
                        state.output_scroll = state.output_lines.len().saturating_sub(1);
                    }
                }
                continue;
            }
            
            // Handle "Hard resetting"
            if trimmed.contains("Hard resetting") {
                {
                    let mut state = dashboard.lock().unwrap();
                    state.output_lines.push(trimmed.to_string());
                    if state.output_lines.len() > 1 {
                        state.output_scroll = state.output_lines.len().saturating_sub(1);
                    }
                }
                continue;
            }
            
            // Add other output lines (but skip empty lines if we have a progress bar)
            if current_address.is_some() && trimmed.is_empty() {
                continue;
            }
            
            // Add regular output
            {
                let mut state = dashboard.lock().unwrap();
                state.output_lines.push(trimmed.to_string());
                if state.output_lines.len() > 1 {
                    state.output_scroll = state.output_lines.len().saturating_sub(1);
                }
            }
        }
    }
    
    // Wait for process to finish
    let exit_status = child.wait();
    
    // Unregister process from process manager (completed normally)
    process_manager.unregister(pid);
    
    {
        let mut state = dashboard.lock().unwrap();
        state.is_running = false;
        
        match exit_status {
            Ok(status) => {
                if status.success() {
                    state.progress_percent = 100.0;
                    state.progress_stage = "Upload complete".to_string();
                    state.status_text = "Upload completed successfully".to_string();
                    state.output_lines.push("Upload completed successfully".to_string());
                } else {
                    state.status_text = format!("Upload failed with exit code: {:?}", status.code());
                    state.output_lines.push(format!("Upload failed with exit code: {:?}", status.code()));
                }
            }
            Err(e) => {
                state.status_text = format!("Error waiting for process: {}", e);
                state.output_lines.push(format!("Error waiting for process: {}", e));
            }
        }
    }
}
