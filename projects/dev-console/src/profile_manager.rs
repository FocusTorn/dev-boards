// Profile Manager - Handles saving and loading settings profiles

use crate::settings::Settings;
use std::fs;
use std::path::PathBuf;

/// Get the profiles directory path
pub fn get_profiles_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("dev-console")
        .join("profiles")
}

/// Get the path for a specific profile
pub fn get_profile_path(profile_name: &str) -> PathBuf {
    get_profiles_dir().join(format!("{}.yaml", profile_name))
}

/// List all available profiles
pub fn list_profiles() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let profiles_dir = get_profiles_dir();
    
    // Create directory if it doesn't exist
    if !profiles_dir.exists() {
        fs::create_dir_all(&profiles_dir)?;
        return Ok(Vec::new());
    }
    
    let mut profiles = Vec::new();
    
    // Read all .yaml files in the profiles directory
    if let Ok(entries) = fs::read_dir(&profiles_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "yaml" || extension == "yml" {
                            if let Some(stem) = path.file_stem() {
                                if let Some(name) = stem.to_str() {
                                    profiles.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Sort profiles alphabetically
    profiles.sort();
    
    Ok(profiles)
}

/// Save a profile with the given name
pub fn save_profile(profile_name: &str, settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    let profile_path = get_profile_path(profile_name);
    
    // Create profiles directory if it doesn't exist
    if let Some(parent) = profile_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Serialize settings to YAML
    let contents = serde_yaml::to_string(settings)?;
    fs::write(&profile_path, contents)?;
    
    // Ensure file is flushed to disk
    use std::io::Write;
    if let Ok(mut file) = std::fs::OpenOptions::new().write(true).open(&profile_path) {
        let _ = file.flush();
    }
    
    Ok(())
}

/// Load a profile by name
pub fn load_profile(profile_name: &str) -> Result<Settings, Box<dyn std::error::Error>> {
    let profile_path = get_profile_path(profile_name);
    
    if !profile_path.exists() {
        return Err(format!("Profile '{}' not found", profile_name).into());
    }
    
    let contents = fs::read_to_string(&profile_path)?;
    let settings: Settings = serde_yaml::from_str(&contents)?;
    
    Ok(settings)
}

/// Delete a profile
#[allow(dead_code)]
pub fn delete_profile(profile_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let profile_path = get_profile_path(profile_name);
    
    if profile_path.exists() {
        fs::remove_file(&profile_path)?;
    }
    
    Ok(())
}

/// Check if a profile exists
#[allow(dead_code)]
pub fn profile_exists(profile_name: &str) -> bool {
    get_profile_path(profile_name).exists()
}
