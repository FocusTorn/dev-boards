use serde::{Deserialize, Serialize};
use crate::config::hardware::{Connection, Device, Mqtt, Sketch};
use color_eyre::Result;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ProfileConfig {
    pub connections: Vec<Connection>,
    pub devices: Vec<Device>,
    pub mqtt: Vec<Mqtt>,
    pub sketches: Vec<Sketch>,
}

impl ProfileConfig {
    pub fn load() -> Result<Self> {
        let config_path = std::path::PathBuf::from("config.yaml");
        let mut file = File::open(&config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_saphyr::from_str(&contents)?)
    }

    pub fn save(&self) -> Result<()> {
        let contents = serde_saphyr::to_string(self)?;
        let mut file = File::create("config.yaml")?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}
