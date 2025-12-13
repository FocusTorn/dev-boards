"""CLI orchestrator - handles all command routing and execution."""

import sys
import shutil
from pathlib import Path
from typing import Optional
from .config import PmakeConfig
from .build import compile_sketch, compile_sketch_progress
from .upload import upload_sketch, upload_sketch_custom, monitor_serial
from .ui import show_interactive_menu, print_help
from .exceptions import PmakeConfigError, PmakeBuildError

# Import UI helpers from outerm
from outerm import error, success, info, warning


def run(config: PmakeConfig, args: Optional[list[str]] = None) -> int:
    """
    Run py-makefile with given config and arguments.
    
    Args:
        config: PmakeConfig instance
        args: Optional argument list (defaults to sys.argv[1:])
        
    Returns:
        Exit code (0 for success, non-zero for failure)
    """
    if args is None:
        args = sys.argv[1:] if len(sys.argv) > 1 else []
    
    # Parse Arguments
    if len(args) > 0:
        action_name = args[0].lower()
    else:
        # Interactive Mode
        result = show_interactive_menu(config)
        if not result:
            return 0
        action_name = result.split(" ")[0].lower()  # Handle "Build - Description"
    
    # Execute Action
    try:
        if action_name in ["build", "compile"]:
            # "Build" is alias for compile, default verbose=False unless explicit or "compile" action
            verbose = "--verbose" in args or action_name == "compile"
            return compile_sketch(config, verbose=verbose)
        
        elif action_name == "progress":
            return compile_sketch_progress(config)
        
        elif action_name == "upload":
            return upload_sketch(config)
        
        elif action_name == "upload_custom" or action_name == "upload-custom":
            return upload_sketch_custom(config)
        
        elif action_name == "monitor":
            monitor_serial(config)
            return 0
        
        elif action_name == "clean":
            if config.build_path.exists():
                try:
                    shutil.rmtree(config.build_path)
                    print(success("Build directory cleaned."))
                except Exception as e:
                    print(error(f"Failed to clean build directory: {e}"))
                    return 1
            else:
                print(info("Build directory does not exist."))
            return 0
        
        elif action_name == "all":
            ret = compile_sketch(config, verbose=True)
            if ret == 0:
                return upload_sketch(config)
            return ret
        
        elif action_name == "help":
            print_help(config)
            return 0
        
        else:
            print(error(f"Unknown action: {action_name}"))
            print_help(config)
            return 1
    
    except PmakeBuildError as e:
        print(error(f"Build error: {e}"))
        return e.returncode or 1
    except KeyboardInterrupt:
        print()
        print(info("Interrupted by user"))
        return 130
    except Exception as e:
        print(error(f"Unexpected error: {e}"))
        return 1


def main() -> int:
    """
    Main entry point - expects config to be passed or created externally.
    
    This is typically called from a project-specific wrapper script.
    """
    # This would need config to be passed in - for standalone use, 
    # call run() with a properly configured PmakeConfig
    raise NotImplementedError(
        "main() requires a PmakeConfig. Use run(config) or create a project-specific wrapper."
    )

