"""
System check utilities for GitHub CLI, SSH, Git, and related tools.
"""

import subprocess
import platform
import json as json_lib
import urllib.request
import ssl
from pathlib import Path
from typing import Dict, Optional, Tuple

from .terminal import write_boxed_header, write_header, COLOR_GREEN, COLOR_YELLOW, COLOR_RED, BOLD_CHECK


def is_windows() -> bool:
    """Check if running on Windows."""
    return platform.system() == "Windows"


def check_command_exists(cmd: str) -> bool:
    """Check if a command exists in PATH."""
    try:
        if is_windows():
            # On Windows, use 'where' or try running with .exe
            result = subprocess.run(
                ["where", cmd] if cmd != "git" else ["git", "version"],
                capture_output=True,
                timeout=5,
                check=False
            )
            return result.returncode == 0
        else:
            # On Linux/Unix, use 'which'
            result = subprocess.run(
                ["which", cmd],
                capture_output=True,
                timeout=5,
                check=False
            )
            return result.returncode == 0
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return False


def check_github_cli() -> Dict[str, bool]:
    """Check GitHub CLI installation and authentication status."""
    status = {"installed": False, "authenticated": False}
    
    if not check_command_exists("gh"):
        return status
    
    status["installed"] = True
    
    # Check authentication
    try:
        result = subprocess.run(
            ["gh", "auth", "status"],
            capture_output=True,
            timeout=10,
            text=True
        )
        status["authenticated"] = result.returncode == 0
    except (subprocess.TimeoutExpired, FileNotFoundError):
        pass
    
    return status


def get_github_username() -> Optional[str]:
    """Get GitHub username from GitHub CLI or Git config."""
    # Try GitHub CLI first
    if check_command_exists("gh"):
        try:
            result = subprocess.run(
                ["gh", "api", "user", "--jq", ".login"],
                capture_output=True,
                timeout=10,
                text=True
            )
            if result.returncode == 0 and result.stdout.strip():
                return result.stdout.strip()
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass
    
    # Fallback to Git config (might be set to GitHub username)
    git_config = check_git_config()
    if git_config.get("name"):
        # Sometimes the git name is the GitHub username
        return git_config["name"]
    
    return None


def check_remote_repo_exists(github_user: str, repo_name: str) -> Tuple[bool, Optional[str]]:
    """
    Check if a remote GitHub repository exists and return its visibility.
    
    Returns:
        tuple: (exists: bool, visibility: Optional[str]) where visibility is 'private' or 'public'
    """
    if not check_command_exists("gh"):
        return False, None
    
    try:
        result = subprocess.run(
            ["gh", "repo", "view", f"{github_user}/{repo_name}", "--json", "visibility"],
            capture_output=True,
            timeout=10,
            text=True
        )
        if result.returncode == 0:
            # Parse JSON to get visibility
            repo_data = json_lib.loads(result.stdout)
            visibility = repo_data.get("visibility", "public")
            return True, visibility
        return False, None
    except (subprocess.TimeoutExpired, FileNotFoundError, ValueError):
        return False, None


def check_local_repo_exists(local_path: str) -> bool:
    """Check if a local git repository exists at the given path."""
    path = Path(local_path).resolve()
    if not path.exists():
        return False
    
    # Check if it's actually a git repository
    # Must have .git directory (not just a file)
    git_dir = path / ".git"
    if git_dir.exists() and git_dir.is_dir():
        # Additional check: verify it's a valid git repo by checking for common git files
        # This prevents false positives for directories that happen to have a .git folder
        if (git_dir / "HEAD").exists() or (git_dir / "config").exists():
            return True
    
    return False


def check_ssh_connection() -> bool:
    """Test SSH connection to GitHub."""
    ssh_dir = Path.home() / ".ssh"
    known_hosts_path = ssh_dir / "known_hosts"
    has_known_host = False
    
    if known_hosts_path.exists():
        try:
            content = known_hosts_path.read_text()
            if "github.com" in content:
                has_known_host = True
        except Exception:
            pass
    
    if not has_known_host:
        return False
    
    try:
        result = subprocess.run(
            ["ssh", "-o", "BatchMode=yes", "-o", "ConnectTimeout=5", "-T", "git@github.com"],
            capture_output=True,
            timeout=10
        )
        # Exit code 1 means authentication succeeded (GitHub returns success message)
        return result.returncode == 1
    except Exception:
        return False


