# Setup SSH Config for GitHub
# Standalone script to configure SSH config for GitHub authentication

[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"

# Configuration
$SSH_CONFIG = "$env:USERPROFILE\.ssh\config"

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
        [string]$Default
    )
    
    $step = @{
        type = $Type
        title = $Title
        key = $Key
    }
    
    if ($Description) { $step.description = $Description }
    if ($Placeholder) { $step.placeholder = $Placeholder }
    if ($Default) { $step.default = $Default }
    
    return $step
} #<

function Build-WizardSteps { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$SshConfigPath,
        
        [Parameter(Mandatory = $false)]
        [array]$ExistingKeys
    )
    
    $steps = @()
    
    # Get SSH key name step
    $defaultKey = $null
    if ($ExistingKeys -and $ExistingKeys.Count -gt 0) {
        $defaultKey = $ExistingKeys | Where-Object { $_.StartsWith("github_") } | Select-Object -First 1
        if (-not $defaultKey) {
            $defaultKey = $ExistingKeys[0]
        }
    }
    
    if (-not $defaultKey) {
        $defaultKey = "github_pi"
    }
    
    # Check if config already exists and uses a key
    $configKeyName = $null
    if (Test-Path $SshConfigPath) {
        $configContent = Get-Content $SshConfigPath -Raw -ErrorAction SilentlyContinue
        if ($configContent -match "IdentityFile\s+~?/?\.ssh/([^\s]+)") {
            $configKeyName = $matches[1]
        }
    }
    
    if ($configKeyName) {
        $steps += New-WizardStep `
            -Type "input" `
            -Title "SSH key name" `
            -Key "ssh_key_name" `
            -Placeholder $configKeyName `
            -Default $configKeyName `
            -Description "Using key from existing SSH config"
    } else {
        $steps += New-WizardStep `
            -Type "input" `
            -Title "SSH key name" `
            -Key "ssh_key_name" `
            -Placeholder $defaultKey `
            -Default $defaultKey
    }
    
    # Check if we need to update config
    if (Test-Path $SshConfigPath) {
        $configContent = Get-Content $SshConfigPath -Raw
        if ($configContent -match "Host github\.com") {
            $steps += New-WizardStep `
                -Type "confirm" `
                -Title "Update existing SSH config?" `
                -Key "update_config" `
                -Description "SSH config already contains GitHub configuration"
        }
    }
    
    return $steps
} #<

function Setup-SshConfig { #>
    Write-BoxedHeader -Title "GitHub SSH Configuration"
    
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
                $existingKeys += $key.BaseName
            }
        }
    }
    
    # Build wizard steps
    $wizardSteps = Build-WizardSteps -SshConfigPath $SSH_CONFIG -ExistingKeys $existingKeys
    
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
    
    # Extract values from wizard results
    $sshKeyName = $null
    if ($wizardResults -and $wizardResults.ContainsKey("ssh_key_name") -and $wizardResults["ssh_key_name"]) {
        $sshKeyName = $wizardResults["ssh_key_name"].ToString().Trim()
    }
    
    if (-not $sshKeyName) {
        Write-Error "SSH key name is required" -ErrorAction Stop
        return
    }
    
    $shouldUpdateConfig = $false
    if ($wizardResults -and $wizardResults.ContainsKey("update_config")) {
        $updateValue = $wizardResults["update_config"]
        if ($updateValue -is [bool]) {
            $shouldUpdateConfig = $updateValue
        } elseif ($updateValue -is [string]) {
            $updateValueLower = $updateValue.ToLower()
            $shouldUpdateConfig = $updateValueLower -eq "true" -or $updateValueLower -eq "yes" -or $updateValueLower -eq "y"
        }
    }
    
    # Check if config already uses the selected key
    if (Test-Path $SSH_CONFIG) {
        $configContent = Get-Content $SSH_CONFIG -Raw
        if ($configContent -match "Host github\.com") {
            $identityFileMatch = [regex]::Match($configContent, '(?s)Host github\.com.*?IdentityFile\s+([^\r\n]+)')
            if ($identityFileMatch.Success) {
                $existingKeyPath = $identityFileMatch.Groups[1].Value.Trim()
                $existingKeyPath = $existingKeyPath -replace '~', "$env:USERPROFILE"
                $existingKeyPath = $existingKeyPath -replace '/', '\'
                $selectedKeyPath = "$env:USERPROFILE\.ssh\$sshKeyName"
                $selectedKeyPath = $selectedKeyPath -replace '/', '\'
                
                if ($existingKeyPath -like "*$sshKeyName" -or $existingKeyPath -eq $selectedKeyPath) {
                    Write-Host "✓ SSH config already uses the selected key ($sshKeyName)" -ForegroundColor DarkGreen
                    Write-Host ""
                    return
                }
            }
        }
    }
    
    # Remove existing GitHub config block if updating
    if ($shouldUpdateConfig) {
        $lines = Get-Content $SSH_CONFIG
        $newLines = @()
        $skip = $false
        foreach ($line in $lines) {
            if ($line -match "^Host github\.com$") {
                $skip = $true
            } elseif ($skip -and $line -match "^\s*$") {
                $skip = $false
                continue
            } elseif (-not $skip) {
                $newLines += $line
            }
        }
        $newLines | Set-Content $SSH_CONFIG
        Write-Host "Removed existing GitHub configuration" -ForegroundColor Cyan
    }
    
    # Add GitHub SSH config if not present
    $sshDir = Split-Path $SSH_CONFIG -Parent
    if (-not (Test-Path $sshDir)) {
        New-Item -ItemType Directory -Path $sshDir -Force | Out-Null
    }
    
    $configContent = if (Test-Path $SSH_CONFIG) { Get-Content $SSH_CONFIG -Raw } else { "" }
    if ($configContent -notmatch "Host github\.com") {
        $githubConfig = @"

Host github.com
    HostName github.com
    User git
    IdentityFile ~/.ssh/$sshKeyName
    IdentitiesOnly yes

"@
        Add-Content -Path $SSH_CONFIG -Value $githubConfig
        Write-Host "✓ SSH config updated" -ForegroundColor Green
    }
    
    # Set permissions
    if (Test-Path $SSH_CONFIG) {
        icacls $SSH_CONFIG /inheritance:r /grant "${env:USERNAME}:F" | Out-Null
    }
} #<

try {
    Setup-SshConfig
} catch {
    Write-Host "Error: $_" -ForegroundColor Red
    exit 1
}
