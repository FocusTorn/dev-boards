"""
Authentication command handler - automatically set up and authenticate everything.
"""

from ..core.terminal import write_boxed_header
from ..core.checks import check_github_cli, check_ssh_connection
from .auth_ssh_selection import select_ssh_key
from .auth_steps import step_github_cli_auth, step_ssh_configuration, step_final_status_check


def cmd_auth():
    """Handle the auth subcommand - automatically set up and authenticate everything."""
    write_boxed_header("AUTOMATIC AUTHENTICATION SETUP")
    
    
    # Step 0: Pre-select SSH key (before authentication)
    # This allows us to use our own prompts and then add the key after auth
    # Check SSH connection first - if already working, skip key selection
    ssh_connected_initially = check_ssh_connection()
    selected_ssh_key_path = None
    new_key_name = None
    new_key_overwrite = False
    ssh_key_uploaded = False
    gh_cli = check_github_cli()
    
    # Only prompt for SSH key if SSH is not already working
    if not ssh_connected_initially:
        selected_ssh_key_path, new_key_name, new_key_overwrite = select_ssh_key()
        if selected_ssh_key_path is None and new_key_name is None:
            # User cancelled
            return 1
    
    # Step 1: GitHub CLI authentication and SSH key setup
    success, ssh_key_uploaded, selected_ssh_key_path = step_github_cli_auth(
        gh_cli, selected_ssh_key_path, new_key_name, new_key_overwrite
    )
    if not success:
        return 1
    
    # Step 2: Configure SSH and test connection
    ssh_working = step_ssh_configuration(selected_ssh_key_path, ssh_key_uploaded)
    
    # Step 3: Verify final status
    step_final_status_check()
    
    return 0
