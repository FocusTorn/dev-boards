// Tool detection module with dependency injection
// Centralizes arduino-cli, python, and uv detection logic

use std::path::PathBuf;
use std::process::Command;

/// Tool detection result
#[derive(Debug, Clone)]
pub struct ToolInfo {
    pub path: PathBuf,
    pub available: bool,
    pub version: Option<String>,
}

/// Trait for tool detection (dependency injection)
pub trait ToolDetector {
    fn detect_arduino_cli(&self, project_root: &PathBuf, env: &str) -> ToolInfo;
    fn detect_python(&self) -> ToolInfo;
    fn detect_uv(&self) -> ToolInfo;
}

/// Default tool detector implementation
pub struct DefaultToolDetector;

impl ToolDetector for DefaultToolDetector {
    fn detect_arduino_cli(&self, project_root: &PathBuf, env: &str) -> ToolInfo {
        if env == "arduino" {
            let workspace_path = project_root.join("Arduino").join("arduino-cli.exe");
            if workspace_path.exists() {
                return ToolInfo {
                    path: workspace_path,
                    available: true,
                    version: None,
                };
            }
        }
        
        // Check if arduino-cli is in PATH
        if which::which("arduino-cli").is_ok() {
            return ToolInfo {
                path: PathBuf::from("arduino-cli"),
                available: true,
                version: None,
            };
        }
        
        ToolInfo {
            path: PathBuf::from("arduino-cli"),
            available: false,
            version: None,
        }
    }
    
    fn detect_python(&self) -> ToolInfo {
        // Check if python is in PATH
        if let Ok(path) = which::which("python") {
            // Try to get version
            let version = Command::new(&path)
                .arg("--version")
                .output()
                .ok()
                .and_then(|output| {
                    String::from_utf8(output.stdout).ok()
                });
            
            return ToolInfo {
                path: path.into(),
                available: true,
                version,
            };
        }
        
        // Try python3 as fallback
        if let Ok(path) = which::which("python3") {
            let version = Command::new(&path)
                .arg("--version")
                .output()
                .ok()
                .and_then(|output| {
                    String::from_utf8(output.stdout).ok()
                });
            
            return ToolInfo {
                path: path.into(),
                available: true,
                version,
            };
        }
        
        ToolInfo {
            path: PathBuf::from("python"),
            available: false,
            version: None,
        }
    }
    
    fn detect_uv(&self) -> ToolInfo {
        // Check if uv is in PATH
        if let Ok(path) = which::which("uv") {
            // Try to get version
            let version = Command::new(&path)
                .arg("--version")
                .output()
                .ok()
                .and_then(|output| {
                    String::from_utf8(output.stdout).ok()
                });
            
            return ToolInfo {
                path: path.into(),
                available: true,
                version,
            };
        }
        
        ToolInfo {
            path: PathBuf::from("uv"),
            available: false,
            version: None,
        }
    }
}

/// Tool manager that uses dependency injection
pub struct ToolManager<T: ToolDetector> {
    detector: T,
    arduino_cli: Option<ToolInfo>,
    python: Option<ToolInfo>,
    uv: Option<ToolInfo>,
}

impl<T: ToolDetector> ToolManager<T> {
    pub fn new(detector: T) -> Self {
        Self {
            detector,
            arduino_cli: None,
            python: None,
            uv: None,
        }
    }
    
    /// Detect all tools (lazy initialization)
    pub fn detect_all(&mut self, project_root: &PathBuf, env: &str) {
        if self.arduino_cli.is_none() {
            self.arduino_cli = Some(self.detector.detect_arduino_cli(project_root, env));
        }
        if self.python.is_none() {
            self.python = Some(self.detector.detect_python());
        }
        if self.uv.is_none() {
            self.uv = Some(self.detector.detect_uv());
        }
    }
    
    /// Get arduino-cli info
    pub fn arduino_cli(&mut self, project_root: &PathBuf, env: &str) -> &ToolInfo {
        if self.arduino_cli.is_none() {
            self.arduino_cli = Some(self.detector.detect_arduino_cli(project_root, env));
        }
        self.arduino_cli.as_ref().unwrap()
    }
    
    /// Get python info
    pub fn python(&mut self) -> &ToolInfo {
        if self.python.is_none() {
            self.python = Some(self.detector.detect_python());
        }
        self.python.as_ref().unwrap()
    }
    
    /// Get uv info
    pub fn uv(&mut self) -> &ToolInfo {
        if self.uv.is_none() {
            self.uv = Some(self.detector.detect_uv());
        }
        self.uv.as_ref().unwrap()
    }
}
