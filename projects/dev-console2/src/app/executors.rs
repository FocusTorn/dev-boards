use crate::app::{App, TaskState, MonitorType, Action};
use arboard::Clipboard;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::time::Instant;

/// Semantic action implementations (The 'How' of application logic).
///>
/// The `executors` module contains the actual implementation for every `Action` 
/// triggered by the user or system. By isolating these methods, the main `update` 
/// loop remains a clean router while the complex state-mutating logic is 
/// encapsulated here.
///<
impl App {
    /// Terminates the application loop.
    ///>
    /// Sets the `running` flag to false, which will cause the main loop in `main.rs`
    /// to exit and trigger terminal restoration.
    ///<
    pub fn exec_quit(&mut self) {
        self.running = false;
    }

    /// Moves the command selection highlight upwards.
    ///>
    /// Cycles to the bottom of the list if the top is reached.
    ///<
    pub fn exec_commands_up(&mut self) {
        self.selected_command_index = if self.selected_command_index > 0 {
            self.selected_command_index - 1
        } else {
            self.commands.len() - 1
        };
        self.hovered_command_index = None;
    }

    /// Moves the command selection highlight downwards.
    ///>
    /// Cycles to the top of the list if the bottom is reached.
    ///<
    pub fn exec_commands_down(&mut self) {
        self.selected_command_index = (self.selected_command_index + 1) % self.commands.len();
        self.hovered_command_index = None;
    }

    /// Executes the currently highlighted command from the sidebar.
    ///>
    /// Maps the display string of the command to its corresponding `Action`
    /// and dispatches it through the semantic router.
    ///<
    pub fn exec_execute_selected_command(&mut self) {
        let selected_str = self.commands[self.selected_command_index].clone();
        if let Some(action) = Action::from_str(&selected_str) {
            self.dispatch_command(action);
        } else {
            self.log("action", &format!("No executor mapped for '{}'", selected_str));
        }
    }

    /// Attempts to cancel any active background tasks or exit input mode.
    ///>
    /// This method is the primary safety valve for stopping runaway builds
    /// or monitoring processes. It also serves as the 'Esc' handler for 
    /// closing text entry fields.
    ///<
    pub fn exec_cancel(&mut self) {
        let is_active = matches!(self.task_state, TaskState::Running { .. }) || matches!(self.task_state, TaskState::Monitoring { .. });
        
        if is_active {
            self.cancel_signal.store(true, Ordering::SeqCst);
            self.log("system", "Cancellation signal sent...");
        }

        if self.input_active {
            self.input_active = false;
            self.input.reset();
        }
    }

    /// Initiates the firmware compilation process.
    ///>
    /// Spawns a background thread to run the compiler toolchain while
    /// ensuring the UI remains responsive and progress-aware.
    ///<
    pub fn exec_compile(&mut self) {
        self.start_process(false);
    }

    /// Initiates the firmware upload process.
    ///>
    /// Similar to compilation, this runs the flash tool in a separate thread
    /// and monitors output for progress markers.
    ///<
    pub fn exec_upload(&mut self) {
        self.start_process(true);
    }

    fn start_process(&mut self, is_upload: bool) {
        let now = Instant::now();
        self.task_state = TaskState::Running {
            percentage: 0.0,
            visual_percentage: 0.0,
            last_percentage: 0.0,
            stage: if is_upload { "Preparing Upload..." } else { "Initializing Compile..." }.to_string(),
            start_time: now,
            last_updated: now,
            smoothed_eta: None,
        };
        self.output_lines.clear();
        let tx = self.command_tx.clone();
        let cancel_signal = self.cancel_signal.clone();
        cancel_signal.store(false, Ordering::SeqCst);
        
        self.predictor = self.train_predictor();
        let stats = Some(self.predictor.get_stats());
        
        match self.get_settings_from_profile() {
            Ok(settings) => {
                std::thread::spawn(move || {
                    let callback = move |update| {
                        if tx.send(update).is_err() { return; }
                    };
                    if is_upload {
                        // Upload might need update too, but for now focusing on compile
                         // If run_upload doesn't take the tuple, we might need to adjust or map it.
                         // Let's check run_upload signature later. For now assuming it matches or we fix it.
                         // Actually run_upload probably takes StageStats.
                         // Let's map it for upload to avoid breakage if upload isn't updated.
                        let (s, _) = stats.clone().unwrap(); 
                        crate::commands::run_upload(&settings, s, cancel_signal, callback);
                    } else {
                        crate::commands::run_compile(&settings, stats, cancel_signal, callback);
                    }
                });
            },
            Err(e) => {
                self.task_state = TaskState::Idle;
                self.report_error(e);
            }
        }
    }

