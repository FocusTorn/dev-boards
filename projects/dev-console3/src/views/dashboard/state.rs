use crate::widgets::components::dropdown::OverlayDropdown;
use crate::config::ProfileConfig;

#[derive(Debug)]
pub struct Dashboard {
    pub profile_config: Option<ProfileConfig>,
    pub profile_dropdown: OverlayDropdown,
    pub commands: Vec<String>,
    pub selected_command: usize,
    pub focus_commands: bool,
}

impl Dashboard {
    pub fn new() -> Self {
        let profile_config = ProfileConfig::load().ok();
        
        let profile_ids = if let Some(config) = &profile_config {
            config.sketches.iter().map(|s| s.id.clone()).collect()
        } else {
            vec!["No Profiles".to_string()]
        };

        let profile_dropdown = OverlayDropdown::new("Sketch Profile".to_string(), profile_ids, 5);
        
        let commands = vec![
            "Compile".to_string(),
            "Upload".to_string(),
            "Monitor-Serial".to_string(),
            "Monitor-MQTT".to_string(),
            "Clean".to_string(),
            "All".to_string(),
        ];

        Self { 
            profile_config,
            profile_dropdown,
            commands,
            selected_command: 0,
            focus_commands: false,
        }
    }
}
