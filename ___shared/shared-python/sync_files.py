#!/usr/bin/env python3
"""
Standalone SSH file sync script.

Transfers files/directories between local and remote hosts using OpenSSH host names.

Usage:
    python sync_files.py <host> <source> <destination> [--direction push|pull]

Examples:
    # Push local file to remote
    python sync_files.py myhost /path/to/file.txt /remote/path/file.txt
    
    # Push local directory to remote
    python sync_files.py myhost /path/to/dir /remote/path/dir
    
    # Pull remote file to local
    python sync_files.py myhost /remote/path/file.txt /local/path/file.txt --direction pull
    
    # Pull remote directory to local
    python sync_files.py myhost /remote/path/dir /local/path/dir --direction pull

Behavior:
    - If destination does not exist: transfers the source
    - If destination exists: throws an error (use --force to overwrite)
"""

import sys
import subprocess
import argparse
import shutil
from pathlib import Path


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


def create_remote_directory(host: str, remote_path: str) -> bool:
    """
    Create a directory on the remote host (and parent directories if needed).
    
    Args:
        host: OpenSSH host name from SSH config
        remote_path: Path on remote host to create
        
    Returns:
        True if directory was created or already exists, False otherwise
    """
    try:
        # Use mkdir -p to create directory and parent directories
        result = subprocess.run(
            ["ssh", host, f"mkdir -p {remote_path}"],
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
            
            # For push: create remote destination directory if it doesn't exist
            if is_dir:
                # For directories, create the destination directory
                if not create_remote_directory(host, destination):
                    print(f"Error: Failed to create remote directory: {destination}", file=sys.stderr)
                    return False
            else:
                # For files, create the parent directory
                remote_parent = str(Path(destination).parent)
                if remote_parent and remote_parent != ".":
                    if not create_remote_directory(host, remote_parent):
                        print(f"Error: Failed to create remote parent directory: {remote_parent}", file=sys.stderr)
                        return False
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
            
            # For pull: create local destination directory if needed
            if is_dir:
                dest_path = Path(destination)
                dest_path.mkdir(parents=True, exist_ok=True)
            else:
                dest_path = Path(destination)
                dest_path.parent.mkdir(parents=True, exist_ok=True)
        
        scp_args = ["scp", "-p"]  # -p preserves timestamps
        if is_dir:
            scp_args.append("-r")  # -r for recursive (directories)
        
        scp_args.extend([scp_source, scp_dest])
        
        print(f"Running: {' '.join(scp_args)}")
        
        result = subprocess.run(
            scp_args,
            timeout=300,  # 5 minutes timeout
            check=False
        )
        
        if result.returncode != 0:
            print(f"Error: Transfer failed with exit code {result.returncode}", file=sys.stderr)
            return False
        
        return True
    except subprocess.TimeoutExpired:
        print("Error: Transfer timed out", file=sys.stderr)
        return False
    except Exception as e:
        print(f"Error: Transfer error: {e}", file=sys.stderr)
        return False


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Transfer files/directories via SSH using OpenSSH host names",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Push local file to remote
  python sync_files.py myhost /path/to/file.txt /remote/path/file.txt
  
  # Push local directory to remote
  python sync_files.py myhost /path/to/dir /remote/path/dir
  
  # Pull remote file to local
  python sync_files.py myhost /remote/path/file.txt /local/path/file.txt --direction pull
  
  # Pull remote directory to local
  python sync_files.py myhost /remote/path/dir /local/path/dir --direction pull
        """
    )
    
    parser.add_argument(
        "host",
        help="OpenSSH host name from SSH config (e.g., 'myhost' from ~/.ssh/config)"
    )
    
    parser.add_argument(
        "source",
        help="Source path (local for push, remote for pull)"
    )
    
    parser.add_argument(
        "destination",
        help="Destination path (remote for push, local for pull)"
    )
    
    parser.add_argument(
        "--direction",
        choices=["push", "pull"],
        default="push",
        help="Transfer direction: push (local to remote) or pull (remote to local). Default: push"
    )
    
    parser.add_argument(
        "--force",
        action="store_true",
        help="Overwrite destination if it exists (not implemented yet)"
    )
    
    args = parser.parse_args()
    
    # Check prerequisites
    # Check if ssh is available
    ssh_path = shutil.which("ssh")
    if not ssh_path:
        print("Error: SSH command not found. Please install OpenSSH.", file=sys.stderr)
        return 1
    
    # Check if scp is available
    scp_path = shutil.which("scp")
    if not scp_path:
        print("Error: SCP command not found. Please install OpenSSH.", file=sys.stderr)
        return 1
    
    # Verify they work by trying to get version (but don't fail if version check fails)
    try:
        subprocess.run(["ssh", "-V"], capture_output=True, check=False, timeout=5)
    except (subprocess.TimeoutExpired, Exception):
        # SSH exists but version check failed - that's okay, we'll try to use it anyway
        pass
    
    try:
        # Try different version flags for scp (some systems use different flags)
        subprocess.run(["scp"], capture_output=True, check=False, timeout=5)
    except (subprocess.TimeoutExpired, Exception):
        # SCP exists but check failed - that's okay, we'll try to use it anyway
        pass
    
    direction = args.direction.lower()
    host = args.host
    source = args.source
    destination = args.destination
    
    # Validate source exists (for push) or is accessible (for pull)
    if direction == "push":
        source_path = Path(source)
        if not source_path.exists():
            print(f"Error: Source path does not exist: {source}", file=sys.stderr)
            return 1
    
    # Check if destination exists
    print(f"Checking if destination exists...")
    if direction == "push":
        # Check remote destination
        dest_exists = check_remote_path_exists(host, destination)
    else:
        # Check local destination
        dest_path = Path(destination)
        dest_exists = dest_path.exists()
    
    if dest_exists:
        if args.force:
            print(f"Warning: Destination exists, but --force specified (overwrite not implemented yet)")
            print(f"Error: --force flag is not yet implemented", file=sys.stderr)
            return 1
        else:
            print(f"Error: Destination already exists: {destination}", file=sys.stderr)
            print(f"  Use --force to overwrite (not implemented yet)", file=sys.stderr)
            return 1
    else:
        print(f"Destination does not exist, proceeding with transfer...")
    
    # Perform transfer
    direction_text = "Pushing" if direction == "push" else "Pulling"
    print(f"{direction_text} {source} -> {destination} on {host}...")
    
    success = transfer_file_or_dir(
        host=host,
        source=source,
        destination=destination,
        direction=direction
    )
    
    if success:
        print("Transfer completed successfully")
        return 0
    else:
        print("Transfer failed", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())

