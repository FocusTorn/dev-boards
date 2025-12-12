"""Path resolution utilities."""

from pathlib import Path
from ..exceptions import PmakeConfigError


def find_project_root(start_path: Path) -> Path:
    """
    Find project root by looking for common markers.
    
    Args:
        start_path: Starting path to search from
        
    Returns:
        Project root path
        
    Raises:
        PmakeConfigError: If project root cannot be determined
    """
    current = start_path.resolve()
    
    # Common markers for project root
    markers = ['.git', 'pyproject.toml', 'package.json', 'workspace.mdc']
    
    # Search up to 5 levels
    for _ in range(5):
        for marker in markers:
            if (current / marker).exists():
                return current
        if current.parent == current:  # Reached filesystem root
            break
        current = current.parent
    
    # Fallback: assume start_path is in projects/ or similar structure
    # Look for common parent directories
    parts = start_path.parts
    if 'projects' in parts:
        idx = parts.index('projects')
        return Path(*parts[:idx])
    elif 'lib' in parts:
        idx = parts.index('lib')
        return Path(*parts[:idx])
    
    raise PmakeConfigError(
        f"Cannot determine project root from {start_path}. "
        "Please specify project_root explicitly."
    )

