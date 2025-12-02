# Demo script showing how to use prompt-huh from PowerShell

$ErrorActionPreference = "Stop"

$SCRIPT_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path
$IMENU_DIR = Split-Path -Parent $SCRIPT_DIR
$PROMPT_BIN = Join-Path $IMENU_DIR "dist\bin\prompt-huh"

# Check for .exe extension on Windows
if (Test-Path "$PROMPT_BIN.exe") {
    $PROMPT_BIN = "$PROMPT_BIN.exe"
}

# Check if prompt-huh is built
if (-not (Test-Path $PROMPT_BIN)) {
    Write-Host "❌ prompt-huh not found. Building it now..." -ForegroundColor Red
    Push-Location $IMENU_DIR
    
    # Check if Go is installed, offer to install if not
    if (-not (Get-Command go -ErrorAction SilentlyContinue)) {
        Write-Host "❌ Go is not installed." -ForegroundColor Red
        Write-Host ""
        $response = Read-Host "Install Go automatically? (Y/n)"
        if ($response -notmatch "^[Nn]$") {
            $bootstrapScript = Join-Path $SCRIPT_DIR "bootstrap-go.ps1"
            if (Test-Path $bootstrapScript) {
                & $bootstrapScript
            } else {
                Write-Host "📥 Please install Go manually:" -ForegroundColor Yellow
                Write-Host "   - Windows: https://go.dev/dl/" -ForegroundColor Yellow
                Write-Host "   - Or use: winget install GoLang.Go" -ForegroundColor Yellow
                Pop-Location
                exit 1
            }
        } else {
            Write-Host "❌ Cannot build without Go. Exiting." -ForegroundColor Red
            Pop-Location
            exit 1
        }
    }
    
    Write-Host "🔨 Building prompt-huh..." -ForegroundColor Yellow
    $binDir = Join-Path $IMENU_DIR "dist\bin"
    if (-not (Test-Path $binDir)) {
        New-Item -ItemType Directory -Path $binDir -Force | Out-Null
    }
    
    try {
        go mod tidy 2>$null
        $sourceDir = Join-Path $IMENU_DIR "cmd\prompt-huh"
        go build -o $PROMPT_BIN $sourceDir
        # Check for .exe extension on Windows
        if (-not (Test-Path $PROMPT_BIN) -and (Test-Path "$PROMPT_BIN.exe")) {
            $PROMPT_BIN = "$PROMPT_BIN.exe"
        }
        Write-Host "✅ Built prompt-huh successfully!" -ForegroundColor Green
        Write-Host ""
    } finally {
        Pop-Location
    }
}

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Write-Host "  Interactive Prompt Demo using huh?"
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Write-Host ""

# Example 1: Text input
Write-Host "📝 Example 1: Text Input" -ForegroundColor Cyan
$RESULT_FILE = [System.IO.Path]::GetTempFileName()
& $PROMPT_BIN input "What is your name?" --result-file $RESULT_FILE
$NAME = Get-Content $RESULT_FILE -Raw
Remove-Item $RESULT_FILE -Force -ErrorAction SilentlyContinue
Write-Host "✅ You entered: $NAME" -ForegroundColor Green
Write-Host ""

# Example 2: Text input with default
Write-Host "📝 Example 2: Text Input with Default" -ForegroundColor Cyan
$RESULT_FILE = [System.IO.Path]::GetTempFileName()
& $PROMPT_BIN input "Repository name:" "my-repo" --result-file $RESULT_FILE
$REPO = Get-Content $RESULT_FILE -Raw
Remove-Item $RESULT_FILE -Force -ErrorAction SilentlyContinue
Write-Host "✅ Repository: $REPO" -ForegroundColor Green
Write-Host ""

# Example 3: Select from options
Write-Host "📝 Example 3: Select from Options" -ForegroundColor Cyan
$RESULT_FILE = [System.IO.Path]::GetTempFileName()
& $PROMPT_BIN select "Choose a color:" "Red" "Blue" "Green" "Yellow" "Purple" --result-file $RESULT_FILE
$COLOR = Get-Content $RESULT_FILE -Raw
Remove-Item $RESULT_FILE -Force -ErrorAction SilentlyContinue
Write-Host "✅ You chose: $COLOR" -ForegroundColor Green
Write-Host ""

# Example 4: Confirmation
Write-Host "📝 Example 4: Confirmation" -ForegroundColor Cyan
$RESULT_FILE = [System.IO.Path]::GetTempFileName()
& $PROMPT_BIN confirm "Do you want to continue with the demo?" --result-file $RESULT_FILE
$RESULT = Get-Content $RESULT_FILE -Raw
Remove-Item $RESULT_FILE -Force -ErrorAction SilentlyContinue
if ($RESULT -match "yes") {
    Write-Host "✅ User confirmed - continuing..." -ForegroundColor Green
} else {
    Write-Host "❌ User cancelled - stopping demo" -ForegroundColor Red
    exit 0
}
Write-Host ""

# Example 5: Multi-select
Write-Host "📝 Example 5: Multi-Select" -ForegroundColor Cyan
Write-Host "Select multiple colors (space to toggle, enter when done):"
$RESULT_FILE = [System.IO.Path]::GetTempFileName()
& $PROMPT_BIN multiselect "Choose your favorite colors:" "Red" "Blue" "Green" "Yellow" "Purple" --result-file $RESULT_FILE
$SELECTED = Get-Content $RESULT_FILE -Raw
Remove-Item $RESULT_FILE -Force -ErrorAction SilentlyContinue
Write-Host "✅ You selected:" -ForegroundColor Green
if ($SELECTED -and $SELECTED.Trim()) {
    $SELECTED.Trim() -split "`n" | ForEach-Object {
        $item = $_.Trim()
        if ($item) {
            Write-Host "  • $item"
        }
    }
} else {
    Write-Host "  (none)"
}
Write-Host ""

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Write-Host "  Demo Complete!"
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

