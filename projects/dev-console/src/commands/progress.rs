// Progress command execution

use crate::dashboard::DashboardState;
use crate::settings::Settings;
use crate::commands::utils::{remove_ansi_escapes, extract_percentage, extract_current_file};
use crate::process_manager::ProcessManager;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs;

/// Execute progress command and parse output
pub fn execute_progress_command(
    dashboard: Arc<Mutex<DashboardState>>,
    settings: Settings,
    process_manager: Arc<ProcessManager>,
) {
    // Build command: python pmake.py progress
    let sketch_dir = PathBuf::from(&settings.sketch_directory);
    
    // Look for pmake.py in the sketch directory or parent
    let pmake_script = sketch_dir.join("pmake.py");
    let pmake_script_parent = sketch_dir.parent().map(|p| p.join("pmake.py"));
    
    let script_path = if pmake_script.exists() {
        pmake_script
    } else if let Some(parent_script) = pmake_script_parent {
        if parent_script.exists() {
            parent_script
        } else {
            {
                let mut state = dashboard.lock().unwrap();
                state.is_running = false;
                state.status_text = "Error: pmake.py not found".to_string();
                state.output_lines.push("Error: Could not find pmake.py script".to_string());
            }
            return;
        }
    } else {
        {
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.status_text = "Error: pmake.py not found".to_string();
            state.output_lines.push("Error: Could not find pmake.py script".to_string());
        }
        return;
    };
    
    // Add initial debug message
    {
        let mut state = dashboard.lock().unwrap();
        state.output_lines.push(format!("Executing: python {:?} progress", script_path));
        state.output_lines.push(format!("Working directory: {:?}", sketch_dir));
        state.output_lines.push(format!("Script exists: {}", script_path.exists()));
        state.output_lines.push("Starting command execution...".to_string());
        if state.output_lines.len() > 1 {
            state.output_scroll = state.output_lines.len().saturating_sub(1);
        }
    }
    
    // Find workspace root
    let workspace_root = sketch_dir
        .ancestors()
        .find(|path| {
            let pyproject = path.join("pyproject.toml");
            if pyproject.exists() {
                if let Ok(content) = fs::read_to_string(&pyproject) {
                    return content.contains("[tool.uv") || content.contains("[project]");
                }
            }
            false
        })
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| {
            sketch_dir.parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| sketch_dir.clone())
        });
    
    // Try to use uv run first (if available), otherwise use python with PYTHONPATH
    let mut cmd = if which::which("uv").is_ok() {
        let mut uv_cmd = Command::new("uv");
        uv_cmd.arg("run");
        uv_cmd.arg("python");
        uv_cmd.arg("-u"); // Unbuffered output
        uv_cmd
    } else {
        let mut py_cmd = Command::new("python");
        py_cmd.arg("-u"); // Unbuffered output
        let pythonpath = workspace_root.to_string_lossy().to_string();
        py_cmd.env("PYTHONPATH", &pythonpath);
        py_cmd
    };
    
    cmd.arg(&script_path);
    cmd.arg("progress");
    cmd.current_dir(&workspace_root);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.env("PYTHONUNBUFFERED", "1");
    
    // Add debug info about command
    {
        let mut state = dashboard.lock().unwrap();
        state.output_lines.push(format!("Workspace root: {:?}", workspace_root));
        state.output_lines.push(format!("Using UV: {}", which::which("uv").is_ok()));
    }
    
    let mut child = match cmd.spawn() {
        Ok(child) => {
            // Register process with process manager for cleanup tracking
            process_manager.register(&child);
            child
        }
        Err(e) => {
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.status_text = format!("Error: {}", e);
            state.output_lines.push(format!("Failed to execute command: {}", e));
            return;
        }
    };
    
    // Store PID for unregistering when process completes
    let pid = child.id();
    
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    
    // Spawn thread to read stderr
    let dashboard_stderr = dashboard.clone();
    if let Some(stderr) = stderr {
        let stderr_reader = BufReader::new(stderr);
        thread::spawn(move || {
            for line in stderr_reader.lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => break,
                };
                
                let line_trimmed = line.trim();
                if line_trimmed.is_empty() {
                    continue;
                }
                
                let cleaned = remove_ansi_escapes(&line);
                let line_trimmed = cleaned.trim();
                
                if !line_trimmed.is_empty() {
                    {
                        let mut state = dashboard_stderr.lock().unwrap();
                        state.output_lines.push(line_trimmed.to_string());
                        if state.output_lines.len() > 1 {
                            state.output_scroll = state.output_lines.len().saturating_sub(1);
                        }
                    }
                }
            }
        });
    }
    
    // Read stdout
    if let Some(stdout) = stdout {
        let mut reader = BufReader::new(stdout);
        let mut line_buffer = Vec::new();
        
        loop {
            line_buffer.clear();
            match reader.read_until(b'\n', &mut line_buffer) {
                Ok(0) => break,
                Ok(_) => {
                    let line = match String::from_utf8(line_buffer.clone()) {
                        Ok(s) => s,
                        Err(_) => continue,
                    };
                    
                    let cleaned = remove_ansi_escapes(&line);
                    
                    let processed_line = if cleaned.contains('\r') {
                        cleaned.split('\r').last().unwrap_or(&cleaned).to_string()
                    } else {
                        cleaned
                    };
                    
                    let trimmed = processed_line.trim();
                    
                    if trimmed.is_empty() {
                        continue;
                    }
                    
                    let is_progress = trimmed.contains('%') && (trimmed.contains("Compiling") || 
                        trimmed.contains("Initializing") || 
                        trimmed.contains("Linking") || 
                        trimmed.contains("Generating"));
                    
                    {
                        let mut state = dashboard.lock().unwrap();
                        
                        if is_progress {
                            if trimmed.contains("Initializing") || trimmed.contains("initializing") {
                                state.progress_stage = "Initializing".to_string();
                            } else if trimmed.contains("Compiling") || trimmed.contains("compiling") {
                                state.progress_stage = "Compiling".to_string();
                            } else if trimmed.contains("Linking") || trimmed.contains("linking") {
                                state.progress_stage = "Linking".to_string();
                            } else if trimmed.contains("Generating") || trimmed.contains("generating") {
                                state.progress_stage = "Generating".to_string();
                            }
                            
                            if let Some(percent) = extract_percentage(&trimmed) {
                                state.progress_percent = percent;
                            }
                            
                            if let Some(file) = extract_current_file(&trimmed) {
                                state.current_file = file;
                            }
                            
                            if !state.output_lines.is_empty() && state.output_lines.last().map(|s| s.contains('%')).unwrap_or(false) {
                                let last_idx = state.output_lines.len() - 1;
                                state.output_lines[last_idx] = trimmed.to_string();
                            } else {
                                state.output_lines.push(trimmed.to_string());
                            }
                        } else {
                            state.output_lines.push(trimmed.to_string());
                        }
                        
                        if state.output_lines.len() > 1 {
                            state.output_scroll = state.output_lines.len().saturating_sub(1);
                        }
                    }
                }
                Err(_) => break,
            }
        }
    }
    
    // Wait for process to finish
    let exit_status = child.wait();
    
    // Unregister process from process manager (completed normally)
    process_manager.unregister(pid);
    
    {
        let mut state = dashboard.lock().unwrap();
        match exit_status {
            Ok(status) => {
                if status.success() {
                    state.output_lines.push("Command completed successfully".to_string());
                } else {
                    state.output_lines.push(format!("Command exited with code: {:?}", status.code()));
                }
            }
            Err(e) => {
                state.output_lines.push(format!("Error waiting for process: {}", e));
            }
        }
    }
    
    // Mark as complete
    {
        let mut state = dashboard.lock().unwrap();
        state.is_running = false;
        if state.progress_percent < 100.0 {
            state.progress_percent = 100.0;
        }
        state.status_text = "Complete".to_string();
    }
}
