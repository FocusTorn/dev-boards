#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Wrapper script for git_py portable CLI tool.

.DESCRIPTION
    Allows running git_py from anywhere by calling the main.py script.
    Place this script in your PATH or project root.

.EXAMPLE
    git-py init
    git-py status
    git-py auth
#>

param([Parameter(ValueFromRemainingArguments)]$args)

# Get the directory where this script is located
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# Path to main.py (assuming git-py.ps1 is in the same directory as git_py/)
$MainScript = Join-Path $ScriptDir "git_py" "main.py"

# If git-py.ps1 is inside git_py directory, adjust path
if (-not (Test-Path $MainScript)) {
    $MainScript = Join-Path (Split-Path -Parent $ScriptDir) "git_py" "main.py"
}

# If still not found, try current directory
if (-not (Test-Path $MainScript)) {
    $MainScript = Join-Path (Get-Location) "git_py" "main.py"
}

if (-not (Test-Path $MainScript)) {
    Write-Host "Error: Could not find git_py/main.py" -ForegroundColor Red
    Write-Host "Make sure git-py.ps1 is in the same directory as git_py/" -ForegroundColor Yellow
    exit 1
}

# Run the script
python $MainScript $args

