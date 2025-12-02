# Setup SSH Key for GitHub Authentication
# Standalone script to create and configure SSH key for GitHub

[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"

# Configuration
$SSH_CONFIG = "$env:USERPROFILE\.ssh\config"
$GIT_EMAIL = if ($env:GIT_EMAIL) { $env:GIT_EMAIL } else { 
    $globalEmail = git config --global user.email 2>$null
    if ($globalEmail) { $globalEmail } else { "user@example.com" }
}

function Invoke-iWizard { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [array]$Steps,
        
        [Parameter(Mandatory = $false)]
        [string]$ResultFile
    )
    
    # Resolve wizard binary path
    $scriptDir = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.PSCommandPath }
    $pathsToCheck = @()
    
    if ($scriptDir) {
        $pathsToCheck += Join-Path $scriptDir "prompt-wizard.exe"
        $pathsToCheck += Join-Path $scriptDir "prompt-wizard"
        $parentDir = Split-Path -Parent $scriptDir
        if ($parentDir) {
            $pathsToCheck += Join-Path $parentDir "prompt-wizard.exe"
            $pathsToCheck += Join-Path $parentDir "prompt-wizard"
        }
    }
    
    $wizardBin = $null
    foreach ($path in $pathsToCheck) {
        if (Test-Path $path) {
            $wizardBin = (Resolve-Path $path).Path
            break
        }
    }
    
    if (-not $wizardBin) {
        Write-Error "❌ Wizard executable not found. Checked:`n$($pathsToCheck -join "`n")" -ErrorAction Stop
        return $null
    }
    
    # Convert steps to JSON array (handle single item case)
    $stepsArray = @($Steps)
    $jsonContent = $stepsArray | ConvertTo-Json -Depth 10
    $jsonContent = $jsonContent.Trim()
    
    # PowerShell's ConvertTo-Json creates an object {} for single items instead of array [{}]
    if ($jsonContent.StartsWith('{')) {
        $jsonContent = "[$jsonContent]"
    }
    
    # Validate JSON
    try {
        $null = $jsonContent | ConvertFrom-Json
    } catch {
        Write-Error "Invalid JSON generated for wizard steps: $_" -ErrorAction Stop
        return $null
    }
    
    # Create temp file for JSON (UTF-8 without BOM)
    $stepsFile = [System.IO.Path]::GetTempFileName()
    $utf8NoBom = New-Object System.Text.UTF8Encoding $false
    [System.IO.File]::WriteAllText($stepsFile, $jsonContent, $utf8NoBom)
    
    $resultFilePath = if ($ResultFile) { 
        if (Test-Path $ResultFile) {
            (Resolve-Path $ResultFile).Path
        } else {
            $ResultFile
        }
    } else { 
        [System.IO.Path]::GetTempFileName() 
    }
    $isTempResult = [string]::IsNullOrWhiteSpace($ResultFile)
    
    try {
        $stepsFileFullPath = if (Test-Path $stepsFile) { 
            (Resolve-Path $stepsFile).Path 
        } else { 
            $stepsFile 
        }
        
        [Console]::Out.Flush()
        [Console]::Error.Flush()
        
        $process = Start-Process -FilePath $wizardBin `
            -ArgumentList $stepsFileFullPath, "--result-file", $resultFilePath `
            -Wait -NoNewWindow -PassThru
        $exitCode = $process.ExitCode
        
        if ($exitCode -eq 0 -and (Test-Path $resultFilePath)) {
            $result = Get-Content -Path $resultFilePath -Raw
            return [string]$result
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

function Get-WizardResults { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$JsonResult
    )
    
    $jsonResult = $JsonResult.Trim()
    if ([string]::IsNullOrWhiteSpace($jsonResult)) {
        Write-Error "Wizard returned empty result" -ErrorAction Stop
        return $null
    }
    
    try {
        $results = $jsonResult | ConvertFrom-Json
        $resultHash = @{}
        
        foreach ($property in $results.PSObject.Properties) {
            $resultHash[$property.Name] = $property.Value
        }
        
        return $resultHash
    } catch {
        Write-Error "Failed to parse wizard result: $_`nResult was: $jsonResult" -ErrorAction Stop
        return $null
    }
} #<

