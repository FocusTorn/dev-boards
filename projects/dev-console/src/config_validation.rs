// Configuration validation module
// Provides validation and error recovery for application configuration

use crate::config::{AppConfig, StatusBarConfigYaml};
use std::path::PathBuf;

/// Configuration validation errors
#[derive(Debug, Clone)]
pub enum ConfigValidationError {
    #[allow(dead_code)] // For future use
    MissingFile(PathBuf),
    #[allow(dead_code)] // For future use
    InvalidYaml(String),
    MissingTitle,
    MissingStatusBar,
    MissingTabBars,
}

impl std::fmt::Display for ConfigValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigValidationError::MissingFile(path) => {
                write!(f, "Configuration file not found: {:?}", path)
            }
            ConfigValidationError::InvalidYaml(msg) => {
                write!(f, "Invalid YAML format: {}", msg)
            }
            ConfigValidationError::MissingTitle => {
                write!(f, "Missing required 'title' field in application configuration")
            }
            ConfigValidationError::MissingStatusBar => {
                write!(f, "Missing required 'status_bar' configuration")
            }
            ConfigValidationError::MissingTabBars => {
                write!(f, "Missing required 'tab_bars' configuration")
            }
        }
    }
}

impl std::error::Error for ConfigValidationError {}

/// Validate application configuration
pub fn validate_config(config: &AppConfig) -> Result<(), ConfigValidationError> {
    // Validate application title
    if config.application.title.is_empty() {
        return Err(ConfigValidationError::MissingTitle);
    }
    
    // Validate status bar configuration
    if config.application.status_bar.default_text.is_empty() {
        return Err(ConfigValidationError::MissingStatusBar);
    }
    
    // Validate tab bars exist
    if config.tab_bars.is_empty() {
        return Err(ConfigValidationError::MissingTabBars);
    }
    
    Ok(())
}

/// Create default configuration when file is missing or invalid
/// Note: This creates a minimal config. For full functionality, a valid config.yaml is required.
pub fn create_default_config() -> AppConfig {
    use crate::config::{BindingConfigYaml, ApplicationConfig};
    use std::collections::HashMap;
    use serde_yaml;
    
    // Create a minimal valid config by parsing a default YAML string
    let default_yaml = r#"
application:
    title: "ESP32-S3 Dev Console"
    bindings:
        - key: "q"
          description: "Quit"
    status_bar:
        default_text: "Ready"
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
        color: "cyan"
        min_tab_width: 8
        tab_tooltips: true
        navigation:
            left: ["Left"]
            right: ["Right"]
        tabs:
            - id: "dashboard"
              name: "Dashboard"
              default: "active"
              content:
                  type: "text"
                  value: "ESP32-S3 Development Settings"
"#;
    
    // Parse the default YAML
    match serde_yaml::from_str::<AppConfig>(default_yaml) {
        Ok(config) => config,
        Err(_) => {
            // Fallback: create minimal config structure
            AppConfig {
                application: ApplicationConfig {
                    title: "ESP32-S3 Dev Console".to_string(),
                    bindings: vec![
                        BindingConfigYaml {
                            key: "q".to_string(),
                            description: "Quit".to_string(),
                        },
                    ],
                    status_bar: StatusBarConfigYaml {
                        default_text: "Ready".to_string(),
                        modal_text: None,
                    },
                },
                tab_bars: HashMap::new(),
            }
        }
    }
}

/// Load and validate configuration with error recovery
pub fn load_and_validate_config(
    config_path: Option<PathBuf>,
) -> Result<AppConfig, Box<dyn std::error::Error>> {
    use crate::config::load_config;
    
    let path = config_path.unwrap_or_else(|| {
        let mut default_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        default_path.push("config.yaml");
        default_path
    });
    
    // Check if file exists
    if !path.exists() {
        eprintln!("Warning: Configuration file not found: {:?}", path);
        eprintln!("Using default configuration");
        return Ok(create_default_config());
    }
    
    // Try to load configuration
    match load_config(Some(path.clone())) {
        Ok(config) => {
            // Validate loaded configuration
            match validate_config(&config) {
                Ok(_) => Ok(config),
                Err(e) => {
                    eprintln!("Warning: Configuration validation failed: {}", e);
                    eprintln!("Using default configuration");
                    Ok(create_default_config())
                }
            }
        }
        Err(e) => {
            eprintln!("Warning: Failed to load configuration: {}", e);
            eprintln!("Using default configuration");
            Ok(create_default_config())
        }
    }
}
