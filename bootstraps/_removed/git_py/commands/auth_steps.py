"""
Authentication steps for the auth command.
"""

import sys
import time

from ..core.terminal import write_header, COLOR_GREEN, COLOR_YELLOW, COLOR_RED, COLOR_RESET, BOLD_CHECK
from ..core.checks import (
    check_github_cli, check_ssh_connection, check_connection_methods,
    is_windows, refresh_github_cli_auth_with_delete_repo_scope
)
from ..operations.ssh import auto_github_cli_login, setup_ssh_config
from .auth_ssh_key_ops import handle_ssh_key_setup


def step_github_cli_auth(
    gh_cli: dict,
    selected_ssh_key_path: str,
    new_key_name: str,
    new_key_overwrite: bool
) -> tuple:
    """
    Step 1: GitHub CLI authentication and SSH key setup.
    
    Args:
        gh_cli: GitHub CLI status dict
        selected_ssh_key_path: Path to selected key, "GENERATE_NEW", or None
        new_key_name: Name for new key if generating
        new_key_overwrite: Whether to overwrite existing key if generating
    
    Returns:
        tuple: (success: bool, ssh_key_uploaded: bool, selected_ssh_key_path: str or None)
    """
    with write_header("Step 1: GitHub CLI Authentication"):
        if not gh_cli["installed"]:
            print(f"{COLOR_RED}✗{COLOR_RESET} GitHub CLI is not installed")
            print("Please install GitHub CLI first:")
            if is_windows():
                print("  winget install --id GitHub.cli")
            else:
                print("  Visit: https://cli.github.com/")
            return (False, False, None)
        
        if not gh_cli["authenticated"]:
            # Use the auto_github_cli_login function (with --skip-ssh-key)
            if not auto_github_cli_login(use_ssh=True, silent=False):
                return (False, False, None)
            
            # After authentication, check and refresh delete_repo scope if needed
            gh_cli_after = check_github_cli()
            if gh_cli_after["authenticated"] and not gh_cli_after.get("has_delete_repo_scope", False):
                if not refresh_github_cli_auth_with_delete_repo_scope():
                    print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Failed to refresh delete_repo scope (you may need to run: gh auth refresh -h github.com -s delete_repo)")
            
            # After authentication, add the pre-selected SSH key if one was chosen
            success, key_path, uploaded = handle_ssh_key_setup(
                selected_ssh_key_path, new_key_name, new_key_overwrite
            )
            if not success:
                return (False, False, None)
            return (True, uploaded, key_path)
        else:
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} GitHub CLI already authenticated")
            
            # Check if delete_repo scope is available, refresh if needed
            if not gh_cli.get("has_delete_repo_scope", False):
                gh_cli_current = check_github_cli()
                if not gh_cli_current.get("has_delete_repo_scope", False):
                    if not refresh_github_cli_auth_with_delete_repo_scope():
                        print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Failed to refresh delete_repo scope (you may need to run: gh auth refresh -h github.com -s delete_repo)")
            
            # Even if already authenticated, we might need to add an SSH key
            if selected_ssh_key_path:
                success, key_path, uploaded = handle_ssh_key_setup(
                    selected_ssh_key_path, new_key_name, new_key_overwrite
                )
                return (True, uploaded, key_path)
            return (True, False, None)


def step_ssh_configuration(selected_ssh_key_path: str, ssh_key_uploaded: bool) -> bool:
    """
    Step 2: Configure SSH and test connection.
    
    Args:
        selected_ssh_key_path: Path to selected key or None
        ssh_key_uploaded: Whether a key was just uploaded
    
    Returns:
        bool: True if SSH is working, False otherwise
    """
    with write_header("Step 2: SSH Configuration"):
        ssh_connected = check_ssh_connection()
        
        if ssh_connected:
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} SSH authentication already working")
            return True
        else:
            # If we added a key in Step 1, configure SSH config for it
            if selected_ssh_key_path and selected_ssh_key_path != "GENERATE_NEW":
                private_key_path = selected_ssh_key_path.replace(".pub", "")
                if setup_ssh_config(private_key_path):
                    print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} SSH config configured")
                else:
                    print(f"{COLOR_YELLOW}⚠{COLOR_RESET} SSH config update failed")
            
            # Test SSH connection
            print("Testing SSH connection...")
            
            # If we just uploaded a key, wait for GitHub to register it
            if ssh_key_uploaded:
                print("Waiting for GitHub to register the new SSH key...")
                time.sleep(2)
            
            ssh_connected = check_ssh_connection()
            if ssh_connected:
                print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} SSH authentication working")
                return True
            else:
                print(f"{COLOR_YELLOW}⚠{COLOR_RESET} SSH connection test failed")
                
                if ssh_key_uploaded:
                    # Key was just uploaded, might need time to propagate
                    print("The SSH key was added to GitHub, but the connection test failed.")
                    print("This may take a few moments to propagate across GitHub's servers.")
                    print("Retrying in 5 seconds...")
                    time.sleep(5)
                    
                    # Retry once
                    ssh_connected = check_ssh_connection()
                    if ssh_connected:
                        print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} SSH authentication working (after retry)")
                        return True
                    else:
                        print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Connection still not working. Please try again in a minute.")
                        print("If the issue persists, verify your SSH key at: https://github.com/settings/keys")
                        return False
                else:
                    # No key was uploaded, so connection failure is for a different reason
                    print("SSH connection is not working.")
                    if selected_ssh_key_path:
                        print("The selected SSH key may not be added to your GitHub account.")
                        print("Add it manually at: https://github.com/settings/keys")
                    else:
                        print("No SSH key was selected or uploaded.")
                        print("You may need to:")
                        print("  1. Add an existing SSH key to GitHub: https://github.com/settings/keys")
                        print("  2. Or run this command again and select a key to upload")
                    return False


def step_final_status_check() -> bool:
    """
    Step 3: Verify final status.
    
    Returns:
        bool: True if authentication setup is complete
    """
    # Re-check everything (before region so variables are in scope)
    final_gh_cli = check_github_cli()
    final_ssh = check_ssh_connection()
    final_connection_methods = check_connection_methods()
    ssh_status = final_connection_methods.get("ssh", {})
    https_status = final_connection_methods.get("https", {})
    
    with write_header("Step 3: Final Status Check"):
        if final_gh_cli["installed"] and final_gh_cli["authenticated"]:
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} GitHub CLI: Authenticated")
        else:
            print(f"{COLOR_RED}✗{COLOR_RESET} GitHub CLI: Not authenticated")
        
        if ssh_status.get("usable", False):
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} SSH: Available and usable")
        else:
            print(f"{COLOR_YELLOW}⚠{COLOR_RESET} SSH: Setup may need a moment to propagate")
        
        if https_status.get("usable", False):
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} HTTPS: Available and usable")
        else:
            print(f"{COLOR_YELLOW}⚠{COLOR_RESET} HTTPS: May require additional configuration")
    
    # Final summary (outside region)
    print()
    if final_gh_cli["authenticated"] and (ssh_status.get("usable", False) or final_ssh):
        print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Authentication setup complete!")
        return True
    else:
        print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Authentication setup mostly complete, but some aspects may need a moment to propagate.")
        return False

