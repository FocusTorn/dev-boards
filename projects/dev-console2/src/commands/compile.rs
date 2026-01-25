use super::{compile_state, compile_parser, path_utils, process::ProcessHandler};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;

#[derive(Clone, Debug)]
pub struct Settings {
    pub sketch_directory: String,
    pub sketch_name: String,
    pub fqbn: String,
    pub port: String,
    pub baudrate: u32,
    pub board_model: String,
    pub env: String,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ProgressUpdate {
    Status(String),
    OutputLine(String),
    Percentage(f64),
    Stage(String),
    Completed,
    CompletedWithMetrics {
        stage_times: std::collections::HashMap<crate::commands::predictor::CompileStage, f64>,
    },
    Failed(String),
}

pub fn run_compile(settings: &Settings, stats: crate::commands::history::StageStats, cancel_signal: Arc<AtomicBool>, progress_callback: impl FnMut(ProgressUpdate) + Send + 'static) {
    let callback = Arc::new(Mutex::new(progress_callback));

    callback.lock().unwrap()(ProgressUpdate::Stage("Initializing".to_string()));

    let sketch_dir = PathBuf::from(&settings.sketch_directory);
    let sketch_file = sketch_dir.join(format!("{}.ino", settings.sketch_name));

    if !sketch_file.exists() {
        callback.lock().unwrap()(ProgressUpdate::Failed(format!("Sketch file not found: {:?}", sketch_file)));
        return;
    }
    
    let project_root = match path_utils::find_workspace_root() {
        Ok(root) => root,
        Err(e) => {
            callback.lock().unwrap()(ProgressUpdate::Failed(format!("Failed to determine workspace root: {}", e)));
            return;
        }
    };

    // Temp directory logic
    let (compile_dir, temp_dir_guard) = match setup_compile_directory(&sketch_dir, &sketch_file, &project_root) {
        Ok((dir, guard)) => (dir, guard),
        Err(e) => {
            callback.lock().unwrap()(ProgressUpdate::Failed(e));
            return;
        }
    };
    
    if temp_dir_guard.is_some() {
        callback.lock().unwrap()(ProgressUpdate::OutputLine(format!("[NOTE] Using temporary compile directory: {:?}", compile_dir)));
    }

    let build_path = sketch_dir.join("build");
    let library_path = path_utils::get_library_path(&project_root, &settings.board_model);
    let arduino_cli = path_utils::find_arduino_cli(&settings.env, &project_root);
    
    callback.lock().unwrap()(ProgressUpdate::OutputLine(format!("Using arduino-cli: {:?}", arduino_cli)));

    let mut cmd = Command::new(&arduino_cli);
    cmd.arg("compile")
        .arg("--fqbn")
        .arg(&settings.fqbn)
        .arg("--libraries")
        .arg(&library_path)
        .arg("--build-path")
        .arg(&build_path)
        .arg("--verbose")
        .arg(&compile_dir)
        .current_dir(&compile_dir);

    let process_handler = match ProcessHandler::spawn(cmd) {
        Ok(handler) => handler,
        Err(e) => {
            callback.lock().unwrap()(ProgressUpdate::Failed(format!("Failed to spawn arduino-cli: {}", e)));
            return;
        }
    };

    let compile_state = Arc::new(Mutex::new(compile_state::CompileState::new(stats.weights, stats.averages)));
    callback.lock().unwrap()(ProgressUpdate::Stage("Compiling".to_string()));

    let callback_clone = callback.clone();
    let state_clone = compile_state.clone();
    let result = process_handler.read_output(cancel_signal.clone(), move |line| {
        let mut cb = callback_clone.lock().unwrap();
        let mut state = state_clone.lock().unwrap();
        let (stage_changed, should_continue) = compile_parser::detect_stage_change(&line, &mut state, 0.0, &mut |msg| {
            cb(ProgressUpdate::OutputLine(msg));
        });
        if !should_continue {
            cb(ProgressUpdate::OutputLine(line));
            return;
        }
        
        if stage_changed {
             cb(ProgressUpdate::Stage(format!("{:?}", state.stage)));
        }

        compile_parser::parse_compilation_info(&line, &mut state);
        let progress = state.calculate_progress();
        cb(ProgressUpdate::Percentage(progress));
        cb(ProgressUpdate::OutputLine(line));

        // Watchdog: Check if we are stuck in a stage without markers
        // This method now handles its own internal "has_warned" state
        if let Some(warning) = state.check_for_missing_markers() {
            cb(ProgressUpdate::OutputLine(warning));
        }
    });

    let mut cb = callback.lock().unwrap();
    match result {
        Ok(true) => {
            // Capture final stage duration
            let mut state = compile_state.lock().unwrap();
            let duration = state.last_marker_time.elapsed().as_secs_f64();
            let stage = state.stage;
            state.stage_durations.insert(stage, duration);
            
            cb(ProgressUpdate::CompletedWithMetrics { 
                stage_times: state.stage_durations.clone() 
            });
        },
        Ok(false) => {
            if cancel_signal.load(std::sync::atomic::Ordering::SeqCst) {
                cb(ProgressUpdate::Failed("Compilation cancelled by user.".to_string()))
            } else {
                cb(ProgressUpdate::Failed("Compilation failed (see output for details).".to_string()))
            }
        },
        Err(e) => cb(ProgressUpdate::Failed(format!("Error reading process output: {}", e))),
    }
}

// RAII guard for cleaning up temp directory
struct TempDirGuard(PathBuf);
impl Drop for TempDirGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn setup_compile_directory(sketch_dir: &Path, sketch_file: &Path, project_root: &Path) -> Result<(PathBuf, Option<TempDirGuard>), String> {
    let sketch_file_name = sketch_file.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let dir_name = sketch_dir.file_name().and_then(|s| s.to_str()).unwrap_or("");

    if sketch_file_name == dir_name {
        Ok((sketch_dir.to_path_buf(), None))
    } else {
        let temp_dir_path = project_root.join(".dev-console").join("temp_compile").join(sketch_file_name);
        
        // Retry logic for Windows file locking
        if temp_dir_path.exists() {
            let mut retries = 5;
            let mut last_err = None;
            
            while retries > 0 {
                match std::fs::remove_dir_all(&temp_dir_path) {
                    Ok(_) => break,
                    Err(e) => {
                        last_err = Some(e);
                        retries -= 1;
                        std::thread::sleep(std::time::Duration::from_millis(200));
                    }
                }
            }
            
            if temp_dir_path.exists() {
                return Err(format!(
                    "Failed to clean up old temp directory after 5 retries: {}. This usually happens if a previous process is still shutting down.", 
                    last_err.map(|e| e.to_string()).unwrap_or_default()
                ));
            }
        }

        std::fs::create_dir_all(&temp_dir_path).map_err(|e| format!("Failed to create temporary compile directory: {}", e))?;

        let temp_sketch_file = temp_dir_path.join(format!("{}.ino", sketch_file_name));
        std::fs::copy(sketch_file, &temp_sketch_file).map_err(|e| format!("Failed to copy sketch to temp directory: {}", e))?;

        if let Ok(entries) = std::fs::read_dir(sketch_dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() && path != sketch_file {
                    // Skip other .ino files - only the main one should be in the temp dir root
                    if path.extension().and_then(|s| s.to_str()) == Some("ino") {
                        continue;
                    }
                    if let Some(file_name) = path.file_name() {
                        let dest = temp_dir_path.join(file_name);
                        let _ = std::fs::copy(&path, &dest);
                    }
                }
            }
        }
        Ok((temp_dir_path.clone(), Some(TempDirGuard(temp_dir_path))))
    }
}