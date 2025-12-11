#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Display a file tree with modification timestamps and line counts.

Usage:
    python file_tree.py <directory>
    python file_tree.py <directory> --exclude-dirs .git,__pycache__,node_modules
"""

import sys
import os
import argparse
from pathlib import Path
from datetime import datetime
from typing import List, Set, Optional

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


def count_lines(file_path: Path) -> int:
    """Count the number of lines in a file."""
    try:
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            return sum(1 for _ in f)
    except (UnicodeDecodeError, PermissionError, IOError):
        # Binary file or permission denied
        return 0


def format_timestamp(mtime: float) -> str:
    """Format modification timestamp."""
    dt = datetime.fromtimestamp(mtime)
    return dt.strftime("%Y-%m-%d %H:%M:%S")


def format_size(size_bytes: int) -> str:
    """Format file size in human-readable format."""
    for unit in ['B', 'KB', 'MB', 'GB']:
        if size_bytes < 1024.0:
            return f"{size_bytes:.1f} {unit}"
        size_bytes /= 1024.0
    return f"{size_bytes:.1f} TB"


def get_file_info(file_path: Path) -> tuple:
    """Get file information: mtime, line count, size."""
    try:
        stat = file_path.stat()
        mtime = stat.st_mtime
        size = stat.st_size
        line_count = count_lines(file_path)
        return mtime, line_count, size
    except (OSError, PermissionError):
        return None, 0, 0


def should_exclude(path: Path, exclude_dirs: Set[str], exclude_patterns: List[str]) -> bool:
    """Check if path should be excluded."""
    # Check if any parent directory is excluded
    for part in path.parts:
        if part in exclude_dirs:
            return True
        # Check patterns
        for pattern in exclude_patterns:
            if pattern in part:
                return True
    return False


def print_tree(
    root: Path,
    prefix: str = "",
    is_last: bool = True,
    exclude_dirs: Optional[Set[str]] = None,
    exclude_patterns: Optional[List[str]] = None,
    show_hidden: bool = False
):
    """
    Print directory tree with file information.
    
    Args:
        root: Root directory path
        prefix: Prefix for current level (for tree drawing)
        is_last: Whether this is the last item at this level
        exclude_dirs: Set of directory names to exclude
        exclude_patterns: List of patterns to exclude
        show_hidden: Whether to show hidden files/directories
    """
    if exclude_dirs is None:
        exclude_dirs = set()
    if exclude_patterns is None:
        exclude_patterns = []
    
    # Skip hidden files/directories if not requested
    if not show_hidden and root.name.startswith('.'):
        return
    
    # Skip excluded directories
    if should_exclude(root, exclude_dirs, exclude_patterns):
        return
    
    # Get connector for tree structure
    connector = "└── " if is_last else "├── "
    
    # Print current item
    if root.is_file():
        mtime, line_count, size = get_file_info(root)
        if mtime is not None:
            timestamp = format_timestamp(mtime)
            size_str = format_size(size)
            print(f"{prefix}{connector}{root.name:<50} [{timestamp}] [{line_count:>6} lines] [{size_str:>10}]")
        else:
            print(f"{prefix}{connector}{root.name} [Error reading file]")
    else:
        # Directory
        print(f"{prefix}{connector}{root.name}/")
    
    # Recurse into subdirectories
    if root.is_dir():
        try:
            # Get all items, sorted
            items = sorted(root.iterdir(), key=lambda x: (x.is_file(), x.name.lower()))
            
            # Filter items
            filtered_items = []
            for item in items:
                if not show_hidden and item.name.startswith('.'):
                    continue
                if should_exclude(item, exclude_dirs, exclude_patterns):
                    continue
                filtered_items.append(item)
            
            # Print each item
            for i, item in enumerate(filtered_items):
                is_last_item = (i == len(filtered_items) - 1)
                extension = "    " if is_last else "│   "
                print_tree(
                    item,
                    prefix + extension,
                    is_last_item,
                    exclude_dirs,
                    exclude_patterns,
                    show_hidden
                )
        except PermissionError:
            print(f"{prefix}{'    ' if is_last else '│   '}    [Permission denied]")


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Display file tree with modification timestamps and line counts",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python file_tree.py .
  python file_tree.py /path/to/dir
  python file_tree.py . --exclude-dirs .git,__pycache__,node_modules
  python file_tree.py . --show-hidden
        """
    )
    
    parser.add_argument(
        "directory",
        nargs="?",
        default=".",
        help="Directory to display (default: current directory)"
    )
    
    parser.add_argument(
        "--exclude-dirs",
        type=str,
        default=".git,__pycache__,node_modules,.venv,venv,env",
        help="Comma-separated list of directory names to exclude (default: .git,__pycache__,node_modules,.venv,venv,env)"
    )
    
    parser.add_argument(
        "--exclude-patterns",
        type=str,
        help="Comma-separated list of patterns to exclude from directory names"
    )
    
    parser.add_argument(
        "--show-hidden",
        action="store_true",
        help="Show hidden files and directories (starting with .)"
    )
    
    args = parser.parse_args()
    
    # Parse directory path
    root_dir = Path(args.directory).resolve()
    
    if not root_dir.exists():
        print(f"Error: Directory '{root_dir}' does not exist.", file=sys.stderr)
        return 1
    
    if not root_dir.is_dir():
        print(f"Error: '{root_dir}' is not a directory.", file=sys.stderr)
        return 1
    
    # Parse exclude directories
    exclude_dirs = set(args.exclude_dirs.split(',')) if args.exclude_dirs else set()
    exclude_dirs = {d.strip() for d in exclude_dirs if d.strip()}
    
    # Parse exclude patterns
    exclude_patterns = []
    if args.exclude_patterns:
        exclude_patterns = [p.strip() for p in args.exclude_patterns.split(',') if p.strip()]
    
    # Print header
    print(f"File Tree: {root_dir}")
    print("=" * 80)
    print()
    
    # Print tree
    print_tree(
        root_dir,
        exclude_dirs=exclude_dirs,
        exclude_patterns=exclude_patterns,
        show_hidden=args.show_hidden
    )
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

