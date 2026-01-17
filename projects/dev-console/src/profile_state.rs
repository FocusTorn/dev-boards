// Profile State - Manages profile selection and list

use std::sync::{Arc, Mutex};

/// Profile state for managing profile selection
pub struct ProfileState {
    /// List of available profiles
    pub profiles: Arc<Mutex<Vec<String>>>,
    /// Currently selected profile index (None if no profile selected)
    pub selected_index: Arc<Mutex<Option<usize>>>,
    /// Whether we're in profile mode (navigating profiles)
    pub is_active: Arc<Mutex<bool>>,
    /// Name of the currently loaded/active profile
    pub active_profile_name: Arc<Mutex<Option<String>>>,
}

impl ProfileState {
    /// Create a new profile state
    pub fn new() -> Self {
        Self {
            profiles: Arc::new(Mutex::new(Vec::new())),
            selected_index: Arc::new(Mutex::new(None)),
            is_active: Arc::new(Mutex::new(false)),
            active_profile_name: Arc::new(Mutex::new(None)),
        }
    }
    
    /// Refresh the profile list
    pub fn refresh_profiles(&self) -> Result<(), Box<dyn std::error::Error>> {
        use crate::profile_manager::list_profiles;
        let profiles = list_profiles()?;
        let mut profiles_guard = self.profiles.lock().unwrap();
        *profiles_guard = profiles;
        Ok(())
    }
    
    /// Get the selected profile name
    pub fn get_selected_profile(&self) -> Option<String> {
        let profiles = self.profiles.lock().unwrap();
        let selected_index = self.selected_index.lock().unwrap();
        if let Some(idx) = *selected_index {
            if idx < profiles.len() {
                return Some(profiles[idx].clone());
            }
        }
        None
    }
    
    /// Select a profile by index
    #[allow(dead_code)]
    pub fn select_index(&self, index: usize) {
        let profiles = self.profiles.lock().unwrap();
        if index < profiles.len() {
            let mut selected = self.selected_index.lock().unwrap();
            *selected = Some(index);
        }
    }
    
    /// Move selection up
    pub fn move_up(&self) {
        let profiles = self.profiles.lock().unwrap();
        let mut selected = self.selected_index.lock().unwrap();
        if let Some(idx) = *selected {
            if idx > 0 {
                *selected = Some(idx - 1);
            } else {
                *selected = Some(profiles.len().saturating_sub(1));
            }
        } else if !profiles.is_empty() {
            *selected = Some(profiles.len() - 1);
        }
    }
    
    /// Move selection down
    pub fn move_down(&self) {
        let profiles = self.profiles.lock().unwrap();
        let mut selected = self.selected_index.lock().unwrap();
        if let Some(idx) = *selected {
            if idx < profiles.len().saturating_sub(1) {
                *selected = Some(idx + 1);
            } else {
                *selected = Some(0);
            }
        } else if !profiles.is_empty() {
            *selected = Some(0);
        }
    }
    
    /// Clear selection
    pub fn clear_selection(&self) {
        let mut selected = self.selected_index.lock().unwrap();
        *selected = None;
    }
}

impl Default for ProfileState {
    fn default() -> Self {
        Self::new()
    }
}
