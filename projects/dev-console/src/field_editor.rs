// Field editor state and settings fields module

use crate::settings::Settings;
use serialport::available_ports;
use tui_input::Input;

/// Settings field editor state
#[derive(Debug, Clone)]
pub enum FieldEditorState {
    Selected {
        field_index: usize,
    },
    Editing {
        field_index: usize,
        input: Input,
    },
    Selecting {
        field_index: usize,
        selected_index: usize,
        options: Vec<String>,
    },
}

/// Settings fields definition and management
pub struct SettingsFields {
    labels: Vec<&'static str>,
    getters: Vec<Box<dyn Fn(&Settings) -> String>>,
    setters: Vec<Box<dyn Fn(&mut Settings, String)>>,
}

impl SettingsFields {
    /// Create a new settings fields instance
    pub fn new() -> Self {
        let mut labels = Vec::new();
        let mut getters: Vec<Box<dyn Fn(&Settings) -> String>> = Vec::new();
        let mut setters: Vec<Box<dyn Fn(&mut Settings, String)>> = Vec::new();
        
        // Sketch Directory
        labels.push("Sketch Directory");
        getters.push(Box::new(|s| s.sketch_directory.clone()));
        setters.push(Box::new(|s, v| s.sketch_directory = v));
        
        // Sketch Name
        labels.push("Sketch Name");
        getters.push(Box::new(|s| s.sketch_name.clone()));
        setters.push(Box::new(|s, v| s.sketch_name = v));
        
        // Environment (arduino, esp-idf)
        labels.push("Environment");
        getters.push(Box::new(|s| s.env.clone()));
        setters.push(Box::new(|s, v| s.env = v));
        
        // Board Model (esp32-s3, ard-nano)
        labels.push("Board Model");
        getters.push(Box::new(|s| s.board_model.clone()));
        setters.push(Box::new(|s, v| s.board_model = v));
        
        // FQBN
        labels.push("FQBN");
        getters.push(Box::new(|s| s.fqbn.clone()));
        setters.push(Box::new(|s, v| s.fqbn = v));
        
        // Port
        labels.push("Port");
        getters.push(Box::new(|s| s.port.clone()));
        setters.push(Box::new(|s, v| s.port = v));
        
        // Baudrate
        labels.push("Baudrate");
        getters.push(Box::new(|s| s.baudrate.to_string()));
        setters.push(Box::new(|s, v| {
            if let Ok(b) = v.parse::<u32>() {
                s.baudrate = b;
            }
        }));
        
        Self {
            labels,
            getters,
            setters,
        }
    }
    
    /// Get value for a field
    pub fn get_value(&self, settings: &Settings, index: usize) -> String {
        (self.getters[index])(settings)
    }
    
    /// Set value for a field
    pub fn set_value(&self, settings: &mut Settings, index: usize, value: String) {
        (self.setters[index])(settings, value);
    }
    
    /// Get the number of fields
    pub fn count(&self) -> usize {
        self.labels.len()
    }
    
    /// Get the label for a field
    pub fn get_label(&self, index: usize) -> &str {
        self.labels[index]
    }
    
    /// Check if a field is a dropdown field
    pub fn is_dropdown(&self, index: usize) -> bool {
        // Environment (index 2) and Port (index 5) are dropdowns
        index == 2 || index == 5
    }
    
    /// Get dropdown options for a field
    pub fn get_dropdown_options(&self, index: usize) -> Vec<String> {
        match index {
            2 => {
                // Environment dropdown
                vec!["arduino".to_string(), "esp-idf".to_string()]
            }
            5 => {
                // Port dropdown - detect available COM ports
                match available_ports() {
                    Ok(ports) => {
                        ports.into_iter()
                            .map(|p| {
                                // On Windows, ports are like "COM1", "COM9", etc.
                                p.port_name
                            })
                            .collect()
                    }
                    Err(_) => {
                        // Fallback to common ports if detection fails
                        vec!["COM1".to_string(), "COM3".to_string(), "COM5".to_string(), "COM7".to_string(), "COM9".to_string()]
                    }
                }
            }
            _ => vec![],
        }
    }
}
