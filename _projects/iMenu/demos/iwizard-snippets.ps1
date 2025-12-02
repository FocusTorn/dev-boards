

# ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
# │               STANDALONE VERSIONS - Copy-paste ready (includes path resolution)                │
# └────────────────────────────────────────────────────────────────────────────────────────────────┘


##>
# Standalone: iwizard-RunJson
# Copy-paste this function into your script - it's completely self-contained
# 
# CONFIGURATION: Edit the $WizardBinPath parameter default below to point to your wizard
# 
# Usage:
#   $result = iwizard-RunJson -JsonInput ".\wizard-config.json"
#   $result = iwizard-RunJson -JsonInput '[{"type":"input","title":"Name","key":"name"}]'
#   $result = iwizard-RunJson -JsonInput ".\config.json" -ResultFile ".\output.json"
#   $result = iwizard-RunJson -JsonInput ".\config.json" -WizardBinPath "..\dist\bin\prompt-wizard"
#<
function iwizard-RunJson { #>
    param( #>
        [Parameter(Mandatory=$true)]
        [string]$JsonInput,
        
        [Parameter(Mandatory=$false)]
        [string]$ResultFile,
        
        [Parameter(Mandatory=$false)]
        [string]$WizardBinPath = ".\dist\bin\prompt-wizard"
    ) #<
    
    #== Resolve wizard binary path ============================================== 
    $wizardBin = $null
    if (Test-Path "$WizardBinPath.exe") {
        $wizardBin = (Resolve-Path "$WizardBinPath.exe").Path
    } elseif (Test-Path $WizardBinPath) {
        $wizardBin = (Resolve-Path $WizardBinPath).Path
    } else {
        Write-Error "❌ Wizard executable not found at: $WizardBinPath (or $WizardBinPath.exe)" -ErrorAction Stop
        return $null
    }
    
    #= Determine if JsonInput is a file path or JSON string ===================== 
    $jsonContent = $JsonInput
    if (Test-Path $JsonInput -ErrorAction SilentlyContinue) {
        $jsonContent = Get-Content -Path $JsonInput -Raw
    }
    
    #= Create temp file for JSON (for UTF-8 without BOM) ======================== 
    $stepsFile = [System.IO.Path]::GetTempFileName()
    $utf8NoBom = New-Object System.Text.UTF8Encoding $false
    [System.IO.File]::WriteAllText($stepsFile, $jsonContent, $utf8NoBom)
    
    # Create result file
    $resultFilePath = if ($ResultFile) { $ResultFile } else { [System.IO.Path]::GetTempFileName() }
    $isTempResult = [string]::IsNullOrWhiteSpace($ResultFile)
    
    try {
        & $wizardBin $stepsFile --result-file $resultFilePath
        $exitCode = $LASTEXITCODE
        
        if ($exitCode -eq 0 -and (Test-Path $resultFilePath)) {
            $result = Get-Content -Path $resultFilePath -Raw
            Write-Output $result
            return $result
        } else {
            Write-Error "Wizard exited with code: $exitCode" -ErrorAction Stop
            return $null
        }
    } catch {
        Write-Error "Error running wizard: $_" -ErrorAction Stop
        return $null
    } finally {
        Remove-Item $stepsFile -Force -ErrorAction SilentlyContinue
        if ($isTempResult -and (Test-Path $resultFilePath)) {
            Remove-Item $resultFilePath -Force -ErrorAction SilentlyContinue
        }
    }
} #<



