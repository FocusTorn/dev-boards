#!/usr/bin/env python3
"""
Add a username and password to Mosquitto MQTT broker.
Multiplatform Python version (Windows/Debian/Linux).

Usage: add-mqtt-user.py <username> [password] [--full-setup]
  If password is not provided, it will be prompted securely.
  --full-setup: On Debian/Linux, attempt to install pyprompt if not available.
"""

import sys
import os
import subprocess
import platform
import argparse
from pathlib import Path
from typing import Optional

# Fix Windows console encoding for Unicode characters
if sys.platform == "win32":
    import io
    if hasattr(sys.stdout, 'reconfigure'):
        sys.stdout.reconfigure(encoding='utf-8')
        sys.stderr.reconfigure(encoding='utf-8')
    else:
        sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
        sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')

# Platform detection
def is_windows() -> bool:
    """Check if running on Windows."""
    return platform.system() == "Windows"

def is_debian() -> bool:
    """Check if running on Debian-based Linux."""
    if is_windows():
        return False
    try:
        with open('/etc/os-release', 'r') as f:
            return 'debian' in f.read().lower() or 'ubuntu' in f.read().lower()
    except (FileNotFoundError, IOError):
        return False

# pyprompt availability check and installation
HAS_PYPROMPT = False
PYPROMPT_AVAILABLE = False

def check_pyprompt_available() -> bool:
    """Check if pyprompt is available."""
    try:
        import pyprompt
        return True
    except ImportError:
        return False

def check_command(cmd: str) -> bool:
    """Check if a command exists in PATH."""
    try:
        if is_windows():
            result = subprocess.run(
                ["where", cmd] if cmd != "git" else ["git", "version"],
                capture_output=True,
                timeout=5,
                check=False
            )
            return result.returncode == 0
        else:
            result = subprocess.run(
                ["which", cmd],
                capture_output=True,
                timeout=5,
                check=False
            )
            return result.returncode == 0
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return False

def install_pyprompt_debian() -> bool:
    """Install pyprompt on Debian/Linux systems."""
    if is_windows():
        return False
    
    print("Installing pyprompt...")
    try:
        # Try UV first (preferred)
        if check_command("uv"):
            result = subprocess.run(
                ["uv", "pip", "install", "pyprompt"],
                capture_output=True,
                text=True,
                check=False
            )
            if result.returncode == 0:
                print("✓ pyprompt installed with UV")
                return True
        
        # Fallback to pip
        if check_command("pip"):
            result = subprocess.run(
                [sys.executable, "-m", "pip", "install", "pyprompt"],
                capture_output=True,
                text=True,
                check=False
            )
            if result.returncode == 0:
                print("✓ pyprompt installed with pip")
                return True
        
        print("✗ Failed to install pyprompt")
        return False
    except Exception as e:
        print(f"✗ Error installing pyprompt: {e}")
        return False

def ensure_pyprompt(full_setup: bool = False) -> bool:
    """
    Ensure pyprompt is available.
    
    Args:
        full_setup: If True, attempt to install pyprompt on Debian if not available.
                    On Windows, this is ignored (pyprompt must be pre-installed).
    
    Returns:
        True if pyprompt is available, False otherwise.
    """
    global HAS_PYPROMPT, PYPROMPT_AVAILABLE
    
    if check_pyprompt_available():
        HAS_PYPROMPT = True
        PYPROMPT_AVAILABLE = True
        return True
    
    # pyprompt not available
    if is_windows():
        print("ERROR: pyprompt is not installed.")
        print()
        print("On Windows, pyprompt must be installed prior to running this script.")
        print("Please install it using:")
        print("  uv pip install pyprompt")
        print("  or")
        print("  pip install pyprompt")
        return False
    
    # On Debian/Linux, optionally install if full_setup is True
    if is_debian() and full_setup:
        print("pyprompt not found. Attempting to install...")
        if install_pyprompt_debian():
            # Try importing again
            if check_pyprompt_available():
                HAS_PYPROMPT = True
                PYPROMPT_AVAILABLE = True
                return True
    
    # pyprompt not available and couldn't install
    print("ERROR: pyprompt is not available.")
    if is_debian():
        print()
        print("Please install pyprompt using:")
        print("  uv pip install pyprompt")
        print("  or")
        print("  pip install pyprompt")
        print()
        print("Or run this script with --full-setup to attempt automatic installation.")
    
    return False

