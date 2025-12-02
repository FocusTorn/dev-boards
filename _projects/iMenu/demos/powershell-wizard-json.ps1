# Demo script showing the wizard mode using a JSON configuration file
# Uses wizard-example.json for the wizard steps configuration

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

# Use the wizard-example.json file
$CONFIG_FILE = Join-Path $SCRIPT_DIR "wizard-example.json"

if (-not (Test-Path $CONFIG_FILE)) {
    Write-Host "❌ Configuration file not found: $CONFIG_FILE" -ForegroundColor Red
    exit 1
}

Write-Host "  Wizard Demo using JSON Configuration" -ForegroundColor Cyan
Write-Host ""
Write-Host "Configuration file: $CONFIG_FILE" -ForegroundColor Gray
Write-Host "Press 'B' at any step to go back to the previous step" -ForegroundColor Yellow
Write-Host ""

# Create result file
$RESULT_FILE = [System.IO.Path]::GetTempFileName()

try {
    # Run wizard with the JSON configuration file
    & $WIZARD_BIN $CONFIG_FILE --result-file $RESULT_FILE
    $exitCode = $LASTEXITCODE
    
    if ($exitCode -ne 0) {
        Write-Host "Wizard exited with code: $exitCode" -ForegroundColor Red
        exit $exitCode
    }
} catch {
    Write-Host "Error running wizard: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "  Results:" -ForegroundColor Cyan
Write-Host ""
try {
    $results = Get-Content $RESULT_FILE -Raw | ConvertFrom-Json
    $results | ConvertTo-Json -Depth 10 | Write-Host
} catch {
    Get-Content $RESULT_FILE -Raw | Write-Host
}
Remove-Item $RESULT_FILE -Force -ErrorAction SilentlyContinue
Write-Host ""

