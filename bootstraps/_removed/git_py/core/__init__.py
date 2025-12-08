"""
Core utilities for terminal output, prompts, and system checks.
"""

from .terminal import (
    write_boxed_header, write_header, write_header_fat,
    COLOR_GREEN, COLOR_YELLOW, COLOR_RED, COLOR_RESET, COLOR_DIM, BOLD_CHECK,
    get_region_indent
)
from .prompts import confirm, HAS_PROMPT_TOOLKIT
from .checks import (
    is_windows, check_command_exists, check_github_cli,
    get_github_username, check_remote_repo_exists, check_local_repo_exists,
    check_ssh_connection, check_https_connection, check_connection_methods,
    check_ssh_keys, check_git_crypt, check_git_config
)

__all__ = [
    'write_boxed_header', 'write_header', 'write_header_fat',
    'COLOR_GREEN', 'COLOR_YELLOW', 'COLOR_RED', 'COLOR_RESET', 'COLOR_DIM', 'BOLD_CHECK',
    'get_region_indent',
    'confirm', 'HAS_PROMPT_TOOLKIT',
    'is_windows', 'check_command_exists', 'check_github_cli',
    'get_github_username', 'check_remote_repo_exists', 'check_local_repo_exists',
    'check_ssh_connection', 'check_https_connection', 'check_connection_methods',
    'check_ssh_keys', 'check_git_crypt', 'check_git_config',
]

