#!/usr/bin/env python3
"""
Send clone-sync-manager.py script to SSH destination.

This script copies the clone-sync-manager.py script to a remote SSH destination.

Usage:
    python send-clone-script.py root@RPi-DietPi/_scripts
    python send-clone-script.py user@host/path/to/destination
"""

import sys
import subprocess
import platform
import argparse
import re
from pathlib import Path
from typing import Tuple


def is_windows() -> bool:
    """Check if running on Windows."""
    return platform.system() == "Windows"


def check_command_exists(cmd: str) -> bool:
    """Check if a command exists in PATH."""
    try:
        if is_windows():
            result = subprocess.run(
                ["where", cmd],
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
    except Exception:
        return False


def parse_ssh_destination(ssh_path: str) -> Tuple[str, str, str]:
    """Parse SSH destination format: user@host/path
    
    Returns:
        Tuple of (user, host, path)
    """
    # Pattern: user@host/path
    match = re.match(r'^([^@]+)@([^/]+)/(.+)$', ssh_path)
    if not match:
        raise ValueError(f"Invalid SSH destination format: {ssh_path}. Expected: user@host/path")
    
    user, host, path = match.groups()
    # Normalize path (remove leading/trailing slashes, ensure it starts with /)
    path = path.strip('/')
    if not path.startswith('/'):
        path = '/' + path
    
    return user, host, path


def test_ssh_connection(user: str, host: str) -> bool:
    """Test SSH connection to remote host."""
    print(f"Testing SSH connection to {user}@{host}...")
    try:
        result = subprocess.run(
            ["ssh", "-o", "BatchMode=yes", "-o", "ConnectTimeout=5", "-o", "StrictHostKeyChecking=no",
             f"{user}@{host}", "echo 'Connection successful'"],
            capture_output=True,
            timeout=10,
            text=True
        )
        if result.returncode == 0:
            print(f"✓ SSH connection successful")
            return True
        else:
            print(f"✗ SSH connection failed: {result.stderr}")
            return False
    except FileNotFoundError:
        print("✗ SSH command not found. Please install OpenSSH client.")
        return False
    except subprocess.TimeoutExpired:
        print("✗ SSH connection timed out")
        return False
    except Exception as e:
        print(f"✗ SSH connection error: {e}")
        return False


def find_clone_script() -> Path:
    """Find the clone-sync-manager.py script."""
    # Try current directory first
    current_dir = Path.cwd()
    script_path = current_dir / "clone-sync-manager.py"
    if script_path.exists():
        return script_path
    
    # Try in sync-manager directory
    sync_manager_path = current_dir / "___shared" / ".sync-manager" / "clone-sync-manager.py"
    if sync_manager_path.exists():
        return sync_manager_path
    
    # Try in dev-boards structure
    possible_paths = [
        Path("d:/_dev/_Projects/dev-boards/___shared/.sync-manager/clone-sync-manager.py"),
        Path("d:/_dev/projects/dev-boards/___shared/.sync-manager/clone-sync-manager.py"),
        Path.home() / "dev-boards" / "___shared" / ".sync-manager" / "clone-sync-manager.py",
    ]
    
    for path in possible_paths:
        if path.exists():
            return path
    
    # If not found, raise error
    raise FileNotFoundError(
        "clone-sync-manager.py not found. Please ensure the script exists in one of:\n"
        "  - Current directory\n"
        "  - ___shared/.sync-manager/\n"
        "  - Or specify with --script-path"
    )


def send_script_to_ssh(script_path: Path, ssh_destination: str) -> bool:
    """Send script to SSH destination using scp."""
    user, host, remote_path = parse_ssh_destination(ssh_destination)
    
    print(f"\nSending script to SSH destination:")
    print(f"  Script: {script_path}")
    print(f"  Destination: {user}@{host}:{remote_path}")
    
    # Test SSH connection first
    if not test_ssh_connection(user, host):
        print("\n✗ Cannot proceed without SSH connection")
        return False
    
    # Check if scp is available
    if not check_command_exists("scp"):
        print("✗ scp command not found. Please install OpenSSH client.")
        return False
    
    # Create remote directory if it doesn't exist
    remote_dir = str(Path(remote_path).parent)
    print(f"\n  Creating remote directory: {remote_dir}")
    try:
        subprocess.run(
            ["ssh", "-o", "StrictHostKeyChecking=no", f"{user}@{host}",
             f"mkdir -p {remote_dir}"],
            check=True,
            capture_output=True,
            timeout=10
        )
    except subprocess.CalledProcessError as e:
        print(f"✗ Failed to create remote directory: {e}")
        return False
    
    # Send script using scp
    remote_file = f"{user}@{host}:{remote_path}"
    print(f"  Copying script to {remote_file}...")
    try:
        result = subprocess.run(
            ["scp", "-o", "StrictHostKeyChecking=no", str(script_path), remote_file],
            check=True,
            capture_output=True,
            text=True,
            timeout=60
        )
        print(f"✓ Successfully copied script to {remote_file}")
        
        # Make script executable on remote
        print(f"  Making script executable...")
        try:
            subprocess.run(
                ["ssh", "-o", "StrictHostKeyChecking=no", f"{user}@{host}",
                 f"chmod +x {remote_path}"],
                check=True,
                capture_output=True,
                timeout=10
            )
            print(f"✓ Script is now executable")
        except subprocess.CalledProcessError as e:
            print(f"⚠ Failed to make script executable: {e}")
            print(f"  You may need to run: chmod +x {remote_path} on the remote host")
        
        return True
    except subprocess.CalledProcessError as e:
        error_msg = e.stderr if e.stderr else e.stdout
        print(f"✗ Copy failed: {error_msg}")
        return False
    except subprocess.TimeoutExpired:
        print("✗ Copy operation timed out")
        return False
    except FileNotFoundError:
        print("✗ scp command not found. Please install OpenSSH client.")
        return False


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description='Send clone-sync-manager.py script to SSH destination',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Send to default location
  python send-clone-script.py root@RPi-DietPi/_scripts

  # Send to custom location
  python send-clone-script.py user@host/path/to/destination

  # Specify script path explicitly
  python send-clone-script.py root@RPi-DietPi/_scripts --script-path=./clone-sync-manager.py
        """
    )
    
    parser.add_argument(
        'ssh_destination',
        type=str,
        help='SSH destination in format user@host/path'
    )
    
    parser.add_argument(
        '--script-path',
        type=str,
        help='Path to clone-sync-manager.py script (auto-detected if not specified)'
    )
    
    args = parser.parse_args()
    
    # Find script
    if args.script_path:
        script_path = Path(args.script_path)
        if not script_path.exists():
            print(f"✗ Error: Script not found at {script_path}")
            sys.exit(1)
    else:
        try:
            script_path = find_clone_script()
        except FileNotFoundError as e:
            print(f"✗ Error: {e}")
            sys.exit(1)
    
    print(f"Found script: {script_path}")
    
    # Parse and validate SSH destination
    try:
        success = send_script_to_ssh(script_path, args.ssh_destination)
    except ValueError as e:
        print(f"✗ Error: {e}")
        sys.exit(1)
    
    if success:
        print("\n✓ Script sent successfully!")
        print(f"\nYou can now run the script on the remote host:")
        print(f"  ssh {args.ssh_destination.split('/')[0]} 'python3 {args.ssh_destination.split('/')[1]}/clone-sync-manager.py --help'")
        sys.exit(0)
    else:
        print("\n✗ Failed to send script")
        sys.exit(1)


if __name__ == "__main__":
    main()

