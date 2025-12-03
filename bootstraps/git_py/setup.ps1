#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Install git_py package in editable mode for global use.

.DESCRIPTION
    This script installs the git_py package in editable mode so it can be
    used from anywhere with 'uv run python -m git_py'.

.EXAMPLE
    .\setup.ps1
#>

$ErrorActionPreference = "Stop"

# Get the script directory (package root)
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$WorkspaceRoot = Split-Path -Parent (Split-Path -Parent $ScriptDir)

Write-Host "Installing git_py package..." -ForegroundColor Cyan
Write-Host "Workspace root: $WorkspaceRoot" -ForegroundColor Gray
Write-Host "Package directory: $ScriptDir" -ForegroundColor Gray
Write-Host ""

# Change to workspace root
Push-Location $WorkspaceRoot

try {
    # Sync workspace dependencies first
    Write-Host "Syncing workspace dependencies..." -ForegroundColor Yellow
    uv sync
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to sync workspace dependencies" -ForegroundColor Red
        exit 1
    }
    
    # Install package in editable mode
    Write-Host "Installing git_py in editable mode..." -ForegroundColor Yellow
    uv pip install -e bootstraps/git_py
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to install git_py package" -ForegroundColor Red
        exit 1
    }
    
    Write-Host ""
    Write-Host "✓ Installation complete!" -ForegroundColor Green
    Write-Host ""
    Write-Host "You can now use git_py from anywhere:" -ForegroundColor Cyan
    Write-Host "  uv run python -m git_py --help" -ForegroundColor White
    Write-Host "  uv run python -m git_py init" -ForegroundColor White
    Write-Host "  uv run python -m git_py status" -ForegroundColor White
    Write-Host "  uv run python -m git_py auth" -ForegroundColor White
    
} finally {
    Pop-Location
}

