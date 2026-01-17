
// IMPORTS ------------------>> 

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

//--------------------------------------------------------<<

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Settings { //>
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
} //<

impl Default for Settings { //>
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
} //<


pub fn get_settings_path() -> PathBuf { //>
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("dev-console")
        .join("settings.yaml")
} //<


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
