use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use color_eyre::eyre; // Explicitly import eyre for the macro
use color_eyre::Result;
use serde_saphyr; // New YAML deserializer

pub use crate::widgets::toast::ToastConfig;

#[derive(Debug, Deserialize)]
struct WidgetConfig {
    toast_widget: ToastConfig,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ApplicationConfig {
    pub title: String,
    #[serde(default = "default_min_width")]
    pub min_width: u16,
    #[serde(default = "default_min_height")]
    pub min_height: u16,
    #[serde(default)]
    pub show_terminal_size: bool,
    #[serde(default)]
    pub show_press_and_modifier: bool,
    #[serde(default)]
    pub status_bar: StatusBarConfig,
    #[serde(default)]
    pub bindings: BindingsConfig,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct StatusBarConfig {
    pub default_text: String,
}

fn default_min_width() -> u16 { 80 }
fn default_min_height() -> u16 { 21 }

#[derive(Debug, Deserialize, Default, Clone)]
pub struct BindingConfig {
    pub key: String,         // Display key (e.g. "[🡙]")
    pub description: String, // Display description
    #[serde(default)]
    pub triggers: std::collections::HashMap<String, String>, // Physical Key -> Semantic Action
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct BindingsConfig {
    #[serde(default = "default_separator")]
    pub separator: String,
    #[serde(default)]
    pub items: Vec<BindingConfig>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct TabBarColors {
    pub active: Option<String>,
    pub negate: Option<String>,
}

// Map the alignment enums locally to avoid tab_bar dependency
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TabBarAlignment {
    #[serde(alias = "Left")]
    Left,
    #[serde(alias = "Center")]
    Center,
    #[serde(alias = "Right")]
    Right,
    #[serde(alias = "Top")]
    Top,
    #[serde(alias = "Bottom")]
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TabBarStyle {
    #[serde(alias = "Tab")]
    Tab,
    #[serde(alias = "Text")]
    Text,
    #[serde(alias = "Boxed")]
    Boxed,
    #[serde(alias = "BoxStatic")]
    BoxStatic,
    #[serde(alias = "TextStatic")]
    TextStatic,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct TabBarConfig {
    pub id: String,
    #[serde(default)]
    pub alignment: Alignment,
    pub style: Option<TabBarStyle>,
    pub color: Option<String>,
    pub colors: Option<TabBarColors>,
    #[serde(default)]
    pub min_tab_width: u16,
    #[serde(default)]
    pub tab_tooltips: bool,
    #[serde(default)]
    pub navigation: Navigation,
    #[serde(default)]
    pub tabs: Vec<TabConfig>,
    #[serde(default)]
    pub tab_bindings: std::collections::HashMap<String, BindingsConfig>,
}

/// High-level configuration container for the application.
///>
/// Combines general application settings, theme definitions, and tab bar 
/// configurations into a single structure for deserialization from YAML.
///<
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub application: ApplicationConfig,
    #[serde(default)]
    pub theme: crate::app::theme::ThemeConfig,
    #[serde(default)]
    pub tab_bars: Vec<TabBarConfig>,
}

fn default_separator() -> String {
    " | ".to_string()
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Alignment {
    pub vertical: Option<TabBarAlignment>,
    pub horizontal: Option<TabBarAlignment>,
    #[serde(default)]
    pub offset_x: i16,
    #[serde(default)]
    pub offset_y: i16,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Navigation {
    #[serde(default)]
    pub left: Vec<String>,
    #[serde(default)]
    pub right: Vec<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct TabConfig {
    pub id: String,
    pub name: String,
    pub default: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Connection {
    pub id: String,
    pub compiler: String,
    pub port: String,
    pub baudrate: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Device {
    pub id: String,
    pub board_model: String,
    pub fbqn: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mqtt {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sketch {
    pub id: String,
    pub path: String,
    pub connection: String,
    pub device: String,
    pub mqtt: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProfileConfig {
    pub connections: Vec<Connection>,
    pub devices: Vec<Device>,
    pub mqtt: Vec<Mqtt>,
    pub sketches: Vec<Sketch>,
}

use crate::commands::Settings;

/// Loads the hardware profile configuration from `config.yaml`.
pub fn load_profile_config() -> Result<ProfileConfig> {
    let config_path = std::path::PathBuf::from("config.yaml");
    let mut file = File::open(&config_path)
        .map_err(|e| eyre::eyre!("Failed to open config.yaml at {:?}: {}", config_path, e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| eyre::eyre!("Failed to read config.yaml: {}", e))?;
    parse_profile_config(&contents)
}

fn parse_profile_config(contents: &str) -> Result<ProfileConfig> {
    serde_saphyr::from_str::<ProfileConfig>(contents)
        .map_err(|e| eyre::eyre!("Failed to parse config.yaml: {}", e))
}

pub fn save_profile_config(config: &ProfileConfig) -> Result<()> {
    save_profile_config_to_path(config, "config.yaml")
}

pub fn save_profile_config_to_path(config: &ProfileConfig, path: &str) -> Result<()> {
    let contents = serde_saphyr::to_string(config)
        .map_err(|e| eyre::eyre!("Failed to serialize config.yaml: {}", e))?;
    let config_path = std::path::PathBuf::from(path);
    let mut file = File::create(&config_path)
        .map_err(|e| eyre::eyre!("Failed to create config at {:?}: {}", config_path, e))?;
    file.write_all(contents.as_bytes())
        .map_err(|e| eyre::eyre!("Failed to write config: {}", e))?;
    Ok(())
}

/// Extracts initial hardware settings from the first available sketch profile.
pub fn load_command_settings() -> Result<Settings> {
    let profile_config = load_profile_config()?;
    extract_settings_from_profile(&profile_config)
}

fn extract_settings_from_profile(profile_config: &ProfileConfig) -> Result<Settings> {
    if let Some(first_sketch) = profile_config.sketches.first() {
        let device = profile_config.devices.iter()
            .find(|d| d.id == first_sketch.device);
        let connection = profile_config.connections.iter()
            .find(|c| c.id == first_sketch.connection);
        
        if let (Some(device), Some(connection)) = (device, connection) {
            let path = std::path::Path::new(&first_sketch.path);
            let sketch_directory = path.parent()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_default();
            let sketch_name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("sketch")
                .to_string();
            
            return Ok(Settings {
                sketch_directory,
                sketch_name,
                fqbn: device.fbqn.clone(),
                port: connection.port.clone(),
                baudrate: connection.baudrate,
                board_model: device.board_model.clone(),
                env: if connection.compiler == "arduino-cli" { "arduino" } else { "windows" }.to_string(),
            });
        }
    }
    Err(eyre::eyre!("No valid sketch configuration found in config.yaml"))
}

/// Loads the main application UI configuration from `build-config.yaml`.
pub fn load_config() -> Result<Config> {
    let mut file = File::open("build-config.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    parse_config(&contents)
}

fn parse_config(contents: &str) -> Result<Config> {
    serde_saphyr::from_str(contents)
        .map_err(|e| eyre::eyre!("Failed to parse build-config.yaml: {}", e))
}

/// Loads the specialized widget configuration for UI components.
pub fn load_widget_config() -> Result<ToastConfig> {
    let config_path = std::path::PathBuf::from("src/widgets/widget-config.yaml");
    let mut file = File::open(&config_path)
        .map_err(|e| eyre::eyre!("Failed to open widget-config.yaml at {:?}: {}", config_path, e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| eyre::eyre!("Failed to read widget-config.yaml: {}", e))?;
    parse_widget_config(&contents)
}

fn parse_widget_config(contents: &str) -> Result<ToastConfig> {
    let widget_config: WidgetConfig = serde_saphyr::from_str(contents)
        .map_err(|e| eyre::eyre!("Failed to parse widget-config.yaml: {}", e))?;
    Ok(widget_config.toast_widget)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config_valid() {
        let yaml = r#"
application:
  title: "Test Console"
  min_width: 100
  min_height: 30
theme:
  styles: {}
tab_bars: []
"#;
        let config = parse_config(yaml).unwrap();
        assert_eq!(config.application.title, "Test Console");
        assert_eq!(config.application.min_width, 100);
        assert_eq!(config.application.min_height, 30);
    }

    #[test]
    fn test_parse_config_defaults() {
        let yaml = "application: { title: 'Default' }";
        let config = parse_config(yaml).unwrap();
        assert_eq!(config.application.min_width, 80);
        assert_eq!(config.application.min_height, 21);
    }

    #[test]
    fn test_parse_config_invalid() {
        let yaml = "application: { min_width: 'not a number' }";
        let result = parse_config(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_profile_config_valid() {
        let yaml = r#"
connections:
  - id: conn1
    compiler: arduino-cli
    port: COM1
    baudrate: 115200
devices:
  - id: dev1
    board_model: esp32
    fbqn: esp32:esp32:esp32
mqtt: []
sketches:
  - id: sketch1
    path: /path/to/sketch.ino
    connection: conn1
    device: dev1
    mqtt: none
"#;
        let config = parse_profile_config(yaml).unwrap();
        assert_eq!(config.connections.len(), 1);
        assert_eq!(config.sketches[0].id, "sketch1");
    }

    #[test]
    fn test_extract_settings_valid() {
        let profile = ProfileConfig {
            connections: vec![Connection {
                id: "c1".to_string(),
                compiler: "arduino-cli".to_string(),
                port: "COM1".to_string(),
                baudrate: 115200,
            }],
            devices: vec![Device {
                id: "d1".to_string(),
                board_model: "m1".to_string(),
                fbqn: "f1".to_string(),
            }],
            mqtt: vec![],
            sketches: vec![Sketch {
                id: "s1".to_string(),
                path: "C:/projects/my_sketch/my_sketch.ino".to_string(),
                connection: "c1".to_string(),
                device: "d1".to_string(),
                mqtt: "m1".to_string(),
            }],
        };
        let settings = extract_settings_from_profile(&profile).unwrap();
        assert_eq!(settings.sketch_name, "my_sketch");
        assert_eq!(settings.port, "COM1");
        assert_eq!(settings.env, "arduino");
    }

    #[test]
    fn test_extract_settings_no_sketches() {
        let profile = ProfileConfig {
            connections: vec![],
            devices: vec![],
            mqtt: vec![],
            sketches: vec![],
        };
        let result = extract_settings_from_profile(&profile);
        assert!(result.is_err());
    }

            #[test]

            fn test_parse_widget_config() {

                let yaml = r#"

        toast_widget:

          duration_seconds: 5

          position: top_right

        "#;

                let config = parse_widget_config(yaml).unwrap();

                assert_eq!(config.duration_seconds, 5.0);

            }

        

                        #[test]

        

                        fn test_parse_tab_bar_config_all_variants() {

        

                            let yaml = r#"

        

                    application: { title: "T" }

        

                    tab_bars:

        

                      - id: "t1"

        

                        style: "boxed"

        

                        alignment: { vertical: "bottom", horizontal: "right" }

        

                      - id: "t2"

        

                        style: "text_static"

        

                        alignment: { vertical: "top", horizontal: "left" }

        

                      - id: "t3"

        

                        style: "box_static"

        

                    "#;

        

                    

        

                        let config = parse_config(yaml).unwrap();

        

                        assert_eq!(config.tab_bars[0].style, Some(TabBarStyle::Boxed));

        

                        assert_eq!(config.tab_bars[0].alignment.vertical, Some(TabBarAlignment::Bottom));

        

                        assert_eq!(config.tab_bars[1].style, Some(TabBarStyle::TextStatic));

        

                        assert_eq!(config.tab_bars[2].style, Some(TabBarStyle::BoxStatic));

        

                    }

        

                

        

                    #[test]

        

                    fn test_extract_settings_windows_env() {

        

                        let profile = ProfileConfig {

        

                            connections: vec![Connection {

        

                                id: "c1".to_string(),

        

                                compiler: "other".to_string(), // Should result in "windows" env

        

                                port: "COM1".to_string(),

        

                                baudrate: 115200,

        

                            }],

        

                            devices: vec![Device {

        

                                id: "d1".to_string(),

        

                                board_model: "m1".to_string(),

        

                                fbqn: "f1".to_string(),

        

                            }],

        

                            mqtt: vec![],

        

                            sketches: vec![Sketch {

        

                                id: "s1".to_string(),

        

                                path: "C:/path/to/my_sketch.ino".to_string(),

        

                                connection: "c1".to_string(),

        

                                device: "d1".to_string(),

        

                                mqtt: "m1".to_string(),

        

                            }],

        

                        };

        

                        let settings = extract_settings_from_profile(&profile).unwrap();

        

                        assert_eq!(settings.env, "windows");

        

                    }

        

                

        

                    #[test]

        

                    fn test_load_config_file_not_found() {

        

                

        

            

                // ...

            }

        }

        

    