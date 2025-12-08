"""
Status command handler - show comprehensive status of all aspects.
"""

from ..core.terminal import (
    write_boxed_header, write_header, COLOR_GREEN, COLOR_YELLOW, COLOR_RED,
    COLOR_RESET, BOLD_CHECK
)
from ..core.checks import (
    check_github_cli, check_ssh_connection, check_ssh_keys,
    check_git_crypt, check_git_config, check_connection_methods
)


def cmd_status():
    """Handle the status subcommand - show comprehensive status of all aspects."""
    write_boxed_header("SYSTEM STATUS REPORT")
    print()  # Blank line after boxed header
    
    # Check all aspects
    gh_cli = check_github_cli()
    ssh_connected = check_ssh_connection()
    ssh_keys_exist, ssh_key_count = check_ssh_keys()
    git_crypt = check_git_crypt()
    git_config = check_git_config()
    connection_methods = check_connection_methods()
    
    # Connection Methods
    with write_header("Connection Methods"):
        ssh_status = connection_methods.get("ssh", {})
        https_status = connection_methods.get("https", {})
        
        if ssh_status.get("usable", False):
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} SSH: Fully available and usable")
        elif ssh_status.get("available", False):
            print(f"{COLOR_YELLOW}⚠{COLOR_RESET} SSH: Available but not usable (needs configuration)")
        else:
            print(f"{COLOR_RED}✗{COLOR_RESET} SSH: Not available (needs setup)")
        
        if https_status.get("usable", False):
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} HTTPS: Fully available and usable")
        elif https_status.get("available", False):
            print(f"{COLOR_YELLOW}⚠{COLOR_RESET} HTTPS: Available but may require authentication")
        else:
            print(f"{COLOR_RED}✗{COLOR_RESET} HTTPS: Not available (needs setup)")
    
    # GitHub CLI
    with write_header("GitHub CLI"):
        if gh_cli["installed"] and gh_cli["authenticated"]:
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Fully installed, authenticated, and usable")
        elif gh_cli["installed"]:
            print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Installed but not authenticated (run: gh auth login)")
        else:
            print(f"{COLOR_RED}✗{COLOR_RESET} Not installed (needs setup)")
    
    # SSH Authentication
    with write_header("SSH Authentication"):
        if ssh_connected:
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Fully configured and working")
        elif ssh_keys_exist:
            print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Keys exist ({ssh_key_count} key(s)) but connection failed (needs configuration)")
        else:
            print(f"{COLOR_RED}✗{COLOR_RESET} No SSH keys found (needs setup)")
    
    # SSH Keys
    with write_header("SSH Keys"):
        if ssh_keys_exist:
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Found {ssh_key_count} key pair(s)")
        else:
            print(f"{COLOR_RED}✗{COLOR_RESET} No SSH keys found (needs setup)")
    
    # Git-Crypt
    with write_header("Git-Crypt"):
        if git_crypt["installed"]:
            if git_crypt["configured"]:
                if git_crypt["locked"]:
                    print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Installed and configured but locked (key needed)")
                else:
                    print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Installed, configured, and unlocked")
            else:
                print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Installed but not configured (needs initialization)")
        else:
            print("ℹ Not installed (optional)")
    
    # Git Config
    with write_header("Git Configuration"):
        if git_config["name"] and git_config["email"]:
            print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Fully configured: Name='{git_config['name']}', Email='{git_config['email']}'")
        elif git_config["name"] or git_config["email"]:
            missing = []
            if not git_config["name"]:
                missing.append("name")
            if not git_config["email"]:
                missing.append("email")
            print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Partially configured (missing: {', '.join(missing)})")
        else:
            print(f"{COLOR_RED}✗{COLOR_RESET} Not configured (needs setup)")
    
    return 0