# Terminal output functions
def error(msg: str) -> str:
    """Print error message."""
    return f"❌ {msg}"

def success(msg: str) -> str:
    """Print success message."""
    return f"✅ {msg}"

def info(msg: str) -> str:
    """Print info message."""
    return f"ℹ️  {msg}"

def warning(msg: str) -> str:
    """Print warning message."""
    return f"⚠️  {msg}"

# Try to import outerm for better terminal output
try:
    from outerm import error, warning, info, success
except ImportError:
    pass  # Use fallback functions above

# Platform-specific paths
def get_mosquitto_paths() -> dict:
    """Get platform-specific Mosquitto paths."""
    if is_windows():
        # Windows paths (adjust as needed)
        return {
            'passwd_file': Path("C:/mosquitto/passwd"),
            'conf_file': Path("C:/mosquitto/mosquitto.conf"),
            'mosquitto_passwd': Path("C:/mosquitto/mosquitto_passwd.exe"),
        }
    else:
        # Linux/Debian paths
        return {
            'passwd_file': Path("/etc/mosquitto/passwd"),
            'conf_file': Path("/etc/mosquitto/mosquitto.conf"),
            'mosquitto_passwd': Path("/usr/bin/mosquitto_passwd"),
        }

# Root check
def check_root() -> None:
    """Check if running as root (Linux/Debian) or admin (Windows)."""
    if is_windows():
        # On Windows, check for admin privileges
        import ctypes
        if not ctypes.windll.shell32.IsUserAnAdmin():
            print(error("This script must be run as administrator on Windows."))
            sys.exit(1)
    else:
        if os.geteuid() != 0:
            print(error("This script must be run as root or with sudo"))
            sys.exit(1)

# Password prompting
def prompt_password() -> Optional[str]:
    """Prompt for password using pyprompt or fallback."""
    if PYPROMPT_AVAILABLE:
        try:
            from pyprompt import text
            password = text("Enter password: ", default="")
            return password
        except ImportError:
            pass
    
    # Fallback to getpass
    import getpass
    return getpass.getpass("Enter password: ")

def prompt_confirm(message: str, default: bool = True) -> bool:
    """Prompt for confirmation using pyprompt or fallback."""
    if PYPROMPT_AVAILABLE:
        try:
            from pyprompt import confirm
            result = confirm(message, default=default)
            return result if result is not None else False
        except ImportError:
            pass
    
    # Fallback to input
    default_text = "[Y/n]" if default else "[y/N]"
    response = input(f"{message} {default_text}: ").strip().lower()
    if not response:
        return default
    return response in ('y', 'yes')

