// Command execution helper - eliminates duplication in command handlers

use crate::dashboard::DashboardState;
use crate::process_manager::ProcessManager;
use crate::settings::Settings;
use crate::commands::{execute_upload_rust, execute_progress_rust, execute_monitor_serial_rust, execute_monitor_mqtt_rust};
use std::sync::{Arc, Mutex};
use std::thread;

/// Execute a command with common setup
/// This eliminates the duplication across all command handlers
pub fn execute_command(
    command: &str,
    dashboard: &Arc<Mutex<DashboardState>>,
    settings: Settings,
    process_manager: Arc<ProcessManager>,
) {
    // Common setup for all commands
    {
        let mut state = dashboard.lock().unwrap();
        state.is_running = true;
        state.progress_percent = 0.0;
        state.set_progress_stage("Initializing");
        state.set_current_file("");
        state.set_status_text(&format!("Running: {}", command));
        state.add_output_line(format!("> {}", command));
    }
    
    // Spawn command-specific thread
    let dashboard_clone = dashboard.clone();
    let process_manager_clone = process_manager.clone();
    
    match command {
        "Compile" => {
            thread::spawn(move || {
                execute_progress_rust(dashboard_clone, settings, process_manager_clone);
            });
        }
        "Upload" => {
            thread::spawn(move || {
                execute_upload_rust(dashboard_clone, settings, process_manager_clone);
            });
        }
        "Monitor-Serial" => {
            thread::spawn(move || {
                execute_monitor_serial_rust(dashboard_clone, settings, process_manager_clone);
            });
        }
        "Monitor-MQTT" => {
            thread::spawn(move || {
                execute_monitor_mqtt_rust(dashboard_clone, settings, process_manager_clone);
            });
        }
        _ => {
            // For other commands, use regular status
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.set_progress_stage("");
            state.set_status_text(&format!("Running: {}", command));
            state.add_output_line(format!("> {}", command));
            state.add_output_line("Command execution not yet implemented".to_string());
        }
    }
}
