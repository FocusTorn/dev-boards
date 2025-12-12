"""Arduino CLI execution with proper output handling."""

import sys
import subprocess
import threading
from pathlib import Path
from typing import List, Optional, Callable
from ..config import PmakeConfig
from ..exceptions import PmakeBuildError


def run_arduino_cli(
    config: PmakeConfig, 
    args: List[str], 
    capture_output: bool = False, 
    output_handler: Optional[Callable[[str], Optional[str]]] = None,
    timeout: Optional[int] = None
) -> subprocess.CompletedProcess:
    """
    Run Arduino CLI command with proper output handling.
    
    Args:
        config: PmakeConfig instance
        args: Command arguments
        capture_output: Whether to capture output
        output_handler: Optional handler for each output line
        timeout: Optional timeout in seconds
        
    Returns:
        CompletedProcess with return code and output
        
    Raises:
        PmakeBuildError: If Arduino CLI execution fails
    """
    cmd = [str(config.arduino_cli_path)] + args
    
    # Ensure arduino-cli exists
    if not config.arduino_cli_path.exists():
        raise PmakeBuildError(
            f"Arduino CLI not found at: {config.arduino_cli_path}",
            returncode=1
        )

    try:
        if capture_output:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                check=False,
                timeout=timeout,
            )
        else:
            # Disable terminal line wrapping to prevent output from wrapping
            DISABLE_WRAP = '\x1b[?7l'
            ENABLE_WRAP = '\x1b[?7h'
            
            # Get the original stream
            original_stdout = sys.stdout
            original_stderr = sys.stderr
            if hasattr(sys.stdout, 'original_stream'):
                # Type narrowing: use getattr after hasattr check
                original_stdout = getattr(sys.stdout, 'original_stream', sys.stdout)
            
            # Disable wrapping
            original_stdout.write(DISABLE_WRAP)
            original_stdout.flush()
            original_stderr.write(DISABLE_WRAP)
            original_stderr.flush()
            
            try:
                process = subprocess.Popen(
                    cmd,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    text=True,
                    bufsize=1,
                    universal_newlines=True,
                )
                
                output_lines = []
                error_lines = []
                
                def read_stdout():
                    stdout = process.stdout
                    if stdout is None:
                        return
                    for line in iter(stdout.readline, ''):
                        if line:
                            if output_handler:
                                processed = output_handler(line)
                                if processed is not None:
                                    sys.stdout.write(processed)
                                    sys.stdout.flush()
                            else:
                                sys.stdout.write(line)
                                sys.stdout.flush()
                            output_lines.append(line)
                    if process.stdout:
                        process.stdout.close()
                
                def read_stderr():
                    stderr = process.stderr
                    if stderr is None:
                        return
                    for line in iter(stderr.readline, ''):
                        if line:
                            sys.stderr.write(line)
                            sys.stderr.flush()
                            error_lines.append(line)
                    if process.stderr:
                        process.stderr.close()
                
                stdout_thread = threading.Thread(target=read_stdout, daemon=True)
                stderr_thread = threading.Thread(target=read_stderr, daemon=True)
                
                stdout_thread.start()
                stderr_thread.start()
                
                return_code = process.wait(timeout=timeout)
                
                stdout_thread.join(timeout=1.0)
                stderr_thread.join(timeout=1.0)
                
                result = subprocess.CompletedProcess(
                    cmd,
                    return_code,
                    stdout=''.join(output_lines),
                    stderr=''.join(error_lines),
                )
            finally:
                # Re-enable wrapping
                original_stdout.write(ENABLE_WRAP)
                original_stdout.flush()
                original_stderr.write(ENABLE_WRAP)
                original_stderr.flush()
        return result
    except subprocess.TimeoutExpired as e:
        raise PmakeBuildError(
            f"Arduino CLI command timed out after {timeout} seconds",
            returncode=1
        ) from e
    except FileNotFoundError as e:
        raise PmakeBuildError(
            f"Arduino CLI not found: {e}",
            returncode=1
        ) from e

