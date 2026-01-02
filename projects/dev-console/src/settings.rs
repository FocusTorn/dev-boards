
// IMPORTS ------------------>> 

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

//--------------------------------------------------------<<

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings { //>
    pub sketch_directory: String,
    pub sketch_name: String,
    pub env: String,
    pub board_model: String,
    pub fqbn: String,
    pub port: String,
    pub baudrate: u32,
    pub create_log: bool,
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
