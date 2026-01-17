// Application state management

use crate::settings_manager::SettingsManager;
use crate::field_editor::{FieldEditorState, SettingsFields};
use crate::dashboard::DashboardState;
use crate::process_manager::ProcessManager;
use crate::profile_state::ProfileState;
use std::sync::{Arc, Mutex};

/// Application state structure
pub struct AppState {
    pub settings: SettingsManager,
    pub settings_fields: SettingsFields,
    pub field_editor_state: FieldEditorState,
    pub profile_state: ProfileState,
    pub dashboard: Arc<Mutex<DashboardState>>,
    pub process_manager: Arc<ProcessManager>,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        let settings = SettingsManager::load();
        let settings_fields = SettingsFields::new();
        let field_editor_state = FieldEditorState::new_selected(0);
        let profile_state = ProfileState::new();
        // Refresh profiles on startup
        let _ = profile_state.refresh_profiles();
        let dashboard_state = DashboardState::new();
        let dashboard = Arc::new(Mutex::new(dashboard_state));
        let process_manager = Arc::new(ProcessManager::new());
        
        Self {
            settings,
            settings_fields,
            field_editor_state,
            profile_state,
            dashboard,
            process_manager,
        }
    }
    
    /// Start a command execution (common setup for all commands)
    #[allow(dead_code)]
    pub fn start_command(&self, command: &str) {
        let mut state = self.dashboard.lock().unwrap();
        state.is_running = true;
        state.progress_percent = 0.0;
        state.set_progress_stage("Initializing");
        state.set_current_file("");
        state.set_status_text(&format!("Running: {}", command));
        state.add_output_line(format!("> {}", command));
    }
    
    /// Cancel running command
    #[allow(dead_code)]
    pub fn cancel_command(&self) {
        self.process_manager.kill_all();
        let mut state = self.dashboard.lock().unwrap();
        state.is_running = false;
        state.set_status_text("Command cancelled");
        state.add_output_line("Command cancelled by user".to_string());
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
