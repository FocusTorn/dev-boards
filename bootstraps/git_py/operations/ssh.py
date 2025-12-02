"""
SSH key management and GitHub CLI authentication utilities.
"""

import os
import sys
import subprocess
from pathlib import Path
from typing import List, Dict

from ..core.terminal import COLOR_GREEN, COLOR_YELLOW, COLOR_RED, COLOR_RESET, BOLD_CHECK
from ..core.checks import check_github_cli, is_windows


def discover_ssh_keys() -> List[Dict[str, str]]:
    """Discover existing SSH key pairs (both private and public key must exist)."""
    ssh_dir = Path.home() / ".ssh"
    existing_keys = []
    
    if not ssh_dir.exists():
        return existing_keys
    
    # Look for public key files (.pub) and verify the corresponding private key exists
    for pub_key_file in ssh_dir.glob("*.pub"):
        # Skip known files
        if pub_key_file.name in ["known_hosts.pub", "config.pub", "authorized_keys.pub"]:
            continue
        
        # Check if corresponding private key exists (same name without .pub extension)
        private_key_path = pub_key_file.with_suffix("")
        if not private_key_path.exists():
            continue
        
        key_info = {
            "name": pub_key_file.stem,  # Name without .pub extension
            "private_path": str(private_key_path),
            "public_path": str(pub_key_file),  # The .pub file path
            "fingerprint": "",
            "comment": ""
        }
        
        try:
            pub_key_content = pub_key_file.read_text().strip()
            parts = pub_key_content.split()
            if len(parts) >= 3:
                key_info["comment"] = parts[2]
            
            result = subprocess.run(
                ["ssh-keygen", "-lf", str(pub_key_file)],
                capture_output=True,
                timeout=5,
                text=True
            )
            if result.returncode == 0:
                fingerprint_parts = result.stdout.strip().split()
                if len(fingerprint_parts) >= 2:
                    key_info["fingerprint"] = fingerprint_parts[1]
        except Exception:
            pass
        
        existing_keys.append(key_info)
    
    # Sort: github_ keys first, then alphabetically
    existing_keys.sort(key=lambda x: ("0" if x["name"].startswith("github_") else "1") + x["name"])
    return existing_keys


def add_github_to_known_hosts() -> bool:
    """Add GitHub's host key to known_hosts if not already present."""
    ssh_dir = Path.home() / ".ssh"
    known_hosts_path = ssh_dir / "known_hosts"
    
    # Check if github.com is already in known_hosts
    if known_hosts_path.exists():
        try:
            content = known_hosts_path.read_text()
            if "github.com" in content:
                return True
        except Exception:
            pass
    
    # GitHub's SSH host keys (as of 2024)
    github_host_keys = [
        "github.com ssh-rsa AAAAB3NzaC1yc2EAAAABIwAAAQEAq2A7hRGmdnm9tUDbO9IDSwBK6TbQa+PXYPCPy6rbTrTtw7PHkccKrpp0yVhp5HdEIcKr6pLlVDBfOLX9QUsyCOV0wzfjIJNlGEYsdlLJizHhbn2mUjvSAhq2qil3+CqX3pItNl1Dq0v3Xw==",
        "github.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl",
        "github.com ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBEmKSENjQEezOmxkZMy7opKgwFB9nkt5YRrYMjNuG5N87uRgg6CLrbo5wAdT/y6v0mKV0U2w0WZ2YB/++Tpockg=",
    ]
    
    try:
        ssh_dir.mkdir(mode=0o700, exist_ok=True)
        
        # Append GitHub host keys to known_hosts
        if known_hosts_path.exists():
            content = known_hosts_path.read_text()
            if not content.endswith("\n"):
                content += "\n"
        else:
            content = ""
        
        for key in github_host_keys:
            if key not in content:
                content += key + "\n"
        
        known_hosts_path.write_text(content)
        
        # Set permissions (Unix-like systems)
        if not is_windows():
            os.chmod(known_hosts_path, 0o644)
        
        return True
    except Exception as e:
        print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Warning: Could not add GitHub to known_hosts: {e}", file=sys.stderr)
        return False


