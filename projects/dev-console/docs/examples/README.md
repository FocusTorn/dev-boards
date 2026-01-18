# Examples

This directory contains practical examples and tutorials for using and extending the dev-console application.

## Basic Usage Examples

### 1. Getting Started

```rust
// examples/basic_usage.rs
use dev_console::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize application state
    let app_state = AppState::new();
    
    // Load settings
    let settings = app_state.settings.get();
    println!("Current sketch directory: {}", settings.sketch_directory);
    
    // Update settings
    app_state.settings.update(|s| {
        s.port = "COM10".to_string();
    })?;
    
    println!("Updated port to: COM10");
    Ok(())
}
```

### 2. Custom Command Handler

```rust
// examples/custom_command.rs
use dev_console::*;
use std::sync::{Arc, Mutex};

fn custom_compile_command(
    dashboard: &Arc<Mutex<DashboardState>>,
    settings: &Settings,
) -> Result<(), Box<dyn std::error::Error>> {
    let command = format!(
        "arduino-cli compile --fqbn {} {}",
        settings.fqbn,
        settings.sketch_directory
    );
    
    // Add custom pre-processing
    {
        let mut state = dashboard.lock().unwrap();
        state.add_output_line("Starting custom compile...".to_string());
        state.set_status_text("Compiling with custom options...".to_string());
    }
    
    // Execute with custom environment
    std::env::set_var("CUSTOM_BUILD_FLAG", "1");
    
    // Use the command helper
    command_helper::execute_command(&command, dashboard, settings.clone(), 
                                   Arc::new(process_manager::ProcessManager::new()));
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dashboard = Arc::new(Mutex::new(DashboardState::new()));
    let settings = Settings::default();
    
    custom_compile_command(&dashboard, &settings)?;
    Ok(())
}
```

### 3. Profile Management

```rust
// examples/profile_management.rs
use dev_console::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a custom profile
    let mut settings = Settings::default();
    settings.sketch_directory = "/home/user/my-esp32-project".to_string();
    settings.sketch_name = "my-project".to_string();
    settings.port = "/dev/ttyUSB0".to_string();
    settings.baudrate = 921600;
    
    // Save profile
    profile_manager::save_profile("high-speed", &settings)?;
    println!("Saved 'high-speed' profile");
    
    // List all profiles
    let profiles = profile_manager::list_profiles()?;
    println!("Available profiles:");
    for profile in profiles {
        println!("  - {}", profile);
    }
    
    // Load a profile
    let loaded_settings = profile_manager::load_profile("high-speed")?;
    println!("Loaded profile with baudrate: {}", loaded_settings.baudrate);
    
    Ok(())
}
```

## Advanced Examples

### 4. Custom Tab Implementation

```rust
// examples/custom_tab.rs
use dev_console::*;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::Span,
    style::{Color, Style},
};

struct CustomTabState {
    counter: u32,
    message: String,
}

impl CustomTabState {
    fn new() -> Self {
        Self {
            counter: 0,
            message: "Custom tab initialized".to_string(),
        }
    }
    
    fn increment(&mut self) {
        self.counter += 1;
        self.message = format!("Counter incremented to {}", self.counter);
    }
    
    fn reset(&mut self) {
        self.counter = 0;
        self.message = "Counter reset".to_string();
    }
}

fn render_custom_tab(
    f: &mut Frame,
    area: Rect,
    state: &CustomTabState,
) {
    let block = Block::default()
        .title("Custom Tab")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    
    let text = vec![
        Span::raw("Counter: "),
        Span::styled(
            format!("{}", state.counter),
            Style::default().fg(Color::Yellow)
        ),
        Span::raw("\n\n"),
        Span::raw(&state.message),
    ];
    
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default());
    
    f.render_widget(paragraph, area);
}

fn handle_custom_tab_events(
    key_code: crossterm::event::KeyCode,
    state: &mut CustomTabState,
) -> bool {
    match key_code {
        crossterm::event::KeyCode::Char('i') => {
            state.increment();
            true
        }
        crossterm::event::KeyCode::Char('r') => {
            state.reset();
            true
        }
        _ => false,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = CustomTabState::new();
    
    // Example of handling events
    println!("Press 'i' to increment counter, 'r' to reset");
    
    // Simulate some events
    handle_custom_tab_events(crossterm::event::KeyCode::Char('i'), &mut state);
    handle_custom_tab_events(crossterm::event::KeyCode::Char('i'), &mut state);
    handle_custom_tab_events(crossterm::event::KeyCode::Char('r'), &mut state);
    
    println!("Final state: {}", state.message);
    Ok(())
}
```

### 5. MQTT Integration Example

