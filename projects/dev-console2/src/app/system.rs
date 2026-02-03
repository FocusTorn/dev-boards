use crate::app::{App, Message, TaskState};
use crate::commands::ProgressUpdate;

/// Event translation and ingestion for background processes.
///>
/// The `system` module acts as the bridge between long-running background tasks
/// (compilation, monitoring) and the main application loop. It translates
/// external `ProgressUpdate` events into internal state transitions, ensuring
/// thread-safe data ingestion and UI synchronization.
///<
impl App {
    /// Ingests pending events from background command channels.
    ///>
    /// This is called once per frame in the main loop to ensure background updates
    /// are translated into internal Messages. This keeps state mutation
    /// single-threaded and predictable.
    ///<
    pub fn poll_system_events(&mut self) {
        while let Ok(update) = self.command_rx.try_recv() {
            // Translate external event to internal message
            self.update(Message::SystemUpdate(update));
        }
    }

    /// Transitions application state based on background task updates.
    ///>
    /// This method manages the lifecycle of 'Running' tasks, including progress
    /// smoothing, stage transitions, and saving performance metrics to history
    /// upon successful completion.
    ///<
    pub fn exec_system_update(&mut self, update: ProgressUpdate) {
        self.should_redraw = true;
        match update {
            ProgressUpdate::OutputLine(line) => {
                if let Some(first_char) = line.chars().next() {
                    let char_len = first_char.len_utf8();
                    match first_char {
                        '⬒' => self.log("system", line[char_len..].trim_start()),
                        '⮻' => self.log("action", line[char_len..].trim_start()),
                        '⇄' => self.log("serial", line[char_len..].trim_start()),
                        '✗' => self.log("error", line[char_len..].trim_start()),
                        '⚠' => self.log("warn", line[char_len..].trim_start()),
                        'ｉ' => self.log("info", &line[char_len..]), // Info: No space/trim
                        _ => {
                            if line.contains('✗') || line.contains("MQTT connection failed") {
                                self.log("serial", &line);
                            } else {
                                self.log("board", &line);
                            }
                        }
                    }
                } else {
                    self.log("board", &line);
                }
            }
            ProgressUpdate::Percentage(p) => {
                let remaining = self.predictor.predict_remaining(p);

                if let TaskState::Running {
                    percentage,
                    smoothed_eta,
                    last_updated,
                    ..
                } = &mut self.task_state
                {
                    *percentage = p;
                    *last_updated = std::time::Instant::now();
                    if let Some(rem) = remaining {
                        *smoothed_eta = Some(rem.as_secs_f64());
                    }
                }
            }
            ProgressUpdate::Stage(s) => {
                let predictor_stage = match s.as_str() {
                    "Initializing" => Some(crate::commands::predictor::CompileStage::Initializing),
                    "DetectingLibraries" => {
                        Some(crate::commands::predictor::CompileStage::DetectingLibraries)
                    }
                    "Compiling" => Some(crate::commands::predictor::CompileStage::Compiling),
                    "Linking" => Some(crate::commands::predictor::CompileStage::Linking),
                    "Generating" => Some(crate::commands::predictor::CompileStage::Generating),
                    _ => None,
                };
                if let Some(stage) = predictor_stage {
                    self.predictor.enter_stage(stage);
                }

                if let TaskState::Running { stage, .. } = &mut self.task_state {
                    *stage = s;
                }
            }
            ProgressUpdate::CompletedWithMetrics { stage_times } => {
                let sketch_id = self
                    .get_current_sketch_id()
                    .unwrap_or_else(|| "default".to_string());
                let history_path = std::path::Path::new(".dev-console/progress_history.json");

                let mut manager = crate::commands::HistoryManager::load(history_path);
                manager.record_run(&sketch_id, stage_times);
                let _ = manager.save(history_path);

                self.task_state = TaskState::Idle;
                self.status_text = "Command completed successfully.".to_string();
                self.log("system", "Command completed successfully (Metrics saved).");
            }
            ProgressUpdate::Failed(e) => {
                self.task_state = TaskState::Idle;
                self.report_error(e);
            }
        }
    }

    /// Advances animations based on elapsed time.
    ///>
    /// This is called on every loop iteration to ensure that visual elements
    /// (like the progress bar) transition smoothly between real data updates.
    ///<
    pub fn tick(&mut self) {
        let now = std::time::Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f64();
        self.last_frame_time = now;

        if let TaskState::Running {
            percentage,
            visual_percentage,
            ..
        } = &mut self.task_state
        {
            let target = *percentage;
            let current = *visual_percentage;

            if (target - current).abs() > 0.01 {
                // Exponential decay: visual = visual + (target - visual) * (1 - exp(-speed * dt))
                let speed = 5.0;
                let factor = 1.0 - (-speed * dt).exp();
                *visual_percentage = current + (target - current) * factor;

                // Snap if very close
                if (target - *visual_percentage).abs() < 0.05 {
                    *visual_percentage = target;
                }
            }
        }
    }

    /// Returns true if any visual elements are still transitioning.
    pub fn is_animating(&self) -> bool {
        if let TaskState::Running {
            percentage,
            visual_percentage,
            ..
        } = &self.task_state
        {
            return (percentage - visual_percentage).abs() > 0.01;
        }
        false
    }
}
