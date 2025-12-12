"""UI interface with dependency injection for external packages."""

from typing import List, Optional, Callable, Protocol, runtime_checkable
from ..config import PmakeConfig

# Import directly from installed packages
from outerm import (
    write_header,
    write_header_fat,
    write_boxed_header,
    error,
    warning,
    info,
    success,
    action,
)
from pyprompt import (
    select,
    confirm,
)


@runtime_checkable
class ProgressUI(Protocol):
    """Protocol for progress UI implementations."""
    
    def update(self, percent: float, stage: str, current_file: str) -> None:
        """Update progress display."""
        ...


def show_interactive_menu(config: PmakeConfig) -> Optional[str]:
    """
    Show interactive menu to select action.
    
    Args:
        config: PmakeConfig instance
        
    Returns:
        Selected action string or None
    """
    # Get folder name for title
    folder_name = config.sketch_dir.name
    title_parts = folder_name.replace("-", " ").replace("_", " ").split()
    title = " ".join(word.capitalize() for word in title_parts) + " - Action Selection"
    
    write_boxed_header(title)
    print()
    
    actions = [
        ("Build", "Compile the sketch (alias for compile)"),
        ("Compile", "Compile with verbose output"),
        ("Progress", "Compile with progress bar"),
        ("Upload", "Upload to ESP32-S3 (no compile)"),
        ("Upload_custom", "Upload with customized output"),
        ("Monitor", "Open serial monitor"),
        ("Clean", "Clean build artifacts"),
        ("All", "Compile and upload in one step"),
        ("Help", "Show help message"),
    ]
    
    choices = [f"{action[0]}" for action in actions]
    
    def get_toolbar(current_index, choices):
        description = actions[current_index][1]
        return description
    
    selected = select(
        "Select action to perform:",
        choices,
        bottom_toolbar=get_toolbar
    )
    
    if not selected:
        error("No action selected")
        return None
    
    return selected


def print_help(config: PmakeConfig) -> None:
    """
    Show help message.
    
    Args:
        config: PmakeConfig instance
    """
    write_header("ESP32-S3 Build Targets")
    print()
    info("Commands:")
    print("  python pmake2.py              - Interactive action selection menu")
    print("  python pmake2.py build        - Compile the sketch (alias for compile)")
    print("  python pmake2.py compile      - Compile with verbose output")
    print("  python pmake2.py progress     - Compile with progress bar")
    print("  python pmake2.py upload       - Upload to ESP32-S3")
    print("  python pmake2.py upload-custom - Upload with customized output")
    print("  python pmake2.py monitor      - Open serial monitor")
    print("  python pmake2.py clean        - Clean build artifacts")
    print("  python pmake2.py all          - Compile and upload")
    print("  python pmake2.py help         - Show this help message")
    print()
    info("Configuration:")
    print(f"  FQBN: {config.fqbn}")
    print(f"  PORT: {config.port}")
    print(f"  Sketch: {config.sketch_name}")

