# Build script for iMenu - Creates distribution package
# Detects shell and OS, cleans dist/, and rebuilds everything

$ErrorActionPreference = "Stop"

# Get script directory
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ScriptDir

# Helper function for colored output
function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$ForegroundColor = "White"
    )
    
    # Use $PSStyle for PowerShell 7+ (handles ANSI automatically)
    if ($PSVersionTable.PSVersion.Major -ge 7 -and $null -ne $PSStyle) {
        $colorMap = @{
            "Black"   = $PSStyle.Foreground.Black
            "Red"     = $PSStyle.Foreground.Red
            "Green"   = $PSStyle.Foreground.Green
            "Yellow"  = $PSStyle.Foreground.Yellow
            "Blue"    = $PSStyle.Foreground.Blue
            "Magenta" = $PSStyle.Foreground.Magenta
            "Cyan"    = $PSStyle.Foreground.Cyan
            "White"   = $PSStyle.Foreground.White
            "Gray"    = $PSStyle.Foreground.BrightBlack
        }
        
        $colorCode = $colorMap[$ForegroundColor]
        if ($colorCode) {
            Write-Output "$colorCode$Message$($PSStyle.Reset)"
        } else {
            Write-Output $Message
        }
    } else {
        # For PowerShell 5.1: Use proper ANSI escape sequences
        $esc = [char]27
        $colorMap = @{
            "Black"   = "$esc[30m"
            "Red"     = "$esc[31m"
            "Green"   = "$esc[32m"
            "Yellow"  = "$esc[33m"
            "Blue"    = "$esc[34m"
            "Magenta" = "$esc[35m"
            "Cyan"    = "$esc[36m"
            "White"   = "$esc[37m"
            "Gray"    = "$esc[90m"
        }
        
        $reset = "$esc[0m"
        $colorCode = $colorMap[$ForegroundColor]
        
        if ($colorCode) {
            Write-Output "$colorCode$Message$reset"
        } else {
            Write-Output $Message
        }
    }
}

# Detect OS
function Detect-OS {
    $os = if ($IsWindows -or $env:OS -eq "Windows_NT") {
        "windows"
    } elseif ($IsMacOS) {
        "darwin"
    } elseif ($IsLinux) {
        "linux"
    } else {
        # Fallback detection
        $uname = if (Get-Command uname -ErrorAction SilentlyContinue) {
            (uname -s).ToLower()
        } else {
            "unknown"
        }
        if ($uname -like "*linux*") {
            "linux"
        } elseif ($uname -like "*darwin*") {
            "darwin"
        } else {
            "unknown"
        }
    }
    return $os
}

# Detect architecture
function Detect-Arch {
    $arch = if ($IsWindows) {
        if ([Environment]::Is64BitOperatingSystem) {
            "amd64"
        } else {
            "386"
        }
    } elseif ($IsMacOS -or $IsLinux) {
        $unameM = if (Get-Command uname -ErrorAction SilentlyContinue) {
            uname -m
        } else {
            "unknown"
        }
        switch ($unameM) {
            { $_ -in "x86_64", "amd64" } { "amd64" }
            { $_ -in "arm64", "aarch64" } { "arm64" }
            { $_ -like "arm*" } { "arm" }
            default { $unameM }
        }
    } else {
        "unknown"
    }
    return $arch
}

$OS = Detect-OS
$ARCH = Detect-Arch

Write-ColorOutput "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-ColorOutput "🔨 Building iMenu Distribution Package" -ForegroundColor Cyan
Write-ColorOutput "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Output ""
Write-ColorOutput "📋 Detected:" -ForegroundColor Yellow
Write-ColorOutput "   OS: $OS" -ForegroundColor Gray
Write-ColorOutput "   Architecture: $ARCH" -ForegroundColor Gray
Write-ColorOutput "   Shell: PowerShell" -ForegroundColor Gray
Write-Output ""

# Check if Go is installed
if (-not (Get-Command go -ErrorAction SilentlyContinue)) {
    Write-Error "❌ Go is not installed. Please install Go first."
    Write-ColorOutput "   Visit: https://go.dev/dl/" -ForegroundColor Yellow
    exit 1
}

$goVersion = go version
Write-ColorOutput "✅ Go found: $goVersion" -ForegroundColor Green
Write-Output ""

