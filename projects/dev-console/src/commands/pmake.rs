// PMake command execution (Build, Compile, Upload)

// IMPORTS ------------------>> 

use crate::dashboard::DashboardState;
use crate::settings::Settings;
use crate::commands::utils::remove_ansi_escapes;
use crate::process_manager::ProcessManager;
use crate::path_utils::{find_workspace_root, find_pmake_script};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

//--------------------------------------------------------<<

/// Execute pmake command (Build, Compile, Upload) and capture output
#[allow(dead_code)]
pub fn execute_pmake_command(
    dashboard: Arc<Mutex<DashboardState>>,
    settings: Settings,
    command: String,
    process_manager: Arc<ProcessManager>,
) {
    let sketch_dir = PathBuf::from(&settings.sketch_directory);
    
    let script_path = match find_pmake_script(&sketch_dir) {
        Some(path) => path,
        None => {
            let mut state = dashboard.lock().unwrap();
            state.set_status_text("Error: pmake.py not found");
            state.add_output_line("Error: Could not find pmake.py script".to_string());
            return;
        }
    };
    
    let workspace_root = find_workspace_root(&sketch_dir);
    
    let pmake_arg = match command.as_str() {
        "Build" => "build",
        "Compile" => "compile",
        "Upload" => "upload",
        _ => {
            let mut state = dashboard.lock().unwrap();
            state.set_status_text(&format!("Error: Unknown command: {}", command));
            state.output_lines.push(format!("Error: Unknown command: {}", command));
            return;
        }
    };
    
    let mut cmd = if which::which("uv").is_ok() {
        let mut uv_cmd = Command::new("uv");
        uv_cmd.arg("run");
        uv_cmd.arg("python");
        uv_cmd.arg("-u");
        uv_cmd
    } else {
        let mut py_cmd = Command::new("python");
        py_cmd.arg("-u");
        let pythonpath = workspace_root.to_string_lossy().to_string();
        py_cmd.env("PYTHONPATH", &pythonpath);
        py_cmd
    };
    
    cmd.arg(&script_path);
    cmd.arg(pmake_arg);
    cmd.current_dir(&workspace_root);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.env("PYTHONUNBUFFERED", "1");
    
    let mut child = match cmd.spawn() {
        Ok(child) => {
            // Register process with process manager for cleanup tracking
            process_manager.register(&child);
            child
        }
        Err(e) => {
            let mut state = dashboard.lock().unwrap();
            state.set_status_text(&format!("Error: {}", e));
            state.output_lines.push(format!("Failed to execute command: {}", e));
            return;
        }
    };
    
    // Store PID for unregistering when process completes
    let pid = child.id();
    
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    
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
                
                {
                    let mut state = dashboard_stderr.lock().unwrap();
                    state.output_lines.push(format!("[stderr] {}", line));
                    if state.output_lines.len() > 1 {
                        // Don't auto-scroll - let user control scrolling manually
                    }
                }
            }
        });
    }
    
    if let Some(stdout) = stdout {
        let reader = BufReader::new(stdout);
        
        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => break,
            };
            
            let cleaned_line = remove_ansi_escapes(&line);
            let line_trimmed = cleaned_line.trim();
            
            if !line_trimmed.is_empty() {
                {
                    let mut state = dashboard.lock().unwrap();
                    state.output_lines.push(cleaned_line.clone());
                    if state.output_lines.len() > 1 {
                        // Don't auto-scroll - let user control scrolling manually
                    }
                }
            }
        }
    }
    
    let exit_status = child.wait();
    
    // Unregister process from process manager (completed normally)
    process_manager.unregister(pid);
    
    {
        let mut state = dashboard.lock().unwrap();
        match exit_status {
            Ok(status) => {
                if status.success() {
                    state.set_status_text(&format!("{} completed successfully", command));
                    state.output_lines.push(format!("{} completed successfully", command));
                } else {
                    state.set_status_text(&format!("{} failed with exit code: {:?}", command, status.code()));
                    state.output_lines.push(format!("{} failed with exit code: {:?}", command, status.code()));
                }
            }
            Err(e) => {
                state.set_status_text(&format!("Command execution error: {}", e));
                state.output_lines.push(format!("Command execution error: {}", e));
            }
        }
    }
}
