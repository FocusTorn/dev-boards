// Field editor state and settings fields module

use crate::settings::Settings;
use serialport::available_ports;
use tui_input::Input;
use std::fs;
use std::path::PathBuf;

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

impl FieldEditorState {
    /// Get the current field index (for future use)
    #[allow(dead_code)]
    pub fn field_index(&self) -> usize {
        match self {
            FieldEditorState::Selected { field_index } => *field_index,
            FieldEditorState::Editing { field_index, .. } => *field_index,
            FieldEditorState::Selecting { field_index, .. } => *field_index,
        }
    }
    
    /// Check if the state is in editing mode (for future use)
    #[allow(dead_code)]
    pub fn is_editing(&self) -> bool {
        matches!(self, FieldEditorState::Editing { .. })
    }
    
    /// Check if the state is in selecting mode (for future use)
    #[allow(dead_code)]
    pub fn is_selecting(&self) -> bool {
        matches!(self, FieldEditorState::Selecting { .. })
    }
    
    /// Check if the state is in selected mode (for future use)
    #[allow(dead_code)]
    pub fn is_selected(&self) -> bool {
        matches!(self, FieldEditorState::Selected { .. })
    }
    
    /// Create a new Selected state (for future use)
    #[allow(dead_code)]
    pub fn new_selected(field_index: usize) -> Self {
        FieldEditorState::Selected { field_index }
    }
    
    /// Create a new Editing state from a value (for future use)
    #[allow(dead_code)]
    pub fn new_editing(field_index: usize, value: String) -> Self {
        let mut input = Input::new(value);
        let _ = input.handle(tui_input::InputRequest::GoToEnd);
        FieldEditorState::Editing { field_index, input }
    }
    
    /// Create a new Selecting state (for future use)
    #[allow(dead_code)]
    pub fn new_selecting(field_index: usize, selected_index: usize, options: Vec<String>) -> Self {
        FieldEditorState::Selecting {
            field_index,
            selected_index,
            options,
        }
    }
}

/// Settings field type enum - type-safe field access
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsField {
    SketchDirectory = 0,
    SketchName = 1,
    Environment = 2,
    BoardModel = 3,
    FQBN = 4,
    Port = 5,
    Baudrate = 6,
}

impl SettingsField {
    /// Get all fields in order
    pub fn all() -> Vec<SettingsField> {
        vec![
            SettingsField::SketchDirectory,
            SettingsField::SketchName,
            SettingsField::Environment,
            SettingsField::BoardModel,
            SettingsField::FQBN,
            SettingsField::Port,
            SettingsField::Baudrate,
        ]
    }
    
