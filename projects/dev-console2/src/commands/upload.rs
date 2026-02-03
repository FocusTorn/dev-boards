use super::{
    compile_state, path_utils,
    process::ProcessHandler,
    traits::{CommandRunner, FileSystem, RealCommandRunner, RealFileSystem},
};
use crate::commands::compile::{ProgressUpdate, Settings};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

/// Spawns a background thread to upload firmware to hardware via `arduino-cli`.
pub fn run_upload(
    settings: &Settings,
    stats: crate::commands::history::StageStats,
    cancel_signal: Arc<AtomicBool>,
    progress_callback: impl FnMut(ProgressUpdate) + Send + 'static,
) {
    run_upload_with_runner(
        &RealCommandRunner,
        &RealFileSystem,
        settings,
        stats,
        cancel_signal,
        progress_callback,
    )
}

/// Spawns a background thread to upload firmware to hardware via `arduino-cli` with a provided runner.
///>
/// Handles the transition into specific upload stages (Resetting, Uploading,
/// Verifying) and parses `esptool` percentage output to provide high-fidelity
/// progress updates to the TUI.
///<
pub fn run_upload_with_runner(
    runner: &dyn CommandRunner,
    fs: &dyn FileSystem,
    settings: &Settings,
    stats: crate::commands::history::StageStats,
    cancel_signal: Arc<AtomicBool>,
    progress_callback: impl FnMut(ProgressUpdate) + Send + 'static,
) {
    let callback = Arc::new(Mutex::new(progress_callback));

    callback.lock().unwrap()(ProgressUpdate::Stage("Initializing".to_string()));

    let sketch_dir = PathBuf::from(&settings.sketch_directory);

    let project_root = match path_utils::find_workspace_root() {
        Ok(root) => root,
        Err(e) => {
            callback.lock().unwrap()(ProgressUpdate::Failed(format!(
                "Failed to determine workspace root: {}",
                e
            )));
            return;
        }
    };

    let arduino_cli = path_utils::find_arduino_cli(fs, &settings.env, &project_root);

    callback.lock().unwrap()(ProgressUpdate::OutputLine(format!(
        "Using arduino-cli for upload: {:?}",
        arduino_cli
    )));

    let mut cmd = Command::new(&arduino_cli);
    cmd.arg("upload")
        .arg("-p")
        .arg(&settings.port)
        .arg("--fqbn")
        .arg(&settings.fqbn)
        .arg("--input-dir")
        .arg(sketch_dir.join("build"))
        .arg("--verbose");

    let process_handler = match ProcessHandler::spawn(runner, cmd) {
        Ok(handler) => handler,
        Err(e) => {
            callback.lock().unwrap()(ProgressUpdate::Failed(format!(
                "Failed to spawn arduino-cli: {}",
                e
            )));
            return;
        }
    };

    let upload_state = Arc::new(Mutex::new(compile_state::CompileState::new(
        stats.weights,
        stats.averages,
    )));

    let callback_clone = callback.clone();
    let state_clone = upload_state.clone();

    let result = process_handler.read_output(cancel_signal.clone(), move |line| {
        let cleaned = crate::commands::utils::remove_ansi_escapes(&line);
        let line_lower = cleaned.to_lowercase();

        let mut cb = callback_clone.lock().unwrap();
        let mut state = state_clone.lock().unwrap();

        let mut next_stage = None;

        // Custom Upload Stage Detection
        if line_lower.contains("resetting") || line_lower.contains("hard resetting") {
            next_stage = Some(crate::commands::predictor::CompileStage::Resetting);
        } else if line_lower.contains("writing at") || line_lower.contains("uploading") {
            next_stage = Some(crate::commands::predictor::CompileStage::Uploading);
        } else if line_lower.contains("verifying") {
            next_stage = Some(crate::commands::predictor::CompileStage::Verifying);
        } else if line_lower.contains("leaving...")
            || line_lower.contains("hard resetting via rts pin")
        {
            next_stage = Some(crate::commands::predictor::CompileStage::Complete);
        }

        if let Some(stage) = next_stage {
            if stage.rank() > state.stage.rank() {
                state.transition_to(stage);
                cb(ProgressUpdate::Stage(format!("{:?}", state.stage)));
            }
        }

        // Parse esptool percentage if available
        if state.stage == crate::commands::predictor::CompileStage::Uploading
            || state.stage == crate::commands::predictor::CompileStage::Verifying
        {
            // Match the specific esptool format: "... (10 %)"
            let re_progress = regex::Regex::new(r"\((\d+)\s*%\)").unwrap();
            if let Some(cap) = re_progress.captures(&cleaned) {
                if let Ok(p) = cap[1].parse::<f64>() {
                    // Only update if it's an increase, or if we just started a new segment (low percentage after high)
                    // Actually, let's just pass it through and let calculate_progress handle the monotonicity
                    state.update_stage_progress(p);
                }
            }
        }

        let progress = state.calculate_progress();
        cb(ProgressUpdate::Percentage(progress));
        cb(ProgressUpdate::OutputLine(line));
    });

    let mut cb = callback.lock().unwrap();
    match result {
        Ok(true) => {
            let mut state = upload_state.lock().unwrap();
            let duration = state.last_marker_time.elapsed().as_secs_f64();
            let stage = state.stage;
            state.stage_durations.insert(stage, duration);

            cb(ProgressUpdate::CompletedWithMetrics {
                stage_times: state.stage_durations.clone(),
            });
        }
        Ok(false) => {
            if cancel_signal.load(std::sync::atomic::Ordering::SeqCst) {
                cb(ProgressUpdate::Failed(
                    "Upload cancelled by user.".to_string(),
                ))
            } else {
                cb(ProgressUpdate::Failed(
                    "Upload failed (see output for details).".to_string(),
                ))
            }
        }
        Err(e) => cb(ProgressUpdate::Failed(format!(
            "Error reading process output: {}",
            e
        ))),
    }
}
