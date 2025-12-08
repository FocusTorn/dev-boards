"""
Repository setup and management utilities.
"""

import sys
import subprocess
import shutil
from pathlib import Path
from typing import Optional, Tuple

from ..core.terminal import write_header, COLOR_YELLOW, COLOR_BRIGHT_YELLOW_BOLD, COLOR_QUESTION_TEXT, COLOR_GREEN, COLOR_RED, COLOR_RESET, BOLD_CHECK, get_region_indent
from ..core.prompts import confirm, HAS_PROMPT_TOOLKIT
from ..core.checks import (
    check_local_repo_exists, check_remote_repo_exists, get_github_username,
    check_github_cli, refresh_github_cli_auth_with_delete_repo_scope
)

from ..core.prompts import text as prompt_text, select as prompt_select


def build_remote_repo_url(github_user: str, repo_name: str, use_ssh: bool = True) -> str:
    """Build remote repository URL from username and repo name."""
    if use_ssh:
        return f"git@github.com:{github_user}/{repo_name}.git"
    else:
        return f"https://github.com/{github_user}/{repo_name}.git"


def setup_local_repository(
    args,
    remote_exists: Optional[bool] = None,
    visibility_mismatch: Optional[bool] = None
) -> Tuple[Optional[str], Optional[str], Optional[str], bool, bool, bool]:
    """
    Handle all Local Repository setup: prompts, summary, proceed confirmation, and conflict resolution.
    
    Args:
        args: Command arguments
        remote_exists: Whether remote repository exists (for conflict resolution)
        visibility_mismatch: Whether visibility mismatch exists (for conflict resolution)
    
    Returns:
        Tuple of (workspace_root, local_folder, local_path, local_exists, proceed, link_to_existing, replace_local)
        Returns (None, None, None, False, False, False, False) if cancelled
    """
    workspace_root = args.workspace_root
    local_folder = args.local_folder
    
    # Local Repository section - all prompts, summary, and confirmations in one header
    with write_header("Local Repository"):
        if not workspace_root or not local_folder:
            # Get default path from current working directory
            default_path = str(Path.cwd())
            if workspace_root and local_folder:
                # Both provided, construct path
                default_path = str(Path(workspace_root) / local_folder)
            elif workspace_root:
                # Only workspace_root provided
                default_path = workspace_root
            elif local_folder:
                # Only local_folder provided, use cwd as workspace
                default_path = str(Path.cwd() / local_folder)
            
            if not HAS_PROMPT_TOOLKIT:
                print("Error: prompt_toolkit is required for interactive prompts", file=sys.stderr)
                print("  Install with: pip install prompt-toolkit", file=sys.stderr)
                sys.exit(1)
            
            # Calculate indentation based on active regions (2 spaces per region)
            indent = get_region_indent()
            local_path_answer = prompt_text(
                "Local path (directory to use as basis for repository creation and initial commit):",  # Descriptive text in parentheses before colon
                default=default_path,
                indent=indent,
            )
            
            if local_path_answer is None:
                print("Cancelled.", file=sys.stderr)
                return (None, None, None, False, False, False, False)
            
            # Split the path into workspace_root and local_folder
            local_path_str = local_path_answer.strip()
            if local_path_str:
                local_path_obj = Path(local_path_str).resolve()
                workspace_root = str(local_path_obj.parent)
                local_folder = local_path_obj.name
        
        # Handle case where only workspace_root is provided
        if workspace_root and not local_folder:
            if HAS_PROMPT_TOOLKIT:
                # Calculate indentation based on active regions (2 spaces per region)
                indent = get_region_indent()
                
                local_folder = prompt_text(
                    "Local folder name:",  # Question text without indent
                    indent=indent,
                )
                if not local_folder:
                    print("Error: Local folder name is required", file=sys.stderr)
                    return (None, None, None, False, False, False, False)
            else:
                local_folder = input("Local folder name: ").strip()
                if not local_folder:
                    print("Error: Local folder name is required", file=sys.stderr)
                    return (None, None, None, False, False, False, False)
        elif local_folder and not workspace_root:
            # Only local_folder provided, use cwd as workspace_root
            workspace_root = str(Path.cwd())
        
        # Build local path
        local_path = str(Path(workspace_root) / local_folder) if workspace_root and local_folder else None
        if not local_path:
            return (None, None, None, False, False, False, False)
        
        # Check if local repository exists
        local_exists = check_local_repo_exists(local_path)
        
        # Display summary
        link_to_existing = False
        replace_local = False
        if local_exists:
            print()  # Blank line above warning
            print(f"{COLOR_YELLOW}⚠ Local repository exists: {local_path}{COLOR_RESET}")
            
            # Always ask if user wants to replace local repo when it exists
            if HAS_PROMPT_TOOLKIT:
                # Region system handles base indentation, so pass empty string
                replace_result = confirm(
                    "Replace existing local repository?",  # Question text - [y/N] added automatically
                    default=False,  # Default to No
                    indent=""  # Empty - region system handles indentation
                )
            else:
                response = input("Replace existing local repository? [y/N]: ").strip().lower()
                replace_result = response in ['y', 'yes']
            
            if replace_result is None:
                print("Cancelled.", file=sys.stderr)
                return (workspace_root, local_folder, local_path, local_exists, False, False)
            replace_local = (replace_result is True)
        else:
            print(f"Path: {local_path}")
        
        # Note: Remote repository visibility mismatch is handled in setup_remote_repository
        # No need to duplicate the prompt here
        
        # Ask for confirmation to proceed (part of Local Repository section)
        proceed = True
        if not args.output_json:
            if HAS_PROMPT_TOOLKIT:
                # Region system handles base indentation, so pass empty string
                # Use custom confirm prompt - supports y/n/Enter shortcuts, shows y/n as default
                proceed = confirm(
                    "Proceed with initialization?",  # Question text - [Y/n] added automatically
                    default=True,  # Default to Yes
                    indent=""  # Empty - region system handles indentation
                )
                if proceed is None or not proceed:
                    print("Cancelled.", file=sys.stderr)
                    return (workspace_root, local_folder, local_path, local_exists, False, link_to_existing, replace_local)
            else:
                # Fallback for non-interactive or no questionary
                response = input("Proceed with initialization? [Y/n]: ").strip().lower()
                if response and response not in ['y', 'yes']:
                    print("Cancelled.", file=sys.stderr)
                    return (workspace_root, local_folder, local_path, local_exists, False, link_to_existing, replace_local)
    
    return (workspace_root, local_folder, local_path, local_exists, proceed, link_to_existing, replace_local)


