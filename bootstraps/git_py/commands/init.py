"""
Init command handler - initialize remote and local repos based on cwd.
"""

import sys
import json

from ..core.terminal import (
    write_header, COLOR_GREEN, COLOR_YELLOW, COLOR_RED,
    COLOR_RESET, BOLD_CHECK, get_region_indent
)
from ..core.prompts import confirm, HAS_PROMPT_TOOLKIT
from ..core.checks import (
    check_github_cli, check_ssh_connection, check_ssh_keys,
    check_git_crypt, check_git_config, check_connection_methods
)
from ..operations.ssh import auto_github_cli_login
from ..operations.repository import setup_local_repository, setup_remote_repository


def cmd_init(args):
    """Handle the init subcommand - initialize remote and local repos based on cwd."""
    # Check prerequisites
    gh_cli = check_github_cli()
    ssh_connected = check_ssh_connection()
    ssh_keys_exist, ssh_key_count = check_ssh_keys()
    git_crypt = check_git_crypt()
    git_config = check_git_config()
    connection_methods = check_connection_methods()
    
    # Print simplified summary
    with write_header("Git Details"):
        # Git Config
        name = git_config.get("name", "")
        email = git_config.get("email", "")
        if name and email:
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Name='{name}', Email='{email}'")
        else:
            print(f"{COLOR_RED}✗{COLOR_RESET} Git Config: Not configured")
        
        # SSH Status
        ssh_status = connection_methods.get("ssh", {})
        if ssh_status.get("usable", False):
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} SSH: Available and usable")
        else:
            print(f"{COLOR_YELLOW}⚠{COLOR_RESET} SSH: Not available")
        
        # HTTPS Status
        https_status = connection_methods.get("https", {})
        if https_status.get("usable", False):
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} HTTPS: Available and usable")
        else:
            print(f"{COLOR_YELLOW}⚠{COLOR_RESET} HTTPS: Not available")
        
        # GitHub CLI Status
        if gh_cli["installed"] and gh_cli["authenticated"]:
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} CLI: Installed and authenticated")
        elif gh_cli["installed"]:
            print(f"{COLOR_YELLOW}⚠{COLOR_RESET} CLI: Installed but not authenticated")
        else:
            print(f"{COLOR_RED}✗{COLOR_RESET} CLI: Not installed")
        
        # Git-Crypt Status
        if git_crypt["installed"]:
            if git_crypt["configured"]:
                if git_crypt["locked"]:
                    print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Git-Crypt: Installed and configured but locked")
                else:
                    print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Git-Crypt: Installed and configured")
            else:
                print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Git-Crypt: Installed but not configured")
        else:
            print(f"{COLOR_RED}✗{COLOR_RESET} Git-Crypt: Not installed")
    
    # Automatically handle GitHub CLI authentication if needed
    # Determine preferred protocol early (default to SSH unless --use-https is specified)
    use_ssh = not args.use_https
    
    if gh_cli["installed"] and not gh_cli["authenticated"]:
        print()
        write_header("GitHub CLI Authentication")
        if not auto_github_cli_login(use_ssh=use_ssh, silent=False):
            print("  Error: Failed to authenticate GitHub CLI. Please run 'gh auth login' manually.", file=sys.stderr)
            return 1
        print()
        # Re-check authentication status
        gh_cli = check_github_cli()
    
    # Setup Remote Repository first (handles prompts, summary, conflict resolution)
    # We need remote info first to pass to local setup for conflict resolution
    github_user, repo_name, is_private, remote_repo_url, remote_exists, remote_visibility, visibility_mismatch, link_to_existing_remote = setup_remote_repository(args)
    if not github_user or not repo_name or not remote_repo_url:
        return 1
    
    # Setup Local Repository (handles prompts, summary, proceed confirmation, and conflict resolution)
    workspace_root, local_folder, local_path, local_exists, proceed, link_to_existing_local = setup_local_repository(
        args, remote_exists=remote_exists, visibility_mismatch=visibility_mismatch
    )
    if not proceed or not workspace_root or not local_folder or not local_path:
        return 1
    
    # Use link_to_existing from remote if both exist, otherwise use local
    link_to_existing = link_to_existing_remote if (remote_exists and not visibility_mismatch) else link_to_existing_local
    
    # Output initialization configuration
    init_config = {
        "github_user": github_user,
        "repo_name": repo_name,
        "remote_repo_url": remote_repo_url,
        "workspace_root": workspace_root,
        "local_folder": local_folder,
        "local_path": local_path,
        "is_private": is_private,
        "remote_exists": remote_exists,
        "remote_visibility": remote_visibility,
        "local_exists": local_exists,
        "visibility_mismatch": visibility_mismatch,
        "link_to_existing": link_to_existing
    }
    
    if args.output_json:
        # Output as JSON with prerequisites
        output = {
            "init_config": init_config,
            "prerequisites": {
                "github_cli": gh_cli,
                "ssh_connected": ssh_connected,
                "ssh_keys_exist": ssh_keys_exist,
                "ssh_key_count": ssh_key_count,
                "git_crypt": git_crypt,
                "git_config": git_config
            }
        }
        print(json.dumps(output, indent=2))
    else:
        with write_header("Remote Repository"):
            print(f"GitHub User: {init_config['github_user']}")
            print(f"Repository: {init_config['repo_name']}")
            print(f"URL: {init_config['remote_repo_url']}")
            visibility_text = "Private" if is_private else "Public"
            print(f"Visibility: {visibility_text}")
            if link_to_existing:
                print("Action: Link local to existing remote")
            elif remote_exists:
                print("Action: Replace existing remote")
        
        with write_header("Local Repository"):
            print(f"Workspace Root: {init_config['workspace_root']}")
            print(f"Local Folder: {init_config['local_folder']}")
            print(f"Path: {init_config['local_path']}")
            if local_exists:
                print("Action: Replace existing local repository")
        
        print()
        print("Note: Folder syncing will be configured in a separate step.")
    
    # Exit with error if critical prerequisites are missing
    if not ssh_connected and not ssh_keys_exist:
        print("Warning: SSH authentication is not set up.", file=sys.stderr)
        return 1
    
    return 0

