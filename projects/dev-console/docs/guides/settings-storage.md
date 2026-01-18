# Settings Storage Guide

This guide explains how value storage and usage works within the settings area of the dev-console application. The settings system provides a robust, thread-safe mechanism for managing application configuration with automatic persistence.

## Overview

The settings system consists of several key components:

1. **Settings Struct** - Core data structure for configuration values
2. **SettingsManager** - Thread-safe manager with automatic persistence
3. **Profile System** - Save and load different configuration sets
4. **Field Editor** - Interactive editing of settings values

## Settings Structure

### Core Settings Schema

The settings are defined in `src/settings.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Settings {
    // Project settings
    pub sketch_directory: String,
    pub sketch_name: String,
    pub env: String,
    
    // Hardware settings
    pub board_model: String,
    pub fqbn: String,
    pub port: String,
    pub baudrate: u32,
    
    // General settings
    pub create_log: bool,
    
    // MQTT settings (optional)
    #[serde(default)]
    pub mqtt_host: Option<String>,
    #[serde(default)]
    pub mqtt_port: Option<u16>,
    #[serde(default)]
    pub mqtt_username: Option<String>,
    #[serde(default)]
    pub mqtt_password: Option<String>,
    #[serde(default)]
    pub mqtt_topic_command: Option<String>,
    #[serde(default)]
    pub mqtt_topic_state: Option<String>,
    #[serde(default)]
    pub mqtt_topic_status: Option<String>,
}
```

### Default Values

Default values are provided for all settings:

```rust
impl Default for Settings {
    fn default() -> Self {
        Self {
            sketch_directory: "".to_string(),
            sketch_name: "".to_string(),
            env: "arduino".to_string(),
            board_model: "esp32-s3".to_string(),
            fqbn: "esp32:esp32:esp32s3".to_string(),
            port: "COM9".to_string(),
            baudrate: 115200,
            create_log: false,
            mqtt_host: None,
            mqtt_port: None,
            mqtt_username: Some("mqtt".to_string()),
            mqtt_password: Some("mqtt".to_string()),
            mqtt_topic_command: Some("controller/esp32-s3-led/command".to_string()),
            mqtt_topic_state: Some("controller/esp32-s3-led/state".to_string()),
            mqtt_topic_status: Some("controller/esp32-s3-led/status".to_string()),
        }
    }
}
```

## SettingsManager

The `SettingsManager` provides thread-safe access to settings with automatic persistence:

### Key Features

```rust
pub struct SettingsManager {
    settings: Arc<Mutex<Settings>>,
    settings_path: PathBuf,
}
```

### Core Operations

#### Loading Settings

```rust
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
}
```

#### Reading Settings

```rust
/// Get a clone of the current settings
/// Use this when you need to pass settings to a thread or function
pub fn get(&self) -> Settings {
    self.settings.lock().unwrap().clone()
}

/// Get a reference to settings (for read-only access within the same thread)
/// Returns a guard that must be dropped before calling other methods
pub fn get_ref(&self) -> std::sync::MutexGuard<'_, Settings> {
    self.settings.lock().unwrap()
}
```

#### Updating Settings

```rust
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
    Ok(())
}

/// Update settings without saving (for batch updates)
/// Call save() explicitly after all updates
pub fn update_without_save<F>(&self, f: F)
where
    F: FnOnce(&mut Settings),
{
    let mut settings = self.settings.lock().unwrap();
    f(&mut settings);
}
```

## File Storage

### Storage Location

Settings are stored in the user's config directory:

```rust
pub fn get_settings_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("dev-console")
        .join("settings.yaml")
}
```

### File Format

Settings are stored in YAML format:

```yaml
sketch_directory: "/path/to/sketch"
sketch_name: "my-esp32-project"
env: "arduino"
board_model: "esp32-s3"
fqbn: "esp32:esp32:esp32s3"
port: "COM9"
baudrate: 115200
create_log: false
mqtt_host: "localhost"
mqtt_port: 1883
mqtt_username: "mqtt"
mqtt_password: "mqtt"
mqtt_topic_command: "controller/esp32-s3-led/command"
mqtt_topic_state: "controller/esp32-s3-led/state"
mqtt_topic_status: "controller/esp32-s3-led/status"
```