def setup_remote_repository(args) -> Tuple[Optional[str], Optional[str], bool, Optional[str], bool, Optional[str], bool, bool]:
    """
    Handle all Remote Repository setup: prompts, summary, and conflict resolution.
    
    Returns:
        Tuple of (github_user, repo_name, is_private, remote_repo_url, remote_exists, 
                  remote_visibility, visibility_mismatch, link_to_existing)
        Returns (None, None, False, None, False, None, False, False) if cancelled
    """
    github_user = args.github_user
    repo_name = args.repo_name
    use_ssh = not args.use_https
    
    # Remote Repository section - prompt for remote repository info
    with write_header("Remote Repository"):
        if not HAS_PROMPT_TOOLKIT:
            print("Error: prompt_toolkit is required for interactive prompts", file=sys.stderr)
            print("  Install with: pip install prompt-toolkit", file=sys.stderr)
            sys.exit(1)
        
        if not github_user:
            # Try to auto-detect GitHub username
            detected_username = get_github_username()
            # Calculate indentation based on active regions (2 spaces per region)
            indent = get_region_indent()
            github_user_answer = prompt_text(
                "GitHub username (or organization name):",  # Descriptive text in parentheses before colon
                default=detected_username or "",
                indent=indent,
            )
            
            if github_user_answer is None:
                print("Cancelled.", file=sys.stderr)
                return (None, None, False, None, False, None, False, False)
            github_user = github_user_answer.strip()
        
        if not repo_name:
            # Calculate indentation based on active regions (2 spaces per region)
            indent = get_region_indent()
            repo_name_answer = prompt_text(
                "Repository name:",  # Question text without indent
                default="Trial",
                indent=indent,
            )
            
            if repo_name_answer is None:
                print("Cancelled.", file=sys.stderr)
                return (None, None, False, None, False, None, False, False)
            repo_name = repo_name_answer.strip()
        
        # Always ask for repository visibility
        # Calculate indentation based on active regions (2 spaces per region)
        indent = get_region_indent()
        visibility_answer = prompt_select(
            "Repository visibility:",
            choices=["Public", "Private"],  # Public is first (default)
            indent=indent,
            pointer=" »"
        )
        
        if visibility_answer is None:
            print("Cancelled.", file=sys.stderr)
            return (None, None, False, None, False, None, False, False)
        is_private = (visibility_answer == "Private")
        
        # Build remote repo URL
        remote_repo_url = build_remote_repo_url(github_user, repo_name, use_ssh)
        
        # Check if remote repository exists and get visibility
        remote_exists, remote_visibility = check_remote_repo_exists(github_user, repo_name)
        
        # Check if remote exists with different visibility
        visibility_mismatch = False
        if remote_exists:
            expected_visibility = "private" if is_private else "public"
            visibility_mismatch = (remote_visibility != expected_visibility)
        
        # Handle conflicts (within the main Remote Repository header)
        link_to_existing = False
        if remote_exists:
            if not visibility_mismatch:
                # Remote exists with matching visibility - offer to link or recreate
                # (within main Remote Repository header - no nested header)
                print()  # Blank line above warning
                visibility_text = remote_visibility.capitalize() if remote_visibility else "Unknown"
                print(f"{COLOR_BRIGHT_YELLOW_BOLD}⚠{COLOR_RESET} {COLOR_QUESTION_TEXT}Already exists: {repo_name} ({visibility_text}){COLOR_RESET}")
                if HAS_PROMPT_TOOLKIT:
                    indent = get_region_indent()
                    action_response = prompt_select(
                        "Action:",
                        choices=["Link local to existing remote", "Remove and recreate remote"],  # Link is first (default)
                        indent=indent,
                        pointer=" »"
                    )
                    if action_response is None:
                        print("Cancelled.", file=sys.stderr)
                        return (None, None, False, None, False, None, False, False)
                    link_to_existing = (action_response == "Link local to existing remote")
                else:
                    response = input("Link to existing remote or recreate? [Link/recreate] (default: Link): ").strip().lower()
                    link_to_existing = response not in ['recreate', 'r']
            else:
                # Remote exists with different visibility - offer options
                # (within main Remote Repository header - no nested header)
                print()  # Blank line above warning
                visibility_text = remote_visibility.capitalize() if remote_visibility else "Unknown"
                print(f"{COLOR_BRIGHT_YELLOW_BOLD}⚠{COLOR_RESET} {COLOR_QUESTION_TEXT}Already exists: {repo_name} ({visibility_text}){COLOR_RESET}")
                
                # Ask user to choose action (part of Remote Repository section)
                if HAS_PROMPT_TOOLKIT:
                    indent = get_region_indent()
                    action_response = prompt_select(
                        "",  # Empty question - choices appear directly below warning
                        choices=["Change visibility and link", "Replace repository"],  # Change visibility is first (default)
                        indent=indent,
                        pointer=" »",
                        warning_mode=True  # Skip question line, show choices directly
                    )
                    if action_response is None:
                        print("Cancelled.", file=sys.stderr)
                        return (None, None, False, None, False, None, False, False)
                    
                    if action_response == "Change visibility and link":
                        # Change visibility and link to existing
                        link_to_existing = True
                        # We'll change visibility in create_remote_repository
                    else:
                        # Replace (delete and recreate)
                        link_to_existing = False
                else:
                    response = input("Change visibility and link, or replace? [Change/replace] (default: Change): ").strip().lower()
                    if response in ['replace', 'r']:
                        link_to_existing = False
                    else:
                        link_to_existing = True
        
        return (github_user, repo_name, is_private, remote_repo_url, remote_exists, remote_visibility, visibility_mismatch, link_to_existing)


