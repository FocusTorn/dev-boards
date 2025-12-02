# Arduino CLI Setup Script for Windows
# This script helps set up Arduino CLI for ESP32-S3 development

$ErrorActionPreference = "Stop"

Write-Host "Arduino CLI Setup Script" -ForegroundColor Green
Write-Host "========================" -ForegroundColor Green
Write-Host ""

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$arduinoCliPath = Join-Path $scriptDir "arduino-cli.exe"

# Check if arduino-cli exists
if (-not (Test-Path $arduinoCliPath)) {
    Write-Host "✗ arduino-cli.exe not found in: $scriptDir" -ForegroundColor Red
    Write-Host ""
    Write-Host "Please download Arduino CLI:" -ForegroundColor Yellow
    Write-Host "  1. Visit: https://arduino.github.io/arduino-cli/latest/installation/" -ForegroundColor Cyan
    Write-Host "  2. Download the Windows 64-bit version" -ForegroundColor Cyan
    Write-Host "  3. Rename it to 'arduino-cli.exe' and place it in:" -ForegroundColor Cyan
    Write-Host "     $scriptDir" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Or use this PowerShell command to download it:" -ForegroundColor Yellow
    Write-Host '  Invoke-WebRequest -Uri "https://downloads.arduino.cc/arduino-cli/arduino-cli_latest_Windows_64bit.zip" -OutFile "arduino-cli.zip"' -ForegroundColor Cyan
    Write-Host '  Expand-Archive -Path "arduino-cli.zip" -DestinationPath "." -Force' -ForegroundColor Cyan
    Write-Host '  Remove-Item "arduino-cli.zip"' -ForegroundColor Cyan
    exit 1
}

Write-Host "✓ Found arduino-cli.exe" -ForegroundColor Green
Write-Host ""

# Get Arduino CLI version
Write-Host "Arduino CLI version:" -ForegroundColor Cyan
& $arduinoCliPath version
Write-Host ""

# Initialize configuration
Write-Host "Initializing Arduino CLI configuration..." -ForegroundColor Cyan
$configPath = Join-Path $scriptDir "arduino-cli.yaml"
$homeDir = Join-Path $scriptDir "home"

if (-not (Test-Path $configPath)) {
    & $arduinoCliPath config init --overwrite
    Write-Host "✓ Configuration file created" -ForegroundColor Green
} else {
    Write-Host "✓ Configuration file already exists" -ForegroundColor Green
}

# Set directories
Write-Host "Configuring directories..." -ForegroundColor Cyan
& $arduinoCliPath config set directories.user $homeDir
& $arduinoCliPath config set directories.data $scriptDir

Write-Host "✓ Directories configured:" -ForegroundColor Green
Write-Host "  User directory: $homeDir" -ForegroundColor Yellow
Write-Host "  Data directory: $scriptDir" -ForegroundColor Yellow
Write-Host ""

# Update core index
Write-Host "Updating core index..." -ForegroundColor Cyan
& $arduinoCliPath core update-index
Write-Host ""

# Install ESP32 core
Write-Host "Installing ESP32 core (this may take a while)..." -ForegroundColor Cyan
& $arduinoCliPath core install esp32:esp32

if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Failed to install ESP32 core" -ForegroundColor Red
    exit 1
}

Write-Host "✓ ESP32 core installed" -ForegroundColor Green
Write-Host ""

# List installed cores
Write-Host "Installed cores:" -ForegroundColor Cyan
& $arduinoCliPath core list
Write-Host ""

# List ESP32-S3 boards
Write-Host "Available ESP32-S3 boards:" -ForegroundColor Cyan
& $arduinoCliPath board listall | Select-String "esp32s3"
Write-Host ""

Write-Host "====================================" -ForegroundColor Green
Write-Host "Setup Complete!" -ForegroundColor Green
Write-Host "====================================" -ForegroundColor Green
Write-Host ""
Write-Host "You can now use Arduino CLI:" -ForegroundColor Cyan
Write-Host "  .\arduino-cli.exe compile --fqbn esp32:esp32:esp32s3 sketchbook\MyProject" -ForegroundColor Yellow
Write-Host ""

