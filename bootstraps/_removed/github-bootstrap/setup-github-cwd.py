#!/usr/bin/env python3
"""
Execute sparse checkout setup for GitHub repository.
Non-interactive script that sets up the GitHub remote repo and local repo with sparse checkout.
Cross-platform (Windows/Linux/Debian) and self-sufficient.
"""

import os
import sys
import subprocess
import json
import argparse
import shutil
import platform
from pathlib import Path
from typing import Dict, Optional, List


def is_windows() -> bool:
    """Check if running on Windows."""
    return platform.system() == "Windows"


def run_git_command(args: List[str], cwd: Optional[Path] = None, check: bool = True) -> subprocess.CompletedProcess:
    """Run a git command and return the result."""
    cmd = ["git"] + args
    try:
        result = subprocess.run(
            cmd,
            cwd=cwd,
            capture_output=True,
            text=True,
            timeout=60,
            check=check
        )
        return result
    except subprocess.TimeoutExpired:
        print(f"Error: Git command timed out: {' '.join(cmd)}", file=sys.stderr)
        sys.exit(1)
    except subprocess.CalledProcessError as e:
        print(f"Error: Git command failed: {' '.join(cmd)}", file=sys.stderr)
        if e.stderr:
            print(f"  {e.stderr}", file=sys.stderr)
        if check:
            sys.exit(1)
        return e


