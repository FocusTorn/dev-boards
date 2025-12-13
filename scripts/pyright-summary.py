#!/usr/bin/env python3
"""
Script to parse basedpyright --stats output and display formatted file statistics.

Usage:
    python scripts/pyright-summary.py              # Show all errors and warnings
    python scripts/pyright-summary.py err          # Show only files with errors
    python scripts/pyright-summary.py warn         # Show only files with warnings
    python scripts/pyright-summary.py --help        # Show help message
    # or from workspace root:
    uv run python scripts/pyright-summary.py
"""

import sys
import os
import subprocess
import re
import argparse
from pathlib import Path
from collections import defaultdict
from typing import Dict, Tuple, Optional

# Get workspace root from environment variable or detect from current directory
workspace_root = os.environ.get('WORKSPACE_ROOT')
if workspace_root:
    workspace_root = Path(workspace_root).resolve()
else:
    # Try to detect workspace root by looking for pyproject.toml
    current = Path(__file__).resolve().parent.parent
    if (current / "pyproject.toml").exists():
        workspace_root = current
    else:
        print("ERROR: WORKSPACE_ROOT environment variable not set and could not detect workspace root.", file=sys.stderr)
        print()
        print("  Please add WORKSPACE_ROOT to your workspace settings file:")
        print("    .vscode/settings.json (VS Code)")
        print("    .cursor/settings.json (Cursor)")
        print()
        print("  Add the following setting:")
        print('    "WORKSPACE_ROOT": "D:\\\\_dev\\\\_Projects\\\\dev-boards"')
        print()
        print("  Or set it as an environment variable in your shell.")
        sys.exit(1)

# Import terminal output utilities from outerm
_script_dir = Path(__file__).resolve().parent.parent
_shared_python = _script_dir / "___shared" / "shared-python"
if str(_shared_python) not in sys.path:
    sys.path.insert(0, str(_shared_python))

try:
    from outerm.terminal import Palette, error, warning
except ImportError:
    print("ERROR: Could not import terminal utilities from outerm", file=sys.stderr)
    sys.exit(1)

# Get color and icon constants from Palette
ERROR_COLOR = Palette.ERROR['ansi']
WARNING_COLOR = Palette.WARNING['ansi']
RESET = Palette.RESET
ICON_ERROR = Palette.ERROR['iChar']
ICON_WARNING = Palette.WARNING['iChar']


def run_basedpyright() -> str:
    """Run basedpyright and return output."""
    try:
        result = subprocess.run(
            ['uv', 'run', 'basedpyright', '-p', '.', '--stats'],
            cwd=workspace_root,
            capture_output=True,
            text=True,
            check=False  # Don't fail on errors, we want to parse the output
        )
        return result.stdout + result.stderr
    except Exception as e:
        print(f"ERROR: Failed to run basedpyright: {e}", file=sys.stderr)
        sys.exit(1)


def parse_basedpyright_output(output: str) -> Dict[str, Tuple[int, int]]:
    """
    Parse basedpyright output and return dict mapping file paths to (error_count, warning_count).
    
    Returns:
        Dict with keys as absolute file paths, values as (error_count, warning_count) tuples
    """
    file_stats: Dict[str, Tuple[int, int]] = defaultdict(lambda: (0, 0))
    
    # Pattern to match error/warning lines
    # Format: "  d:\_dev\_Projects\dev-boards\path\to\file.py:123:45 - error/warning: message"
    # Or: "d:\_dev\_Projects\dev-boards\path\to\file.py:123:45 - error/warning: message"
    error_pattern = re.compile(r'^[\s]*(.+?):(\d+):(\d+)\s+-\s+(error|warning):')
    
    lines = output.split('\n')
    
    for line in lines:
        match = error_pattern.match(line)
        if match:
            file_path = match.group(1)
            issue_type = match.group(4)
            
            error_count, warning_count = file_stats[file_path]
            if issue_type == 'error':
                error_count += 1
            elif issue_type == 'warning':
                warning_count += 1
            file_stats[file_path] = (error_count, warning_count)
    
    return file_stats