function Write-BoxedHeader { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$Title,
        
        [Parameter(Mandatory = $false)]
        [int]$Width = 80
    )
    
    $displayTitle = if ($Title.Length % 2 -eq 1) { "$Title " } else { $Title }
    $padding = [Math]::Max(0, ($Width - $displayTitle.Length) / 2)
    $leftPad = " " * [Math]::Floor($padding)
    $rightPad = " " * [Math]::Ceiling($padding)
    $topBottom = "━" * $Width
    
    Write-Host "┏$topBottom┓"
    Write-Host "┃$leftPad$displayTitle$rightPad┃"
    Write-Host "┗$topBottom┛"
    Write-Host ""
} #<

function New-WizardStep { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [ValidateSet("input", "confirm", "select", "multiselect", "textarea")]
        [string]$Type,
        
        [Parameter(Mandatory = $true)]
        [string]$Title,
        
        [Parameter(Mandatory = $true)]
        [string]$Key,
        
        [Parameter(Mandatory = $false)]
        [string]$Description,
        
        [Parameter(Mandatory = $false)]
        [string]$Placeholder,
        
        [Parameter(Mandatory = $false)]
        [string]$Default,
        
        [Parameter(Mandatory = $false)]
        [array]$Options
    )
    
    $step = @{
        type = $Type
        title = $Title
        key = $Key
    }
    
    if ($Description) { $step.description = $Description }
    if ($Placeholder) { $step.placeholder = $Placeholder }
    if ($Default) { $step.default = $Default }
    if ($Options) { $step.options = $Options }
    
    return $step
} #<

function Build-WizardSteps { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [array]$ExistingKeys
    )
    
    $steps = @()
    
    if ($ExistingKeys.Count -gt 0) {
        # Build options for select menu
        # Put "Create new key" first so it's the default (wizard always selects index 0)
        $menuOptions = @("Create new key")
        foreach ($key in $ExistingKeys) {
            $fingerprint = if ($key.Fingerprint) { "($($key.Fingerprint))" } else { "" }
            $comment = if ($key.Comment) { " - $($key.Comment)" } else { "" }
            $menuOptions += "$($key.Name)$comment $fingerprint"
        }
        
        $steps += New-WizardStep `
            -Type "select" `
            -Title "Select SSH Key" `
            -Key "key_selection" `
            -Description "Choose an existing key or create a new one" `
            -Options $menuOptions
    } else {
        # No existing keys, just ask for new key name
        $steps += New-WizardStep `
            -Type "input" `
            -Title "SSH key name" `
            -Key "new_key_name" `
            -Placeholder "github_pi" `
            -Description "Note: 'github_' will be automatically prefixed if not present"
    }
    
    return $steps
} #<

