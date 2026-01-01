// Application state management

use crate::settings::Settings;
use crate::field_editor::{FieldEditorState, SettingsFields};
use crate::dashboard::DashboardState;
use crate::process_manager::ProcessManager;
use std::sync::{Arc, Mutex};

/// Application state structure
pub struct AppState {
    pub settings: Settings,
    pub settings_fields: SettingsFields,
    pub field_editor_state: FieldEditorState,
    pub dashboard_state: DashboardState,
    pub dashboard_arc: Arc<Mutex<DashboardState>>,
    pub process_manager: Arc<ProcessManager>,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        let settings = Settings::load();
        let settings_fields = SettingsFields::new();
        let field_editor_state = FieldEditorState::new_selected(0);
        let dashboard_state = DashboardState::new();
        let dashboard_arc = Arc::new(Mutex::new(dashboard_state.clone()));
        let process_manager = Arc::new(ProcessManager::new());
        
        Self {
            settings,
            settings_fields,
            field_editor_state,
            dashboard_state,
            dashboard_arc,
            process_manager,
        }
    }
    
    /// Save settings
    pub fn save_settings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.settings.save()
    }
    
    /// Sync dashboard state to Arc
    pub fn sync_dashboard_state(&mut self) {
        if let Ok(mut state) = self.dashboard_arc.lock() {
            *state = self.dashboard_state.clone();
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
