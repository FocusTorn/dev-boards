"""Upload functionality."""

import sys
import os
import re
import subprocess
from typing import Optional
from ..config import PmakeConfig
from ..core import run_arduino_cli
from ..exceptions import PmakeUploadError

# Compiled regex for upload progress
RE_WRITING_AT = re.compile(
    r'Writing at (0x[0-9a-fA-F]+).*?(\d+\.?\d*)%',
    re.IGNORECASE
)

# Try to import UI helpers
try:
    from ..ui import action, success, error  
except ImportError:
    def action(colored_text: str, non_colored_text: str = '') -> str:
        return colored_text + non_colored_text
    def success(colored_text: str, non_colored_text: str = '') -> str:
        result = f"SUCCESS: {colored_text}{non_colored_text}"
        print(result)
        return result
    def error(colored_text: str, non_colored_text: str = '') -> str:
        result = f"ERROR: {colored_text}{non_colored_text}"
        print(result, file=sys.stderr)
        return result


def upload_sketch(config: PmakeConfig, custom_output: bool = False) -> int:
    """
    Upload sketch to device.
    
    Args:
        config: PmakeConfig instance
        custom_output: Whether to use custom progress output
        
    Returns:
        Exit code (0 for success, non-zero for failure)
    """
    if custom_output:
        return upload_sketch_custom(config)
    
    try:
        from ..ui import write_header_fat  
    except ImportError:
        from typing import Any
        def write_header_fat(title: str, width: Optional[int] = None, use_bold: bool = True, start_region: bool = True, footer: bool = False) -> Any:  # type: ignore[misc]
            print(f"=== {title} ===")
            return None
    
    # write_header_fat returns a context manager, but we're not using it here
    _ = write_header_fat("Uploading")
    print()
    print(action(f"Uploading to ESP32-S3 on {config.port}..."))
    print()
    
    args = [
        "upload",
        "--port", config.port,
        "--fqbn", config.fqbn,
        "--input-dir", str(config.build_path),
    ]
    
    try:
        result = run_arduino_cli(config, args)
        
        if result.returncode == 0:
            print()
            success("Upload successful!")
        else:
            print()
            error("Upload failed!")
        
        return result.returncode
    except PmakeUploadError as e:
        error(f"Upload error: {e}")
        return e.returncode or 1


def upload_sketch_custom(config: PmakeConfig) -> int:
    """
    Upload with custom progress output.
    
    Args:
        config: PmakeConfig instance
        
    Returns:
        Exit code (0 for success, non-zero for failure)
    """
    try:
        from alive_progress import alive_bar  
    except ImportError:
        error("alive_progress not available. Please install alive-progress.")
        return 1
    
    print()
    print(action(f"Uploading to ESP32-S3 on {config.port}..."))
    print()
    
    cmd = [
        str(config.arduino_cli_path),
        "upload",
        "-p", config.port,
        "--fqbn", config.fqbn,
        "--build-path", str(config.build_path),
        str(config.sketch_dir)
    ]
    
    # Force terminal width
    os.environ["COLUMNS"] = "160"
    
    try:
        process = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1,
            universal_newlines=True,
        )
        
        current_bar = None
        current_bar_cm = None
        flash_count = 0
        
        stdout = process.stdout
        if stdout is None:
            return 1  # Return error code if stdout is not available
        
        while True:
            line = stdout.readline()
            if not line and process.poll() is not None:
                break
            
            if line:
                line_strip = line.strip()
                
                # Suppress "Hash of data verified"
                if "Hash of data verified" in line:
                    continue
                
                # Suppress "Compressed" lines after first block
                if "Compressed" in line and "bytes to" in line:
                    if flash_count > 0:
                        continue
                
                # Handle "Writing at" lines - Progress
                if "Writing at" in line:
                    match = RE_WRITING_AT.search(line)
                    if match:
                        addr = match.group(1)
                        percent = float(match.group(2))
                        
                        if current_bar is None:
                            print()
                            flash_count += 1
                            current_bar_cm = alive_bar(
                                manual=True,
                                title=f"Writing at {addr}",
                                bar="smooth",
                                spinner="dots",
                                monitor=False,
                                stats=True,
                                receipt=False
                            )
                            current_bar = current_bar_cm.__enter__()
                        
                        current_bar.title(f"Writing at {addr}")
                        current_bar(percent / 100.0)
                        continue
                
                # Handle "Wrote" lines
                elif "Wrote" in line and "compressed" in line:
                    if current_bar and current_bar_cm is not None:
                        current_bar_cm.__exit__(None, None, None)
                        current_bar = None
                        current_bar_cm = None
                    
                    sys.stdout.write("\033[F")
                    sys.stdout.write("\033[K")
                    print(line_strip)
                    continue
                
                # Handle "Hard resetting"
                if "Hard resetting" in line:
                    print()
                    print(line_strip)
                    continue
                
                if current_bar:
                    if not line_strip:
                        continue
                else:
                    if flash_count == 0:
                        print(line.rstrip())
                    else:
                        if line_strip:
                            print(line_strip)
        
        return_code = process.wait()
        
        if current_bar_cm:
            current_bar_cm.__exit__(None, None, None)
        
        if return_code == 0:
            print()
            success("Upload successful!")
            return 0
        else:
            print()
            error("Upload failed!")
            return 1
    
    except Exception as e:
        error(f"Error during upload: {e}")
        return 1


def monitor_serial(config: PmakeConfig) -> None:
    """
    Open serial monitor.
    
    Args:
        config: PmakeConfig instance
    """
    try:
        from ..ui import action  
    except ImportError:
        def action(colored_text: str, non_colored_text: str = '') -> str:
            return colored_text + non_colored_text
    
    print(action(f"Opening serial monitor on {config.port} at {config.baudrate} baud..."))
    args = [
        "monitor",
        "--port", config.port,
        "--config", f"baudrate={config.baudrate}",
    ]
    
    run_arduino_cli(config, args)

