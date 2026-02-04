use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TabBarAlignment {
    Left,
    Center,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Alignment {
    pub vertical: Option<TabBarAlignment>,
    pub horizontal: Option<TabBarAlignment>,
    #[serde(default)]
    pub offset_x: i16,
    #[serde(default)]
    pub offset_y: i16,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct TabBarColors {
    pub active: Option<String>,
    pub negate: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct TabConfig {
    pub id: String,
    pub name: String,
    pub default: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct BindingConfig {
    pub key: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct BindingsConfig {
    #[serde(default = "default_separator")]
    pub separator: String,
    #[serde(default)]
    pub items: Vec<BindingConfig>,
}

fn default_separator() -> String {
    " | ".to_string()
}