```rust
// examples/mqtt_integration.rs
use dev_console::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;

struct MqttMessageHandler {
    dashboard: Arc<Mutex<DashboardState>>,
}

impl MqttMessageHandler {
    fn new(dashboard: Arc<Mutex<DashboardState>>) -> Self {
        Self { dashboard }
    }
    
    fn handle_message(&self, topic: &str, payload: &str) {
        let mut state = self.dashboard.lock().unwrap();
        
        match topic {
            "device/status" => {
                state.add_output_line(format!("Status: {}", payload));
                state.set_status_text(format!("Device status: {}", payload));
            }
            "device/data" => {
                state.add_output_line(format!("Data: {}", payload));
            }
            _ => {
                state.add_output_line(format!("Unknown topic [{}]: {}", topic, payload));
            }
        }
    }
}

fn simulate_mqtt_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    let dashboard = Arc::new(Mutex::new(DashboardState::new()));
    let handler = MqttMessageHandler::new(dashboard.clone());
    
    // Simulate receiving MQTT messages
    let messages = vec![
        ("device/status", "online"),
        ("device/data", "{\"temperature\": 25.5, \"humidity\": 60.2}"),
        ("device/status", "processing"),
        ("device/data", "{\"temperature\": 26.0, \"humidity\": 59.8}"),
        ("device/status", "idle"),
    ];
    
    for (topic, payload) in messages {
        handler.handle_message(topic, payload);
        std::thread::sleep(Duration::from_millis(500));
    }
    
    // Print all output
    let state = dashboard.lock().unwrap();
    println!("Dashboard Output:");
    for line in &state.output_lines {
        println!("  {}", line);
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simulate_mqtt_monitoring()
}
```

### 6. Custom Field Types

```rust
// examples/custom_fields.rs
use dev_console::*;

#[derive(Debug, Clone)]
enum CustomFieldType {
    FilePath,
    PortSelector,
    BaudrateSelector,
    ToggleSwitch,
}

#[derive(Debug, Clone)]
struct CustomField {
    hwnd: &'static str,
    label: String,
    field_type: CustomFieldType,
    value: String,
    validation: Option<Box<dyn Fn(&str) -> Result<(), String>>>,
}

impl CustomField {
    fn new(
        hwnd: &'static str,
        label: &str,
        field_type: CustomFieldType,
        default_value: &str,
    ) -> Self {
        Self {
            hwnd,
            label: label.to_string(),
            field_type,
            value: default_value.to_string(),
            validation: None,
        }
    }
    
    fn with_validation<F>(mut self, validator: F) -> Self 
    where
        F: Fn(&str) -> Result<(), String> + 'static,
    {
        self.validation = Some(Box::new(validator));
        self
    }
    
    fn validate(&self) -> Result<(), String> {
        if let Some(validator) = &self.validation {
            validator(&self.value)
        } else {
            Ok(())
        }
    }
    
    fn get_options(&self) -> Vec<String> {
        match self.field_type {
            CustomFieldType::PortSelector => {
                // Detect available serial ports
                serialport::available_ports()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|p| p.port_name)
                    .collect()
            }
            CustomFieldType::BaudrateSelector => {
                vec![
                    "9600".to_string(),
                    "19200".to_string(),
                    "38400".to_string(),
                    "57600".to_string(),
                    "115200".to_string(),
                    "230400".to_string(),
                    "460800".to_string(),
                    "921600".to_string(),
                ]
            }
            _ => vec![],
        }
    }
}

fn create_custom_fields() -> Vec<CustomField> {
    vec![
        CustomField::new(
            "hwndCustomSketchPath",
            "Sketch Path",
            CustomFieldType::FilePath,
            "/home/user/arduino",
        ).with_validation(|path| {
            if std::path::Path::new(path).exists() {
                Ok(())
            } else {
                Err("Path does not exist".to_string())
            }
        }),
        
        CustomField::new(
            "hwndCustomPort",
            "Serial Port",
            CustomFieldType::PortSelector,
            "COM9",
        ),
        
        CustomField::new(
            "hwndCustomBaudrate",
            "Baud Rate",
            CustomFieldType::BaudrateSelector,
            "115200",
        ).with_validation(|baud| {
            baud.parse::<u32>()
                .map_err(|_| "Invalid baudrate".to_string())
                .and_then(|b| {
                    if b > 0 {
                        Ok(())
                    } else {
                        Err("Baudrate must be positive".to_string())
                    }
                })
        }),
        
        CustomField::new(
            "hwndCustomLogging",
            "Enable Logging",
            CustomFieldType::ToggleSwitch,
            "false",
        ),
    ]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fields = create_custom_fields();
    
    println!("Custom Fields Configuration:");
    for field in &fields {
        println!("\nField: {}", field.label);
        println!("  HWND: {}", field.hwnd);
        println!("  Type: {:?}", field.field_type);
        println!("  Value: {}", field.value);
        
        // Validate field
        match field.validate() {
            Ok(_) => println!("  Validation: ✓ Passed"),
            Err(e) => println!("  Validation: ✗ {}", e),
        }
        
        // Show options for selector fields
        let options = field.get_options();
        if !options.is_empty() {
            println!("  Options: {}", options.join(", "));
        }
    }
    
    Ok(())
}
```

