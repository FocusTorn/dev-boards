#!/usr/bin/env python3
"""
Pmake2 - Thin project-specific wrapper
Passes project configuration to pmake2 package orchestrator.
"""

# Project-specific configuration
FQBN = "esp32:esp32:esp32s3"
SKETCH_NAME = "sht21-solo.ino"
PORT = "COM9"
BAUDRATE = 115200
CREATE_LOG = False


import sys
from pathlib import Path

# Import py_makefile package
try:
    from py_makefile import PmakeConfig, run
    from py_makefile.exceptions import PmakeConfigError
except ImportError as e:
    print(f"Error: py-makefile not found. Install with: pip install -e lib/py_makefile")
    print(f"Import error: {e}")
    sys.exit(1)


def main() -> int:
    """Create config and run py-makefile orchestrator."""
    script_path = Path(__file__).resolve()
    
    try:
        # Use from_script_path for automatic project root detection
        config = PmakeConfig.from_script_path(
            script_path=script_path,
            arduino_cli_path=script_path.parent.parent.parent.parent / "Arduino" / "arduino-cli.exe",
            fqbn=FQBN,
            sketch_name=SKETCH_NAME,
            port=PORT,
            baudrate=BAUDRATE,
            create_log=CREATE_LOG,
        )
    except PmakeConfigError as e:
        print(f"Configuration error: {e}")
        return 1
    
    # Pass everything to package orchestrator
    return run(config)


if __name__ == "__main__":
    sys.exit(main())