### Persistence Logic

```rust
impl Settings {
    pub fn load() -> Self {
        let path = get_settings_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(settings) = serde_yaml::from_str::<Settings>(&contents) {
                return settings;
            }
        }
        Self::default()
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = get_settings_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = serde_yaml::to_string(self)?;
        fs::write(&path, contents)?;
        // Ensure file is flushed to disk
        use std::io::Write;
        if let Ok(mut file) = std::fs::OpenOptions::new().write(true).open(&path) {
            let _ = file.flush();
        }
        Ok(())
    }
}
```

## Profile System

The profile system allows users to save and load different configuration sets:

### Profile Storage

Profiles are stored as separate YAML files:

```rust
// Profile path: ~/.config/dev-console/profiles/<profile_name>.yaml
pub fn get_profile_path(profile_name: &str) -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("dev-console")
        .join("profiles")
        .join(format!("{}.yaml", profile_name))
}
```

### Profile Operations

```rust
// Save current settings as a profile
pub fn save_profile(name: &str, settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_profile_path(name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let contents = serde_yaml::to_string(settings)?;
    fs::write(&path, contents)?;
    Ok(())
}

// Load a profile
pub fn load_profile(name: &str) -> Result<Settings, Box<dyn std::error::Error>> {
    let path = get_profile_path(name);
    let contents = fs::read_to_string(&path)?;
    let settings = serde_yaml::from_str::<Settings>(&contents)?;
    Ok(settings)
}

// List available profiles
pub fn list_profiles() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let profiles_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("dev-console")
        .join("profiles");
    
    if !profiles_dir.exists() {
        return Ok(vec![]);
    }
    
    let mut profiles = Vec::new();
    for entry in fs::read_dir(profiles_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                profiles.push(name.to_string());
            }
        }
    }
    profiles.sort();
    Ok(profiles)
}
```

## Field Editor Integration

The field editor provides interactive editing of settings:

### Field Types

Different field types are handled differently:

```rust
pub enum FieldType {
    Text,           // String fields
    Number,         // Numeric fields
    Boolean,        // Toggle fields
    Dropdown(Vec<String>), // Selection from options
    Optional,       // Optional fields (can be None)
}
```

### Field Configuration

Fields are configured with metadata:

```rust
pub struct FieldConfig {
    pub hwnd: &'static str,
    pub label: &'static str,
    pub field_type: FieldType,
    pub required: bool,
    pub validation: Option<ValidationRule>,
}
```

### Value Updates

When users edit fields, the values are updated through the SettingsManager:

```rust
// Example: Updating sketch directory
settings_manager.update(|settings| {
    settings.sketch_directory = new_value;
})?;

// Example: Updating optional MQTT host
settings_manager.update(|settings| {
    if new_value.is_empty() {
        settings.mqtt_host = None;
    } else {
        settings.mqtt_host = Some(new_value);
    }
})?;
```

## Usage in Application

### Initialization

```rust
// In main.rs
let app_state = AppState::new();
let settings_manager = SettingsManager::load();
```

### Accessing Settings

```rust
// Getting current settings
let current_settings = app_state.settings.get();

// Updating settings
app_state.settings.update(|settings| {
    settings.port = new_port;
    settings.baudrate = new_baudrate;
})?;
```

### Command Execution

Settings are used when executing commands:

```rust
// In command execution
let settings = settings_manager.get();
let command = format!(
    "arduino-cli compile --fqbn {} {}",
    settings.fqbn,
    settings.sketch_directory
);
```

## Validation

### Field Validation

Settings can include validation rules:

```rust
pub enum ValidationRule {
    NonEmpty,
    PortNumber,
    Baudrate,
    PathExists,
    Custom(Box<dyn Fn(&str) -> Result<(), String>>),
}

impl ValidationRule {
    pub fn validate(&self, value: &str) -> Result<(), String> {
        match self {
            ValidationRule::NonEmpty => {
                if value.trim().is_empty() {
                    Err("Value cannot be empty".to_string())
                } else {
                    Ok(())
                }
            }
            ValidationRule::PortNumber => {
                value.parse::<u16>()
                    .map_err(|_| "Invalid port number".to_string())
                    .and_then(|port| {
                        if port == 0 {
                            Err("Port cannot be 0".to_string())
                        } else {
                            Ok(())
                        }
                    })
            }
            // ... other validation rules
        }
    }
}
```

