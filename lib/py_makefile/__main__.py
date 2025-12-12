"""
Entry point for running py_makefile as a module: python -m py_makefile
Note: This requires configuration from a project-specific wrapper.
"""

import sys
from .cli import run
from .exceptions import PmakeConfigError

# This module entry point requires project-specific configuration
# Project scripts should use run(config) directly

if __name__ == "__main__":
    print("py_makefile must be configured with project-specific settings.")
    print("Use a project-specific wrapper script or call run(config) directly.")
    sys.exit(1)