def read_command_from_file(file_path: Path) -> Optional[Dict]:
    """Read command configuration from JSON file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
            # Handle both formats: direct command dict or nested in "command" key
            if "command" in data:
                return data["command"]
            return data
    except Exception as e:
        print(f"Error reading command file: {e}", file=sys.stderr)
        return None


def build_remote_repo_url(github_user: str, repo_name: str, use_ssh: bool = True) -> str:
    """Build remote repository URL from username and repo name."""
    if use_ssh:
        return f"git@github.com:{github_user}/{repo_name}.git"
    else:
        return f"https://github.com/{github_user}/{repo_name}.git"


def setup_sparse_checkout(
    github_user: str,
    repo_name: str,
    workspace_root: str,
    local_folder: str,
    folders_to_sync: List[str],
    branch: str = "main",
    folder_mapping: Optional[Dict[str, str]] = None,
    force: bool = False,
    use_ssh: bool = True
) -> int:
    """Execute the sparse checkout setup."""
    remote_repo_url = build_remote_repo_url(github_user, repo_name, use_ssh)
    workspace_path = Path(workspace_root).resolve()
    local_path = workspace_path / local_folder
    
    print("Setting up sparse checkout...")
    print("-" * 70)
    print(f"GitHub User: {github_user}")
    print(f"Repository: {repo_name}")
    print(f"Remote Repo URL: {remote_repo_url}")
    print(f"Workspace: {workspace_path}")
    print(f"Local Folder: {local_path}")
    print(f"Folders to Sync: {', '.join(folders_to_sync)}")
    print(f"Branch: {branch}")
    if folder_mapping:
        print(f"Folder Mapping: {json.dumps(folder_mapping)}")
    print("-" * 70)
    print()
    
    # Create workspace directory if it doesn't exist
    try:
        workspace_path.mkdir(parents=True, exist_ok=True)
    except Exception as e:
        print(f"Error: Cannot create workspace directory: {e}", file=sys.stderr)
        return 1
    
    # Check if directory already exists
    if local_path.exists():
        if not force:
            print(f"Error: Directory {local_path} already exists.", file=sys.stderr)
            print("  Use --force to remove and re-clone", file=sys.stderr)
            return 1
        
        print(f"Warning: Directory {local_path} already exists.")
        print("Removing existing directory...")
        try:
            shutil.rmtree(local_path)
            print("  ✓ Removed existing directory")
        except Exception as e:
            print(f"Error: Cannot remove directory: {e}", file=sys.stderr)
            return 1
    
    # Step 1: Clone repository without checking out files
    print(f"Cloning repository (no checkout)...")
    result = run_git_command(
        ["clone", "--no-checkout", remote_repo_url, str(local_path)],
        cwd=workspace_path,
        check=False
    )
    
    if result.returncode != 0:
        print(f"Error: Failed to clone repository", file=sys.stderr)
        if result.stderr:
            print(f"  {result.stderr}", file=sys.stderr)
        return 1
    
    print("  ✓ Repository cloned")
    
    # Step 2: Enable sparse checkout (cone mode)
    print("Enabling sparse checkout...")
    result = run_git_command(
        ["sparse-checkout", "init", "--cone"],
        cwd=local_path,
        check=False
    )
    
    if result.returncode != 0:
        print(f"Error: Failed to initialize sparse checkout", file=sys.stderr)
        if result.stderr:
            print(f"  {result.stderr}", file=sys.stderr)
        return 1
    
    print("  ✓ Sparse checkout initialized")
    
    # Step 3: Set folders to check out
    print(f"Configuring folders to sync...")
    result = run_git_command(
        ["sparse-checkout", "set"] + folders_to_sync,
        cwd=local_path,
        check=False
    )
    
    if result.returncode != 0:
        print(f"Error: Failed to set sparse checkout folders", file=sys.stderr)
        if result.stderr:
            print(f"  {result.stderr}", file=sys.stderr)
        return 1
    
    print("  ✓ Folders configured")
    
    # Step 4: Check out the branch
    print(f"Checking out branch: {branch}...")
    result = run_git_command(
        ["checkout", branch],
        cwd=local_path,
        check=False
    )
    
    if result.returncode != 0:
        # Try 'main' if branch doesn't exist
        if branch != "main":
            print(f"  Branch '{branch}' not found, trying 'main'...")
            result = run_git_command(
                ["checkout", "main"],
                cwd=local_path,
                check=False
            )
        
        if result.returncode != 0:
            print(f"Error: Failed to checkout branch", file=sys.stderr)
            if result.stderr:
                print(f"  {result.stderr}", file=sys.stderr)
            return 1
    
    print("  ✓ Branch checked out")
    
    # Step 5: Rename folders if mapping provided
    if folder_mapping:
        print("Renaming folders according to mapping...")
        for remote_name, local_name in folder_mapping.items():
            remote_path = local_path / remote_name
            local_target = local_path / local_name
            
            if remote_path.exists():
                if local_target.exists():
                    print(f"  Warning: {local_name} already exists, skipping {remote_name}")
                else:
                    try:
                        remote_path.rename(local_target)
                        print(f"  ✓ Renamed: {remote_name} -> {local_name}")
                    except Exception as e:
                        print(f"  Error renaming {remote_name}: {e}", file=sys.stderr)
            else:
                print(f"  Warning: {remote_name} not found, skipping")
    
    # Step 6: Show current sparse checkout paths
    print()
    print("Current sparse checkout paths:")
    result = run_git_command(
        ["sparse-checkout", "list"],
        cwd=local_path,
        check=False
    )
    if result.returncode == 0 and result.stdout:
        for line in result.stdout.strip().split('\n'):
            if line.strip():
                print(f"  {line}")
    
    print()
    print("✓ Sparse checkout setup complete!")
    print()
    print(f"Navigate to {local_path} to work with the synced folders.")
    
    return 0


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Execute sparse checkout setup for GitHub repository"
    )
    parser.add_argument(
        "--command-file",
        type=str,
        help="JSON file containing the command from create-github-setup-command.py"
    )
    parser.add_argument(
        "--github-user",
        type=str,
        help="GitHub username or organization name"
    )
    parser.add_argument(
        "--repo-name",
        type=str,
        help="Repository name"
    )
    parser.add_argument(
        "--use-https",
        action="store_true",
        help="Use HTTPS URL instead of SSH (default: SSH)"
    )
    parser.add_argument(
        "--workspace-root",
        type=str,
        help="Root directory for the workspace"
    )
    parser.add_argument(
        "--local-folder",
        type=str,
        help="Local folder name to create"
    )
    parser.add_argument(
        "--folders",
        nargs="+",
        help="Folders to sync from remote repository"
    )
    parser.add_argument(
        "--branch",
        default="main",
        help="Branch to checkout (default: main)"
    )
    parser.add_argument(
        "--folder-mapping",
        type=str,
        help="JSON object mapping remote folder names to local folder names"
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="Force removal of existing directory if it exists"
    )
    
    args = parser.parse_args()
    
    # Determine source of parameters
    github_user = None
    repo_name = None
    workspace_root = None
    local_folder = None
    folders = None
    branch = args.branch
    folder_mapping = None
    use_ssh = not args.use_https
    
    # Parse folder mapping if provided
    if args.folder_mapping:
        try:
            folder_mapping = json.loads(args.folder_mapping)
        except json.JSONDecodeError:
            print("Error: Invalid JSON in --folder-mapping", file=sys.stderr)
            sys.exit(1)
    
    # Option 1: Read from command file
    if args.command_file:
        cmd_file = Path(args.command_file)
        if not cmd_file.exists():
            print(f"Error: Command file not found: {cmd_file}", file=sys.stderr)
            sys.exit(1)
        
        command_data = read_command_from_file(cmd_file)
        if not command_data:
            print("Error: Could not read command from file", file=sys.stderr)
            sys.exit(1)
        
        # Support both old format (remote_repo_url) and new format (github_user + repo_name)
        if 'github_user' in command_data and 'repo_name' in command_data:
            github_user = command_data.get('github_user')
            repo_name = command_data.get('repo_name')
        elif 'remote_repo_url' in command_data:
            # Parse URL to extract user and repo
            url = command_data.get('remote_repo_url')
            if url.startswith('git@github.com:'):
                # SSH format: git@github.com:user/repo.git
                parts = url.replace('git@github.com:', '').replace('.git', '').split('/')
                if len(parts) == 2:
                    github_user = parts[0]
                    repo_name = parts[1]
            elif url.startswith('https://github.com/'):
                # HTTPS format: https://github.com/user/repo.git
                parts = url.replace('https://github.com/', '').replace('.git', '').split('/')
                if len(parts) == 2:
                    github_user = parts[0]
                    repo_name = parts[1]
                    use_ssh = False
        
        workspace_root = command_data.get('workspace_root')
        local_folder = command_data.get('local_folder')
        folders = command_data.get('folders_to_sync')
        if 'branch' in command_data:
            branch = command_data['branch']
        if 'folder_mapping' in command_data:
            folder_mapping = command_data['folder_mapping']
    
    # Option 2: Use direct arguments
    else:
        github_user = args.github_user
        repo_name = args.repo_name
        workspace_root = args.workspace_root
        local_folder = args.local_folder
        folders = args.folders
    
    # Validate required parameters
    if not all([github_user, repo_name, workspace_root, local_folder, folders]):
        print("Error: Missing required parameters", file=sys.stderr)
        print("  Required: --github-user, --repo-name, --workspace-root, --local-folder, --folders", file=sys.stderr)
        print("  Or use: --command-file", file=sys.stderr)
        sys.exit(1)
    
    # Execute setup
    exit_code = setup_sparse_checkout(
        github_user=github_user,
        repo_name=repo_name,
        workspace_root=workspace_root,
        local_folder=local_folder,
        folders_to_sync=folders,
        branch=branch,
        folder_mapping=folder_mapping,
        force=args.force,
        use_ssh=use_ssh
    )
    
    if exit_code == 0:
        print()
        print("✓ Setup completed successfully!")
    else:
        print()
        print("✗ Setup failed.", file=sys.stderr)
    
    return exit_code


if __name__ == "__main__":
    sys.exit(main())