    /// Get field label
    pub fn label(&self) -> &'static str {
        match self {
            SettingsField::SketchDirectory => "Sketch Directory",
            SettingsField::SketchName => "Sketch Name",
            SettingsField::Environment => "Environment",
            SettingsField::BoardModel => "Board Model",
            SettingsField::FQBN => "FQBN",
            SettingsField::Port => "Port",
            SettingsField::Baudrate => "Baudrate",
        }
    }
    
    /// Get value from settings
    pub fn get_value(&self, settings: &Settings) -> String {
        match self {
            SettingsField::SketchDirectory => settings.sketch_directory.clone(),
            SettingsField::SketchName => settings.sketch_name.clone(),
            SettingsField::Environment => settings.env.clone(),
            SettingsField::BoardModel => settings.board_model.clone(),
            SettingsField::FQBN => settings.fqbn.clone(),
            SettingsField::Port => settings.port.clone(),
            SettingsField::Baudrate => settings.baudrate.to_string(),
        }
    }
    
    /// Set value in settings
    pub fn set_value(&self, settings: &mut Settings, value: String) {
        match self {
            SettingsField::SketchDirectory => settings.sketch_directory = value,
            SettingsField::SketchName => settings.sketch_name = value,
            SettingsField::Environment => settings.env = value,
            SettingsField::BoardModel => settings.board_model = value,
            SettingsField::FQBN => settings.fqbn = value,
            SettingsField::Port => settings.port = value,
            SettingsField::Baudrate => {
                if let Ok(b) = value.parse::<u32>() {
                    settings.baudrate = b;
                }
            }
        }
    }
    
    /// Check if field is a dropdown
    pub fn is_dropdown(&self) -> bool {
        matches!(self, SettingsField::Environment | SettingsField::Port | SettingsField::SketchName)
    }
    
    /// Get dropdown options for a field
    pub fn get_dropdown_options(&self, settings: &Settings) -> Vec<String> {
        match self {
            SettingsField::Environment => {
                vec!["arduino".to_string(), "esp-idf".to_string()]
            }
            SettingsField::Port => {
                // Port dropdown - detect available COM ports
                match available_ports() {
                    Ok(ports) => {
                        ports.into_iter()
                            .map(|p| p.port_name)
                            .collect()
                    }
                    Err(_) => {
                        // Fallback to common ports if detection fails
                        vec!["COM1".to_string(), "COM3".to_string(), "COM5".to_string(), "COM7".to_string(), "COM9".to_string()]
                    }
                }
            }
            SettingsField::SketchName => {
                // Sketch Name dropdown - scan sketch directory for .ino files
                if settings.sketch_directory.is_empty() {
                    return vec![];
                }
                
                let sketch_dir = PathBuf::from(&settings.sketch_directory);
                if !sketch_dir.exists() || !sketch_dir.is_dir() {
                    return vec![];
                }
                
                match fs::read_dir(&sketch_dir) {
                    Ok(entries) => {
                        let mut ino_files: Vec<String> = entries
                            .filter_map(|entry| {
                                if let Ok(entry) = entry {
                                    let path = entry.path();
                                    if path.is_file() {
                                        if let Some(ext) = path.extension() {
                                            if ext == "ino" {
                                                // Return just the filename without extension
                                                if let Some(file_name) = path.file_stem() {
                                                    return Some(file_name.to_string_lossy().to_string());
                                                }
                                            }
                                        }
                                    }
                                }
                                None
                            })
                            .collect();
                        
                        // Sort alphabetically for consistent display
                        ino_files.sort();
                        ino_files
                    }
                    Err(_) => vec![],
                }
            }
            _ => vec![],
        }
    }
    
    /// Convert from index (for backward compatibility)
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(SettingsField::SketchDirectory),
            1 => Some(SettingsField::SketchName),
            2 => Some(SettingsField::Environment),
            3 => Some(SettingsField::BoardModel),
            4 => Some(SettingsField::FQBN),
            5 => Some(SettingsField::Port),
            6 => Some(SettingsField::Baudrate),
            _ => None,
        }
    }
    
    /// Convert to index (for backward compatibility, for future use)
    #[allow(dead_code)]
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

/// Settings fields definition and management (backward compatibility wrapper)
pub struct SettingsFields;

impl SettingsFields {
    /// Create a new settings fields instance
    pub fn new() -> Self {
        Self
    }
    
    /// Get value for a field (by index for backward compatibility)
    pub fn get_value(&self, settings: &Settings, index: usize) -> String {
        SettingsField::from_index(index)
            .map(|field| field.get_value(settings))
            .unwrap_or_default()
    }
    
    /// Set value for a field (by index for backward compatibility)
    pub fn set_value(&self, settings: &mut Settings, index: usize, value: String) {
        if let Some(field) = SettingsField::from_index(index) {
            field.set_value(settings, value);
        }
    }
    
    /// Get the number of fields
    pub fn count(&self) -> usize {
        SettingsField::all().len()
    }
    
    /// Get the label for a field
    pub fn get_label(&self, index: usize) -> &str {
        SettingsField::from_index(index)
            .map(|field| field.label())
            .unwrap_or("")
    }
    
    /// Check if a field is a dropdown field
    pub fn is_dropdown(&self, index: usize) -> bool {
        SettingsField::from_index(index)
            .map(|field| field.is_dropdown())
            .unwrap_or(false)
    }
    
    /// Get dropdown options for a field
    pub fn get_dropdown_options(&self, index: usize, settings: &Settings) -> Vec<String> {
        SettingsField::from_index(index)
            .map(|field| field.get_dropdown_options(settings))
            .unwrap_or_default()
    }
}
