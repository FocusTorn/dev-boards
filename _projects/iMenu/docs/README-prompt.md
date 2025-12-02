# Interactive Prompt Tools

Tools for creating interactive prompts in bash scripts using `huh?` from charmbracelet.

## Quick Start

### Option 1: Automatic (Recommended)

The demo script will automatically install Go and build the tool:

```bash
/root/_playground/_scripts/demo-prompt.sh
```

### Option 2: Manual Installation

**1. Install Go (if needed):**
```bash
# Automatic via bootstrap script
/root/_playground/_scripts/bootstrap-go.sh

# Or manually via package manager:
# Debian/Ubuntu:
sudo apt-get install golang-go

# Or download from: https://go.dev/dl/
```

**2. Build the prompt tool:**
```bash
cd /root/_playground/_scripts
go mod tidy
go build -o prompt-huh prompt-huh.go
```

**3. Use it:**
```bash
./prompt-huh select "Choose:" "Option 1" "Option 2" "Option 3"
```

## Lightweight Go Installation Options

### 1. Package Manager (Easiest)
```bash
# Debian/Ubuntu/Raspberry Pi OS
sudo apt-get install golang-go

# Fedora/RHEL
sudo dnf install golang

# Arch Linux
sudo pacman -S go

# macOS
brew install go
```

**Size:** ~100-200MB (includes compiler, standard library, tools)

### 2. Official Binary (Minimal)
Download from https://go.dev/dl/ - just extract and add to PATH.

**Size:** ~120MB compressed, ~300MB extracted

### 3. Go Version Manager (g)
```bash
curl -sSL https://git.io/g-install | sh -s
```

**Size:** Similar to official binary, but easier to manage versions

## Important Notes

- **After building, you don't need Go anymore!** The `prompt-huh` binary is standalone.
- Once built, you can copy `prompt-huh` to any system (same architecture) without Go.
- The binary is typically 5-10MB, much smaller than the Go toolchain.

## Usage Examples

```bash
# Text input
NAME=$(./prompt-huh input "What is your name?")
echo "Hello, $NAME!"

# Select from options
COLOR=$(./prompt-huh select "Choose a color:" "Red" "Blue" "Green")

# Confirmation
if ./prompt-huh confirm "Continue?"; then
    echo "Yes!"
fi

# Multi-select
SELECTED=$(./prompt-huh multiselect "Choose:" "Opt1" "Opt2" "Opt3")
```

## Files

- `prompt-huh.go` - Source code (requires Go to build)
- `prompt-huh` - Compiled binary (standalone, no Go needed)
- `bootstrap-go.sh` - Script to install Go automatically
- `demo-prompt.sh` - Interactive demo of all features