### Validation in Field Editor

```rust
// When saving field edits
if let Some(validation) = field_config.validation {
    match validation.validate(&new_value) {
        Ok(_) => {
            // Valid, save the value
            settings_manager.update(|settings| {
                update_field_value(settings, field_config.hwnd, &new_value);
            })?;
        }
        Err(error) => {
            // Show validation error
            toasts.push(Toast::new(
                format!("Validation error: {}", error),
                ToastType::Error,
            ));
        }
    }
}
```

## Best Practices

### 1. Thread Safety

Always use the SettingsManager for thread-safe access:

```rust
// Good: Use SettingsManager
let settings = settings_manager.get();

// Avoid: Direct access to shared state
```

### 2. Atomic Updates

Use the update method for atomic changes:

```rust
// Good: Atomic update
settings_manager.update(|settings| {
    settings.port = new_port;
    settings.baudrate = new_baudrate;
})?;

// Avoid: Separate updates that might be inconsistent
settings_manager.update(|settings| {
    settings.port = new_port;
})?;
settings_manager.update(|settings| {
    settings.baudrate = new_baudrate;
})?;
```

### 3. Error Handling

Always handle save errors:

```rust
match settings_manager.update(|settings| {
    settings.sketch_directory = new_dir;
}) {
    Ok(_) => {
        toasts.push(Toast::new("Settings saved", ToastType::Success));
    }
    Err(e) => {
        toasts.push(Toast::new(
            format!("Failed to save settings: {}", e),
            ToastType::Error,
        ));
    }
}
```

### 4. Validation

Validate user input before saving:

```rust
// Validate before updating
if validate_port(&new_port) {
    settings_manager.update(|settings| {
        settings.port = new_port;
    })?;
} else {
    toasts.push(Toast::new("Invalid port", ToastType::Error));
}
```

## Troubleshooting

### 1. Settings Not Persisting

Check file permissions and disk space:

```rust
// Debug settings path
eprintln!("Settings path: {:?}", get_settings_path());

// Test write access
let test_path = get_settings_path().with_extension("test");
fs::write(&test_path, "test").map_err(|e| {
    format!("Cannot write to settings directory: {}", e)
})?;
fs::remove_file(&test_path)?;
```

### 2. Corrupted Settings

Implement recovery logic:

```rust
pub fn load_with_recovery() -> Settings {
    let path = get_settings_path();
    match fs::read_to_string(&path) {
        Ok(contents) => {
            match serde_yaml::from_str::<Settings>(&contents) {
                Ok(settings) => settings,
                Err(e) => {
                    eprintln!("Failed to parse settings: {}, using defaults", e);
                    // Backup corrupted file
                    let backup_path = path.with_extension("yaml.backup");
                    let _ = fs::copy(&path, &backup_path);
                    Settings::default()
                }
            }
        }
        Err(_) => Settings::default(),
    }
}
```

### 3. Profile Loading Issues

Check profile directory structure:

```rust
pub fn debug_profile_structure() {
    let profiles_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("dev-console")
        .join("profiles");
    
    eprintln!("Profiles directory: {:?}", profiles_dir);
    
    if profiles_dir.exists() {
        for entry in fs::read_dir(&profiles_dir).unwrap() {
            let entry = entry.unwrap();
            eprintln!("Profile file: {:?}", entry.path());
        }
    } else {
        eprintln!("Profiles directory does not exist");
    }
}
```

## Summary

The settings storage system provides a robust foundation for managing application configuration. By following these guidelines, you can effectively work with settings values and ensure data persistence and consistency.

Key takeaways:
- Use SettingsManager for thread-safe access
- Always validate user input before saving
- Handle errors gracefully with user feedback
- Use profiles for different development environments
- Implement recovery mechanisms for corrupted settings