def format_output(file_stats: Dict[str, Tuple[int, int]], filter_type: Optional[str] = None) -> None:
    """
    Format and print the file statistics with proper alignment and colors.
    
    Args:
        file_stats: Dictionary mapping file paths to (error_count, warning_count) tuples
        filter_type: Optional filter type - 'err' to show only errors, 'warn' to show only warnings
    
    Format:
        ✗:N  ⚠:M  path/to/file.py
    
    Where:
        - ✗:N is aligned (error count in red)
        - ⚠:M is aligned 2 spaces left of longest ✗:N (warning count in yellow)
        - path is aligned 2 spaces left of longest ⚠:M (relative path)
    """
    # Filter based on filter_type
    if filter_type == 'err':
        # Show only files with errors
        file_stats = {path: (err, warn) for path, (err, warn) in file_stats.items() if err > 0}
    elif filter_type == 'warn':
        # Show only files with warnings
        file_stats = {path: (err, warn) for path, (err, warn) in file_stats.items() if warn > 0}
    else:
        # Show files with either errors or warnings
        file_stats = {path: (err, warn) for path, (err, warn) in file_stats.items() if err > 0 or warn > 0}
    
    if not file_stats:
        if filter_type == 'err':
            print("No files with errors found.")
        elif filter_type == 'warn':
            print("No files with warnings found.")
        else:
            print("No errors or warnings found.")
        return
    
    # Convert absolute paths to relative paths
    # workspace_root is guaranteed to be a Path at this point (checked in main)
    assert workspace_root is not None, "workspace_root must be set"
    workspace_path = workspace_root
    
    relative_stats: Dict[str, Tuple[int, int]] = {}
    for abs_path, (err, warn) in file_stats.items():
        try:
            rel_path = Path(abs_path).relative_to(workspace_path)
            # Convert to forward slashes for display (works on Windows too)
            rel_path_str = str(rel_path).replace('\\', '/')
            relative_stats[rel_path_str] = (err, warn)
        except ValueError:
            # Path is not under workspace root, use as-is
            relative_stats[abs_path] = (err, warn)
    
    # Calculate maximum widths for alignment
    max_error_width = max(len(f"✗:{err}") for err, _ in relative_stats.values()) if relative_stats else 0
    max_warning_width = max(len(f"⚠:{warn}") for _, warn in relative_stats.values()) if relative_stats else 0
    
    # Sort alphabetically by path
    sorted_files = sorted(
        relative_stats.items(),
        key=lambda x: x[0]  # Sort by path (first element of tuple)
    )
    
    # Print formatted output
    for rel_path, (err_count, warn_count) in sorted_files:
        if filter_type == 'err':
            # Show only error count
            error_str = f"✗:{err_count}"
            error_formatted = f"{ERROR_COLOR}{error_str}{RESET}"
            error_padding = max_error_width - len(error_str)
            line = f"{error_formatted}{' ' * error_padding}  {rel_path}"
        elif filter_type == 'warn':
            # Show only warning count
            warning_str = f"⚠:{warn_count}"
            warning_formatted = f"{WARNING_COLOR}{warning_str}{RESET}"
            warning_padding = max_warning_width - len(warning_str)
            line = f"{warning_formatted}{' ' * warning_padding}  {rel_path}"
        else:
            # Show both error and warning counts
            error_str = f"✗:{err_count}"
            error_formatted = f"{ERROR_COLOR}{error_str}{RESET}"
            error_padding = max_error_width - len(error_str)
            
            warning_str = f"⚠:{warn_count}"
            warning_formatted = f"{WARNING_COLOR}{warning_str}{RESET}"
            warning_padding = max_warning_width - len(warning_str)
            
            # Build the line with proper alignment
            # Format: ✗:N  ⚠:M  path
            # ⚠:M starts 2 spaces after the longest ✗:N column ends
            # path starts 2 spaces after the longest ⚠:M column ends
            # Note: ANSI codes don't affect string length for padding calculations
            line = (
                f"{error_formatted}{' ' * error_padding}  "
                f"{warning_formatted}{' ' * warning_padding}  "
                f"{rel_path}"
            )
        print(line)


def parse_arguments() -> argparse.Namespace:
    """Parse command-line arguments."""
    parser = argparse.ArgumentParser(
        description='Parse basedpyright --stats output and display formatted file statistics.',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python scripts/pyright-summary.py              Show all errors and warnings
  python scripts/pyright-summary.py err         Show only files with errors
  python scripts/pyright-summary.py warn        Show only files with warnings
        """
    )
    
    parser.add_argument(
        'filter',
        nargs='?',
        choices=['err', 'warn'],
        help='Filter output: "err" to show only errors, "warn" to show only warnings'
    )
    
    return parser.parse_args()


def main():
    """Main entry point."""
    # Parse command-line arguments
    args = parse_arguments()
    filter_type = args.filter
    
    # Run basedpyright
    output = run_basedpyright()
    
    # Check if we got any meaningful output
    if not output or len(output.strip()) < 100:
        print("ERROR: basedpyright did not produce expected output.", file=sys.stderr)
        print("The command may have failed. Check that basedpyright is installed and can run.", file=sys.stderr)
        sys.exit(1)
    
    # Parse output
    file_stats = parse_basedpyright_output(output)
    
    # Format and display
    format_output(file_stats, filter_type=filter_type)


if __name__ == '__main__':
    main()

