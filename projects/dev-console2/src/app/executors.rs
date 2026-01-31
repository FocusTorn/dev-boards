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
    pub fn exec_quit(&mut self) {
        self.running = false;
    }

    /// Moves the command selection highlight upwards.
    pub fn exec_commands_up(&mut self) {
        self.selected_command_index = if self.selected_command_index > 0 {
            self.selected_command_index - 1
        } else {
            self.commands.len() - 1
        };
        self.hovered_command_index = None;
        
        if self.dispatch_mode == crate::app::DispatchMode::OnHighlight {
            self.exec_execute_selected_command();
        }
    }

    /// Moves the command selection highlight downwards.
    pub fn exec_commands_down(&mut self) {
        self.selected_command_index = (self.selected_command_index + 1) % self.commands.len();
        self.hovered_command_index = None;

        if self.dispatch_mode == crate::app::DispatchMode::OnHighlight {
            self.exec_execute_selected_command();
        }
    }

    pub fn exec_settings_up(&mut self) {
        self.selected_settings_category_index = if self.selected_settings_category_index > 0 {
            self.selected_settings_category_index - 1
        } else {
            self.settings_categories.len().saturating_sub(1)
        };
        
        if self.dispatch_mode == crate::app::DispatchMode::OnHighlight {
            // Future: Trigger content switch
        }
    }

    pub fn exec_settings_down(&mut self) {
        if !self.settings_categories.is_empty() {
            self.selected_settings_category_index = (self.selected_settings_category_index + 1) % self.settings_categories.len();
        }

        if self.dispatch_mode == crate::app::DispatchMode::OnHighlight {
            // Future: Trigger content switch
        }
    }

    /// Executes the currently highlighted command from the sidebar.
    pub fn exec_execute_selected_command(&mut self) {
        let selected_str = self.commands[self.selected_command_index].clone();
        if let Some(action) = Action::from_str(&selected_str) {
            self.dispatch_command(action);
        } else {
            self.log("action", &format!("No executor mapped for '{}'", selected_str));
        }
    }

    /// Attempts to cancel any active background tasks or exit input mode.
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
    pub fn exec_compile(&mut self) {
        self.start_process(false);
    }

    /// Initiates the firmware upload process.
    pub fn exec_upload(&mut self) {
        self.start_process(true);
    }

    /// Core logic for spawning background build/upload threads.
    ///>
    /// This method resets progress, trains the time predictor with latest history,
    /// and dispatches the long-running task to a background thread.
    ///< 
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
        self.output_cached_lines.clear();
        let tx = self.command_tx.clone();
        let cancel_signal = self.cancel_signal.clone();
        cancel_signal.store(false, Ordering::SeqCst);
        
        self.predictor = self.train_predictor();
        let stats = self.predictor.get_stats();
        
        match self.get_settings_from_profile() {
            Ok(settings) => {
                std::thread::spawn(move || {
                    let callback = move |update| {
                        if tx.send(update).is_err() { return; }
                    };
                    if is_upload {
                        crate::commands::run_upload(&settings, stats, cancel_signal, callback);
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

    /// Opens a serial port and begins monitoring hardware output.
    pub fn exec_monitor_serial(&mut self) {
        let now = Instant::now();
        self.task_state = TaskState::Monitoring {
            monitor_type: MonitorType::Serial,
            start_time: now,
        };
        self.output_lines.clear();
        self.output_cached_lines.clear();
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

    /// Connects to the MQTT broker defined in the current profile.
    pub fn exec_monitor_mqtt(&mut self) {
        let now = Instant::now();
        self.task_state = TaskState::Monitoring {
            monitor_type: MonitorType::Mqtt,
            start_time: now,
        };
        self.output_lines.clear();
        self.output_cached_lines.clear();
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
            // Recalculate layout for the new tab
            self.layout = self.calculate_layout(self.view_area);
        }
    }

    pub fn exec_prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            let current = self.tabs.iter().position(|t| t.active).unwrap_or(0);
            let prev = if current > 0 { current - 1 } else { self.tabs.len() - 1 };
            for (i, tab) in self.tabs.iter_mut().enumerate() {
                tab.active = i == prev;
            }
            // Recalculate layout for the new tab
            self.layout = self.calculate_layout(self.view_area);
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

    pub fn exec_toggle_focus(&mut self) {
        self.focus = match self.focus {
            crate::app::Focus::Sidebar => crate::app::Focus::Content,
            crate::app::Focus::Content => crate::app::Focus::Sidebar,
        };
    }

    pub fn exec_profile_new(&mut self) {
        if let Some(config) = &mut self.profile_config {
            let new_id = "new_profile".to_string();
            // Ensure unique ID
            let mut final_id = new_id.clone();
            let mut count = 1;
            while config.sketches.iter().any(|s| s.id == final_id) {
                final_id = format!("{}_{}", new_id, count);
                count += 1;
            }

            let new_sketch = crate::config::Sketch {
                id: final_id.clone(),
                path: "".to_string(),
                connection: config.connections.first().map(|c| c.id.clone()).unwrap_or_default(),
                device: config.devices.first().map(|d| d.id.clone()).unwrap_or_default(),
                mqtt: config.mqtt.first().map(|m| m.id.clone()).unwrap_or_default(),
            };

            config.sketches.push(new_sketch);
            self.profile_ids.push(final_id);
            self.selected_profile_index = self.profile_ids.len() - 1;
            self.log("system", &format!("Created new profile: {}", self.profile_ids[self.selected_profile_index]));
        }
    }

    pub fn exec_profile_clone(&mut self) {
        if let Some(config) = &mut self.profile_config {
            if let Some(current_id) = self.profile_ids.get(self.selected_profile_index) {
                if let Some(current_sketch) = config.sketches.iter().find(|s| s.id == *current_id).cloned() {
                    let mut new_sketch = current_sketch;
                    new_sketch.id = format!("{}_copy", current_id);
                    
                    // Ensure unique
                    let mut final_id = new_sketch.id.clone();
                    let mut count = 1;
                    while config.sketches.iter().any(|s| s.id == final_id) {
                        final_id = format!("{}_copy_{}", current_id, count);
                        count += 1;
                    }
                    new_sketch.id = final_id.clone();

                    config.sketches.push(new_sketch);
                    self.profile_ids.push(final_id);
                    self.selected_profile_index = self.profile_ids.len() - 1;
                    self.log("system", &format!("Cloned profile to: {}", self.profile_ids[self.selected_profile_index]));
                }
            }
        }
    }

    pub fn exec_profile_delete(&mut self) {
        if let Some(config) = &mut self.profile_config {
            if !self.profile_ids.is_empty() {
                let id_to_remove = self.profile_ids.remove(self.selected_profile_index);
                config.sketches.retain(|s| s.id != id_to_remove);
                
                if self.selected_profile_index >= self.profile_ids.len() && !self.profile_ids.is_empty() {
                    self.selected_profile_index = self.profile_ids.len() - 1;
                }
                self.log("system", &format!("Deleted profile: {}", id_to_remove));
            }
        }
    }

    pub fn exec_profile_save(&mut self) {
        if let Some(config) = &self.profile_config {
            match crate::config::save_profile_config_to_path(config, &self.profile_config_path) {
                Ok(_) => {
                    self.log("system", &format!("Configuration saved to {}", self.profile_config_path));
                    self.toast_manager.success("Settings Saved");
                },
                Err(e) => {
                    self.report_error(format!("Failed to save config: {}", e));
                }
            }
        }
    }
}