def setup_ssh_config(key_path: str) -> bool:
    """Configure SSH config to use the specified key for GitHub."""
    ssh_config_path = Path.home() / ".ssh" / "config"
    ssh_dir = ssh_config_path.parent
    
    # Create .ssh directory if it doesn't exist
    ssh_dir.mkdir(mode=0o700, exist_ok=True)
    
    # Check if config already has GitHub entry
    github_config_exists = False
    if ssh_config_path.exists():
        try:
            content = ssh_config_path.read_text()
            if "Host github.com" in content:
                github_config_exists = True
                # Check if it already uses this key
                if key_path.replace("\\", "/") in content.replace("\\", "/"):
                    print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} SSH config already configured for this key")
                    return True
        except Exception:
            pass
    
    # Build GitHub config block
    # Use ~/.ssh/ format for cross-platform compatibility
    key_path_rel = key_path.replace(str(Path.home()), "~").replace("\\", "/")
    
    github_config = f'''
Host github.com
    HostName github.com
    User git
    IdentityFile {key_path_rel}
    IdentitiesOnly yes
'''
    
    try:
        if github_config_exists:
            # Update existing config
            lines = ssh_config_path.read_text().splitlines()
            new_lines = []
            skip = False
            for line in lines:
                if line.strip() == "Host github.com":
                    skip = True
                    new_lines.append(github_config.strip())
                elif skip and line.strip().startswith("Host "):
                    skip = False
                    new_lines.append(line)
                elif not skip:
                    new_lines.append(line)
            
            if skip:
                # Remove trailing blank lines if we were in a GitHub block
                while new_lines and not new_lines[-1].strip():
                    new_lines.pop()
            
            ssh_config_path.write_text("\n".join(new_lines) + "\n")
        else:
            # Append to config
            if ssh_config_path.exists():
                content = ssh_config_path.read_text()
                if not content.endswith("\n"):
                    content += "\n"
                ssh_config_path.write_text(content + github_config)
            else:
                ssh_config_path.write_text(github_config)
        
        # Set permissions (Unix-like systems)
        if not is_windows():
            os.chmod(ssh_config_path, 0o600)
        
        print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} SSH config updated")
        return True
    except Exception as e:
        print(f"{COLOR_RED}✗{COLOR_RESET} Failed to update SSH config: {e}", file=sys.stderr)
        return False


def auto_github_cli_login(use_ssh: bool = True, silent: bool = False) -> bool:
    """
    Automatically handle GitHub CLI authentication if needed.
    
    Args:
        use_ssh: Whether SSH protocol should be used (default: True)
        silent: If True, suppress output messages (default: False)
    
    Returns:
        True if authenticated (was already authenticated or successfully authenticated), False otherwise
    
    Note: Output is automatically indented if called within a region context.
    """
    gh_cli = check_github_cli()
    
    if not gh_cli["installed"]:
        if not silent:
            print(f"{COLOR_RED}✗{COLOR_RESET} GitHub CLI is not installed", file=sys.stderr)
        return False
    
    if gh_cli["authenticated"]:
        return True
    
    # Need to authenticate
    if not silent:
        print(f"{COLOR_YELLOW}⚠{COLOR_RESET} GitHub CLI is not authenticated")
        print("Authenticating GitHub CLI via web browser...")
        print("(A browser window will open for authentication)")
    
    try:
        # Run gh auth login with web browser
        # All flags are set upfront to combine authentication and configuration into one step
        # This means the user only needs to copy the code once in the browser
        # --web: Use browser-based authentication
        # --skip-ssh-key: Skip SSH key upload prompt (we handle it separately)
        # --git-protocol ssh: Set Git protocol to SSH (no prompt)
        # --scopes: Request all needed scopes upfront (combines auth and scope selection)
        result = subprocess.run(
            ["gh", "auth", "login", "--web", "--skip-ssh-key", "--git-protocol", "ssh",
             "--scopes", "repo,read:org,admin:public_key,admin:gpg_key,workflow"],
            timeout=300  # 5 minutes for user to complete
        )
        
        if result.returncode != 0:
            if not silent:
                print(f"{COLOR_RED}✗{COLOR_RESET} GitHub CLI authentication failed or was cancelled", file=sys.stderr)
            return False
        
        # Verify authentication succeeded
        gh_cli_after = check_github_cli()
        if gh_cli_after["authenticated"]:
            if not silent:
                print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} GitHub CLI authenticated")
            return True
        else:
            if not silent:
                print(f"{COLOR_RED}✗{COLOR_RESET} GitHub CLI authentication verification failed", file=sys.stderr)
            return False
            
    except subprocess.TimeoutExpired:
        if not silent:
            print(f"{COLOR_RED}✗{COLOR_RESET} Authentication timed out", file=sys.stderr)
        return False
    except Exception as e:
        if not silent:
            print(f"{COLOR_RED}✗{COLOR_RESET} Error during authentication: {e}", file=sys.stderr)
        return False

