# GitHub Setup Wizard (Python)
# Wrapper script that calls the Python wizard

[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [switch]$SkipSsh,
    
    [Parameter(Mandatory = $false)]
    [switch]$SkipLocalRepo,
    
    [Parameter(Mandatory = $false)]
    [switch]$SkipRemote,
    
    [Parameter(Mandatory = $false)]
    [string]$Output
)

$ErrorActionPreference = "Stop"

# Get script directory
$scriptDir = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.PSCommandPath }
$pythonScript = Join-Path $scriptDir "py-setup-github.py"

if (-not (Test-Path $pythonScript)) {
    Write-Error "❌ Python script not found: $pythonScript" -ErrorAction Stop
    exit 1
}

# Check if Python is available
$pythonCmd = $null
$pythonCommands = @("python3", "python", "py")
foreach ($cmd in $pythonCommands) {
    if (Get-Command $cmd -ErrorAction SilentlyContinue) {
        $pythonCmd = $cmd
        break
    }
}

if (-not $pythonCmd) {
    Write-Error "❌ Python is not installed or not in PATH. Please install Python 3.7+" -ErrorAction Stop
    Write-Host "   Visit: https://www.python.org/downloads/" -ForegroundColor Yellow
    exit 1
}

# Check Python version
try {
    $pythonVersion = & $pythonCmd --version 2>&1
    Write-Host "✅ Found: $pythonVersion" -ForegroundColor Green
} catch {
    Write-Error "❌ Failed to run Python: $_" -ErrorAction Stop
    exit 1
}

# Build arguments
$arguments = @()

if ($SkipSsh) {
    $arguments += "--skip-ssh"
}

if ($SkipLocalRepo) {
    $arguments += "--skip-local-repo"
}

if ($SkipRemote) {
    $arguments += "--skip-remote"
}

if ($Output) {
    $arguments += "--output"
    $arguments += $Output
}

# Check if questionary is installed
$checkImport = @"
import sys
try:
    import questionary
    sys.exit(0)
except ImportError:
    sys.exit(1)
"@

$checkResult = $checkImport | & $pythonCmd - 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "⚠️  questionary library not found. Installing..." -ForegroundColor Yellow
    & $pythonCmd -m pip install questionary
    if ($LASTEXITCODE -ne 0) {
        Write-Error "❌ Failed to install questionary. Please run: pip install questionary" -ErrorAction Stop
        exit 1
    }
    Write-Host "✅ questionary installed successfully" -ForegroundColor Green
}

# Run the Python script
Write-Host ""
Write-Host "🚀 Starting GitHub Setup Wizard..." -ForegroundColor Cyan
Write-Host ""

& $pythonCmd $pythonScript @arguments

$exitCode = $LASTEXITCODE
exit $exitCode

