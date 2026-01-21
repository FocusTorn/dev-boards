// Path resolution utility module

use std::path::{Path, PathBuf};

/// Find workspace root by looking for pyproject.toml with [tool.uv] or [project] sections
pub fn find_workspace_root(start_path: &Path) -> PathBuf {
    start_path
        .ancestors()
        .find(|path| {
            let pyproject = path.join("pyproject.toml");
            if pyproject.exists() {
                if let Ok(content) = std::fs::read_to_string(&pyproject) {
                    return content.contains("[tool.uv") || content.contains("[project]");
                }
            }
            false
        })
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| {
            // Fallback: go up 4 levels from start_path
            start_path
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| start_path.to_path_buf())
        })
}

/// Find project root (workspace root) - same logic as Python version
/// Assumes structure: workspace_root/projects/project_name/sketch_dir
pub fn find_project_root(sketch_dir: &Path) -> PathBuf {
    sketch_dir
        .parent()  // project_name
        .and_then(|p| p.parent())  // projects
        .and_then(|p| p.parent())  // workspace root
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| sketch_dir.to_path_buf())
}

/// Find pmake.py script in sketch directory or parent
pub fn find_pmake_script(sketch_dir: &Path) -> Option<PathBuf> {
    let pmake_script = sketch_dir.join("pmake.py");
    if pmake_script.exists() {
        return Some(pmake_script);
    }
    
    let pmake_script_parent = sketch_dir.parent().map(|p| p.join("pmake.py"));
    if let Some(parent_script) = pmake_script_parent {
        if parent_script.exists() {
            return Some(parent_script);
        }
    }
    
    None
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
