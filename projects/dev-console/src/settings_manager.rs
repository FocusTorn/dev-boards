// Settings Manager - Centralized settings management
// Single source of truth for settings loading, saving, and updates

use crate::settings::{Settings, get_settings_path};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

/// Centralized settings manager
/// Provides thread-safe access to settings with automatic persistence
pub struct SettingsManager {
    settings: Arc<Mutex<Settings>>,
    settings_path: PathBuf,
}

impl SettingsManager {
    /// Create a new settings manager by loading from disk
    pub fn load() -> Self {
        let settings = Settings::load();
        let settings_path = get_settings_path();
        Self {
            settings: Arc::new(Mutex::new(settings)),
            settings_path,
        }
    }
    
    /// Get a clone of the current settings
    /// Use this when you need to pass settings to a thread or function
    pub fn get(&self) -> Settings {
        self.settings.lock().unwrap().clone()
    }
    
    /// Get a reference to settings (for read-only access within the same thread)
    /// Returns a guard that must be dropped before calling other methods
    #[allow(dead_code)]
    pub fn get_ref(&self) -> std::sync::MutexGuard<'_, Settings> {
        self.settings.lock().unwrap()
    }
    
    /// Update settings with a closure and automatically save
    /// This ensures settings are always persisted after changes
    pub fn update<F>(&self, f: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut Settings),
    {
        let mut settings = self.settings.lock().unwrap();
        f(&mut settings);
        // Save to disk and ensure it's flushed
        settings.save()?;
        // Verify the update was applied to the in-memory copy
        // (settings is already updated, we just need to ensure save succeeded)
        Ok(())
    }
    
    /// Update settings without saving (for batch updates)
    /// Call save() explicitly after all updates
    #[allow(dead_code)]
    pub fn update_without_save<F>(&self, f: F)
    where
        F: FnOnce(&mut Settings),
    {
        let mut settings = self.settings.lock().unwrap();
        f(&mut settings);
    }
    
    /// Save current settings to disk
    #[allow(dead_code)]
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings = self.settings.lock().unwrap();
        settings.save()?;
        Ok(())
    }
    
    /// Reload settings from disk (useful after external changes)
    pub fn reload(&self) -> Result<(), Box<dyn std::error::Error>> {
        let new_settings = Settings::load();
        let mut settings = self.settings.lock().unwrap();
        *settings = new_settings;
        Ok(())
    }
    
    /// Get the settings path (for debugging/logging)
    #[allow(dead_code)]
    pub fn path(&self) -> &PathBuf {
        &self.settings_path
    }
}

impl Clone for SettingsManager {
    fn clone(&self) -> Self {
        Self {
            settings: Arc::clone(&self.settings),
            settings_path: self.settings_path.clone(),
        }
    }
}