def check_https_connection() -> Dict[str, bool]:
    """Check HTTPS connection and usability to GitHub."""
    status = {"reachable": False, "usable": False}
    
    # Check if we can reach GitHub via HTTPS
    try:
        # Try to connect to GitHub API (lightweight check)
        context = ssl.create_default_context()
        req = urllib.request.Request("https://api.github.com", method="HEAD")
        req.add_header("User-Agent", "Git-Sparse-Checkout-Setup")
        
        with urllib.request.urlopen(req, context=context, timeout=5) as response:
            status["reachable"] = True
            status["usable"] = response.status in [200, 401, 403]  # Any response means it's usable
    except Exception:
        # If HTTPS fails, check if git can use HTTPS (credentials might be needed)
        try:
            result = subprocess.run(
                ["git", "ls-remote", "--heads", "https://github.com/octocat/Hello-World.git"],
                capture_output=True,
                timeout=5
            )
            # If we can at least connect (even if auth fails), HTTPS is usable
            status["reachable"] = True
            status["usable"] = result.returncode in [0, 128]  # 0 = success, 128 = repo not found but connection works
        except Exception:
            pass
    
    return status


def check_connection_methods() -> Dict[str, Dict[str, bool]]:
    """Check both SSH and HTTPS connection methods."""
    ssh_connected = check_ssh_connection()
    https_status = check_https_connection()
    
    return {
        "ssh": {
            "available": ssh_connected,
            "usable": ssh_connected
        },
        "https": {
            "available": https_status["reachable"],
            "usable": https_status["usable"]
        }
    }


def check_ssh_keys() -> Tuple[bool, int]:
    """Check if SSH keys exist."""
    ssh_dir = Path.home() / ".ssh"
    if not ssh_dir.exists():
        return False, 0
    
    key_count = 0
    for key_file in ssh_dir.glob("*"):
        if (key_file.suffix == ".pub" or 
            key_file.name in ["known_hosts", "config", "authorized_keys"] or
            key_file.name.endswith(".bak")):
            continue
        
        pub_key_path = key_file.with_suffix(".pub")
        if pub_key_path.exists():
            key_count += 1
    
    return key_count > 0, key_count


def check_git_crypt() -> Dict[str, bool]:
    """Check git-crypt installation and configuration."""
    status = {"installed": False, "configured": False, "locked": False}
    
    if not check_command_exists("git-crypt"):
        return status
    
    status["installed"] = True
    
    # Check if git-crypt is configured in current repo
    try:
        result = subprocess.run(
            ["git", "rev-parse", "--git-dir"],
            capture_output=True,
            timeout=5
        )
        if result.returncode == 0:
            git_dir = Path(result.stdout.decode().strip())
            if git_dir.is_absolute():
                repo_root = git_dir.parent
            else:
                repo_root = Path.cwd() / git_dir.parent
            
            git_crypt_path = repo_root / ".git-crypt"
            status["configured"] = git_crypt_path.exists()
            
            if status["configured"]:
                # Check if locked
                try:
                    crypt_result = subprocess.run(
                        ["git-crypt", "status"],
                        capture_output=True,
                        timeout=5,
                        text=True
                    )
                    output = crypt_result.stdout.lower()
                    status["locked"] = "not unlocked" in output or "locked" in output or "no key" in output
                except Exception:
                    pass
    except Exception:
        pass
    
    return status


def check_git_config() -> Dict[str, Optional[str]]:
    """Check Git configuration."""
    config = {"name": None, "email": None}
    
    try:
        result = subprocess.run(
            ["git", "config", "--global", "user.name"],
            capture_output=True,
            timeout=5,
            text=True
        )
        if result.returncode == 0:
            config["name"] = result.stdout.strip()
    except Exception:
        pass
    
    try:
        result = subprocess.run(
            ["git", "config", "--global", "user.email"],
            capture_output=True,
            timeout=5,
            text=True
        )
        if result.returncode == 0:
            config["email"] = result.stdout.strip()
    except Exception:
        pass
    
    return config

