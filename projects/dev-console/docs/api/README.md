# API Reference

This document provides a comprehensive API reference for the dev-console application, covering all major modules, structs, and functions.

## Core Modules

### app_state.rs

Application state management and coordination.

#### `AppState`

Main application state container.

```rust
pub struct AppState {
    pub settings: SettingsManager,
    pub settings_fields: SettingsFields,
    pub field_editor_state: FieldEditorState,
    pub profile_state: ProfileState,
    pub dashboard: Arc<Mutex<DashboardState>>,
    pub process_manager: Arc<ProcessManager>,
}
```

**Methods:**

```rust
impl AppState {
    /// Create a new application state
    pub fn new() -> Self;
}
```

### settings_manager.rs

Thread-safe settings management with automatic persistence.

#### `SettingsManager`

Centralized settings manager.

```rust
pub struct SettingsManager {
    settings: Arc<Mutex<Settings>>,
    settings_path: PathBuf,
}
```

**Methods:**

```rust
impl SettingsManager {
    /// Create a new settings manager by loading from disk
    pub fn load() -> Self;
    
    /// Get a clone of the current settings
    pub fn get(&self) -> Settings;
    
    /// Get a reference to settings (read-only access)
    pub fn get_ref(&self) -> std::sync::MutexGuard<'_, Settings>;
    
    /// Update settings with a closure and automatically save
    pub fn update<F>(&self, f: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut Settings);
    
    /// Update settings without saving (for batch updates)
    pub fn update_without_save<F>(&self, f: F)
    where
        F: FnOnce(&mut Settings);
    
    /// Save current settings to disk
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Reload settings from disk
    pub fn reload(&self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Get the settings path
    pub fn path(&self) -> &PathBuf;
}
```

### settings.rs

Settings data structure and persistence.

#### `Settings`

