# git_py - Portable GitHub Repository Bootstrapper

A portable, cross-platform CLI tool for initializing GitHub repositories. Copy this directory into any project and use it immediately.

## Quick Start

### Copy the Tool

Simply copy the `git_py` directory into your project. **Keep the directory name as `git_py`** - it's required for the imports to work:

```bash
# Copy the entire git_py directory (keep the name!)
cp -r /path/to/git_py ./git_py

# Or on Windows
xcopy /E /I git_py .\git_py

# The directory structure should be:
# your-project/
# └── git_py/          ← Keep this name!
#     ├── main.py
#     ├── core/
#     ├── commands/
#     └── operations/
```

### Run It

**Single Cross-Platform Executor** (recommended):

```bash
# Works on Windows, Linux, and macOS!
python git_py/git-py init
python git_py/git-py status
python git_py/git-py auth

# Or make it executable (Unix/Linux/Mac)
chmod +x git_py/git-py
./git_py/git-py init

# On Windows, you can also use the .bat file (if .py association works)
git_py\git-py.bat init
```

**Alternative methods:**

```bash
# Direct execution of main.py
python git_py/main.py init

# Or make main.py executable (Unix/Linux/Mac)
chmod +x git_py/main.py
./git_py/main.py init
```

### Windows Usage

```powershell
# Direct execution
python git_py\main.py init

# Or create a wrapper script
git_py.ps1  # See below
```

## Features

- ✅ **Portable**: Copy into any project, no installation needed
- ✅ **Cross-platform**: Works on Windows, Linux, and macOS
- ✅ **Self-contained**: All code in one directory
- ✅ **No dependencies**: Uses only Python standard library (except prompt-toolkit)

## Dependencies

The tool requires `prompt-toolkit` for interactive prompts. Install it with:

```bash
# Using pip
pip install prompt-toolkit

# Using uv (recommended)
uv pip install prompt-toolkit

# Or add to your project's requirements.txt
echo "prompt-toolkit>=3.0.0" >> requirements.txt
```

## Usage Examples

### Initialize a Repository

```bash
python git_py/main.py init
```

This will:
1. Check prerequisites (GitHub CLI, SSH, Git config)
2. Prompt for repository details (username, repo name, visibility)
3. Prompt for local path
4. Create remote repository on GitHub
5. Initialize local git repository
6. Set up remote connection
7. Create initial commit
8. Push to remote

### Check Status

```bash
python git_py/main.py status
```

Shows comprehensive status of:
- GitHub CLI installation and authentication
- SSH keys and connection
- HTTPS authentication
- Git configuration
- Git-Crypt status

### Authenticate Everything

```bash
python git_py/main.py auth
```

Automatically sets up and authenticates:
- GitHub CLI
- SSH keys
- SSH config

## Single Cross-Platform Executor

The `git-py` file (no extension) is a **single Python script that works on all platforms**:

- **Windows**: `python git_py/git-py init`
- **Linux/Mac**: `python git_py/git-py init` or `./git_py/git-py init` (after `chmod +x`)

This is the recommended way to run the tool - one file, works everywhere!

### Adding to PATH (Optional)

To use `git-py` from anywhere, add it to your PATH:

**Unix/Linux/Mac:**
```bash
# Create a symlink or copy git-py to a directory in your PATH
ln -s /path/to/git_py/git-py ~/bin/git-py
# Or copy it
cp git_py/git-py ~/bin/
chmod +x ~/bin/git-py

# Now you can use it from anywhere:
git-py init
```

**Windows:**
```powershell
# Add git_py directory to PATH, or create a git-py.bat wrapper in a PATH directory
# The git-py.bat file is included for Windows convenience
```

## Project Structure

```
your-project/
├── git_py/              # Copy this entire directory
│   ├── git-py          # ⭐ Single cross-platform executor (recommended)
│   ├── git-py.bat      # Windows convenience wrapper
│   ├── main.py         # Main entry point
│   ├── __main__.py     # For python -m git_py usage
│   ├── commands/        # Command implementations
│   ├── core/           # Core utilities
│   └── operations/     # Repository operations
└── ...
```

## How It Works

The tool is designed to be completely portable:

1. **Self-contained**: All code is in the `git_py` directory
2. **Path handling**: When `main.py` runs, it adds the parent directory to `sys.path` so `git_py` becomes an importable package
3. **Package structure**: Uses standard Python package structure with relative imports (`from ..core`)
4. **No installation**: Works immediately after copying - just ensure the directory is named `git_py`
5. **Cross-platform**: Works on Windows, Linux, and macOS

## Troubleshooting

### "No module named 'core'"

Make sure you're running `main.py` directly, not importing it. The path setup only works when running as a script.

### "prompt-toolkit not found"

Install the dependency:
```bash
pip install prompt-toolkit
# or
uv pip install prompt-toolkit
```

### Import errors

- **"No module named 'git_py'"**: Make sure the directory is named exactly `git_py` (not `git-py` or anything else)
- **"No module named 'core'"**: Make sure you're running `main.py` directly, not importing it as a module
- **Relative import errors**: Ensure the directory structure is intact and the directory name is `git_py`

## Best Practices

1. **Copy, don't move**: Keep the original `git_py` directory intact
2. **Version control**: You can commit `git_py` to your project's repository
3. **Update strategy**: To update, replace the entire `git_py` directory
4. **Dependencies**: Document `prompt-toolkit` in your project's requirements

## License

[Add your license here]

