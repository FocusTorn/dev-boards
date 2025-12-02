# Demo script showing the wizard mode with back navigation

$ErrorActionPreference = "Stop"

$SCRIPT_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path
$IMENU_DIR = Split-Path -Parent $SCRIPT_DIR
$WIZARD_BIN = Join-Path $IMENU_DIR "dist\bin\prompt-wizard"

# Check for .exe extension on Windows
if (Test-Path "$WIZARD_BIN.exe") {
    $WIZARD_BIN = "$WIZARD_BIN.exe"
}

# Check if wizard is built
if (-not (Test-Path $WIZARD_BIN)) {
    Write-Host "❌ prompt-wizard not found. Building it now..." -ForegroundColor Red
    Push-Location $IMENU_DIR
    
    if (-not (Get-Command go -ErrorAction SilentlyContinue)) {
        Write-Host "❌ Go is not installed." -ForegroundColor Red
        Pop-Location
        exit 1
    }
    
    $binDir = Join-Path $IMENU_DIR "dist\bin"
    if (-not (Test-Path $binDir)) {
        New-Item -ItemType Directory -Path $binDir -Force | Out-Null
    }
    
    try {
        go mod tidy 2>$null
        $sourceDir = Join-Path $IMENU_DIR "cmd\prompt-wizard"
        go build -o $WIZARD_BIN $sourceDir
        # Check for .exe extension on Windows
        if (-not (Test-Path $WIZARD_BIN) -and (Test-Path "$WIZARD_BIN.exe")) {
            $WIZARD_BIN = "$WIZARD_BIN.exe"
        }
        Write-Host "✅ Built prompt-wizard successfully!" -ForegroundColor Green
        Write-Host ""
    } finally {
        Pop-Location
    }
}

# Define wizard steps as PowerShell hashtables (more PowerShell-friendly than JSON)
$STEPS = @(
    @{
        type        = "input"
        title       = "What is your name?"
        description = "Enter your full name"
        key         = "name"
        placeholder = "John Doe"
    },
    @{
        type        = "textarea"
        title       = "Tell us about yourself"
        description = "Enter a brief description (multi-line)"
        key         = "description"
        placeholder = "Enter your bio here..."
    },
    @{
        type        = "select"
        title       = "Choose your favorite color"
        description = "Select one option"
        key         = "color"
        options     = @("Red", "Blue", "Green", "Yellow", "Purple")
    },
    @{
        type        = "multiselect"
        title       = "Select your hobbies"
        description = "You can select multiple options"
        key         = "hobbies"
        options     = @("Reading", "Gaming", "Sports", "Music", "Travel")
    },
    @{
        type        = "confirm"
        title       = "Do you want to continue?"
        description = "Final confirmation"
        key         = "continue"
    }
)

# Convert PowerShell hashtables to JSON
$STEPS_JSON = $STEPS | ConvertTo-Json -Depth 10

# Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Write-Host "  Wizard Demo with Back Navigation"
# Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Write-Host ""
Write-Host "Press 'B' at any step to go back to the previous step"
Write-Host ""

# Write JSON to temporary file to avoid command-line argument issues
# Use UTF-8 without BOM to avoid parsing errors
$STEPS_FILE = [System.IO.Path]::GetTempFileName()
$utf8NoBom = New-Object System.Text.UTF8Encoding $false
[System.IO.File]::WriteAllText($STEPS_FILE, $STEPS_JSON, $utf8NoBom)
$RESULT_FILE = [System.IO.Path]::GetTempFileName()

try {
    & $WIZARD_BIN $STEPS_FILE --result-file $RESULT_FILE
} finally {
    # Clean up steps file
    Remove-Item $STEPS_FILE -Force -ErrorAction SilentlyContinue
}

Write-Host ""
# Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Write-Host "  Results:"
# Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
try {
    Get-Content $RESULT_FILE -Raw | ConvertFrom-Json | ConvertTo-Json -Depth 10 | Write-Host
} catch {
    Get-Content $RESULT_FILE -Raw | Write-Host
}
Remove-Item $RESULT_FILE -Force -ErrorAction SilentlyContinue
Write-Host ""

