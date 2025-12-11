# Installing git_py for Global Use

To use `git_py` from anywhere with `uv run python -m git_py`, you need to install it in editable mode.

## Installation Steps

### Option 1: Install from Workspace Root (Recommended)

From the workspace root (`D:\_dev\projects\dev-boards`):

```powershell
# Sync workspace dependencies first
uv sync

# Install git_py in editable mode
uv pip install -e bootstraps/git_py
```

### Option 2: Install from Package Directory

From the package directory (`bootstraps/git_py`):

```powershell
# Make sure you're in the workspace virtual environment
cd D:\_dev\projects\dev-boards
uv sync

# Then install the package
cd bootstraps/git_py
uv pip install -e .
```

## Verify Installation

After installation, you should be able to run from anywhere:

```powershell
# From any directory
uv run python -m git_py --help
uv run python -m git_py init
uv run python -m git_py status
uv run python -m git_py auth
```

## Troubleshooting

If you get "No module named git_py":
1. Make sure UV is installed and in your PATH
2. Run `uv sync` from the workspace root first
3. Install the package with `uv pip install -e bootstraps/git_py`
4. Verify with `uv run python -c "import git_py; print(git_py.__version__)"`

## Uninstall

To uninstall:

```powershell
uv pip uninstall git-py
```

