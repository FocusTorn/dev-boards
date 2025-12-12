#!/usr/bin/env python3
"""
Pmake Orchestrator
Python-based build script using libs/pmake.
"""

import sys
import os
import shutil
from pathlib import Path

# --- BOOTSTRAP: Add shared-python and libs to sys.path ---
# Get current script directory
_script_dir = Path(__file__).parent.resolve()
# Project root (assumed to be 4 levels up: projects/esp32-s3__LB-Gold/sht21-solo/ -> root)
# But wait, config says: D:/_dev/_projects/dev-boards/projects/esp32-s3__LB-Gold/sht21-solo
# So root is D:/_dev/_projects/dev-boards
# _script_dir = .../sht21-solo
# parent = .../esp32-s3__LB-Gold
# parent.parent = .../projects
# parent.parent.parent = .../dev-boards (Root)

PROJECT_ROOT = _script_dir.parent.parent.parent
SHARED_PYTHON_DIR = PROJECT_ROOT / "___shared" / "shared-python"
LIBS_DIR = PROJECT_ROOT / "lib"

if str(SHARED_PYTHON_DIR) not in sys.path:
    sys.path.insert(0, str(SHARED_PYTHON_DIR))
    
if str(LIBS_DIR) and str(LIBS_DIR) not in sys.path:
    # Insert libs so we can import pmake
    sys.path.insert(0, str(LIBS_DIR))

# --- IMPORTS ---
try:
    from pmake import PmakeConfig
    from pmake.build import compile_sketch, compile_sketch_progress, ProgressMonitor, verify_sketch
    from pmake.upload import upload_sketch, upload_sketch_custom, monitor_serial
    from pmake.ui import show_interactive_menu, print_help
    from pmake.core import run_arduino_cli
    
    from local_imports import error, success, info, warning
except ImportError as e:
    print(f"Error importing required modules: {e}")
# --- CONFIGURATION ---
# These values are specific to this sketch/project instance
FQBN = "esp32:esp32:esp32s3"
SKETCH_NAME = "sht21-solo.ino"
PORT = "COM9"
BAUDRATE = 115200
CREATE_LOG = False

def main():
    # Initialize Config
    config = PmakeConfig(
        project_root=PROJECT_ROOT,
        sketch_dir=_script_dir,
        arduino_cli_path=PROJECT_ROOT / "Arduino/arduino-cli.exe",
        fqbn=FQBN,
        sketch_name=SKETCH_NAME,
        port=PORT,
        baudrate=BAUDRATE,
        create_log=CREATE_LOG,
        # Library path can be overridden if needed, defaulting to standard
    )
    
    # Verify config paths
    if not config.arduino_cli_path.exists():
        error(f"Arduino CLI not found at: {config.arduino_cli_path}")
        sys.exit(1)

    # Clean legacy build path/make sure build path is set
    # Original script: BUILD_PATH = SKETCH_DIR / "build"
    # Config default: build_path property handles this.
    
    # Parse Arguments
    if len(sys.argv) > 1:
        action_name = sys.argv[1].lower()
    else:
        # Interactive Mode
        result = show_interactive_menu(config)
        if not result:
            sys.exit(0)
        action_name = result.split(" ")[0].lower() # Handle "Build - Description"

    # Execute Action
    if action_name in ["build", "compile"]:
        # "Build" is alias for compile, default verbose=False unless explicit or "compile" action
        verbose = "--verbose" in sys.argv or action_name == "compile"
        sys.exit(compile_sketch(config, verbose=verbose))
        
    elif action_name == "progress":
        sys.exit(compile_sketch_progress(config))
        
    elif action_name == "upload":
        sys.exit(upload_sketch(config))
        
    elif action_name == "upload_custom" or action_name == "upload-custom":
        sys.exit(upload_sketch_custom(config))

        
    elif action_name == "monitor":
        monitor_serial(config)
        
    elif action_name == "clean":
        if config.build_path.exists():
            try:
                shutil.rmtree(config.build_path)
                success("Build directory cleaned.")
            except Exception as e:
                error(f"Failed to clean build directory: {e}")
        else:
            info("Build directory does not exist.")
            
    elif action_name == "all":
        ret = compile_sketch(config, verbose=True)
        if ret == 0:
            sys.exit(upload_sketch(config))
        sys.exit(ret)
        
    elif action_name == "help":
        print_help(config)
        
    else:
        error(f"Unknown action: {action_name}")
        print_help(config)
        sys.exit(1)

if __name__ == "__main__":
    main()
