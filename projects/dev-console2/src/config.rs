use serde::Deserialize;
use std::fs::File;
use std::io::Read;
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
    #[serde(default)]
    pub name: String,
    pub title: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub author: String,
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
    pub hover: Option<String>,
    pub disabled: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct TabBarConfig {
    pub id: String,
    #[serde(default)]
    pub alignment: Alignment,
    pub style: Option<TabBarStyle>,
    pub color: Option<String>,
    #[serde(rename = "type")]
    pub bar_type: Option<String>,
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

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub application: ApplicationConfig,
    #[serde(default)]
    pub tab_bars: Vec<TabBarConfig>,
}

fn default_separator() -> String {
    " | ".to_string()
}

use crate::widgets::tab_bar::{TabBarAlignment, TabBarStyle};

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
    #[serde(default)]
    pub content: Content,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Connection {
    pub id: String,
    pub compiler: String,
    pub port: String,
    pub baudrate: u32,
}

#[derive(Debug, Deserialize)]
pub struct Device {
    pub id: String,
    pub board_model: String,
    pub fbqn: String,
}

#[derive(Debug, Deserialize)]
pub struct Mqtt {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Sketch {
    pub id: String,
    pub path: String,
    pub connection: String,
    pub device: String,
    pub mqtt: String,
}

#[derive(Debug, Deserialize)]
pub struct ProfileConfig {
    pub connections: Vec<Connection>,
    pub devices: Vec<Device>,
    pub mqtt: Vec<Mqtt>,
    pub sketches: Vec<Sketch>,
}

use crate::commands::Settings;


pub fn load_profile_config() -> Result<ProfileConfig> {
    let config_path = std::path::PathBuf::from("config.yaml");
    let mut file = match File::open(&config_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(eyre::eyre!("Failed to open config.yaml at {:?}: {}", config_path, e));
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(e) => {
            return Err(eyre::eyre!("Failed to read config.yaml: {}", e));
        }
    }
    
    match serde_saphyr::from_str::<ProfileConfig>(&contents) {
        Ok(config) => {
            Ok(config)
        },
        Err(e) => {
            Err(eyre::eyre!("Failed to parse config.yaml: {}", e))
        }
    }
}

pub fn load_command_settings() -> Result<Settings> {
    // Try to load from profile config
    let profile_config = load_profile_config()?;

    if let Some(first_sketch) = profile_config.sketches.first() {
        // Find the device and connection for this sketch
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

pub fn load_config() -> Result<Config> {
// ... (existing content) ...
    // For now, we only load of build-config.yaml
    // Later, we will implement of search and merge for config.yaml
    let mut file = File::open("build-config.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let config: Config = serde_saphyr::from_str(&contents)?;
    
    Ok(config)
}

pub fn load_widget_config() -> Result<ToastConfig> {
    let config_path = std::path::PathBuf::from("src/widgets/widget-config.yaml");
    let mut file = match File::open(&config_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(eyre::eyre!("Failed to open widget-config.yaml at {:?}: {}", config_path, e));
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(e) => {
            return Err(eyre::eyre!("Failed to read widget-config.yaml: {}", e));
        }
    }
    
    // Directly deserialize the widget config
    let widget_config: WidgetConfig = serde_saphyr::from_str(&contents)
        .map_err(|e| eyre::eyre!("Failed to parse widget-config.yaml: {}", e))?;
    
    Ok(widget_config.toast_widget)
}