function Add-SshKeyToGitHub { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$PublicKeyPath,
        
        [Parameter(Mandatory = $true)]
        [string]$KeyName
    )
    
    Write-Host "Attempting to add SSH key to GitHub..." -ForegroundColor Cyan
    Write-Host ""
    
    # Try GitHub API first
    $githubToken = $env:GITHUB_TOKEN
    if (-not $githubToken -and (Test-Path "$env:USERPROFILE\.secrets")) {
        $secretsContent = Get-Content "$env:USERPROFILE\.secrets" -ErrorAction SilentlyContinue
        $tokenLine = $secretsContent | Select-String -Pattern "^GITHUB_TOKEN="
        if ($tokenLine) {
            $githubToken = ($tokenLine -split "=")[1] -replace '"', '' -replace "'", ''
        }
    }
    
    if ($githubToken) {
        Write-Host "Using GitHub API (fully automated)..." -ForegroundColor DarkGray
        
        $publicKeyContent = Get-Content $PublicKeyPath -Raw
        $keyTitle = "Windows ($KeyName)"
        
        $body = @{
            title = $keyTitle
            key = $publicKeyContent.Trim()
        } | ConvertTo-Json
        
        $headers = @{
            Accept = "application/vnd.github.v3+json"
            Authorization = "token $githubToken"
        }
        
        try {
            $response = Invoke-RestMethod -Uri "https://api.github.com/user/keys" -Method Post -Headers $headers -Body $body -ErrorAction Stop
            Write-Host "  ✓ SSH key successfully added to GitHub via API!" -ForegroundColor Green
            Write-Host "     Title: $keyTitle" -ForegroundColor Green
            Write-Host "     Key ID: $($response.id)" -ForegroundColor Green
            Write-Host ""
            return $true
        } catch {
            if ($_.Exception.Response.StatusCode -eq 422) {
                Write-Host "  Key may already exist on GitHub" -ForegroundColor Yellow
                $errorContent = $_.ErrorDetails.Message
                if ($errorContent -match "already exists" -or $errorContent -match "key is already in use") {
                    Write-Host "  This key is already registered on your GitHub account" -ForegroundColor Cyan
                    return $true
                }
            }
        }
    }
    
    # Try GitHub CLI as fallback
    if (Get-Command gh -ErrorAction SilentlyContinue) {
        $null = gh auth status 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "Using GitHub CLI to add SSH key..." -ForegroundColor DarkGray
            $keyTitle = "Windows ($KeyName)"
            $absoluteKeyPath = (Resolve-Path $PublicKeyPath -ErrorAction SilentlyContinue).Path
            if (-not $absoluteKeyPath) {
                $absoluteKeyPath = $PublicKeyPath
            }
            
            # Capture output silently to avoid showing "Error:" messages
            # Use Start-Process with separate files for stdout and stderr
            $tempStdoutFile = [System.IO.Path]::GetTempFileName()
            $tempStderrFile = [System.IO.Path]::GetTempFileName()
            $process = Start-Process -FilePath "gh" `
                -ArgumentList "ssh-key", "add", $absoluteKeyPath, "--title", $keyTitle `
                -RedirectStandardOutput $tempStdoutFile `
                -RedirectStandardError $tempStderrFile `
                -NoNewWindow `
                -Wait `
                -PassThru
            $exitCode = $process.ExitCode
            $stdout = Get-Content -Path $tempStdoutFile -Raw -ErrorAction SilentlyContinue
            $stderr = Get-Content -Path $tempStderrFile -Raw -ErrorAction SilentlyContinue
            $output = if ($stdout) { $stdout } else { "" }
            $output += if ($stderr) { "`n" + $stderr } else { "" }
            Remove-Item -Path $tempStdoutFile -Force -ErrorAction SilentlyContinue
            Remove-Item -Path $tempStderrFile -Force -ErrorAction SilentlyContinue
            
            # Check if key already exists (success case) - check output regardless of exit code
            # This handles cases where GitHub CLI returns error code but key exists
            if ($output -match "already exists" -or $output -match "already in use" -or $output -match "Public key already exists") {
                Write-Host "  ✓ Selected SSH key already exists on your GitHub account" -ForegroundColor Green
                Write-Host ""
                return $true
            }
            
            if ($exitCode -eq 0) {
                Write-Host "  ✓ SSH key successfully added to GitHub!" -ForegroundColor Green
                Write-Host "     Title: $keyTitle" -ForegroundColor Green
                Write-Host ""
                return $true
            }
        }
    }
    
    Write-Host "  Could not automatically add SSH key to GitHub" -ForegroundColor Yellow
    Write-Host ""
    return $false
} #<