def create_remote_repository(
    github_user: str,
    repo_name: str,
    is_private: bool,
    remote_exists: bool,
    link_to_existing: bool,
    remote_visibility: Optional[str] = None,
    local_path: Optional[str] = None,
    skip_header: bool = False
) -> bool:
    """
    Create remote repository on GitHub using gh CLI.
    
    Args:
        github_user: GitHub username or organization
        repo_name: Repository name
        is_private: Whether repository should be private
        remote_exists: Whether remote repository already exists
        link_to_existing: Whether to link to existing remote (skip creation or change visibility)
        remote_visibility: Current visibility of existing remote (if remote_exists is True)
        local_path: Path to local repository directory (optional, for --source flag)
        skip_header: If True, don't create a header (for use within existing header context)
    
    Returns:
        True if successful, False otherwise
    """
    if remote_exists and link_to_existing:
        # Check if we need to change visibility
        if remote_visibility:
            expected_visibility = "private" if is_private else "public"
            if remote_visibility != expected_visibility:
                # Change visibility of existing repo
                return _change_repository_visibility(github_user, repo_name, is_private, skip_header)
        # Visibility matches, just link to existing
        return True
    
    # Create header only if not skipping (for standalone use)
    if skip_header:
        # Operations happen within existing header context
        pass
    else:
        with write_header("Creating Remote Repository"):
            return _create_remote_repository_impl(github_user, repo_name, is_private, remote_exists, remote_visibility, local_path)
    
    # When skip_header is True, execute operations directly
    return _create_remote_repository_impl(github_user, repo_name, is_private, remote_exists, remote_visibility, local_path)


