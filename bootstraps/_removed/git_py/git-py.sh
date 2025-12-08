#!/bin/bash
# Wrapper script for git_py portable CLI tool
# Place this script in your PATH or project root

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Path to main.py (assuming git-py.sh is in the same directory as git_py/)
MAIN_SCRIPT="$SCRIPT_DIR/git_py/main.py"

# If git-py.sh is inside git_py directory, adjust path
if [ ! -f "$MAIN_SCRIPT" ]; then
    MAIN_SCRIPT="$(dirname "$SCRIPT_DIR")/git_py/main.py"
fi

# If still not found, try current directory
if [ ! -f "$MAIN_SCRIPT" ]; then
    MAIN_SCRIPT="$(pwd)/git_py/main.py"
fi

if [ ! -f "$MAIN_SCRIPT" ]; then
    echo "Error: Could not find git_py/main.py" >&2
    echo "Make sure git-py.sh is in the same directory as git_py/" >&2
    exit 1
fi

# Run the script
python "$MAIN_SCRIPT" "$@"