##> 
# Standalone: iwizard-RunInline
# Copy-paste this function into your script - it's completely self-contained
# 
# CONFIGURATION: Edit the $WizardBinPath parameter default below to point to your wizard
# 
# Usage:
#   $steps = @(
#       @{ type = "input"; title = "Name"; key = "name"; placeholder = "John" },
#       @{ type = "select"; title = "Color"; key = "color"; options = @("Red", "Blue") }
#   )
#   $result = iwizard-RunInline -Steps $steps
#   $result = iwizard-RunInline -Steps $steps -ResultFile ".\output.json"
#   $result = iwizard-RunInline -Steps $steps -WizardBinPath "..\dist\bin\prompt-wizard"
#<
function iwizard-RunInline { #>
    param( #>
        [Parameter(Mandatory=$true)]
        [array]$Steps,
        
        [Parameter(Mandatory=$false)]
        [string]$ResultFile,
        
        [Parameter(Mandatory=$false)]
        [string]$WizardBinPath = ".\dist\bin\prompt-wizard"
    ) #<
    
    #= Resolve wizard binary path =============================================== 
    $wizardBin = $null
    if (Test-Path "$WizardBinPath.exe") {
        $wizardBin = (Resolve-Path "$WizardBinPath.exe").Path
    } elseif (Test-Path $WizardBinPath) {
        $wizardBin = (Resolve-Path $WizardBinPath).Path
    } else {
        Write-Error "❌ Wizard executable not found at: $WizardBinPath (or $WizardBinPath.exe)" -ErrorAction Stop
        return $null
    }
    
    #= Convert PowerShell hashtables to JSON ==================================== 
    $jsonContent = $Steps | ConvertTo-Json -Depth 10
    
    #= Create temp file for JSON (UTF-8 without BOM) ============================ 
    $stepsFile = [System.IO.Path]::GetTempFileName()
    $utf8NoBom = New-Object System.Text.UTF8Encoding $false
    [System.IO.File]::WriteAllText($stepsFile, $jsonContent, $utf8NoBom)
    
    #= Create result file ======================================================= 
    $resultFilePath = if ($ResultFile) { $ResultFile } else { [System.IO.Path]::GetTempFileName() }
    $isTempResult = [string]::IsNullOrWhiteSpace($ResultFile)
    
    try {
        & $wizardBin $stepsFile --result-file $resultFilePath
        $exitCode = $LASTEXITCODE
        
        if ($exitCode -eq 0 -and (Test-Path $resultFilePath)) {
            $result = Get-Content -Path $resultFilePath -Raw
            Write-Output $result
            return $result
        } else {
            Write-Error "Wizard exited with code: $exitCode" -ErrorAction Stop
            return $null
        }
    } catch {
        Write-Error "Error running wizard: $_" -ErrorAction Stop
        return $null
    } finally {
        Remove-Item $stepsFile -Force -ErrorAction SilentlyContinue
        if ($isTempResult -and (Test-Path $resultFilePath)) {
            Remove-Item $resultFilePath -Force -ErrorAction SilentlyContinue
        }
    }
} #<












# # ============================================================================
# # iWizard PowerShell Snippets
# # Copy-paste these functions into your scripts
# # ============================================================================

# # CONFIGURATION: Set the relative path to the prompt-wizard executable
# # Examples:
# #   $IWIZARD_BIN_PATH = ".\dist\bin\prompt-wizard.exe"  # Windows
# #   $IWIZARD_BIN_PATH = "..\dist\bin\prompt-wizard"     # Relative path
# #   $IWIZARD_BIN_PATH = ".\tools\prompt-wizard.exe"    # Custom location
# $IWIZARD_BIN_PATH = ".\dist\bin\prompt-wizard"

# # Resolve the full path to the wizard binary (handles .exe extension automatically)
# function _Resolve-WizardPath {
#     $basePath = $IWIZARD_BIN_PATH
#     if (Test-Path "$basePath.exe") {
#         return (Resolve-Path "$basePath.exe").Path
#     } elseif (Test-Path $basePath) {
#         return (Resolve-Path $basePath).Path
#     } else {
#         Write-Error "❌ Wizard executable not found at: $basePath (or $basePath.exe)" -ErrorAction Stop
#         return $null
#     }
# }

# # ============================================================================
# # Function: iwizard-RunJson
# # Runs wizard with JSON input (file path or JSON string)
# # 
# # Usage:
# #   $result = iwizard-RunJson -JsonInput ".\wizard-config.json"
# #   $result = iwizard-RunJson -JsonInput '[{"type":"input","title":"Name","key":"name"}]'
# #   $result = iwizard-RunJson -JsonInput ".\config.json" -ResultFile ".\output.json"
# # ============================================================================
# function iwizard-RunJson {
#     param(
#         [Parameter(Mandatory=$true)]
#         [string]$JsonInput,
        
#         [Parameter(Mandatory=$false)]
#         [string]$ResultFile
#     )
    
