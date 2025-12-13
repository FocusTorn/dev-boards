#!/usr/bin/env python3
"""
Cross-platform uninstaller script for py-makefile.
Uninstalls from UV workspace venv if available, otherwise from pip.
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


def is_package_installed(package_name: str, use_uv: bool = False) -> bool:
    """Check if package is installed."""
    try:
        if use_uv:
            result = subprocess.run(
                ["uv", "pip", "list"],
                capture_output=True,
                text=True,
                check=True
            )
            return package_name.lower() in result.stdout.lower()
        else:
            result = subprocess.run(
                [sys.executable, "-m", "pip", "list"],
                capture_output=True,
                text=True,
                check=True
            )
            return package_name.lower() in result.stdout.lower()
    except (subprocess.CalledProcessError, FileNotFoundError):
        return False


def uninstall_with_uv(package_name: str) -> bool:
    """Uninstall package using UV."""
    print(f"  {info('Uninstalling with UV (workspace venv)...', '')}")
    try:
        result = subprocess.run(
            ["uv", "pip", "uninstall", "-y", package_name],
            check=True
        )
        print(success('Successfully uninstalled with UV', ''))
        return True
    except subprocess.CalledProcessError as e:
        error_output = e.stderr.decode() if e.stderr else e.stdout.decode() if e.stdout else ""
        if "not installed" in error_output.lower() or "not found" in error_output.lower():
            print(f"  {info('Package not found in UV (may have been already removed)', '')}")
            return True
        print(error(f'UV uninstallation failed: {e}', ''), file=sys.stderr)
        return False


def uninstall_with_pip(package_name: str) -> bool:
    """Uninstall package using pip."""
    print(f"  {info('Uninstalling with pip...', '')}")
    try:
        result = subprocess.run(
            [sys.executable, "-m", "pip", "uninstall", "-y", package_name],
            check=True
        )
        print(success('Successfully uninstalled with pip', ''))
        return True
    except subprocess.CalledProcessError as e:
        error_output = e.stderr.decode() if e.stderr else e.stdout.decode() if e.stdout else ""
        if "not installed" in error_output.lower() or "not found" in error_output.lower():
            print(f"  {info('Package not found in pip (may have been already removed)', '')}")
            return True
        print(error(f'pip uninstallation failed: {e}', ''), file=sys.stderr)
        return False


def main():
    """Main uninstallation function."""
    package_name = "py-makefile"
    
    print(f"  {info(f'Uninstalling {package_name}...', '')}")
    print()
    
    uv_available = check_command("uv")
    uv_installed = False
    pip_installed = False
    
    # Check where package is installed
    if uv_available and has_uv_workspace_venv(_workspace_root):
        uv_installed = is_package_installed(package_name, use_uv=True)
        if uv_installed:
            print(f"  {info('Package found in UV workspace venv', '')}")
    
    pip_installed = is_package_installed(package_name, use_uv=False)
    if pip_installed:
        print(f"  {info('Package found in pip environment', '')}")
    
    if not uv_installed and not pip_installed:
        print(f"  {info(f'Package {package_name} is not installed in either UV or pip', '')}")
        return 0
    
    print()
    uv_uninstalled = False
    pip_uninstalled = False
    
    # Try UV first if available and package is installed there
    if uv_available and has_uv_workspace_venv(_workspace_root) and uv_installed:
        if uninstall_with_uv(package_name):
            uv_uninstalled = True
        print()
    
    # Always try pip (even if UV succeeded, in case it's installed in both)
    if pip_installed:
        if uninstall_with_pip(package_name):
            pip_uninstalled = True
        print()
    
    if uv_uninstalled or pip_uninstalled:
        print(success('Uninstallation complete', ''))
        return 0
    elif (uv_available and has_uv_workspace_venv(_workspace_root) and uv_installed) or pip_installed:
        print(warning('Uninstallation attempted but may not have been fully successful', ''), file=sys.stderr)
        return 1
    else:
        print(success('Uninstallation complete', ''))
        return 0


if __name__ == "__main__":
    sys.exit(main())

