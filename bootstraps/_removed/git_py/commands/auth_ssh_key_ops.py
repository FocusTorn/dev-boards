"""
SSH key generation and upload operations for authentication command.
"""

import os
import sys
import subprocess
from pathlib import Path

from ..core.terminal import COLOR_GREEN, COLOR_YELLOW, COLOR_RED, COLOR_RESET, BOLD_CHECK
from ..core.checks import check_git_config, is_windows
from ..operations.ssh import add_github_to_known_hosts, setup_ssh_config


def generate_ssh_key(key_name: str, overwrite: bool = False) -> tuple:
    """
    Generate a new SSH key.
    
    Args:
        key_name: Name for the SSH key (without .pub extension)
        overwrite: Whether to overwrite existing key files
    
    Returns:
        tuple: (success: bool, key_path_public: str or None)
    """
    ssh_dir = Path.home() / ".ssh"
    ssh_dir.mkdir(mode=0o700, exist_ok=True)
    
    key_path_private = ssh_dir / key_name
    key_path_public = ssh_dir / f"{key_name}.pub"
    
    # Check if key already exists
    if key_path_private.exists() or key_path_public.exists():
        if overwrite:
            # User chose to overwrite, so delete existing key files first
            print(f"Overwriting existing SSH key: {key_name}")
            try:
                if key_path_private.exists():
                    key_path_private.unlink()
                if key_path_public.exists():
                    key_path_public.unlink()
            except Exception as e:
                print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Warning: Could not delete existing key files: {e}", file=sys.stderr)
        else:
            # User chose not to overwrite, reuse existing key
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Using existing SSH key: {key_name}")
            return (True, str(key_path_public))
    
    # Generate new key
    print("Generating new SSH key...")
    try:
        result = subprocess.run(
            ["ssh-keygen", "-t", "ed25519", "-C", key_name, "-f", str(key_path_private), "-N", ""],
            capture_output=True,
            text=True,
            timeout=60
        )
        if result.returncode != 0:
            error_msg = result.stderr if result.stderr else result.stdout
            print(f"{COLOR_RED}✗{COLOR_RESET} Failed to generate SSH key: {error_msg}", file=sys.stderr)
            return (False, None)
        print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Generated SSH key: {key_name}")
        
        # Set correct permissions on the private key (Unix-like systems)
        if not is_windows():
            try:
                os.chmod(key_path_private, 0o600)
            except Exception:
                pass
        
        # Add GitHub to known_hosts if not already present
        add_github_to_known_hosts()
        
        # Set up SSH config for the new key
        if setup_ssh_config(str(key_path_private)):
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} SSH config configured for new key")
        
        return (True, str(key_path_public))
    except subprocess.TimeoutExpired:
        print(f"{COLOR_RED}✗{COLOR_RESET} SSH key generation timed out.", file=sys.stderr)
        return (False, None)
    except Exception as e:
        print(f"{COLOR_RED}✗{COLOR_RESET} Error generating SSH key: {e}", file=sys.stderr)
        return (False, None)


def upload_ssh_key_to_github(public_key_path: str) -> bool:
    """
    Upload SSH key to GitHub.
    
    Args:
        public_key_path: Path to the public key file
    
    Returns:
        bool: True if uploaded successfully or already exists, False otherwise
    """
    print("Adding SSH key to GitHub...")
    try:
        # Get key name from path stem (e.g., "github_windows" from "github_windows.pub")
        key_name_from_path = Path(public_key_path).stem
        # Format as "github-windows (github_windows)"
        key_title = f"{key_name_from_path.replace('_', '-')} ({key_name_from_path})"
        result = subprocess.run(
            ["gh", "ssh-key", "add", public_key_path, "--title", key_title],
            capture_output=True,
            timeout=30,
            text=True
        )
        
        if result.returncode == 0:
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} SSH key added to GitHub")
            return True
        else:
            if "already exists" in result.stderr.lower() or "already in use" in result.stderr.lower():
                print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} SSH key already exists on GitHub")
                return True
            else:
                print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Failed to add SSH key: {result.stderr}", file=sys.stderr)
                print("You may need to add it manually at: https://github.com/settings/keys")
                return False
    except Exception as e:
        print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Error adding SSH key: {e}", file=sys.stderr)
        return False


def handle_ssh_key_setup(
    selected_ssh_key_path: str,
    new_key_name: str,
    new_key_overwrite: bool
) -> tuple:
    """
    Handle SSH key setup: generate if needed, then upload to GitHub.
    
    Args:
        selected_ssh_key_path: Path to selected key, "GENERATE_NEW", or None
        new_key_name: Name for new key if generating
        new_key_overwrite: Whether to overwrite existing key if generating
    
    Returns:
        tuple: (success: bool, key_path_public: str or None, uploaded: bool)
    """
    if not selected_ssh_key_path:
        return (True, None, False)
    
    if selected_ssh_key_path == "GENERATE_NEW":
        # Generate new key
        success, key_path_public = generate_ssh_key(new_key_name, new_key_overwrite)
        if not success:
            return (False, None, False)
        
        # Upload to GitHub
        uploaded = upload_ssh_key_to_github(key_path_public)
        return (True, key_path_public, uploaded)
    else:
        # Use existing key - just upload to GitHub
        uploaded = upload_ssh_key_to_github(selected_ssh_key_path)
        return (True, selected_ssh_key_path, uploaded)

