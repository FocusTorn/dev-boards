"""
Sync command handler - transfer files/directories via SSH using OpenSSH host names.
"""

import sys
import subprocess
from pathlib import Path
from typing import Optional

from ..core.terminal import (
    write_header, COLOR_GREEN, COLOR_YELLOW, COLOR_RED,
    COLOR_RESET, BOLD_CHECK
)
from ..core.checks import check_command_exists, is_windows


def check_remote_path_exists(host: str, remote_path: str) -> bool:
    """
    Check if a path exists on the remote host.
    
    Args:
        host: OpenSSH host name from SSH config
        remote_path: Path on remote host to check
        
    Returns:
        True if path exists, False otherwise
    """
    try:
        # Use 'test -e' to check if path exists (works for both files and directories)
        result = subprocess.run(
            ["ssh", host, f"test -e {remote_path}"],
            capture_output=True,
            timeout=10,
            check=False
        )
        return result.returncode == 0
    except (subprocess.TimeoutExpired, FileNotFoundError):
        return False


def transfer_file_or_dir(
    host: str,
    source: str,
    destination: str,
    direction: str = "push"
) -> bool:
    """
    Transfer a file or directory using scp.
    
    Args:
        host: OpenSSH host name from SSH config
        source: Source path (local for push, remote for pull)
        destination: Destination path (remote for push, local for pull)
        direction: "push" (local to remote) or "pull" (remote to local)
        
    Returns:
        True if transfer succeeded, False otherwise
    """
    try:
        if direction == "push":
            # Push: local source to remote destination
            # Format: scp [options] source host:destination
            scp_source = source
            scp_dest = f"{host}:{destination}"
        else:
            # Pull: remote source to local destination
            # Format: scp [options] host:source destination
            scp_source = f"{host}:{source}"
            scp_dest = destination
        
        # Use -r flag for directories, -p to preserve timestamps
        # Check if source is a directory
        is_dir = False
        if direction == "push":
            source_path = Path(source)
            is_dir = source_path.is_dir()
        else:
            # For pull, check remotely
            try:
                result = subprocess.run(
                    ["ssh", host, f"test -d {source}"],
                    capture_output=True,
                    timeout=10,
                    check=False
                )
                is_dir = result.returncode == 0
            except Exception:
                pass
        
        scp_args = ["scp", "-p"]  # -p preserves timestamps
        if is_dir:
            scp_args.append("-r")  # -r for recursive (directories)
        
        scp_args.extend([scp_source, scp_dest])
        
        result = subprocess.run(
            scp_args,
            capture_output=True,
            timeout=300,  # 5 minutes timeout
            check=False
        )
        
        if result.returncode != 0:
            error_msg = result.stderr.decode('utf-8', errors='replace') if result.stderr else "Unknown error"
            print(f"{COLOR_RED}✗{COLOR_RESET} Transfer failed: {error_msg}", file=sys.stderr)
            return False
        
        return True
    except subprocess.TimeoutExpired:
        print(f"{COLOR_RED}✗{COLOR_RESET} Transfer timed out", file=sys.stderr)
        return False
    except Exception as e:
        print(f"{COLOR_RED}✗{COLOR_RESET} Transfer error: {e}", file=sys.stderr)
        return False


def cmd_sync(args):
    """
    Handle the sync subcommand - transfer files/directories via SSH.
    
    Logic:
    - If destination doesn't exist, transfer the source
    - If destination exists, throw an error
    """
    # Check prerequisites
    if not check_command_exists("ssh"):
        print(f"{COLOR_RED}✗{COLOR_RESET} SSH command not found. Please install OpenSSH.", file=sys.stderr)
        return 1
    
    if not check_command_exists("scp"):
        print(f"{COLOR_RED}✗{COLOR_RESET} SCP command not found. Please install OpenSSH.", file=sys.stderr)
        return 1
    
    host = args.host
    source = args.source
    destination = args.destination
    direction = args.direction.lower() if hasattr(args, 'direction') else "push"
    
    # Validate source exists (for push) or is accessible (for pull)
    if direction == "push":
        source_path = Path(source)
        if not source_path.exists():
            print(f"{COLOR_RED}✗{COLOR_RESET} Source path does not exist: {source}", file=sys.stderr)
            return 1
    else:
        # For pull, we'll check during transfer
        pass
    
    # Check if destination exists on remote (for push) or local (for pull)
    with write_header("Checking Destination"):
        if direction == "push":
            # Check remote destination
            print(f"Checking if destination exists on {host}...")
            dest_exists = check_remote_path_exists(host, destination)
        else:
            # Check local destination
            dest_path = Path(destination)
            dest_exists = dest_path.exists()
            if dest_exists:
                print(f"Checking if destination exists locally...")
            else:
                print(f"Destination does not exist locally (will create)...")
        
        if dest_exists:
            print(f"{COLOR_RED}✗{COLOR_RESET} Destination already exists: {destination}", file=sys.stderr)
            print(f"  Use --force to overwrite (not implemented yet)", file=sys.stderr)
            return 1
        else:
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Destination does not exist, proceeding with transfer")
    
    # Perform transfer
    with write_header("Transferring Files"):
        direction_text = "Pushing" if direction == "push" else "Pulling"
        print(f"{direction_text} {source} to {destination} on {host}...")
        
        success = transfer_file_or_dir(
            host=host,
            source=source,
            destination=destination,
            direction=direction
        )
        
        if success:
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Transfer completed successfully")
            return 0
        else:
            print(f"{COLOR_RED}✗{COLOR_RESET} Transfer failed", file=sys.stderr)
            return 1




