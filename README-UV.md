# UV Workspace Setup

This workspace uses UV for dependency management with a hybrid approach:

## Architecture

- **Workspace-level venv**: Shared dependencies (like `prompt-toolkit`) used across multiple projects
- **Project-level venvs**: Individual projects can extend the workspace dependencies with their own specific needs

## Setup

### 1. Install UV

```powershell
# Windows (PowerShell)
irm https://astral.sh/uv/install.ps1 | iex

# Or using pip
pip install uv
```

### 2. Initialize Workspace Environment

From the workspace root (`D:\_dev\projects\dev-boards`):

```powershell
# Create workspace-level virtual environment
uv venv

# Activate the workspace venv
.\.venv\Scripts\Activate.ps1

# Install workspace dependencies
uv sync
```

### 3. Working with Projects

#### Install workspace + project dependencies:
```powershell
# From workspace root
uv sync

# Or from project directory
cd bootstraps\git_py
uv sync
```

#### Run scripts:
```powershell
# Using UV (recommended - uses workspace venv)
uv run python bootstraps\git_py\main.py init

# Or activate venv first
.\.venv\Scripts\Activate.ps1
python bootstraps\git_py\main.py init
```

## Adding New Dependencies

### Workspace-level (shared across all projects):
Edit `pyproject.toml` at the root:
```toml
[project]
dependencies = [
    "prompt-toolkit>=3.0.0",
    "new-package>=1.0.0",  # Add here
]
```

Then run: `uv sync`

### Project-level (specific to git_py):
Edit `bootstraps/git_py/pyproject.toml`:
```toml
[project]
dependencies = [
    "project-specific-package>=1.0.0",  # Add here
]
```

Then run: `uv sync`

## Benefits

✅ **Shared dependencies**: Common packages installed once at workspace level  
✅ **Isolation**: Projects can still have their own specific dependencies  
✅ **Fast**: UV is much faster than pip  
✅ **Reproducible**: Lock files ensure consistent environments  
✅ **Simple**: One command (`uv sync`) handles everything

## Notes

### Hardlink Warning

If you see a warning about hardlinks failing, it's because:
- UV cache is on `C:` drive (`C:\Users\...\AppData\Local\uv\cache`)
- Workspace venv is on `D:` drive (`D:\_dev\projects\dev-boards\.venv`)

Hardlinks only work within the same filesystem, so UV falls back to copying files. This is configured in `pyproject.toml` with `link-mode = "copy"` to suppress the warning. Performance impact is minimal.

## Project Structure

```
dev-boards/
├── pyproject.toml          # Workspace-level dependencies
├── uv.lock                 # Workspace lock file (auto-generated)
├── .venv/                  # Workspace virtual environment
├── .python-version         # Python version specification
└── bootstraps/
    └── git_py/
        ├── pyproject.toml  # Project-specific dependencies
        └── ...
```

