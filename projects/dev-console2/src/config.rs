use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use color_eyre::eyre::Result;

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

pub fn load_config() -> Result<Config> {
    // For now, we only load the build-config.yaml
    // Later, we will implement the search and merge for config.yaml
    let mut file = File::open("build-config.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let config: Config = serde_yaml::from_str(&contents)?;
    
    Ok(config)
}