#     $wizardBin = _Resolve-WizardPath
#     if (-not $wizardBin) { return $null }
    
#     # Determine if JsonInput is a file path or JSON string
#     $jsonContent = $JsonInput
#     if (Test-Path $JsonInput -ErrorAction SilentlyContinue) {
#         $jsonContent = Get-Content -Path $JsonInput -Raw
#     }
    
#     # Create temp file for JSON (to avoid command-line issues)
#     $stepsFile = [System.IO.Path]::GetTempFileName()
#     $utf8NoBom = New-Object System.Text.UTF8Encoding $false
#     [System.IO.File]::WriteAllText($stepsFile, $jsonContent, $utf8NoBom)
    
#     # Create result file
#     $resultFilePath = if ($ResultFile) { $ResultFile } else { [System.IO.Path]::GetTempFileName() }
#     $isTempResult = [string]::IsNullOrWhiteSpace($ResultFile)
    
#     try {
#         & $wizardBin $stepsFile --result-file $resultFilePath
#         $exitCode = $LASTEXITCODE
        
#         if ($exitCode -eq 0 -and (Test-Path $resultFilePath)) {
#             $result = Get-Content -Path $resultFilePath -Raw
#             Write-Output $result
#             return $result
#         } else {
#             Write-Error "Wizard exited with code: $exitCode" -ErrorAction Stop
#             return $null
#         }
#     } catch {
#         Write-Error "Error running wizard: $_" -ErrorAction Stop
#         return $null
#     } finally {
#         Remove-Item $stepsFile -Force -ErrorAction SilentlyContinue
#         if ($isTempResult -and (Test-Path $resultFilePath)) {
#             Remove-Item $resultFilePath -Force -ErrorAction SilentlyContinue
#         }
#     }
# }

# # ============================================================================
# # Function: iwizard-RunInline
# # Runs wizard with inline PowerShell hashtables (converts to JSON automatically)
# # 
# # Usage:
# #   $steps = @(
# #       @{ type = "input"; title = "Name"; key = "name"; placeholder = "John" },
# #       @{ type = "select"; title = "Color"; key = "color"; options = @("Red", "Blue") }
# #   )
# #   $result = iwizard-RunInline -Steps $steps
# #   $result = iwizard-RunInline -Steps $steps -ResultFile ".\output.json"
# # ============================================================================
# function iwizard-RunInline {
#     param(
#         [Parameter(Mandatory=$true)]
#         [array]$Steps,
        
#         [Parameter(Mandatory=$false)]
#         [string]$ResultFile
#     )
    
#     $wizardBin = _Resolve-WizardPath
#     if (-not $wizardBin) { return $null }
    
#     # Convert PowerShell hashtables to JSON
#     $jsonContent = $Steps | ConvertTo-Json -Depth 10
    
#     # Create temp file for JSON (UTF-8 without BOM)
#     $stepsFile = [System.IO.Path]::GetTempFileName()
#     $utf8NoBom = New-Object System.Text.UTF8Encoding $false
#     [System.IO.File]::WriteAllText($stepsFile, $jsonContent, $utf8NoBom)
    
#     # Create result file
#     $resultFilePath = if ($ResultFile) { $ResultFile } else { [System.IO.Path]::GetTempFileName() }
#     $isTempResult = [string]::IsNullOrWhiteSpace($ResultFile)
    
#     try {
#         & $wizardBin $stepsFile --result-file $resultFilePath
#         $exitCode = $LASTEXITCODE
        
#         if ($exitCode -eq 0 -and (Test-Path $resultFilePath)) {
#             $result = Get-Content -Path $resultFilePath -Raw
#             Write-Output $result
#             return $result
#         } else {
#             Write-Error "Wizard exited with code: $exitCode" -ErrorAction Stop
#             return $null
#         }
#     } catch {
#         Write-Error "Error running wizard: $_" -ErrorAction Stop
#         return $null
#     } finally {
#         Remove-Item $stepsFile -Force -ErrorAction SilentlyContinue
#         if ($isTempResult -and (Test-Path $resultFilePath)) {
#             Remove-Item $resultFilePath -Force -ErrorAction SilentlyContinue
#         }
#     }
# }