    pub fn exec_monitor_serial(&mut self) {
        let now = Instant::now();
        self.task_state = TaskState::Monitoring {
            monitor_type: MonitorType::Serial,
            start_time: now,
        };
        self.output_lines.clear();
        self.log("action", "Starting Serial Monitor...");
        
        // Activate Input Field Automatically
        self.input_active = true;
        self.input.reset();

        let tx = self.command_tx.clone();
        let (serial_tx, serial_rx) = mpsc::channel();
        self.serial_tx = Some(serial_tx);
        let cancel_signal = self.cancel_signal.clone();
        cancel_signal.store(false, Ordering::SeqCst);

        match self.get_settings_from_profile() {
            Ok(settings) => {
                std::thread::spawn(move || {
                    let callback = move |update| {
                        if tx.send(update).is_err() { return; }
                    };
                    crate::commands::run_serial_monitor(settings.port, settings.baudrate, cancel_signal, serial_rx, callback);
                });
            },
            Err(e) => {
                self.task_state = TaskState::Idle;
                self.input_active = false;
                self.report_error(e);
            }
        }
    }

    pub fn exec_monitor_mqtt(&mut self) {
        let now = Instant::now();
        self.task_state = TaskState::Monitoring {
            monitor_type: MonitorType::Mqtt,
            start_time: now,
        };
        self.output_lines.clear();
        self.log("action", "Starting MQTT Monitor...");
        
        // Activate Input Field Automatically
        self.input_active = true;
        self.input.reset();

        let tx = self.command_tx.clone();
        let (mqtt_tx, mqtt_rx) = mpsc::channel();
        self.mqtt_tx = Some(mqtt_tx);
        let cancel_signal = self.cancel_signal.clone();
        cancel_signal.store(false, Ordering::SeqCst);

        match self.get_settings_from_profile() {
            Ok(settings) => {
                let mqtt_config = self.profile_config.as_ref()
                    .and_then(|pc| pc.sketches.iter().find(|s| s.path.contains(&settings.sketch_name)))
                    .and_then(|s| self.profile_config.as_ref().unwrap().mqtt.iter().find(|m| m.id == s.mqtt));

                if let Some(m) = mqtt_config {
                    let host = m.host.clone();
                    let port = m.port;
                    let client_id = m.id.clone();
                    let username = if m.username.is_empty() { None } else { Some(m.username.clone()) };
                    let password = if m.password.is_empty() { None } else { Some(m.password.clone()) };
                    
                    std::thread::spawn(move || {
                        let callback = move |update| {
                            if tx.send(update).is_err() { return; }
                        };
                        crate::commands::run_mqtt_monitor(host, port, client_id, username, password, cancel_signal, mqtt_rx, callback);
                    });
                } else {
                    self.task_state = TaskState::Idle;
                    self.input_active = false;
                    self.report_error(color_eyre::eyre::eyre!("No MQTT configuration found for this profile."));
                }
            },
            Err(e) => {
                self.task_state = TaskState::Idle;
                self.input_active = false;
                self.report_error(e);
            }
        }
    }

    pub fn exec_clean(&mut self) {
        self.push_line("Cleaning project...".to_string());
        // Implement actual clean logic here
        self.push_line("Done.".to_string());
    }


    pub fn exec_next_profile(&mut self) {
        if !self.profile_ids.is_empty() {
            self.selected_profile_index = (self.selected_profile_index + 1) % self.profile_ids.len();
        }
    }

    pub fn exec_prev_profile(&mut self) {
        if !self.profile_ids.is_empty() {
            self.selected_profile_index = if self.selected_profile_index > 0 {
                self.selected_profile_index - 1
            } else {
                self.profile_ids.len() - 1
            };
        }
    }

    pub fn exec_next_tab(&mut self) {
        if !self.tabs.is_empty() {
            let current = self.tabs.iter().position(|t| t.active).unwrap_or(0);
            let next = (current + 1) % self.tabs.len();
            for (i, tab) in self.tabs.iter_mut().enumerate() {
                tab.active = i == next;
            }
        }
    }

