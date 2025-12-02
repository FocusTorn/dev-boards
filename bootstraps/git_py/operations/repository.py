"""
Repository setup and management utilities.
"""

import sys
from pathlib import Path
from typing import Optional, Tuple

from ..core.terminal import write_header, COLOR_YELLOW, COLOR_RESET, get_region_indent
from ..core.prompts import confirm, HAS_PROMPT_TOOLKIT
from ..core.checks import check_local_repo_exists, check_remote_repo_exists, get_github_username

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
        Tuple of (workspace_root, local_folder, local_path, local_exists, proceed, link_to_existing)
        Returns (None, None, None, False, False, False) if cancelled
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
                return (None, None, None, False, False, False)
            
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
                    return (None, None, None, False, False, False)
            else:
                local_folder = input("Local folder name: ").strip()
                if not local_folder:
                    print("Error: Local folder name is required", file=sys.stderr)
                    return (None, None, None, False, False, False)
        elif local_folder and not workspace_root:
            # Only local_folder provided, use cwd as workspace_root
            workspace_root = str(Path.cwd())
        
        # Build local path
        local_path = str(Path(workspace_root) / local_folder) if workspace_root and local_folder else None
        if not local_path:
            return (None, None, None, False, False, False)
        
        # Check if local repository exists
        local_exists = check_local_repo_exists(local_path)
        
        # Display summary
        link_to_existing = False
        if local_exists:
            print()  # Blank line above warning
            print(f"{COLOR_YELLOW}⚠ Local repository exists: {local_path}{COLOR_RESET}")
        else:
            print(f"Path: {local_path}")
        
        # Handle local repository conflicts if both exist with visibility mismatch
        if local_exists and remote_exists and visibility_mismatch:
            if HAS_PROMPT_TOOLKIT:
                # Region system handles base indentation, so pass empty string
                replace_result = confirm(
                    "Replace existing repository/repositories?",  # Question text - [y/N] added automatically
                    default=False,  # Default to No
                    indent=""  # Empty - region system handles indentation
                )
            else:
                response = input("Replace existing repository/repositories? [y/N]: ").strip().lower()
                replace_result = response in ['y', 'yes']
            
            if replace_result is None or not replace_result:
                print("Cancelled.", file=sys.stderr)
                return (workspace_root, local_folder, local_path, local_exists, False, False)
            link_to_existing = False
        
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
                    return (workspace_root, local_folder, local_path, local_exists, False, link_to_existing)
            else:
                # Fallback for non-interactive or no questionary
                response = input("Proceed with initialization? [Y/n]: ").strip().lower()
                if response and response not in ['y', 'yes']:
                    print("Cancelled.", file=sys.stderr)
                    return (workspace_root, local_folder, local_path, local_exists, False, link_to_existing)
    
    return (workspace_root, local_folder, local_path, local_exists, proceed, link_to_existing)


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
    
    # Display summary
    with write_header("Remote Repository"):
        print(f"URL: {remote_repo_url}")
        visibility_text = "Private" if is_private else "Public"
        print(f"Visibility: {visibility_text}")
        if remote_exists:
            print()  # Blank line above warning
            existing_visibility = remote_visibility.capitalize() if remote_visibility else "Unknown"
            if visibility_mismatch:
                print(f"{COLOR_YELLOW}⚠ Already exists (current: {existing_visibility}, requested: {visibility_text}){COLOR_RESET}")
            else:
                print(f"{COLOR_YELLOW}⚠ Already exists ({existing_visibility}){COLOR_RESET}")
    
    # Handle conflicts
    link_to_existing = False
    if remote_exists:
        if not visibility_mismatch:
            # Remote exists with matching visibility - offer to link or recreate
            with write_header("Remote Repository"):
                print()  # Blank line above warning
                visibility_text = remote_visibility.capitalize() if remote_visibility else "Unknown"
                print(f"{COLOR_YELLOW}⚠ Already exists ({visibility_text}){COLOR_RESET}")
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
                        return (github_user, repo_name, is_private, remote_repo_url, remote_exists, remote_visibility, visibility_mismatch, False)
                    link_to_existing = (action_response == "Link local to existing remote")
                else:
                    response = input("Link to existing remote or recreate? [Link/recreate] (default: Link): ").strip().lower()
                    link_to_existing = response not in ['recreate', 'r']
        else:
            # Remote exists with different visibility - ask to replace
            with write_header("Remote Repository"):
                print()  # Blank line above warning
                visibility_text = remote_visibility.capitalize() if remote_visibility else "Unknown"
                expected_text = "Private" if is_private else "Public"
                print(f"{COLOR_YELLOW}⚠ Already exists (current: {visibility_text}, requested: {expected_text}){COLOR_RESET}")
                
                # Ask to replace/recreate (part of Remote Repository section)
                if HAS_PROMPT_TOOLKIT:
                    indent = get_region_indent()
                    # Use custom confirm prompt - supports y/n/Enter shortcuts, shows y/n as default
                    replace_result = confirm(
                        "Replace existing repository/repositories?",  # Question text - [y/N] added automatically
                        default=False,  # Default to No
                        indent=indent
                    )
                else:
                    response = input("Replace existing repository/repositories? [y/N]: ").strip().lower()
                    replace_result = response in ['y', 'yes']
                
                if replace_result is None or not replace_result:
                    print("Cancelled.", file=sys.stderr)
                    return (github_user, repo_name, is_private, remote_repo_url, remote_exists, remote_visibility, visibility_mismatch, False)
                link_to_existing = False
    
    return (github_user, repo_name, is_private, remote_repo_url, remote_exists, remote_visibility, visibility_mismatch, link_to_existing)

