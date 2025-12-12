"""Bootstrap logic separated from orchestrator."""

import sys
from pathlib import Path
from typing import Tuple, Optional


def setup_paths(script_path: Path) -> Tuple[Path, Path]:
    """
    Setup sys.path for shared-python and libs.
    
    Args:
        script_path: Path to the project-specific wrapper script
        
    Returns:
        Tuple of (shared_python_dir, libs_dir)
    """
    script_path = Path(script_path).resolve()
    
    # Find project root
    current = script_path.parent
    for _ in range(5):
        if (current / ".git").exists() or (current / "pyproject.toml").exists():
            break
        if current.parent == current:
            break
        current = current.parent
    else:
        # Fallback: look for common parent directories
        parts = script_path.parts
        if 'projects' in parts:
            idx = parts.index('projects')
            current = Path(*parts[:idx])
        elif 'lib' in parts:
            idx = parts.index('lib')
            current = Path(*parts[:idx])
        else:
            current = script_path.parent.parent.parent
    
    project_root = current
    
    shared_python_dir = project_root / "___shared" / "shared-python"
    libs_dir = project_root / "lib"
    
    # Add to sys.path if not already present
    if str(shared_python_dir) not in sys.path:
        sys.path.insert(0, str(shared_python_dir))
    
    if str(libs_dir) not in sys.path:
        sys.path.insert(0, str(libs_dir))
    
    return shared_python_dir, libs_dir

