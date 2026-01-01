// Progress command execution (Rust-based, direct arduino-cli call)

use crate::dashboard::DashboardState;
use crate::settings::Settings;
use crate::commands::utils::remove_ansi_escapes;
use crate::process_manager::ProcessManager;
use crate::path_utils::{find_project_root, find_arduino_cli, get_library_path};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
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
                
                if self.total_files > 0 {
                    let file_progress = self.files_compiled as f64 / self.total_files as f64;
                    let file_based = 5.0 + (file_progress * 60.0);
                    let time_based = 5.0 + (compile_elapsed * 2.0).min(60.0);
                    (file_based * 0.9 + time_based * 0.1).min(65.0)
                } else {
                    5.0 + (compile_elapsed * 2.0).min(60.0)
                }
            }
            CompileStage::Linking => {
                let link_elapsed = self.link_stage_start
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                65.0 + (link_elapsed * 5.0).min(25.0)
            }
            CompileStage::Generating => {
                let gen_elapsed = self.generate_stage_start
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                90.0 + (gen_elapsed * 3.0).min(9.9)
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
    
    // Add initial message
    {
        let mut state = dashboard.lock().unwrap();
        state.output_lines.push(format!("Executing: {:?} compile --fqbn {} --libraries {:?} --verbose {:?}", 
            arduino_cli, settings.fqbn, library_path, sketch_file));
        state.output_lines.push(format!("Build path: {:?}", build_path));
        state.output_lines.push(format!("Library path: {:?}", library_path));
        state.output_lines.push(format!("Library path exists: {}", library_path.exists()));
        state.output_lines.push(format!("Arduino CLI path: {:?}", arduino_cli));
        state.output_lines.push(format!("Arduino CLI exists: {}", arduino_cli.exists()));
        state.is_running = true;
        state.set_progress_stage("Initializing");
        state.progress_percent = 0.0;
    }
    
    // Check if arduino-cli exists (unless it's in PATH)
    if !arduino_cli.exists() && arduino_cli.to_string_lossy() != "arduino-cli" {
        let mut state = dashboard.lock().unwrap();
        state.is_running = false;
        state.set_status_text(&format!("Error: arduino-cli not found at: {:?}", arduino_cli));
        state.output_lines.push(format!("Error: arduino-cli not found at: {:?}", arduino_cli));
        state.output_lines.push("Please ensure arduino-cli.exe is installed in the Arduino directory at the workspace root.".to_string());
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
            state.output_lines.push(format!("Error: Failed to start arduino-cli: {}", e));
            state.output_lines.push(format!("Tried path: {:?}", arduino_cli));
            if !arduino_cli.exists() && arduino_cli.to_string_lossy() != "arduino-cli" {
                state.output_lines.push("The arduino-cli executable was not found at the expected location.".to_string());
            }
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
    
    // Read stdout and parse
    let mut compile_state = CompileState::new();
    
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
            
            // Add to output
            {
                let mut state = dashboard.lock().unwrap();
                state.output_lines.push(trimmed.to_string());
                if state.output_lines.len() > 1 {
                    state.output_scroll = state.output_lines.len().saturating_sub(1);
                }
            }
            
            // Parse line for compilation state
            // Detect errors
            if line_lower.contains("error") || line_lower.contains("fatal") {
                // Error detected - already added to output
                continue;
            }
            
            // Detect stages
            if line_lower.contains("detecting libraries") || line_lower.contains("detecting library") {
                compile_state.stage = CompileStage::Compiling;
                if compile_state.compile_stage_start.is_none() {
                    compile_state.compile_stage_start = Some(std::time::Instant::now());
                }
            } else if line_lower.contains("generating function prototypes") || line_lower.contains("generating prototypes") {
                compile_state.stage = CompileStage::Compiling;
            } else if line_lower.contains("linking everything together") || (line_lower.contains("linking") && line_lower.contains("together")) {
                compile_state.stage = CompileStage::Linking;
                compile_state.current_file.clear();
                if compile_state.link_stage_start.is_none() {
                    compile_state.link_stage_start = Some(std::time::Instant::now());
                }
            } else if line_lower.contains("creating esp32") || line_lower.contains("creating image") || 
                      (line_lower.contains("esptool") && line_lower.contains("elf2image")) {
                compile_state.stage = CompileStage::Generating;
                compile_state.current_file.clear();
                if compile_state.generate_stage_start.is_none() {
                    compile_state.generate_stage_start = Some(std::time::Instant::now());
                }
            } else if line_lower.contains("sketch uses") || line_lower.contains("global variables use") {
                compile_state.stage = CompileStage::Complete;
                compile_state.current_file.clear();
            }
            
            // Detect compilation commands/files
            if line.contains("xtensa-esp32s3-elf-g++") || line.contains("xtensa-esp32s3-elf-gcc") {
                if line.contains("-c") {
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
            
            // Update dashboard state
            {
                let mut state = dashboard.lock().unwrap();
                state.progress_percent = compile_state.calculate_progress();
                
                match compile_state.stage {
                    CompileStage::Initializing => state.set_progress_stage("Initializing"),
                    CompileStage::Compiling => state.set_progress_stage("Compiling"),
                    CompileStage::Linking => state.set_progress_stage("Linking"),
                    CompileStage::Generating => state.set_progress_stage("Generating"),
                    CompileStage::Complete => state.set_progress_stage("Complete"),
                }
                
                state.set_current_file(&compile_state.current_file);
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
                    state.set_progress_stage("Complete");
                    state.set_status_text("Compilation completed successfully");
                    state.output_lines.push("Compilation completed successfully".to_string());
                } else {
                    state.set_status_text(&format!("Compilation failed with exit code: {:?}", status.code()));
                    state.output_lines.push(format!("Compilation failed with exit code: {:?}", status.code()));
                }
            }
            Err(e) => {
                state.set_status_text(&format!("Error waiting for process: {}", e));
                state.output_lines.push(format!("Error waiting for process: {}", e));
            }
        }
    }
}
