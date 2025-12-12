#!/usr/bin/env python3
"""
Interactive sketch selector for ESP32-S3 BME680 project.

Uses pyprompt to show an interactive menu for selecting which sketch to compile.
Outputs the selected sketch name (without .ino extension) to stdout for use by Makefile.
"""

import sys
from pathlib import Path

# Add shared-python directory to sys.path
_script_dir = Path(__file__).parent.resolve()
_shared_python_dir = _script_dir.parent.parent.parent / "___shared" / "shared-python"

if str(_shared_python_dir) not in sys.path:
    sys.path.insert(0, str(_shared_python_dir))

# Import from local_imports
from local_imports import (  
    select,
    HAS_PROMPT_TOOLKIT,
    write_header,
    error,
    warning,
    info,
)

# Sketch definitions
SKETCHES = [
    {
        "name": "bme680",
        "display": "bme680.ino - Full version (with calculations)",
        "description": "Includes heatsoak and IAQ calculations on ESP32",
    },
    {
        "name": "bme680-simplified",
        "display": "bme680-simplified.ino - Simplified (raw data only)",
        "description": "Publishes raw sensor data only, calculations done on RPi",
    },
]


def find_available_sketches(sketch_dir: Path) -> list:
    """Find all available .ino files in the sketch directory."""
    available = []
    for sketch_info in SKETCHES:
        sketch_file = sketch_dir / f"{sketch_info['name']}.ino"
        if sketch_file.exists():
            available.append(sketch_info)
    return available


def select_sketch_interactive() -> str:
    """Show interactive menu and return selected sketch name."""
    sketch_dir = Path(__file__).parent.resolve()
    available_sketches = find_available_sketches(sketch_dir)
    
    if not available_sketches:
        error("No sketch files found!")
        sys.exit(1)
    
    if not HAS_PROMPT_TOOLKIT:
        warning("prompt_toolkit not available, falling back to default sketch")
        return available_sketches[0]["name"]
    
    # Show header
    write_header("ESP32-S3 BME680 - Sketch Selection")
    print()
    
    # Create choices list
    choices = [sketch["display"] for sketch in available_sketches]
    
    # Show selection menu
    selected_display = select(
        "Select sketch to compile:",
        choices,
        pointer=" »",
    )
    
    if not selected_display:
        # User cancelled or no selection
        error("No sketch selected")
        sys.exit(1)
    
    # Find the sketch name from the selected display
    for sketch in available_sketches:
        if sketch["display"] == selected_display:
            return sketch["name"]
    
    # Fallback (shouldn't happen)
    error(f"Selected sketch not found: {selected_display}")
    sys.exit(1)


def main():
    """Main entry point - outputs selected sketch name to stdout."""
    try:
        selected_sketch = select_sketch_interactive()
        # Output to stdout (without newline for Makefile)
        print(selected_sketch, end="")
        return 0
    except KeyboardInterrupt:
        print("", file=sys.stderr)  # New line after ^C
        error("Selection cancelled by user")
        return 1
    except Exception as e:
        error(f"Error during sketch selection: {e}")
        return 1


if __name__ == "__main__":
    sys.exit(main())