# Clean dist directory
Write-ColorOutput "🧹 Cleaning dist directory..." -ForegroundColor Yellow
if (Test-Path "dist") {
    Remove-Item -Path "dist" -Recurse -Force
}
Write-ColorOutput "✅ Cleaned" -ForegroundColor Green
Write-Output ""

# Recreate dist structure
Write-ColorOutput "📁 Creating distribution structure..." -ForegroundColor Yellow
New-Item -ItemType Directory -Path "dist\bin" -Force | Out-Null
New-Item -ItemType Directory -Path "dist\lib" -Force | Out-Null
New-Item -ItemType Directory -Path "dist\docs" -Force | Out-Null
Write-ColorOutput "✅ Structure created" -ForegroundColor Green
Write-Output ""

# Update Go dependencies
Write-ColorOutput "📦 Updating Go dependencies..." -ForegroundColor Yellow
go mod tidy
Write-ColorOutput "✅ Dependencies updated" -ForegroundColor Green
Write-Output ""

# Build executables
Write-ColorOutput "🔨 Building executables..." -ForegroundColor Yellow
Write-Output ""

# Build prompt-wizard
Write-ColorOutput "   Building prompt-wizard..." -ForegroundColor Gray
$exeName = if ($OS -eq "windows") { "prompt-wizard.exe" } else { "prompt-wizard" }
$exePath = Join-Path "dist\bin" $exeName

go build -o $exePath .\cmd\prompt-wizard
if (Test-Path $exePath) {
    Write-ColorOutput "   ✅ Built: $exePath" -ForegroundColor Green
} else {
    Write-Error "   ❌ Failed to build prompt-wizard"
    exit 1
}

# Build prompt-huh (optional)
if (Test-Path "cmd\prompt-huh\main.go") {
    Write-ColorOutput "   Building prompt-huh..." -ForegroundColor Gray
    $huhExeName = if ($OS -eq "windows") { "prompt-huh.exe" } else { "prompt-huh" }
    $huhExePath = Join-Path "dist\bin" $huhExeName
    try {
        go build -o $huhExePath .\cmd\prompt-huh 2>$null
        if (Test-Path $huhExePath) {
            Write-ColorOutput "   ✅ Built: $huhExePath" -ForegroundColor Green
        }
    } catch {
        Write-ColorOutput "   ⚠️  prompt-huh build skipped (optional)" -ForegroundColor Yellow
    }
}

Write-Output ""

# Copy wrapper scripts to dist/lib
Write-ColorOutput "📋 Copying wrapper scripts..." -ForegroundColor Yellow
Copy-Item "wizard.sh" "dist\lib\" -Force
Copy-Item "wizard.ps1" "dist\lib\" -Force
Write-ColorOutput "✅ Wrapper scripts copied" -ForegroundColor Green
Write-Output ""

# Copy documentation to dist/docs
Write-ColorOutput "📚 Copying documentation..." -ForegroundColor Yellow
if (Test-Path "docs") {
    Copy-Item "docs\*.md" "dist\docs\" -Force -ErrorAction SilentlyContinue
    Write-ColorOutput "✅ Documentation copied" -ForegroundColor Green
} else {
    Write-ColorOutput "⚠️  No docs directory found" -ForegroundColor Yellow
}
Write-Output ""

# Note about dist README
if (Test-Path "dist\README.md") {
    Write-ColorOutput "✅ Distribution README exists" -ForegroundColor Green
} else {
    Write-ColorOutput "📝 Note: dist/README.md should exist for distribution package" -ForegroundColor Yellow
}

Write-ColorOutput "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-ColorOutput "✅ Build complete!" -ForegroundColor Green
Write-ColorOutput "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Output ""
Write-ColorOutput "📦 Distribution package created in: dist\" -ForegroundColor Yellow
Write-Output ""
Write-ColorOutput "📁 Structure:" -ForegroundColor Cyan
Write-ColorOutput "   dist\" -ForegroundColor Gray
Write-ColorOutput "   ├── bin\          # Executables" -ForegroundColor Gray
Write-ColorOutput "   ├── lib\          # Wrapper scripts" -ForegroundColor Gray
Write-ColorOutput "   ├── docs\         # Documentation" -ForegroundColor Gray
Write-ColorOutput "   └── README.md     # Package README" -ForegroundColor Gray
Write-Output ""
Write-ColorOutput "💡 Usage:" -ForegroundColor Cyan
Write-ColorOutput "   cd dist" -ForegroundColor Gray
Write-ColorOutput "   . .\lib\wizard.ps1" -ForegroundColor Gray
Write-Output ""

