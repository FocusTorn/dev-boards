#!/usr/bin/env python3
"""
GitHub Repository Bootstrapper

Main entry point for initializing GitHub remote and local repositories.
Cross-platform (Windows/Linux/Debian).

This script orchestrates the GitHub repository setup process by delegating
to the git_py package modules.
"""

import sys
from pathlib import Path

# Add bootstraps directory to path so we can import git-py
bootstraps_dir = Path(__file__).parent
sys.path.insert(0, str(bootstraps_dir))

from git_py.main import main

if __name__ == "__main__":
    sys.exit(main())
