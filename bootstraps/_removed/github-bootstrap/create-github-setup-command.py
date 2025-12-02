#!/usr/bin/env python3
"""
Generate sparse checkout setup command with prerequisite status check.
Non-interactive script that checks prerequisites and outputs the command to run.
Cross-platform (Windows/Linux/Debian).

Main entry point - imports from git-py package.
"""

import sys
from pathlib import Path

# Add bootstraps directory to path so we can import git-py
bootstraps_dir = Path(__file__).parent
sys.path.insert(0, str(bootstraps_dir))

from git_py.main import main

if __name__ == "__main__":
    sys.exit(main())