## Configuration Examples

### 7. Custom Configuration

```yaml
# examples/custom_config.yaml
application:
    title: "Custom ESP32 Console"
    min_width: 120
    min_height: 30
    bindings:
        - key: "[q]"
          description: "Quit"
        - key: "[F1]"
          description: "Help"
        - key: "[Ctrl+r]"
          description: "Refresh"
        - key: "[Ctrl+s]"
          description: "Save"
    status_bar:
        default_text: "Custom Console | [F1] Help | [Ctrl+S] Save"
        modal_text: "Modal Mode - [Esc] to exit"

tab_bars:
    main_content_tab_bar:
        hwnd: "hwndMainContentTabBar"
        anchor: "hwndMainContentBox"
        alignment:
            vertical: "top"
            horizontal: "center"
            offset_x: 0
            offset_y: 0
        style: "tabbed"
        color: "green"
        min_tab_width: 12
        tab_tooltips: true
        navigation:
            left: ["Left", "h"]
            right: ["Right", "l"]
        tabs:
            - id: "dashboard"
              name: "Dashboard"
              default: "active"
            - id: "settings"
              name: "Settings"
            - id: "monitor"
              name: "Monitor"
            - id: "tools"
              name: "Tools"

tab_content:
    - tab_id: "dashboard"
      bindings:
        - key: "[Enter]"
          description: "Execute command"
        - key: "[Ctrl+c]"
          description: "Cancel command"
        - key: "[F5]"
          description: "Refresh commands"
    
    - tab_id: "settings"
      bindings:
        - key: "[Tab]"
          description: "Next field"
        - key: "[Shift+Tab]"
          description: "Previous field"
        - key: "[Enter]"
          description: "Edit field"
        - key: "[s]"
          description: "Save profile"
        - key: "[l]"
          description: "Load profile"
    
    - tab_id: "monitor"
      bindings:
        - key: "[Space]"
          description: "Start/Stop monitoring"
        - key: "[c]"
          description: "Clear output"
        - key: "[f]"
          description: "Follow output"
    
    - tab_id: "tools"
      bindings:
        - key: "[b]"
          description: "Board info"
        - key: "[p]"
          description: "Port scan"
        - key: "[d]"
          description: "Device detect"
```

### 8. Multi-Environment Settings

```yaml
# examples/production_settings.yaml
sketch_directory: "/opt/esp32-production/firmware"
sketch_name: "production-controller"
env: "arduino"
board_model: "esp32-s3"
fqbn: "esp32:esp32:esp32s3"
port: "/dev/ttyUSB0"
baudrate: 115200
create_log: true

# Production MQTT configuration
mqtt_host: "mqtt.production.company.com"
mqtt_port: 8883
mqtt_username: "prod_controller"
mqtt_password: "secure_production_password"
mqtt_topic_command: "factory/controller/command"
mqtt_topic_state: "factory/controller/state"
mqtt_topic_status: "factory/controller/status"
```

```yaml
# examples/development_settings.yaml
sketch_directory: "/home/developer/esp32-dev"
sketch_name: "dev-controller"
env: "platformio"
board_model: "esp32-s3"
fqbn: "esp32:esp32:esp32s3"
port: "/dev/ttyUSB1"
baudrate: 921600
create_log: true

# Development MQTT configuration
mqtt_host: "localhost"
mqtt_port: 1883
mqtt_username: "dev_user"
mqtt_password: "dev_password"
mqtt_topic_command: "dev/controller/command"
mqtt_topic_state: "dev/controller/state"
mqtt_topic_status: "dev/controller/status"
```

## Testing Examples

### 9. Unit Test Examples

