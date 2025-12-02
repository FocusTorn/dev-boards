#!/usr/bin/env python3
"""
Generate sparse checkout setup command with prerequisite status check.
Non-interactive script that checks prerequisites and outputs the command to run.
Cross-platform (Windows/Linux/Debian).
"""

import sys
import argparse
from pathlib import Path

# Fix Windows console encoding for Unicode box-drawing characters
if sys.platform == "win32":
    import io
    # Set stdout/stderr to UTF-8 encoding
    if hasattr(sys.stdout, 'reconfigure'):
        sys.stdout.reconfigure(encoding='utf-8')
        sys.stderr.reconfigure(encoding='utf-8')
    else:
        # Fallback for older Python versions
        sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
        sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')

# Handle both package import and direct execution
if __name__ == "__main__":
    # When run directly, add parent directory to path
    import os
    parent_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    if parent_dir not in sys.path:
        sys.path.insert(0, parent_dir)
    from git_py.core.checks import is_windows
    from git_py.commands import cmd_status, cmd_auth, cmd_init
else:
    # When imported as package, use relative imports
    from .core.checks import is_windows
    from .commands import cmd_status, cmd_auth, cmd_init


def print_template():
    """Print a copy-pasteable template for the command."""
    script_name = "github_bootstrapper.py"  # Main script name
    is_windows_system = is_windows()
    
    if is_windows_system:
        # Windows PowerShell/CMD format
        template = f'''# Windows (PowerShell/CMD) Template
python bootstraps\\{script_name} init \\
  --github-user "your-username" \\
  --repo-name "your-repo-name" \\
  --workspace-root "WS1-Root" \\
  --local-folder "local1" \\
  --output-json > config.json

# Using HTTPS instead of SSH:
python bootstraps\\{script_name} init \\
  --github-user "your-username" \\
  --repo-name "your-repo-name" \\
  --use-https \\
  --workspace-root "WS1-Root" \\
  --local-folder "local1" \\
  --output-json > config.json'''
    else:
        # Linux/Unix format
        template = f'''# Linux/Debian Template
python3 bootstraps/{script_name} init \\
  --github-user "your-username" \\
  --repo-name "your-repo-name" \\
  --workspace-root "WS1-Root" \\
  --local-folder "local1" \\
  --output-json > config.json

# Using HTTPS instead of SSH:
python3 bootstraps/{script_name} init \\
  --github-user "your-username" \\
  --repo-name "your-repo-name" \\
  --use-https \\
  --workspace-root "WS1-Root" \\
  --local-folder "local1" \\
  --output-json > config.json'''
    
    print(template)
    print()
    print("# After generating config.json, run setup:")
    if is_windows_system:
        print("python bootstraps\\git-py\\setup-github-cwd.py --command-file config.json --force")
    else:
        print("python3 bootstraps/git-py/setup-github-cwd.py --command-file config.json --force")


def main():
    """Main entry point."""
    # If no arguments provided, show template
    if len(sys.argv) == 1:
        print_template()
        return 0
    
    parser = argparse.ArgumentParser(
        description="Initialize GitHub remote and local repositories based on current working directory",
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    
    subparsers = parser.add_subparsers(
        dest="command",
        help="Subcommands",
        metavar="COMMAND",
        required=True
    )
    
    # Init subcommand
    gen_parser = subparsers.add_parser(
        "init",
        help="Initialize remote and local repos based on current working directory",
        description="Initialize GitHub remote repository and local repository based on current working directory"
    )
    gen_parser.add_argument(
        "--github-user",
        required=False,
        help="GitHub username or organization name (prompted if not provided)"
    )
    gen_parser.add_argument(
        "--repo-name",
        required=False,
        help="Repository name (prompted if not provided)"
    )
    gen_parser.add_argument(
        "--use-https",
        action="store_true",
        help="Use HTTPS URL instead of SSH (default: SSH)"
    )
    gen_parser.add_argument(
        "--workspace-root",
        required=False,
        help="Root directory for the workspace (prompted if not provided)"
    )
    gen_parser.add_argument(
        "--local-folder",
        required=False,
        help="Local folder name to create (prompted if not provided)"
    )
    gen_parser.add_argument(
        "--output-json",
        action="store_true",
        help="Output command as JSON instead of shell command"
    )
    
    # Template subcommand
    subparsers.add_parser(
        "template",
        help="Show copy-pasteable command template",
        description="Output a copy-pasteable template for generating setup commands"
    )
    
    # Status subcommand
    subparsers.add_parser(
        "status",
        help="Show comprehensive status of all system aspects",
        description="Display status of GitHub CLI, SSH, HTTPS, Git-Crypt, and Git configuration"
    )
    
    # Auth subcommand
    subparsers.add_parser(
        "auth",
        help="Automatically set up and authenticate all aspects",
        description="Automatically authenticate GitHub CLI, set up SSH keys, and configure authentication"
    )
    
    args = parser.parse_args()
    
    # Handle subcommands
    if args.command == "template":
        print_template()
        return 0
    elif args.command == "status":
        return cmd_status()
    elif args.command == "auth":
        return cmd_auth()
    elif args.command == "init":
        return cmd_init(args)
    else:
        parser.print_help()
        return 1


if __name__ == "__main__":
    sys.exit(main())

