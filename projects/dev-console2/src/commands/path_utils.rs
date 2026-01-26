// Path resolution utility module

use std::env;
use std::path::{Path, PathBuf};

/// Find workspace root using WORKSPACE_ROOT environment variable
pub fn find_workspace_root() -> Result<PathBuf, String> {
    env::var("WORKSPACE_ROOT")
        .map(PathBuf::from)
        .map_err(|_| "WORKSPACE_ROOT environment variable not set".to_string())
}

/// Find arduino-cli executable
pub fn find_arduino_cli(env: &str, project_root: &Path) -> PathBuf {
    if env == "arduino" {
        let workspace_path = project_root.join("Arduino").join("arduino-cli.exe");
        if workspace_path.exists() {
            return workspace_path;
        }
        
        // Check if arduino-cli is in PATH
        if which::which("arduino-cli").is_ok() {
            return PathBuf::from("arduino-cli");
        }
        
        workspace_path
    } else {
        PathBuf::from("arduino-cli")
    }
}

/// Calculate library path for arduino-cli
pub fn get_library_path(project_root: &Path, board_model: &str) -> PathBuf {
    project_root.join("lib").join(board_model)
}
