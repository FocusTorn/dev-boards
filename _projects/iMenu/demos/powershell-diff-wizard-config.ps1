# Demo script showing all 4 methods of providing wizard input

$ErrorActionPreference = "Stop"

$SCRIPT_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path
$IMENU_DIR = Split-Path -Parent $SCRIPT_DIR
. (Join-Path $IMENU_DIR "wizard.ps1")

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Write-Host "  iMenu Wizard - 4 Methods Demo"
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Write-Host ""

# ┌────────────────────────────────────────────────────────────────────────────┐
# │         Method 1: Inline JSON string directly in the function call         │
# └────────────────────────────────────────────────────────────────────────────┘
Write-Host "Method 1: Inline JSON string" -ForegroundColor Cyan
$json1 = @'
[
    {
        "type": "confirm",
        "title": "Proceed with setup?",
        "key": "proceed",
        "description": "Continue with the demo"
    },
    {
        "type": "input",
        "title": "Enter your name:",
        "key": "name",
        "placeholder": "User"
    }
]
'@

try {
    $results1 = iwizard-RunInline -JsonString $json1
    
    if ($results1) {
        Write-Host "Results (Method 1):" -ForegroundColor Yellow
        try {
            $results1 | ConvertFrom-Json | ConvertTo-Json -Depth 10 | Write-Host
        } catch {
            Write-Host $results1
        }
    } else {
        Write-Host "Wizard cancelled (Method 1)" -ForegroundColor Red
    }
} catch {
    Write-Host "Wizard cancelled (Method 1): $_" -ForegroundColor Red
}
Write-Host ""

# ┌────────────────────────────────────────────────────────────────────────────┐
# │         Method 2: JSON string in a variable, then pass to function         │
# └────────────────────────────────────────────────────────────────────────────┘

Write-Host "Method 2: JSON in variable" -ForegroundColor Cyan
$wizard_config = @'
[
    {
        "type": "select",
        "title": "Select service type:",
        "key": "service",
        "options": [
            "Web Server",
            "Database",
            "Cache"
        ]
    },
    {
        "type": "multiselect",
        "title": "Select features:",
        "key": "features",
        "options": [
            "SSL/TLS",
            "Monitoring",
            "Backup"
        ]
    }
]
'@

try {
    $results2 = iwizard-RunInline -JsonString $wizard_config
    
    if ($results2) {
        Write-Host "Results (Method 2):" -ForegroundColor Yellow
        try {
            $results2 | ConvertFrom-Json | ConvertTo-Json -Depth 10 | Write-Host
        } catch {
            Write-Host $results2
        }
    } else {
        Write-Host "Wizard cancelled (Method 2)" -ForegroundColor Red
    }
} catch {
    Write-Host "Wizard cancelled (Method 2): $_" -ForegroundColor Red
}
Write-Host ""

# ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
# │            Method 3: Using iwizard-RunJson directly (auto-detects file vs string)             │
# └────────────────────────────────────────────────────────────────────────────────────────────────┘

Write-Host "Method 3: Direct call with JSON string (auto-detect)" -ForegroundColor Cyan
$json3 = @'
[
    {
        "type": "confirm",
        "title": "Complete setup?",
        "key": "complete",
        "description": "Finish the demo"
    }
]
'@

try {
    $results3 = iwizard-RunJson -JsonInput $json3
    
    if ($results3) {
        Write-Host "Results (Method 3):" -ForegroundColor Yellow
        try {
            $results3 | ConvertFrom-Json | ConvertTo-Json -Depth 10 | Write-Host
        } catch {
            Write-Host $results3
        }
    } else {
        Write-Host "Wizard cancelled (Method 3)" -ForegroundColor Red
    }
} catch {
    Write-Host "Wizard cancelled (Method 3): $_" -ForegroundColor Red
}
Write-Host ""

# ┌────────────────────────────────────────────────────────────────────────────┐
# │         Method 4: JSON file path (with comments support)                    │
# └────────────────────────────────────────────────────────────────────────────┘

Write-Host "Method 4: JSON file with comments" -ForegroundColor Cyan

# Use the example JSON file (or create a temporary one for demo)
$WIZARD_FILE = Join-Path $SCRIPT_DIR "wizard-example.json"
if (-not (Test-Path $WIZARD_FILE)) {
    # Create temporary file if example doesn't exist
    $WIZARD_FILE = Join-Path $SCRIPT_DIR "wizard_input.json"
    $jsonContent = @'
[
    {
        "type": "confirm",
        "title": "Proceed?",
        "key": "proceed",
        "description": "This is from a file"
    },
    // This is a single-line comment
    {
        "type": "multiselect",
        "title": "Which services?",
        "key": "services",
        "options": [
            "Sensor readings",
            "IAQ (Air quality calculation)",
            "Heat soak detection"
        ]
    },
    {
        "type": "confirm",
        "title": "Final confirmation?",
        "key": "final",
        "description": "Last step"
    }
    
    /* This is a
       multi-line comment */
]
'@
    Set-Content -Path $WIZARD_FILE -Value $jsonContent
    $tempFileCreated = $true
} else {
    $tempFileCreated = $false
}

try {
    $results4 = iwizard-RunJson -JsonInput $WIZARD_FILE
    
    if ($results4) {
        Write-Host "Results (Method 4):" -ForegroundColor Yellow
        try {
            $results4 | ConvertFrom-Json | ConvertTo-Json -Depth 10 | Write-Host
        } catch {
            Write-Host $results4
        }
    } else {
        Write-Host "Wizard cancelled (Method 4)" -ForegroundColor Red
    }
} catch {
    Write-Host "Wizard cancelled (Method 4): $_" -ForegroundColor Red
}

# Clean up (only if it was a temporary file)
if ($tempFileCreated -and (Test-Path $WIZARD_FILE)) {
    Remove-Item -Path $WIZARD_FILE -Force -ErrorAction SilentlyContinue
}

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Write-Host "  Demo Complete!"
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