# Main function
def add_user(username: str, password: Optional[str] = None, full_setup: bool = False) -> int:
    """Add or update MQTT user."""
    # Ensure pyprompt is available
    if not ensure_pyprompt(full_setup=full_setup):
        return 1
    
    # Check root/admin
    check_root()
    
    paths = get_mosquitto_paths()
    
    # Check if mosquitto_passwd is available
    if not paths['mosquitto_passwd'].exists() and not check_command("mosquitto_passwd"):
        print(error(f"mosquitto_passwd not found at {paths['mosquitto_passwd']}"))
        if not is_windows():
            print("Please install mosquitto-clients package first")
        return 1
    
    # Use system mosquitto_passwd if available
    mosquitto_passwd_cmd = "mosquitto_passwd" if check_command("mosquitto_passwd") else str(paths['mosquitto_passwd'])
    
    # Check if password file exists, create if it doesn't
    if not paths['passwd_file'].exists():
        print(info(f"Creating password file at {paths['passwd_file']}..."))
        try:
            paths['passwd_file'].touch()
            if not is_windows():
                # Set ownership and permissions (Linux/Debian only)
                subprocess.run(["chown", "root:mosquitto", str(paths['passwd_file'])], check=False)
                subprocess.run(["chmod", "640", str(paths['passwd_file'])], check=False)
        except Exception as e:
            print(error(f"Failed to create password file: {e}"))
            return 1
    
    # Check if user already exists
    user_exists = False
    if paths['passwd_file'].exists():
        try:
            with open(paths['passwd_file'], 'r') as f:
                for line in f:
                    if line.startswith(f"{username}:"):
                        user_exists = True
                        break
        except IOError:
            pass
    
    if user_exists:
        print(warning(f"User '{username}' already exists in password file"))
        if not prompt_confirm("Update password?", default=True):
            print(info("Cancelled"))
            return 0
        
        # Update existing user
        if password:
            # Password provided as argument
            result = subprocess.run(
                [mosquitto_passwd_cmd, "-b", str(paths['passwd_file']), username],
                input=password,
                text=True,
                capture_output=True,
                check=False
            )
        else:
            # Prompt for password
            password = prompt_password()
            if not password:
                print(error("Password is required"))
                return 1
            result = subprocess.run(
                [mosquitto_passwd_cmd, str(paths['passwd_file']), username],
                input=password,
                text=True,
                capture_output=True,
                check=False
            )
        
        if result.returncode == 0:
            print(success(f"Password updated for user '{username}'"))
        else:
            print(error(f"Failed to update password: {result.stderr}"))
            return 1
    else:
        # Add new user
        if password:
            # Password provided as argument
            result = subprocess.run(
                [mosquitto_passwd_cmd, "-b", str(paths['passwd_file']), username],
                input=password,
                text=True,
                capture_output=True,
                check=False
            )
        else:
            # Prompt for password
            password = prompt_password()
            if not password:
                print(error("Password is required"))
                return 1
            result = subprocess.run(
                [mosquitto_passwd_cmd, "-b", str(paths['passwd_file']), username],
                input=password,
                text=True,
                capture_output=True,
                check=False
            )
        
        if result.returncode == 0:
            print(success(f"User '{username}' added to password file"))
        else:
            print(error(f"Failed to add user: {result.stderr}"))
            return 1
    
    # Ensure proper permissions (Linux/Debian only)
    if not is_windows() and paths['passwd_file'].exists():
        subprocess.run(["chown", "root:mosquitto", str(paths['passwd_file'])], check=False)
        subprocess.run(["chmod", "640", str(paths['passwd_file'])], check=False)
    
    # Restart mosquitto service (Linux/Debian only)
    if not is_windows():
        print(info("Restarting mosquitto service..."))
        result = subprocess.run(
            ["systemctl", "is-active", "--quiet", "mosquitto"],
            capture_output=True
        )
        if result.returncode == 0:
            subprocess.run(["systemctl", "restart", "mosquitto"], check=False)
            print(success("Mosquitto service restarted"))
        else:
            print(warning("Mosquitto service is not running"))
            print("Start it with: systemctl start mosquitto")
    
    # List current users
    print()
    print(info("Current users in password file:"))
    if paths['passwd_file'].exists():
        try:
            with open(paths['passwd_file'], 'r') as f:
                for line in f:
                    if ':' in line:
                        user = line.split(':')[0]
                        print(f"   • {user}")
        except IOError:
            pass
    
    print()
    print(success(f"Done! User '{username}' is now configured for MQTT authentication"))
    return 0

def main() -> int:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description='Add a username and password to Mosquitto MQTT broker',
        epilog='If password is not provided, it will be prompted securely.'
    )
    parser.add_argument('username', help='Username to add or update')
    parser.add_argument('password', nargs='?', help='Password (optional, will prompt if not provided)')
    parser.add_argument('--full-setup', action='store_true',
                       help='On Debian/Linux, attempt to install pyprompt if not available')
    
    args = parser.parse_args()
    
    return add_user(args.username, args.password, args.full_setup)

if __name__ == "__main__":
    sys.exit(main())
