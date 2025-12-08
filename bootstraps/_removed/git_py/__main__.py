"""
Entry point for running git_py as a module: python -m git_py
"""

import sys
from .main import main

if __name__ == "__main__":
    sys.exit(main())