def _create_remote_repository_impl(
    github_user: str,
    repo_name: str,
    is_private: bool,
    remote_exists: bool,
    remote_visibility: Optional[str] = None,
    local_path: Optional[str] = None
) -> bool:
    """Internal implementation of remote repository creation."""
    visibility_flag = "--private" if is_private else "--public"
    
    # If remote exists but we're not linking, we need to delete it first
    if remote_exists:
        # Check if delete_repo scope is available, refresh if needed
        gh_cli = check_github_cli()
        if not gh_cli.get("has_delete_repo_scope", False):
            if not refresh_github_cli_auth_with_delete_repo_scope():
                print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Failed to refresh delete_repo scope. Attempting delete anyway...")
        
        print(f"{COLOR_YELLOW}⚠ Removing existing repository...{COLOR_RESET}")
        delete_result = subprocess.run(
            ["gh", "repo", "delete", f"{github_user}/{repo_name}", "--yes"],
            capture_output=True,
            text=True
        )
        if delete_result.returncode != 0:
            # Check if error is about missing scope
            if "delete_repo" in delete_result.stderr.lower() or "scope" in delete_result.stderr.lower():
                print(f"{COLOR_RED}✗{COLOR_RESET} Failed to delete existing repository: missing delete_repo scope", file=sys.stderr)
                print(f"  Run: gh auth refresh -h github.com -s delete_repo", file=sys.stderr)
            else:
                print(f"{COLOR_RED}✗{COLOR_RESET} Failed to delete existing repository", file=sys.stderr)
                print(delete_result.stderr, file=sys.stderr)
            return False
    
    # Build gh repo create command
    create_cmd = ["gh", "repo", "create", repo_name, visibility_flag]
    
    # If local_path is provided and contains a git repo, use --source
    # But we need to handle existing remotes carefully
    has_existing_remote = False
    temp_remote_url = None
    if local_path:
        local_path_obj = Path(local_path)
        git_dir = local_path_obj / ".git"
        if git_dir.exists():
            # Check if remote "origin" already exists
            remote_check = subprocess.run(
                ["git", "remote", "get-url", "origin"],
                cwd=local_path,
                capture_output=True,
                text=True
            )
            has_existing_remote = (remote_check.returncode == 0)
            
            if has_existing_remote:
                # If remote exists, create repo without --source to avoid "Unable to add remote" error
                # The remote will be updated by initialize_local_repository
                print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Local repository has existing remote. Creating repo without --source...")
                # Don't add --source, just create the repo
                # Remote will be handled by initialize_local_repository
            else:
                # No existing remote, safe to use --source and --remote
                create_cmd.extend(["--source", str(local_path_obj), "--remote", "origin"])
    
    # Create the repository
    create_result = subprocess.run(
        create_cmd,
        capture_output=True,
        text=True
    )
    
    if create_result.returncode == 0:
        print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Remote repository created: {github_user}/{repo_name}")
        # If we temporarily removed a remote, it's now been re-added by gh repo create
        # The remote URL will be updated by initialize_local_repository if needed
        return True
    else:
        print(f"{COLOR_RED}✗{COLOR_RESET} Failed to create remote repository", file=sys.stderr)
        # If we removed a remote and creation failed, try to restore it
        if has_existing_remote and local_path:
            restore_result = subprocess.run(
                ["git", "remote", "add", "origin", temp_remote_url],
                cwd=local_path,
                capture_output=True,
                text=True
            )
            if restore_result.returncode != 0:
                print(f"  Warning: Could not restore original remote URL", file=sys.stderr)
        print(create_result.stderr, file=sys.stderr)
        return False