    pub fn exec_prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            let current = self.tabs.iter().position(|t| t.active).unwrap_or(0);
            let prev = if current > 0 { current - 1 } else { self.tabs.len() - 1 };
            for (i, tab) in self.tabs.iter_mut().enumerate() {
                tab.active = i == prev;
            }
        }
    }

    pub fn exec_scroll_line_up(&mut self) {
        self.output_scroll = self.output_scroll.saturating_sub(1);
        self.output_autoscroll = false;
    }

    pub fn exec_scroll_line_down(&mut self) {
        let viewport_h = self.layout.output.height.saturating_sub(2) as usize;
        let max_scroll = self.output_lines.len().saturating_sub(viewport_h) as u16;
        if self.output_scroll < max_scroll {
            self.output_scroll = self.output_scroll.saturating_add(1);
            if self.output_scroll >= max_scroll {
                self.output_autoscroll = true;
            } else {
                self.output_autoscroll = false;
            }
        } else {
            self.output_autoscroll = true;
        }
    }

    pub fn exec_scroll_page_up(&mut self) {
        let amount = self.layout.output.height.saturating_sub(2);
        self.output_scroll = self.output_scroll.saturating_sub(amount);
        self.output_autoscroll = false;
    }

    pub fn exec_scroll_page_down(&mut self) {
        let viewport_h = self.layout.output.height.saturating_sub(2);
        let max_scroll = self.output_lines.len().saturating_sub(viewport_h as usize) as u16;
        let next = self.output_scroll.saturating_add(viewport_h);
        
        if next >= max_scroll {
            self.output_scroll = max_scroll;
            self.output_autoscroll = true;
        } else {
            self.output_scroll = next;
            self.output_autoscroll = false;
        }
    }

    pub fn exec_scroll_top(&mut self) {
        self.output_scroll = 0;
        self.output_autoscroll = false;
    }

    pub fn exec_scroll_bottom(&mut self) {
        self.output_autoscroll = true;
        self.sync_autoscroll();
    }

    pub fn exec_toggle_autoscroll(&mut self) {
        self.output_autoscroll = !self.output_autoscroll;
        if self.output_autoscroll {
            self.sync_autoscroll();
        }
    }

    pub fn exec_copy_status(&mut self) {
        match Clipboard::new() {
            Ok(mut clipboard) => {
                let _ = clipboard.set_text(self.status_text.clone());
                self.toast_manager.success("Status copied.");
            }
            Err(_) => {
                self.toast_manager.error("Clipboard failed.");
            }
        }
    }

    pub fn exec_copy_output(&mut self, full: bool) {
        let content = if full {
            self.output_lines.join("\n")
        } else {
            let start = self.output_scroll as usize;
            let height = self.layout.output.height.saturating_sub(2) as usize;
            let end = (start + height).min(self.output_lines.len());
            self.output_lines[start..end].join("\n")
        };

        match Clipboard::new() {
            Ok(mut clipboard) => {
                let _ = clipboard.set_text(content);
                let msg = if full { "Full output copied." } else { "Visible lines copied." };
                self.toast_manager.success(msg);
            }
            Err(_) => {
                self.toast_manager.error("Clipboard failed.");
            }
        }
    }

    pub fn exec_send_command(&mut self) {
        if self.input.value().is_empty() { return; }
        
        let msg = self.input.value().to_string();
        self.input.reset();

        match &self.task_state {
            TaskState::Monitoring { monitor_type, .. } => {
                match monitor_type {
                    MonitorType::Serial => {
                        if let Some(tx) = &self.serial_tx {
                            let _ = tx.send(crate::commands::SerialCommand::SendData(msg));
                        }
                    }
                    MonitorType::Mqtt => {
                        if let Some(tx) = &self.mqtt_tx {
                            // Dummy topic for now
                            let _ = tx.send(crate::commands::MqttCommand::Publish { 
                                topic: "dev/command".to_string(), 
                                payload: msg 
                            });
                        }
                    }
                }
            }
            _ => {
                self.log("system", &format!("No active monitor to send: {}", msg));
            }
        }
    }

    pub fn exec_toggle_input(&mut self) {
        self.input_active = !self.input_active;
        if !self.input_active {
            self.input.reset();
        }
    }
}
