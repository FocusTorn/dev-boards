# Multiplatform Python Setup Guide

This guide explains how to convert bash scripts to multiplatform Python scripts that work on both **Debian/Linux** and **Windows**, using `pyprompt` for interactive prompts.

## Key Features

- ✅ **Cross-platform**: Works on Windows and Debian/Linux
- ✅ **pyprompt integration**: Beautiful interactive prompts using `pyprompt`
- ✅ **Smart installation**: On Debian, can optionally install `pyprompt` if missing
- ✅ **Windows compatibility**: On Windows, `pyprompt` must be pre-installed (can't be downloaded during script execution)

## Platform Detection

```python
import platform

def is_windows() -> bool:
    """Check if running on Windows."""
    return platform.system() == "Windows"

def is_debian() -> bool:
    """Check if running on Debian-based Linux."""
    if is_windows():
        return False
    try:
        with open('/etc/os-release', 'r') as f:
            return 'debian' in f.read().lower() or 'ubuntu' in f.read().lower()
    except (FileNotFoundError, IOError):
        return False
```

## pyprompt Availability Check

The scripts check for `pyprompt` availability and handle installation differently based on platform:

### On Debian/Linux

- **Without `--full-setup`**: Script fails gracefully if `pyprompt` is not installed, with instructions
- **With `--full-setup`**: Script attempts to install `pyprompt` automatically using `uv` (preferred) or `pip`

### On Windows

- **Always**: Script requires `pyprompt` to be pre-installed
- **Reason**: Windows may not have network access or package managers available during script execution
- **Installation**: User must install `pyprompt` manually before running the script

## Usage Examples

### Basic Usage (pyprompt pre-installed)

```bash
# Debian/Linux
python3 add-mqtt-user.py myuser mypassword

# Windows
python add-mqtt-user.py myuser mypassword
```

### Full Setup (Debian/Linux only)

```bash
# Automatically installs pyprompt if not available
python3 add-mqtt-user.py myuser mypassword --full-setup
```

### Interactive Prompts

```bash
# Password will be prompted securely
python3 add-mqtt-user.py myuser --full-setup
```

## Installation Requirements

### Pre-installing pyprompt (Required on Windows, Optional on Debian)

```bash
# Using UV (preferred)
uv pip install pyprompt

# Using pip
pip install pyprompt
```

### Workspace Setup (if using workspace dependencies)

If `pyprompt` is part of your workspace dependencies (via `pyproject.toml`):

```bash
# Sync workspace dependencies
uv sync

# Or activate workspace venv
source .venv/bin/activate  # Linux
.venv\Scripts\Activate.ps1  # Windows
```

## Script Structure

All multiplatform scripts follow this pattern:

1. **Platform Detection**: Detect Windows vs Debian/Linux
2. **pyprompt Check**: Check if `pyprompt` is available
3. **Installation Logic**: On Debian with `--full-setup`, attempt installation
4. **Graceful Failure**: On Windows or without `--full-setup`, provide clear error messages
5. **Platform-Specific Paths**: Use appropriate paths for each platform
6. **Root/Admin Check**: Check for root (Linux) or admin (Windows) privileges
7. **Interactive Prompts**: Use `pyprompt` for all user interactions

## Converting Bash Scripts

When converting bash scripts to multiplatform Python:

### 1. Replace `read` with `pyprompt.text()`

**Bash:**
```bash
read -p "Enter password: " password
```

**Python:**
```python
from pyprompt import text
password = text("Enter password: ", default="")
```

### 2. Replace `read -sp` (password input) with `pyprompt.text()` or `getpass`

**Bash:**
```bash
read -sp "Enter password: " password
```

**Python:**
```python
from pyprompt import text
password = text("Enter password: ", default="")  # pyprompt handles password masking
```

### 3. Replace `read -p` with confirmations with `pyprompt.confirm()`

**Bash:**
```bash
read -p "Continue? (y/N): " -n 1 -r
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
fi
```

**Python:**
```python
from pyprompt import confirm
if not confirm("Continue?", default=False):
    sys.exit(1)
```

### 4. Replace menu selections with `pyprompt.select()`

**Bash:**
```bash
echo "Select option:"
echo "1) Option 1"
echo "2) Option 2"
read -p "Choice: " choice
```

**Python:**
```python
from pyprompt import select
choice = select("Select option:", ["Option 1", "Option 2"])
```

### 5. Handle Platform-Specific Paths

**Bash:**
```bash
PASSWD_FILE="/etc/mosquitto/passwd"  # Linux only
```

**Python:**
```python
def get_mosquitto_paths():
    if is_windows():
        return {'passwd_file': Path("C:/mosquitto/passwd")}
    else:
        return {'passwd_file': Path("/etc/mosquitto/passwd")}
```

### 6. Handle Root/Admin Checks

**Bash:**
```bash
if [ "$EUID" -ne 0 ]; then
    echo "Must run as root"
    exit 1
fi
```

**Python:**
```python
def check_root():
    if is_windows():
        import ctypes
        if not ctypes.windll.shell32.IsUserAnAdmin():
            print(error("Must run as administrator"))
            sys.exit(1)
    else:
        if os.geteuid() != 0:
            print(error("Must run as root"))
            sys.exit(1)
```

## Command Line Arguments

All scripts support:

- `--full-setup`: On Debian/Linux, attempt to install `pyprompt` if not available
- Standard arguments: Script-specific arguments (username, password, etc.)

## Error Handling

Scripts provide clear error messages:

- **Missing pyprompt on Windows**: Instructions to install manually
- **Missing pyprompt on Debian**: Instructions to install or use `--full-setup`
- **Missing dependencies**: Platform-specific instructions
- **Permission errors**: Clear messages about required privileges

## Testing

### Test on Debian/Linux

```bash
# Test without pyprompt installed
python3 add-mqtt-user.py testuser --full-setup

# Test with pyprompt pre-installed
python3 add-mqtt-user.py testuser
```

### Test on Windows

```bash
# Ensure pyprompt is installed first
uv pip install pyprompt

# Then run script
python add-mqtt-user.py testuser
```

## Examples

See the following converted scripts:

- `add-mqtt-user.py`: Complete example with password prompting
- `mqtt-helper.py`: Template for command-based scripts

## Notes

- **Windows Limitations**: On Windows, `pyprompt` cannot be installed during script execution due to potential network/package manager limitations
- **Debian Flexibility**: On Debian/Linux, `--full-setup` allows automatic installation
- **Fallback Support**: Scripts include fallback prompts using `getpass` if `pyprompt` is unavailable
- **Terminal Output**: Scripts use `outerm` for terminal output if available, with fallback functions
