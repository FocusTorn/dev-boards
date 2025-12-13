#!/usr/bin/env python3
"""
Cross-platform installer script for py-makefile.
Installs using UV if available, otherwise falls back to pip.
"""

import sys
import os
import subprocess
from pathlib import Path

# Get workspace root from environment variable
root_path = os.environ.get('WORKSPACE_ROOT')
if not root_path:
    raise RuntimeError("WORKSPACE_ROOT environment variable not set")
_workspace_root = Path(root_path)

# Import terminal output utilities from outerm package
from outerm import error, warning, info, success  


def check_command(cmd: str) -> bool:
    """Check if a command is available."""
    try:
        subprocess.run([cmd, "--version"], capture_output=True, check=True)
        return True
    except (subprocess.CalledProcessError, FileNotFoundError):
        return False


def has_uv_workspace_venv(workspace_root: Path) -> bool:
    """Check if UV workspace virtual environment exists."""
    venv_path = workspace_root / ".venv"
    if venv_path.exists() and (venv_path / "pyvenv.cfg").exists():
        return True
    return False


def install_with_uv(package_path: Path, workspace_root: Path) -> bool:
    """Install package using UV (UV doesn't support --user flag).
    
    UV requires a virtual environment. If workspace venv exists, installs there.
    Otherwise, falls back to pip with --user.
    """
    # Check if workspace venv exists
    if not has_uv_workspace_venv(workspace_root):
        print(f"  {info('UV workspace venv not found, skipping UV...', '')}")
        return False
    
    print(f"  {info('Installing with UV (workspace venv)...', '')}")
    try:
        result = subprocess.run(
            ["uv", "pip", "install", "-e", str(package_path)],
            check=True
        )
        print(success('Successfully installed with UV', ''))
        print(warning('Note: Package installed to workspace venv. Use "uv run python pmake.py" or activate venv.', ''))
        return True
    except subprocess.CalledProcessError as e:
        print(error(f'UV installation failed: {e}', ''), file=sys.stderr)
        return False


def install_with_pip(package_path: Path) -> bool:
    """Install package using pip with --user flag (recommended for development)."""
    print(f"  {info('Installing with pip (--user)...', '')}")
    try:
        result = subprocess.run(
            [sys.executable, "-m", "pip", "install", "-e", str(package_path), "--user"],
            check=True
        )
        print(success('Successfully installed with pip', ''))
        return True
    except subprocess.CalledProcessError as e:
        print(error(f'pip installation failed: {e}', ''), file=sys.stderr)
        return False


def main():
    """Main installation function."""
    script_dir = Path(__file__).parent.resolve()
    package_path = script_dir
    
    # Check if pyproject.toml exists
    if not (package_path / "pyproject.toml").exists():
        print(error(f'pyproject.toml not found in {package_path}', ''), file=sys.stderr)
        return 1
    
    print(f"  {info(f'Installing py-makefile from: {package_path}', '')}")
    print()
    
    # Check for UV first (preferred package manager)
    # Note: UV doesn't support --user flag, so we only use it if workspace venv exists
    # For --user installations (development packages), we use pip
    if check_command("uv"):
        if has_uv_workspace_venv(_workspace_root):
            print(f"  {info('UV detected with workspace venv, using UV for installation...', '')}")
            if install_with_uv(package_path, _workspace_root):
                return 0
            print()
            print(warning('UV installation failed, falling back to pip...', ''))
            print()
        else:
            print(f"  {info('UV detected but no workspace venv found. UV requires venv (does not support --user).', '')}")
            print(f"  {info('Using pip with --user for development installation...', '')}")
            print()
    
    # Fall back to pip
    if check_command("pip"):
        if install_with_pip(package_path):
            return 0
    
    print(error('Installation failed with both UV and pip', ''), file=sys.stderr)
    return 1


if __name__ == "__main__":
    sys.exit(main())