Core settings structure.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Settings {
    pub sketch_directory: String,
    pub sketch_name: String,
    pub env: String,
    pub board_model: String,
    pub fqbn: String,
    pub port: String,
    pub baudrate: u32,
    pub create_log: bool,
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

**Methods:**

```rust
impl Settings {
    /// Load settings from disk
    pub fn load() -> Self;
    
    /// Save settings to disk
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl Default for Settings {
    /// Create default settings
    fn default() -> Self;
}
```

**Functions:**

```rust
/// Get the settings file path
pub fn get_settings_path() -> PathBuf;
```

### field_editor.rs

Interactive field editing functionality.

#### `FieldEditorState`

State of the field editor.

```rust
pub enum FieldEditorState {
    Normal,
    Selected { field_index: usize },
    Editing { field_index: usize, input: Input },
    Selecting { field_index: usize, selected_index: usize, options: Vec<String> },
    ProfileSelecting { selected_index: usize, options: Vec<String> },
}
```

#### `SettingsFields`

Collection of settings fields.

```rust
pub struct SettingsFields {
    pub fields: Vec<SettingsField>,
}

pub struct SettingsField {
    pub hwnd: &'static str,
    pub label: String,
    pub value: String,
    pub field_type: FieldType,
}
```

#### `FieldType`

Types of settings fields.

```rust
pub enum FieldType {
    Text,
    Number,
    Boolean,
    Dropdown(Vec<String>),
    Optional,
}
```

### profile_manager.rs

Profile management functionality.

**Functions:**

```rust
/// Save current settings as a profile
pub fn save_profile(name: &str, settings: &Settings) -> Result<(), Box<dyn std::error::Error>>;

/// Load a profile
pub fn load_profile(name: &str) -> Result<Settings, Box<dyn std::error::Error>>;

/// List available profiles
pub fn list_profiles() -> Result<Vec<String>, Box<dyn std::error::Error>>;

/// Delete a profile
pub fn delete_profile(name: &str) -> Result<(), Box<dyn std::error::Error>>;

/// Get profile file path
pub fn get_profile_path(profile_name: &str) -> PathBuf;
```

### dashboard.rs

Dashboard state and command management.

#### `DashboardState`

Dashboard state management.

```rust
pub struct DashboardState {
    pub commands: Vec<String>,
    pub selected_command: usize,
    pub output_lines: Vec<String>,
    pub scroll_offset: usize,
    pub is_running: bool,
    pub status_text: String,
}
```

**Methods:**

```rust
impl DashboardState {
    /// Create a new dashboard state
    pub fn new() -> Self;
    
    /// Add a line to the output
    pub fn add_output_line(&mut self, line: String);
    
    /// Set the status text
    pub fn set_status_text(&mut self, text: String);
    
    /// Scroll output up
    pub fn scroll_up(&mut self);
    
    /// Scroll output down
    pub fn scroll_down(&mut self);
    
    /// Get visible output lines
    pub fn get_visible_output(&self, height: usize) -> &[String];
}
```

### event_handler.rs

Event handling for keyboard and mouse input.

#### Event Handler Functions

```rust
/// Handle dashboard keyboard events
pub fn handle_dashboard_key_event(
    key_code: crossterm::event::KeyCode,
    dashboard: &Arc<Mutex<DashboardState>>,
    settings_manager: &SettingsManager,
    process_manager: Arc<ProcessManager>,
) -> bool;

/// Handle dashboard mouse scroll events
pub fn handle_dashboard_scroll(
    mouse_event: &crossterm::event::MouseEvent,
    dashboard: &Arc<Mutex<DashboardState>>,
    registry: &RectRegistry,
);

/// Handle field editor keyboard events
pub fn handle_field_editor_key_event(
    key_code: crossterm::event::KeyCode,
    key_modifiers: crossterm::event::KeyModifiers,
    field_editor_state: &FieldEditorState,
    settings_manager: &SettingsManager,
    settings_fields: &SettingsFields,
    profile_state: &ProfileState,
    registry: &mut RectRegistry,
    tab_bar: &TabBarManager,
    tab_style: TabBarStyle,
) -> FieldEditorEventResult;

/// Handle profile keyboard events
pub fn handle_profile_key_event(
    key_code: crossterm::event::KeyCode,
    key_modifiers: crossterm::event::KeyModifiers,
    profile_state: &ProfileState,
    settings_manager: &SettingsManager,
) -> ProfileEventResult;

/// Handle settings field mouse clicks
pub fn handle_settings_field_click(
    mouse_event: &crossterm::event::MouseEvent,
    settings_manager: &SettingsManager,
    settings_fields: &SettingsFields,
    registry: &RectRegistry,
    tab_bar: &TabBarManager,
) -> Option<FieldEditorState>;

/// Handle tab mouse clicks
pub fn handle_tab_click(
    mouse_event: &crossterm::event::MouseEvent,
    current_tab_bar: &Option<(TabBar, RectHandle)>,
    registry: &mut RectRegistry,
    tab_bar: &TabBarManager,
    tab_style: TabBarStyle,
);

/// Handle text input during editing
pub fn handle_editing_input(
    key_code: crossterm::event::KeyCode,
    key_modifiers: crossterm::event::KeyModifiers,
    input: &mut Input,
);

/// Handle dropdown navigation
pub fn handle_dropdown_navigation(
    key_code: crossterm::event::KeyCode,
    selected_index: &mut usize,
    options: &[String],
);
```

#### Event Result Types

```rust
/// Result of handling a field editor event
#[derive(Debug)]
pub enum FieldEditorEventResult {
    Continue,
    Exit,
    Toast(Toast),
    StateChanged(FieldEditorState),
}

/// Result of handling a profile event
#[derive(Debug)]
pub enum ProfileEventResult {
    Continue,
    SaveProfile(String),
    LoadProfile(String),
    RefreshProfiles,
    Toast(Toast),
}
```

### layout_manager.rs

Layout calculations and caching.

#### `LayoutManager`

Centralized layout manager.

```rust
pub struct LayoutManager {
    cache: LayoutCache,
}
```

**Methods:**

```rust
impl LayoutManager {
    /// Create a new layout manager
    pub fn new() -> Self;
    
    /// Get content area with caching
    pub fn get_content_area(&mut self, content_rect: Rect) -> Option<Rect>;
    
    /// Get the underlying cache
    pub fn cache_mut(&mut self) -> &mut LayoutCache;
    
    /// Clear the cache
    pub fn clear_cache(&mut self);
}
```

### process_manager.rs

Process management for command execution.

#### `ProcessManager`

Manages child processes.

```rust
pub struct ProcessManager {
    processes: Arc<Mutex<Vec<Child>>>,
}
```

**Methods:**

```rust
impl ProcessManager {
    /// Create a new process manager
    pub fn new() -> Self;
    
    /// Execute a command
    pub fn execute(&self, command: &str, dashboard: &Arc<Mutex<DashboardState>>) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Kill all running processes
    pub fn kill_all(&self);
    
    /// Cleanup on exit
    pub fn cleanup(&self);
    
    /// Check if any processes are running
    pub fn is_running(&self) -> bool;
}
```

### command_helper.rs

Command execution utilities.

**Functions:**

```rust
/// Execute a command with the given settings
pub fn execute_command(
    command: &str,
    dashboard: &Arc<Mutex<DashboardState>>,
    settings: Settings,
    process_manager: Arc<ProcessManager>,
);

/// Format command with settings
pub fn format_command(command: &str, settings: &Settings) -> String;

/// Validate command before execution
pub fn validate_command(command: &str) -> Result<(), String>;
```

## Constants

### constants.rs

Application-wide constants.

```rust
/// Minimum terminal width in pixels
pub const MIN_WIDTH_PIXELS: u16 = 80;

/// Minimum terminal height in pixels
pub const MIN_HEIGHT_PIXELS: u16 = 21;

/// Field height in lines
pub const FIELD_HEIGHT: u16 = 3;

/// Spacing between fields
pub const FIELD_SPACING: u16 = 0;

/// Content area width percentage
pub const CONTENT_WIDTH_PERCENT: u16 = 50;

/// Content area height percentage
pub const CONTENT_HEIGHT_PERCENT: u16 = 50;

/// Maximum output lines to keep in memory
pub const MAX_OUTPUT_LINES: usize = 1000;

/// Toast display duration in seconds
pub const TOAST_DURATION_SECS: f64 = 1.5;

// HWND constants
pub const HWND_MAIN_CONTENT_BOX: &str = "hwndMainContentBox";
pub const HWND_MAIN_CONTENT_TAB_BAR: &str = "hwndMainContentTabBar";
pub const HWND_SETTINGS_FIELD_SKETCH_DIR: &str = "hwndSettingsFieldSketchDir";
pub const HWND_SETTINGS_FIELD_SKETCH_NAME: &str = "hwndSettingsFieldSketchName";
pub const HWND_SETTINGS_FIELD_ENV: &str = "hwndSettingsFieldEnv";
pub const HWND_SETTINGS_FIELD_BOARD_MODEL: &str = "hwndSettingsFieldBoardModel";
pub const HWND_SETTINGS_FIELD_FQBN: &str = "hwndSettingsFieldFQBN";
pub const HWND_SETTINGS_FIELD_PORT: &str = "hwndSettingsFieldPort";
pub const HWND_SETTINGS_FIELD_BAUDRATE: &str = "hwndSettingsFieldBaudrate";
pub const HWND_SETTINGS_SECTION_DEVICE: &str = "hwndSettingsSectionDevice";
pub const HWND_SETTINGS_SECTION_CONNECTION: &str = "hwndSettingsSectionConnection";
pub const HWND_SETTINGS_SECTION_MQTT: &str = "hwndSettingsSectionMQTT";
pub const HWND_SETTINGS_FIELD_MQTT_HOST: &str = "hwndSettingsFieldMqttHost";
pub const HWND_SETTINGS_FIELD_MQTT_PORT: &str = "hwndSettingsFieldMqttPort";
pub const HWND_SETTINGS_FIELD_MQTT_USERNAME: &str = "hwndSettingsFieldMqttUsername";
pub const HWND_SETTINGS_FIELD_MQTT_PASSWORD: &str = "hwndSettingsFieldMqttPassword";
pub const HWND_SETTINGS_FIELD_MQTT_TOPIC_COMMAND: &str = "hwndSettingsFieldMqttTopicCommand";
pub const HWND_SETTINGS_FIELD_MQTT_TOPIC_STATE: &str = "hwndSettingsFieldMqttTopicState";
pub const HWND_SETTINGS_FIELD_MQTT_TOPIC_STATUS: &str = "hwndSettingsFieldMqttTopicStatus";
pub const HWND_PROFILE_BOX: &str = "hwndProfileBox";
pub const HWND_PROFILE_LIST: &str = "hwndProfileList";
pub const HWND_PROFILE_SELECTOR: &str = "hwndProfileSelector";
```

## Configuration

### config.rs

Configuration data structures.

#### `AppConfig`

Main application configuration.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub application: ApplicationConfig,
    pub tab_bars: HashMap<String, TabBarConfig>,
    pub tab_content: Vec<TabContentConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub title: String,
    pub min_width: u16,
    pub min_height: u16,
    pub bindings: Vec<BindingConfig>,
    pub status_bar: StatusBarConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabBarConfig {
    pub hwnd: String,
    pub anchor: String,
    pub alignment: AlignmentConfig,
    pub style: String,
    pub color: String,
    pub min_tab_width: u16,
    pub tab_tooltips: bool,
    pub navigation: NavigationConfig,
    pub tabs: Vec<TabConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabContentConfig {
    pub tab_id: String,
    pub bindings: Vec<BindingConfig>,
}
```

**Functions:**

```rust
/// Load and validate configuration from YAML file
pub fn load_and_validate_config(config_path: Option<PathBuf>) -> Result<AppConfig, Box<dyn std::error::Error>>;
```

## Error Handling

### error_format.rs

Error formatting and display utilities.

**Functions:**

```rust
/// Format error for display
pub fn format_error(error: &Box<dyn std::error::Error>) -> String;

/// Format error with context
pub fn format_error_with_context(error: &Box<dyn std::error::Error>, context: &str) -> String;

/// Get user-friendly error message
pub fn get_user_friendly_error(error: &Box<dyn std::error::Error>) -> String;
```

## Utility Modules

### path_utils.rs

Path manipulation utilities.

**Functions:**

```rust
/// Normalize a path for the current platform
pub fn normalize_path(path: &str) -> PathBuf;

/// Get relative path from base
pub fn get_relative_path(base: &Path, target: &Path) -> Option<PathBuf>;

/// Validate path exists and is accessible
pub fn validate_path(path: &Path) -> Result<(), String>;

/// Expand user home directory in path
pub fn expand_home_path(path: &str) -> PathBuf;
```

### string_intern.rs

String interning for performance.

**Functions:**

```rust
/// Intern a string
pub fn intern_string(s: &str) -> StringId;

/// Get interned string by ID
pub fn get_interned_string(id: StringId) -> Option<&'static str>;

/// Clear string intern cache
pub fn clear_intern_cache();
```

### tool_detector.rs

Development tool detection.

**Functions:**

```rust
/// Detect available development tools
pub fn detect_tools() -> HashMap<String, bool>;

/// Check if Arduino CLI is available
pub fn check_arduino_cli() -> bool;

/// Check if PlatformIO is available
pub fn check_platformio() -> bool;

/// Get tool version
pub fn get_tool_version(tool: &str) -> Option<String>;
```

## Render Modules

### render/mod.rs

Rendering module exports.

### render/dashboard.rs

Dashboard rendering functions.

**Functions:**

```rust
/// Render the dashboard tab
pub fn render_dashboard(
    f: &mut Frame,
    area: Rect,
    dashboard: &Arc<Mutex<DashboardState>>,
    settings_manager: &SettingsManager,
    profile_state: &ProfileState,
    registry: &RectRegistry,
    tab_bar: &TabBarManager,
);

/// Render command list
pub fn render_command_list(
    f: &mut Frame,
    area: Rect,
    dashboard: &DashboardState,
    selected_index: usize,
);

/// Render output area
pub fn render_output_area(
    f: &mut Frame,
    area: Rect,
    dashboard: &DashboardState,
);

/// Render status bar
pub fn render_status_bar(
    f: &mut Frame,
    area: Rect,
    dashboard: &DashboardState,
);
```

### render/settings.rs

Settings rendering functions.

**Functions:**

```rust
/// Render the settings tab
pub fn render_settings(
    f: &mut Frame,
    area: Rect,
    settings_manager: &SettingsManager,
    settings_fields: &SettingsFields,
    field_editor_state: &FieldEditorState,
    profile_state: &ProfileState,
    registry: &RectRegistry,
    tab_bar: &TabBarManager,
);

/// Render settings fields
pub fn render_settings_fields(
    f: &mut Frame,
    area: Rect,
    settings_fields: &SettingsFields,
    field_editor_state: &FieldEditorState,
);

/// Render profile selector
pub fn render_profile_selector(
    f: &mut Frame,
    area: Rect,
    profile_state: &ProfileState,
    field_editor_state: &FieldEditorState,
);
```

## Command Modules

### commands/mod.rs

Command module exports.

### commands/compile_parser.rs

Compilation command parsing and execution.

**Functions:**

```rust
/// Parse compile command output
pub fn parse_compile_output(output: &str) -> CompileResult;

/// Execute compile command
pub fn execute_compile(settings: &Settings) -> Result<CompileResult, Box<dyn std::error::Error>>;

/// Get compile command string
pub fn get_compile_command(settings: &Settings) -> String;
```

### commands/upload.rs

Upload command execution.

**Functions:**

```rust
/// Execute upload command
pub fn execute_upload(settings: &Settings) -> Result<(), Box<dyn std::error::Error>>;

/// Get upload command string
pub fn get_upload_command(settings: &Settings) -> String;

/// Detect upload progress
pub fn parse_upload_progress(output: &str) -> Option<UploadProgress>;
```

### commands/monitor_serial.rs

Serial monitoring functionality.

**Functions:**

```rust
/// Start serial monitor
pub fn start_serial_monitor(
    port: &str,
    baudrate: u32,
    dashboard: &Arc<Mutex<DashboardState>>,
) -> Result<(), Box<dyn std::error::Error>>;

/// Parse serial output
pub fn parse_serial_output(line: &str) -> SerialMessage;

/// Format serial data for display
pub fn format_serial_message(message: &SerialMessage) -> String;
```

### commands/monitor_mqtt.rs

MQTT monitoring functionality.

**Functions:**

```rust
/// Start MQTT monitor
pub fn start_mqtt_monitor(
    host: &str,
    port: u16,
    topics: Vec<String>,
    dashboard: &Arc<Mutex<DashboardState>>,
) -> Result<(), Box<dyn std::error::Error>>;

/// Parse MQTT message
pub fn parse_mqtt_message(topic: &str, payload: &str) -> MqttMessage;

/// Format MQTT message for display
pub fn format_mqtt_message(message: &MqttMessage) -> String;
```

## Types and Enums

### Common Types

```rust
/// String identifier for interned strings
pub type StringId = u32;

/// Result type for operations
pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Callback function type
pub type Callback = Box<dyn Fn() + Send + Sync>;
```

### Enums

```rust
/// Application mode
#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Dashboard,
    Settings,
}

/// Field validation result
#[derive(Debug, Clone)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
    Warning(String),
}

/// Command execution status
#[derive(Debug, Clone, PartialEq)]
pub enum CommandStatus {
    Pending,
    Running,
    Success,
    Failed,
    Cancelled,
}
```

## Macros

### Utility Macros

```rust
/// Log debug message
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!("[DEBUG] {}", format!($($arg)*));
    };
}

/// Create toast message
macro_rules! toast {
    ($message:expr, $toast_type:expr) => {
        Toast::new($message.to_string(), $toast_type)
    };
}

/// Handle result with toast
macro_rules! handle_result {
    ($result:expr, $toasts:expr, $success_msg:expr) => {
        match $result {
            Ok(_) => {
                $toasts.push(toast!($success_msg, ToastType::Success));
            }
            Err(e) => {
                $toasts.push(toast!(format!("Error: {}", e), ToastType::Error));
            }
        }
    };
}
```

This API reference provides comprehensive documentation for all major components of the dev-console application. Use this as a guide when extending or modifying the application functionality.
