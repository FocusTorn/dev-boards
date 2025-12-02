# iMenu - Interactive Menu System

Interactive menu and wizard system built with Go, supporting both bash and PowerShell scripts.

## 📁 Directory Structure

```
iMenu/
├── dist/                    # Distribution package (relocatable, self-contained)
│   ├── bin/                 # Built executables
│   │   ├── prompt-wizard     # Main wizard (Linux/Mac)
│   │   └── prompt-wizard.exe # Main wizard (Windows)
│   ├── lib/                 # Wrapper scripts (standalone versions)
│   │   ├── wizard.sh        # Bash wrapper
│   │   └── wizard.ps1       # PowerShell wrapper
│   ├── docs/                # Documentation
│   └── README.md           # Distribution package README
├── demos/                   # Demo scripts and examples
│   ├── demo-*.sh           # Bash demo scripts
│   ├── demo-*.ps1          # PowerShell demo scripts
│   └── wizard-example.json # Example wizard configuration
├── docs/                    # Source documentation
│   ├── README-prompt.md    # Prompt tools documentation
│   └── README-powershell.md # PowerShell usage guide
├── src/                     # Go source files
│   ├── prompt-wizard.go    # Main wizard implementation
│   ├── prompt-huh.go       # Alternative prompt tool
│   └── *.go               # Other Go source files
├── wizard.sh                 # Bash wrapper functions (development version)
├── wizard.ps1                # PowerShell wrapper functions (development version)
├── build.sh                # Build script (bash)
├── build.ps1               # Build script (PowerShell)
├── go.mod                  # Go module definition
└── go.sum                  # Go module checksums
```

## 🚀 Quick Start

### Bash

```bash
# Source the wrapper functions
source wizard.sh

# Use the wizard
result=$(iwizard_run_inline '[{"type":"input","title":"Name","key":"name"}]')
echo "$result" | jq
```

### PowerShell

```powershell
# Source the wrapper functions
. .\wizard.ps1

# Use the wizard
$result = iwizard-RunInline -JsonString '[{"type":"input","title":"Name","key":"name"}]'
$parsed = $result | ConvertFrom-Json
Write-Host "Hello, $($parsed.name)"
```

## 📦 Building

### Automatic Build (Recommended)

The executables are automatically built on first use when using the wrapper scripts.

### Manual Build

Use the build script to create a complete distribution package:

```bash
# Auto-detects shell (bash or PowerShell)
./build

# Or explicitly use bash
./build.sh

# Or explicitly use PowerShell
pwsh build.ps1
```

The build script will:
- ✅ Detect your shell (Bash or PowerShell)
- ✅ Detect your OS (Linux, macOS, Windows)
- ✅ Clean the `dist/` directory
- ✅ Recreate the complete distribution structure
- ✅ Build all executables for your platform
- ✅ Copy wrapper scripts and documentation

**Build Output:**
```
dist/
├── bin/          # Executables (prompt-wizard, prompt-huh)
├── lib/          # Wrapper scripts
├── docs/         # Documentation
└── README.md     # Distribution package README
```

### Manual Build (Without Script)

```bash
cd /root/_playground/projects/iMenu
go mod tidy
mkdir -p dist/bin
go build -o dist/bin/prompt-wizard ./src/prompt-wizard.go
```

On Windows, Go automatically adds the `.exe` extension.

## 🎯 Features

- **Cross-platform**: Works on Linux, macOS, and Windows
- **Dual shell support**: Bash and PowerShell wrappers
- **Interactive TUI**: Beautiful terminal user interface using bubbletea
- **JSON-based configuration**: Define wizards with simple JSON
- **Auto-build**: Executables built automatically on first use

## 📚 Documentation

- **[Prompt Tools](docs/README-prompt.md)** - Overview of interactive prompt tools
- **[PowerShell Guide](docs/README-powershell.md)** - PowerShell-specific usage

## 🧪 Examples

See the `demos/` directory for demo scripts:

- `demo-wizard.sh` - Multi-step wizard example (bash)
- `demo-powershell.ps1` - PowerShell wizard examples
- `wizard-example.json` - Example wizard JSON configuration

## 🔧 Components

### Executables (in `dist/`)

- **prompt-wizard** - Main interactive wizard tool
  - Supports input, select, multiselect, and confirm step types
  - JSON-based step configuration
  - Back navigation support
  - Results output as JSON

### Wrapper Scripts

**Development versions** (in project root):
- **wizard.sh** - Bash wrapper functions (auto-builds executables)
- **wizard.ps1** - PowerShell wrapper functions (auto-builds executables)

**Distribution versions** (in `dist/lib/`):
- Standalone, relocatable versions that work without source code
- Pre-built executables in `dist/bin/`
- Same function interface as development versions

### Source Files

- **prompt-wizard.go** - Main wizard implementation
- **prompt-huh.go** - Alternative prompt tool using huh?
- **prompt-bubbletea.go** - Alternative prompt tool using bubbletea
- **prompt-survey.go** - Alternative prompt tool using survey

## 📝 Usage Examples

### Simple Input

```bash
# Bash
result=$(iwizard_run_inline '[{"type":"input","title":"Name","key":"name"}]')
```

```powershell
# PowerShell
$result = iwizard-RunInline -JsonString '[{"type":"input","title":"Name","key":"name"}]'
```

### Multi-Step Wizard

```json
[
  {"type":"input","title":"Name","key":"name"},
  {"type":"select","title":"Color","key":"color","options":["Red","Blue"]},
  {"type":"confirm","title":"Continue?","key":"continue"}
]
```

### Step Types

- **`input`** - Text input field
- **`select`** - Single selection from options
- **`multiselect`** - Multiple selections from options
- **`confirm`** - Yes/No confirmation

## 🛠️ Development

### Prerequisites

- Go 1.21 or later
- Bash (for bash scripts)
- PowerShell (for PowerShell scripts)

### Building

```bash
go mod tidy
go build -o dist/prompt-wizard prompt-wizard.go
```

### Testing

Run the demo scripts in the `examples/` directory to test functionality.

## 📄 License

Part of the _playground project.