function Setup-SshKey { #>
    Write-BoxedHeader -Title "SSH Key Setup"
    
    # Discover existing SSH keys
    $sshDir = "$env:USERPROFILE\.ssh"
    $existingKeys = @()
    
    if (Test-Path $sshDir) {
        $allKeys = Get-ChildItem -Path $sshDir -File -ErrorAction SilentlyContinue | 
            Where-Object { 
                $_.Extension -ne ".pub" -and 
                $_.Name -notmatch "^(known_hosts|config|authorized_keys)" -and
                $_.Name -notmatch "\.bak$"
            }
        
        foreach ($key in $allKeys) {
            $pubKeyPath = "$($key.FullName).pub"
            if (Test-Path $pubKeyPath) {
                $keyInfo = @{
                    Name = $key.BaseName
                    PrivatePath = $key.FullName
                    PublicPath = $pubKeyPath
                    Fingerprint = ""
                    Comment = ""
                }
                
                $pubKeyContent = Get-Content $pubKeyPath -Raw -ErrorAction SilentlyContinue
                if ($pubKeyContent) {
                    $keyParts = $pubKeyContent.Trim() -split '\s+', 3
                    if ($keyParts.Count -ge 3) {
                        $keyInfo.Comment = $keyParts[2]
                    }
                    
                    $fingerprint = ssh-keygen -lf $pubKeyPath 2>$null
                    if ($fingerprint) {
                        $keyInfo.Fingerprint = ($fingerprint -split '\s+')[1]
                    }
                }
                
                $existingKeys += $keyInfo
            }
        }
        
        $existingKeys = $existingKeys | Sort-Object {
            if ($_.Name.StartsWith("github_")) { "0$($_.Name)" } else { "1$($_.Name)" }
        }
    }
    
    # Build wizard steps
    $wizardSteps = Build-WizardSteps -ExistingKeys $existingKeys
    
    # Run wizard if we have steps
    $wizardResults = $null
    if ($wizardSteps.Count -gt 0) {
        if (-not [Environment]::UserInteractive) {
            Write-Warning "Console is not interactive. Wizard may not work properly."
        }
        
        $wizardJsonResult = Invoke-iWizard -Steps $wizardSteps
        if (-not $wizardJsonResult) {
            Write-Error "Wizard was cancelled or failed" -ErrorAction Stop
            return
        }
        
        $jsonResultString = if ($wizardJsonResult -is [array]) {
            ($wizardJsonResult | Out-String).Trim()
        } else {
            [string]$wizardJsonResult
        }
        
        $wizardResults = Get-WizardResults -JsonResult $jsonResultString
        if (-not $wizardResults) {
            return
        }
    }
    
    # Process wizard results
    $sshKeyName = $null
    $sshKeyPath = $null
    $sshPublicKey = $null
    $createNewKey = $false
    
    if ($wizardResults) {
        if ($wizardResults.ContainsKey("key_selection")) {
            $selected = $wizardResults["key_selection"]
            
            if ($selected -eq "Create new key") {
                # Need to get new key name
                $newKeySteps = @(
                    (New-WizardStep `
                        -Type "input" `
                        -Title "SSH key name" `
                        -Key "new_key_name" `
                        -Placeholder "github_pi" `
                        -Description "Note: 'github_' will be automatically prefixed if not present")
                )
                
                $newKeyResult = Invoke-iWizard -Steps $newKeySteps
                if ($newKeyResult) {
                    $newKeyJsonString = if ($newKeyResult -is [array]) {
                        ($newKeyResult | Out-String).Trim()
                    } else {
                        [string]$newKeyResult
                    }
                    $newKeyResults = Get-WizardResults -JsonResult $newKeyJsonString
                    if ($newKeyResults -and $newKeyResults.ContainsKey("new_key_name")) {
                        $keyNameInput = $newKeyResults["new_key_name"].ToString().Trim()
                        if (-not $keyNameInput.StartsWith("github_")) {
                            $sshKeyName = "github_$keyNameInput"
                        } else {
                            $sshKeyName = $keyNameInput
                        }
                        $createNewKey = $true
                    }
                }
            } else {
                # Find selected key
                foreach ($key in $existingKeys) {
                    $fingerprint = if ($key.Fingerprint) { "($($key.Fingerprint))" } else { "" }
                    $comment = if ($key.Comment) { " - $($key.Comment)" } else { "" }
                    $keyDisplay = "$($key.Name)$comment $fingerprint"
                    if ($keyDisplay -eq $selected) {
                        $sshKeyName = $key.Name
                        $sshKeyPath = $key.PrivatePath
                        $sshPublicKey = Get-Content $key.PublicPath -Raw
                        break
                    }
                }
            }
        } elseif ($wizardResults.ContainsKey("new_key_name")) {
            $keyNameInput = $wizardResults["new_key_name"].ToString().Trim()
            if (-not $keyNameInput.StartsWith("github_")) {
                $sshKeyName = "github_$keyNameInput"
            } else {
                $sshKeyName = $keyNameInput
            }
            $createNewKey = $true
        }
    }
    
    if (-not $sshKeyName) {
        Write-Error "SSH key name is required" -ErrorAction Stop
        return
    }
    
    if ($createNewKey) {
        $sshKeyPath = "$sshDir\$sshKeyName"
        
        if (Test-Path $sshKeyPath) {
            Write-Host "SSH key already exists at $sshKeyPath" -ForegroundColor Yellow
            Write-Host "Removing existing key..." -ForegroundColor Cyan
            Remove-Item $sshKeyPath -ErrorAction SilentlyContinue
            Remove-Item "$sshKeyPath.pub" -ErrorAction SilentlyContinue
        }
        
        Write-Host "Generating SSH key pair..." -ForegroundColor Cyan
        if (-not (Test-Path $sshDir)) {
            New-Item -ItemType Directory -Path $sshDir -Force | Out-Null
        }
        
        ssh-keygen -t ed25519 -C $GIT_EMAIL -f $sshKeyPath -N '""'
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✓ SSH key created: $sshKeyPath" -ForegroundColor Green
            
            $keyFile = Get-Item $sshKeyPath
            $keyFile.Attributes = "Archive"
            icacls $sshKeyPath /inheritance:r /grant "${env:USERNAME}:F" | Out-Null
            
            $pubKeyFile = Get-Item "$sshKeyPath.pub"
            $pubKeyFile.Attributes = "Archive"
            
            $sshPublicKey = Get-Content "$sshKeyPath.pub" -Raw
        } else {
            Write-Host "Failed to generate SSH key" -ForegroundColor Red
            return
        }
    }
    
    # Try to automatically add the key to GitHub
    $sshKeyAdded = Add-SshKeyToGitHub -PublicKeyPath "$sshKeyPath.pub" -KeyName $sshKeyName
    
    if (-not $sshKeyAdded -and $sshPublicKey) {
        Write-Host ""
        Write-Host "Your public SSH key:" -ForegroundColor Cyan
        Write-Host "============================================================"
        Write-Host $sshPublicKey.Trim()
        Write-Host "============================================================"
        Write-Host ""
        Write-Host "Could not automatically add SSH key to GitHub" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "Manual steps to add SSH key:" -ForegroundColor Cyan
        Write-Host "   1. Go to: https://github.com/settings/keys"
        Write-Host "   2. Click 'New SSH key'"
        Write-Host "   3. Title: 'Windows ($sshKeyName)'"
        Write-Host "   4. Key type: 'Authentication Key'"
        Write-Host "   5. Paste the public key shown above"
        Write-Host "   6. Click 'Add SSH key'"
        Write-Host ""
    }
} #<

try {
    Setup-SshKey
} catch {
    Write-Host "Error: $_" -ForegroundColor Red
    exit 1
}
