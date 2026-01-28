// Path resolution utility module

use std::env;
use std::path::{Path, PathBuf};

use crate::commands::traits::FileSystem;

/// Find workspace root using WORKSPACE_ROOT environment variable.
///>
/// This is the primary method for resolving relative paths within the 
/// heterogeneous development environment.
///<
pub fn find_workspace_root() -> Result<PathBuf, String> {
    env::var("WORKSPACE_ROOT")
        .map(PathBuf::from)
        .map_err(|_| "WORKSPACE_ROOT environment variable not set".to_string())
}

/// Resolves the absolute path to the `arduino-cli` executable.
///>
/// Checks the workspace's managed `Arduino/` directory first, falling back 
/// to the system PATH if necessary.
///<
pub fn find_arduino_cli(fs: &dyn FileSystem, env: &str, project_root: &Path) -> PathBuf {
    if env == "arduino" { //>
        let workspace_path = project_root.join("Arduino").join("arduino-cli.exe");
        if fs.exists(&workspace_path) { //>
            return workspace_path;
        } //<
        
        // Check if arduino-cli is in PATH
        if which::which("arduino-cli").is_ok() { //>
            return PathBuf::from("arduino-cli");
        } //<
        
        workspace_path
    } else {
        PathBuf::from("arduino-cli")
    } //<
}

/// Constructs the library search path for the given board model.
pub fn get_library_path(project_root: &Path, board_model: &str) -> PathBuf {
    project_root.join("lib").join(board_model)
}