```rust
// examples/testing_examples.rs
use dev_console::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_validation() {
        let mut settings = Settings::default();
        
        // Test valid port
        settings.port = "COM9".to_string();
        assert!(validate_port(&settings.port).is_ok());
        
        // Test invalid port
        settings.port = "".to_string();
        assert!(validate_port(&settings.port).is_err());
        
        // Test valid baudrate
        settings.baudrate = 115200;
        assert!(validate_baudrate(settings.baudrate).is_ok());
        
        // Test invalid baudrate
        settings.baudrate = 0;
        assert!(validate_baudrate(settings.baudrate).is_err());
    }

    #[test]
    fn test_profile_operations() {
        let settings = Settings::default();
        
        // Test saving profile
        let result = profile_manager::save_profile("test_profile", &settings);
        assert!(result.is_ok());
        
        // Test loading profile
        let loaded_settings = profile_manager::load_profile("test_profile");
        assert!(loaded_settings.is_ok());
        assert_eq!(loaded_settings.unwrap(), settings);
        
        // Test listing profiles
        let profiles = profile_manager::list_profiles();
        assert!(profiles.is_ok());
        let profile_list = profiles.unwrap();
        assert!(profile_list.contains(&"test_profile".to_string()));
        
        // Cleanup
        let _ = profile_manager::delete_profile("test_profile");
    }

    #[test]
    fn test_dashboard_state() {
        let mut dashboard = DashboardState::new();
        
        // Test initial state
        assert_eq!(dashboard.selected_command, 0);
        assert!(!dashboard.is_running);
        assert!(dashboard.output_lines.is_empty());
        
        // Test adding output
        dashboard.add_output_line("Test line".to_string());
        assert_eq!(dashboard.output_lines.len(), 1);
        assert_eq!(dashboard.output_lines[0], "Test line");
        
        // Test status text
        dashboard.set_status_text("Test status".to_string());
        assert_eq!(dashboard.status_text, "Test status");
        
        // Test scrolling
        dashboard.scroll_up();
        dashboard.scroll_down();
        
        // Test command selection
        dashboard.commands = vec!["cmd1".to_string(), "cmd2".to_string()];
        dashboard.selected_command = 1;
        assert_eq!(dashboard.selected_command, 1);
    }

    fn validate_port(port: &str) -> Result<(), String> {
        if port.trim().is_empty() {
            Err("Port cannot be empty".to_string())
        } else {
            Ok(())
        }
    }

    fn validate_baudrate(baudrate: u32) -> Result<(), String> {
        if baudrate == 0 {
            Err("Baudrate must be positive".to_string())
        } else {
            Ok(())
        }
    }
}

fn main() {
    println!("Run tests with: cargo test --example testing_examples");
}
```

### 10. Integration Test Example

```rust
// examples/integration_test.rs
use dev_console::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn test_full_workflow() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize application state
    let app_state = AppState::new();
    
    // Test settings update
    app_state.settings.update(|s| {
        s.sketch_directory = "/tmp/test-sketch".to_string();
        s.sketch_name = "test-project".to_string();
        s.port = "COM9".to_string();
    })?;
    
    // Verify settings were updated
    let settings = app_state.settings.get();
    assert_eq!(settings.sketch_name, "test-project");
    
    // Test dashboard operations
    {
        let mut dashboard = app_state.dashboard.lock().unwrap();
        dashboard.add_output_line("Test output line".to_string());
        dashboard.set_status_text("Test status".to_string());
    }
    
    // Test profile operations
    let settings = app_state.settings.get();
    profile_manager::save_profile("integration-test", &settings)?;
    
    let loaded_settings = profile_manager::load_profile("integration-test")?;
    assert_eq!(loaded_settings.sketch_name, "test-project");
    
    // Cleanup
    let _ = profile_manager::delete_profile("integration-test");
    
    println!("Integration test passed!");
    Ok(())
}

fn test_concurrent_access() -> Result<(), Box<dyn std::error::Error>> {
    let settings_manager = SettingsManager::load();
    let settings_manager_clone = settings_manager.clone();
    
    // Spawn background thread that updates settings
    let handle = thread::spawn(move || {
        for i in 0..5 {
            let _ = settings_manager_clone.update(|s| {
                s.sketch_name = format!("concurrent-test-{}", i);
            });
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Main thread reads settings
    for i in 0..5 {
        let settings = settings_manager.get();
        println!("Iteration {}: {}", i, settings.sketch_name);
        thread::sleep(Duration::from_millis(50));
    }
    
    handle.join().unwrap();
    println!("Concurrent access test passed!");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_full_workflow()?;
    test_concurrent_access()?;
    Ok(())
}
```

## Running Examples

Each example can be run individually:

```bash
# Run basic usage example
cargo run --example basic_usage

# Run custom command example
cargo run --example custom_command

# Run profile management example
cargo run --example profile_management

# Run custom tab example
cargo run --example custom_tab

# Run MQTT integration example
cargo run --example mqtt_integration

# Run custom fields example
cargo run --example custom_fields

# Run testing examples
cargo test --example testing_examples

# Run integration tests
cargo run --example integration_test
```

These examples provide practical demonstrations of how to use and extend the dev-console application for various use cases and scenarios.
