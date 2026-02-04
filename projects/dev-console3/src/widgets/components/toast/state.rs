use serde::Deserialize;
use std::time::{Duration, Instant};
use std::fs;
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl ToastLevel {
    pub fn color(&self) -> Color {
        match self {
            ToastLevel::Info => Color::Cyan,
            ToastLevel::Success => Color::Green,
            ToastLevel::Warning => Color::Yellow,
            ToastLevel::Error => Color::Red,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ToastLevel::Info => "ℹ",
            ToastLevel::Success => "✓",
            ToastLevel::Warning => "⚠",
            ToastLevel::Error => "✗",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    #[default]
    BottomCenter,
    BottomRight,
    Center,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToastConfig {
    pub position: ToastPosition,
    pub duration_seconds: f32,
    pub fade_out_seconds: f32,
}

impl Default for ToastConfig {
    fn default() -> Self {
        Self {
            position: ToastPosition::BottomCenter,
            duration_seconds: 1.5,
            fade_out_seconds: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub level: ToastLevel,
    pub shown_at: Instant,
    pub duration: Duration,
    pub opacity: f64,
}

#[derive(Debug)]
pub struct ToastManager {
    pub toasts: Vec<Toast>,
    pub config: ToastConfig,
}

impl ToastManager {
    pub fn new() -> Self {
        let config = Self::load_config().unwrap_or_default();
        Self {
            toasts: Vec::new(),
            config,
        }
    }

    fn load_config() -> color_eyre::Result<ToastConfig> {
        let config_path = std::path::PathBuf::from("src/widgets/components/toast/config.yaml");
        let content = fs::read_to_string(&config_path)?;
        Ok(serde_saphyr::from_str(&content)?)
    }

    pub fn add(&mut self, message: String, level: ToastLevel) {
        let total_duration = Duration::from_secs_f32(self.config.duration_seconds + self.config.fade_out_seconds);
        self.toasts.push(Toast {
            message,
            level,
            shown_at: Instant::now(),
            duration: total_duration,
            opacity: 1.0,
        });
    }
}