def initialize_local_repository(local_path: str, remote_repo_url: str, local_exists: bool, replace_local: bool = False, skip_header: bool = False) -> bool:
    """
    Initialize local git repository and set up remote connection.
    
    Args:
        local_path: Path to local repository directory
        remote_repo_url: URL of remote repository
        local_exists: Whether local repository already exists
        replace_local: If True and local_exists is True, remove and reinitialize the repo
        skip_header: If True, don't create a header (for use within existing header context)
    
    Returns:
        True if successful, False otherwise
    """
    local_path_obj = Path(local_path)
    
    # Create header only if not skipping (for standalone use)
    if skip_header:
        # Operations happen within existing header context
        return _initialize_local_repository_impl(local_path, remote_repo_url, local_exists, replace_local)
    else:
        with write_header("Initializing Local Repository"):
            return _initialize_local_repository_impl(local_path, remote_repo_url, local_exists, replace_local)


def _initialize_local_repository_impl(local_path: str, remote_repo_url: str, local_exists: bool, replace_local: bool = False) -> bool:
    """Internal implementation of local repository initialization."""
    local_path_obj = Path(local_path)
    # Create directory if it doesn't exist
    if not local_path_obj.exists():
        local_path_obj.mkdir(parents=True, exist_ok=True)
        print(f"Created directory: {local_path}")
    
    # If replacing local repo, remove .git directory first
    if replace_local and local_exists:
        git_dir = local_path_obj / ".git"
        if git_dir.exists():
            try:
                shutil.rmtree(git_dir)
                print(f"{COLOR_YELLOW}⚠ Removed existing .git directory{COLOR_RESET}")
                local_exists = False  # Now it doesn't exist, so we'll initialize it
            except Exception as e:
                print(f"{COLOR_RED}✗{COLOR_RESET} Failed to remove existing .git directory: {e}", file=sys.stderr)
                return False
    
    # Initialize git repository if it doesn't exist
    if not local_exists:
        init_result = subprocess.run(
            ["git", "init"],
            cwd=local_path,
            capture_output=True,
            text=True
        )
        
        if init_result.returncode != 0:
            print(f"{COLOR_RED}✗{COLOR_RESET} Failed to initialize git repository", file=sys.stderr)
            print(init_result.stderr, file=sys.stderr)
            return False
        
        print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Git repository initialized")
    else:
        print(f"{COLOR_YELLOW}⚠ Local repository already exists{COLOR_RESET}")
    
    # Set up remote connection
    # Check if remote already exists
    remote_check = subprocess.run(
        ["git", "remote", "get-url", "origin"],
        cwd=local_path,
        capture_output=True,
        text=True
    )
    
    if remote_check.returncode == 0:
        # Remote exists, update it
        remote_set_result = subprocess.run(
            ["git", "remote", "set-url", "origin", remote_repo_url],
            cwd=local_path,
            capture_output=True,
            text=True
        )
        if remote_set_result.returncode != 0:
            print(f"{COLOR_RED}✗{COLOR_RESET} Failed to update remote URL", file=sys.stderr)
            print(remote_set_result.stderr, file=sys.stderr)
            return False
        print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Remote 'origin' updated: {remote_repo_url}")
    else:
        # Remote doesn't exist, add it
        remote_add_result = subprocess.run(
            ["git", "remote", "add", "origin", remote_repo_url],
            cwd=local_path,
            capture_output=True,
            text=True
        )
        if remote_add_result.returncode != 0:
            print(f"{COLOR_RED}✗{COLOR_RESET} Failed to add remote", file=sys.stderr)
            print(remote_add_result.stderr, file=sys.stderr)
            return False
        print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Remote 'origin' added: {remote_repo_url}")
    
    return True


