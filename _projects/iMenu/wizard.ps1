# Wizard Helper Functions (PowerShell)
# Provides easy-to-use wrapper functions for the prompt-wizard
# Usage: . .\wizard.ps1

# Get the directory where this script is located
$script:IWIZARD_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path
$script:IWIZARD_DIST = Join-Path $script:IWIZARD_DIR "dist"
$script:IWIZARD_BIN_DIR = Join-Path $script:IWIZARD_DIST "bin"
# Detect executable name based on what Go built (Windows gets .exe, others don't)
$script:IWIZARD_BIN = if (Test-Path (Join-Path $script:IWIZARD_BIN_DIR "prompt-wizard.exe")) {
    Join-Path $script:IWIZARD_BIN_DIR "prompt-wizard.exe"
} else {
    Join-Path $script:IWIZARD_BIN_DIR "prompt-wizard"
}

# Ensure wizard is built
function _Ensure-WizardBuilt {
    if (-not (Test-Path $script:IWIZARD_BIN)) {
        if (-not (Get-Command go -ErrorAction SilentlyContinue)) {
            Write-Error "❌ Go is not installed. Cannot build wizard." -ErrorAction Stop
            return $false
        }
        Write-Host "🔨 Building prompt-wizard..." -ForegroundColor Yellow
        Push-Location $script:IWIZARD_DIR
        try {
            if (-not (Test-Path $script:IWIZARD_BIN_DIR)) {
                New-Item -ItemType Directory -Path $script:IWIZARD_BIN_DIR -Force | Out-Null
            }
            go mod tidy 2>$null
            go build -o $script:IWIZARD_BIN .\cmd\prompt-wizard 2>$null
            # On Windows, Go will automatically add .exe extension
            if (-not (Test-Path $script:IWIZARD_BIN)) {
                # Try without .exe extension (Linux/Mac)
                $script:IWIZARD_BIN = Join-Path $script:IWIZARD_BIN_DIR "prompt-wizard"
                if (-not (Test-Path $script:IWIZARD_BIN)) {
                    Write-Error "❌ Failed to build prompt-wizard" -ErrorAction Stop
                    return $false
                }
            }
        } finally {
            Pop-Location
        }
    }
    return $true
}

# Run wizard with JSON input (auto-detects file vs string)
# Usage: iwizard-RunJson '<json-string>' or iwizard-RunJson '/path/to/file.json'
# Usage: iwizard-RunJson '<json-string>' -ResultFile 'path/to/result.json'
function iwizard-RunJson {
    param(
        [Parameter(Mandatory=$true)]
        [string]$JsonInput,
        
        [Parameter(Mandatory=$false)]
        [string]$ResultFile
    )
    
    if ([string]::IsNullOrWhiteSpace($JsonInput)) {
        Write-Error "❌ Error: No JSON input provided" -ErrorAction Stop
        Write-Host "Usage: iwizard-RunJson '<json-string>' [-ResultFile 'path']" -ForegroundColor Yellow
        Write-Host "   Or: iwizard-RunJson '/path/to/file.json' [-ResultFile 'path']" -ForegroundColor Yellow
        return $null
    }
    
    if (-not (_Ensure-WizardBuilt)) {
        return $null
    }
    
    # Create temp file if not provided
    $tempFile = $null
    $isTempFile = $false
    if ([string]::IsNullOrWhiteSpace($ResultFile)) {
        $tempFile = [System.IO.Path]::GetTempFileName()
        $isTempFile = $true
    } else {
        $tempFile = $ResultFile
    }
    
    try {
        # Check if JsonInput is a file path
        $jsonContent = $JsonInput
        if ((Test-Path $JsonInput -ErrorAction SilentlyContinue)) {
            $jsonContent = Get-Content -Path $JsonInput -Raw
        }
        
        # Run wizard directly (not via Start-Process) so it can use the terminal interactively
        # Pass JSON as argument - PowerShell will handle escaping
        & $script:IWIZARD_BIN $jsonContent --result-file $tempFile
        $exitCode = $LASTEXITCODE
        
        if ($exitCode -eq 0) {
            # Read and output results
            if (Test-Path $tempFile) {
                $result = Get-Content -Path $tempFile -Raw
                Write-Output $result
                return $result
            }
            return $null
        } else {
            Write-Error "Wizard exited with code: $exitCode" -ErrorAction Stop
            return $null
        }
    } catch {
        Write-Error "Error running wizard: $_" -ErrorAction Stop
        return $null
    } finally {
        # Clean up temp file if we created it
        if ($isTempFile -and (Test-Path $tempFile)) {
            Remove-Item -Path $tempFile -Force -ErrorAction SilentlyContinue
        }
    }
}

# Run wizard with inline JSON string
# Usage: iwizard-RunInline '<json-string>' [-ResultFile 'path']
function iwizard-RunInline {
    param(
        [Parameter(Mandatory=$true)]
        [string]$JsonString,
        
        [Parameter(Mandatory=$false)]
        [string]$ResultFile
    )
    
    if ([string]::IsNullOrWhiteSpace($JsonString)) {
        Write-Error "❌ Error: No JSON string provided" -ErrorAction Stop
        Write-Host "Usage: iwizard-RunInline '<json-string>' [-ResultFile 'path']" -ForegroundColor Yellow
        return $null
    }
    
    if (-not (_Ensure-WizardBuilt)) {
        return $null
    }
    
    # Create temp file if not provided
    $tempFile = $null
    $isTempFile = $false
    if ([string]::IsNullOrWhiteSpace($ResultFile)) {
        $tempFile = [System.IO.Path]::GetTempFileName()
        $isTempFile = $true
    } else {
        $tempFile = $ResultFile
    }
    
    try {
        # Run wizard directly (not via Start-Process) so it can use the terminal interactively
        # Pass JSON as argument - PowerShell will handle escaping
        & $script:IWIZARD_BIN $JsonString --result-file $tempFile
        $exitCode = $LASTEXITCODE
        
        if ($exitCode -eq 0) {
            # Read and output results
            if (Test-Path $tempFile) {
                $result = Get-Content -Path $tempFile -Raw
                Write-Output $result
                return $result
            }
            return $null
        } else {
            Write-Error "Wizard exited with code: $exitCode" -ErrorAction Stop
            return $null
        }
    } catch {
        Write-Error "Error running wizard: $_" -ErrorAction Stop
        return $null
    } finally {
        # Clean up temp file if we created it
        if ($isTempFile -and (Test-Path $tempFile)) {
            Remove-Item -Path $tempFile -Force -ErrorAction SilentlyContinue
        }
    }
}

# Export functions (make them available after sourcing)
Export-ModuleMember -Function iwizard-RunJson, iwizard-RunInline

