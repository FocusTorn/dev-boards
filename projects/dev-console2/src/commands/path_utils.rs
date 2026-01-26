/// Path resolution and environment discovery utilities.
///>
/// This module provides the logic for locating critical workspace artifacts, 
/// toolchain executables, and hardware-specific libraries.
///<
use std::env;
use std::path::{Path, PathBuf};

/// Locates the absolute path to the workspace root.
///>
/// This relies on the `WORKSPACE_ROOT` environment variable, which is 
/// expected to be set by the calling shell or environment setup script.
///<
pub fn find_workspace_root() -> Result<PathBuf, String> {
    env::var("WORKSPACE_ROOT")
        .map(PathBuf::from)
        .map_err(|_| "WORKSPACE_ROOT environment variable not set".to_string())
}

/// Resolves the path to the `arduino-cli` executable.
///>
/// Priority:
/// 1. Local executable within the `Arduino/` workspace directory.
/// 2. System PATH if no local version is found.
/// 3. Fallback to the default name for execution attempts.
///<
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

/// Resolves the library directory for a specific hardware board model.
///>
/// This ensures that the compiler is provided with the correct architecture-specific 
/// versions of shared libraries.
///<
pub fn get_library_path(project_root: &Path, board_model: &str) -> PathBuf {
    project_root.join("lib").join(board_model)
}