def create_initial_commit(local_path: str, commit_message: Optional[str] = None) -> bool:
    """
    Create initial commit with all files in the repository.
    
    Args:
        local_path: Path to local repository directory
        commit_message: Commit message (default: "Initial commit")
    
    Returns:
        True if successful, False otherwise
    """
    if commit_message is None:
        commit_message = "Initial commit"
    
    with write_header("Creating Initial Commit"):
        # Check if there are any changes to commit
        status_result = subprocess.run(
            ["git", "status", "--porcelain"],
            cwd=local_path,
            capture_output=True,
            text=True
        )
        
        if not status_result.stdout.strip():
            # No changes to commit
            print(f"{COLOR_YELLOW}⚠ No changes to commit{COLOR_RESET}")
            return True
        
        # Add all files
        add_result = subprocess.run(
            ["git", "add", "."],
            cwd=local_path,
            capture_output=True,
            text=True
        )
        
        if add_result.returncode != 0:
            print(f"{COLOR_RED}✗{COLOR_RESET} Failed to add files", file=sys.stderr)
            print(add_result.stderr, file=sys.stderr)
            return False
        
        # Create commit
        commit_result = subprocess.run(
            ["git", "commit", "-m", commit_message],
            cwd=local_path,
            capture_output=True,
            text=True
        )
        
        if commit_result.returncode != 0:
            print(f"{COLOR_RED}✗{COLOR_RESET} Failed to create commit", file=sys.stderr)
            print(commit_result.stderr, file=sys.stderr)
            return False
        
        print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Initial commit created: {commit_message}")
        return True


def push_to_remote(local_path: str, branch: str = "main") -> bool:
    """
    Push local repository to remote.
    
    Args:
        local_path: Path to local repository directory
        branch: Branch name to push (default: "main")
    
    Returns:
        True if successful, False otherwise
    """
    with write_header("Pushing to Remote"):
        # Check if branch exists locally
        branch_check = subprocess.run(
            ["git", "rev-parse", "--verify", f"refs/heads/{branch}"],
            cwd=local_path,
            capture_output=True,
            text=True
        )
        
        if branch_check.returncode != 0:
            # Branch doesn't exist, create it
            branch_create = subprocess.run(
                ["git", "checkout", "-b", branch],
                cwd=local_path,
                capture_output=True,
                text=True
            )
            if branch_create.returncode != 0:
                print(f"{COLOR_RED}✗{COLOR_RESET} Failed to create branch '{branch}'", file=sys.stderr)
                print(branch_create.stderr, file=sys.stderr)
                return False
        
        # Push to remote
        push_result = subprocess.run(
            ["git", "push", "-u", "origin", branch],
            cwd=local_path,
            capture_output=True,
            text=True
        )
        
        if push_result.returncode != 0:
            print(f"{COLOR_RED}✗{COLOR_RESET} Failed to push to remote", file=sys.stderr)
            print(push_result.stderr, file=sys.stderr)
            return False
        
        print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Pushed to remote: origin/{branch}")
        return True

