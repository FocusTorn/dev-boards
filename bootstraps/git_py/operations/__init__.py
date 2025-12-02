"""
Operations for SSH and repository management.
"""

from .ssh import (
    discover_ssh_keys, add_github_to_known_hosts,
    setup_ssh_config, auto_github_cli_login
)
from .repository import (
    build_remote_repo_url, setup_local_repository, setup_remote_repository
)

__all__ = [
    'discover_ssh_keys', 'add_github_to_known_hosts',
    'setup_ssh_config', 'auto_github_cli_login',
    'build_remote_repo_url', 'setup_local_repository', 'setup_remote_repository',
]

