use crate::config::ProfileConfig;
use crate::widgets::components::dropdown::OverlayDropdown;

#[derive(Debug)]
pub struct Profiles {
    pub config: Option<ProfileConfig>,
    pub categories: Vec<String>,
    pub selected_category: usize,
    pub selected_field: usize,
    
    // Dropdowns for specific fields
    pub port_dropdown: OverlayDropdown,
    pub baud_dropdown: OverlayDropdown,
}

impl Profiles {
    pub fn new() -> Self {
        let config = ProfileConfig::load().ok();
        let categories = vec!["Device".into(), "Connections".into(), "MQTT".into()];
        
        // Mock data for dropdowns
        let ports = vec!["COM1".into(), "COM3".into(), "COM4".into(), "/dev/ttyUSB0".into()];
        let bauds = vec!["9600".into(), "57600".into(), "115200".into(), "230400".into()];

        Self {
            config,
            categories,
            selected_category: 0,
            selected_field: 0,
            port_dropdown: OverlayDropdown::new("Serial Port".into(), ports, 4),
            baud_dropdown: OverlayDropdown::new("Baud Rate".into(), bauds, 4),
        }
    }
}
