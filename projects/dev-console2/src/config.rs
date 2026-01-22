use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use color_eyre::eyre;
use color_eyre::Result;

// In src/config.rs, add these helper functions
mod deserializers {
    use serde::{Deserialize, Deserializer};
    use std::fmt;

    // Helper to deserialize an anchored Connection object into its ID string
    pub fn deserialize_connection_id<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize as a temporary struct that only contains the ID
        #[derive(Deserialize)]
        struct IdHelper {
            id: String,
        }
        let helper = IdHelper::deserialize(deserializer)?;
        Ok(helper.id)
    }

    // Helper to deserialize an anchored Device object into its ID string
    pub fn deserialize_device_id<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct IdHelper {
            id: String,
        }
        let helper = IdHelper::deserialize(deserializer)?;
        Ok(helper.id)
    }

    // Helper to deserialize an anchored Mqtt object into its ID string
    pub fn deserialize_mqtt_id<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct IdHelper {
            id: String,
        }
        let helper = IdHelper::deserialize(deserializer)?;
        Ok(helper.id)
    }
}


#[derive(Debug, Deserialize, Default)]
pub struct ApplicationConfig {
    pub title: String,
    #[serde(default = "default_min_width")]
    pub min_width: u16,
    #[serde(default = "default_min_height")]
    pub min_height: u16,
    #[serde(default)]
    pub bindings: Vec<BindingConfig>,
    #[serde(default)]
    pub status_bar: StatusBarConfig,
}

fn default_min_width() -> u16 { 80 }
fn default_min_height() -> u16 { 21 }

#[derive(Debug, Deserialize, Default)]
pub struct BindingConfig {
    pub id: String,
    pub key: String,
    pub display: String,
    pub on_press: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct StatusBarConfig {
    pub default_text: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub application: ApplicationConfig,
    #[serde(default)]
    pub tab_bars: Vec<TabBarConfig>,
}

#[derive(Debug, Deserialize, Default)]
pub struct TabBindingConfig {
    pub key: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct TabBarConfig {
    pub id: String,
    #[serde(default)]
    pub alignment: Alignment,
    pub style: Option<String>,
    pub color: Option<String>,
    #[serde(default)]
    pub min_tab_width: u16,
    #[serde(default)]
    pub tab_tooltips: bool,
    #[serde(default)]
    pub navigation: Navigation,
    #[serde(default)]
    pub tabs: Vec<TabConfig>,
    #[serde(default)]
    pub tab_bindings: std::collections::HashMap<String, Vec<TabBindingConfig>>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Alignment {
    pub vertical: Option<String>,
    pub horizontal: Option<String>,
    #[serde(default)]
    pub offset_x: i16,
    #[serde(default)]
    pub offset_y: i16,
}

#[derive(Debug, Deserialize, Default)]
pub struct Navigation {
    #[serde(default)]
    pub left: Vec<String>,
    #[serde(default)]
    pub right: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct TabConfig {
    pub id: String,
    pub name: String,
    pub default: Option<String>,
    #[serde(default)]
    pub content: Content,
}

#[derive(Debug, Deserialize, Default)]
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
    pub fbqn: String,  // Note: YAML has FBQN (typo), keeping for compatibility
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
    #[serde(deserialize_with = "deserializers::deserialize_connection_id")]
    pub connection: String,
    #[serde(deserialize_with = "deserializers::deserialize_device_id")]
    pub device: String,
    #[serde(deserialize_with = "deserializers::deserialize_mqtt_id")]
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

// ... (existing content) ...

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
    
    match serde_yaml::from_str::<ProfileConfig>(&contents) {
        Ok(config) => {
            Ok(config)
        },
        Err(e) => {
            Err(eyre::eyre!("Failed to parse config.yaml: {}", e))
        }
    }
}

pub fn load_command_settings() -> Settings {
    // Try to load from profile config, fallback to dummy data
    if let Ok(profile_config) = load_profile_config() {
        if let Some(first_sketch) = profile_config.sketches.first() {
            // Find the device and connection for this sketch
            let device = profile_config.devices.iter()
                .find(|d| d.id == first_sketch.device);
            let connection = profile_config.connections.iter()
                .find(|c| c.id == first_sketch.connection);
            
            if let (Some(device), Some(connection)) = (device, connection) {
                // Extract sketch name from path
                let sketch_name = std::path::Path::new(&first_sketch.path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("sketch")
                    .to_string();
                
                return Settings {
                    sketch_directory: first_sketch.path.clone(),
                    sketch_name,
                    fqbn: device.fbqn.clone(),  // Using FBQN from YAML
                    board_model: device.board_model.clone(),
                    env: if connection.compiler == "arduino-cli" { "arduino" } else { "windows" }.to_string(),
                };
            }
        }
    }
    
    // Fallback to dummy data
    Settings {
        sketch_directory: "D:/_dev/_Projects/dev-boards/Arduino/sketchbook/sht21-bme680-led-mqtt".to_string(),
        sketch_name: "sht21-bme680-led-mqtt".to_string(),
        fqbn: "esp32:esp32:lilygo-t-display-s3".to_string(),
        board_model: "esp32-s3".to_string(),
        env: "arduino".to_string(),
    }
}

pub fn load_config() -> Result<Config> {
// ... (existing content) ...
    // For now, we only load the build-config.yaml
    // Later, we will implement the search and merge for config.yaml
    let mut file = File::open("build-config.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let config: Config = serde_yaml::from_str(&contents)?;
    
    Ok(config)
}
