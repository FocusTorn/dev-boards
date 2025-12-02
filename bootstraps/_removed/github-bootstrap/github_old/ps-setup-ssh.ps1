# Setup SSH Key and Config for GitHub Authentication
# Unified script to create/configure SSH key and SSH config for GitHub

[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$KeyName,
    
    [Parameter(Mandatory = $false)]
    [bool]$CreateNew = $false,
    
    [Parameter(Mandatory = $false)]
    [bool]$AddToGitHub = $false,
    
    [Parameter(Mandatory = $false)]
    [bool]$ConfigureConfig = $false
)

$ErrorActionPreference = "Stop"

# Configuration
$SSH_CONFIG = "$env:USERPROFILE\.ssh\config"
$GIT_EMAIL = if ($env:GIT_EMAIL) { $env:GIT_EMAIL } else { 
    $globalEmail = git config --global user.email 2>$null
    if ($globalEmail) { $globalEmail } else { "user@example.com" }
}

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

function Add-SshKeyToGitHub { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$PublicKeyPath,
        
        [Parameter(Mandatory = $true)]
        [string]$KeyName
    )
    
    Write-Host "  Attempting to add SSH key to GitHub..." -ForegroundColor DarkGray
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
        Write-Host "  Using GitHub API (fully automated)..." -ForegroundColor DarkGray
        
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
            Write-Host "    ✓ SSH key successfully added to GitHub via API!" -ForegroundColor DarkGreen
            Write-Host "      Title: $keyTitle" -ForegroundColor DarkGreen
            Write-Host "      Key ID: $($response.id)" -ForegroundColor DarkGreen
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
            Write-Host "  Using GitHub CLI to add SSH key..." -ForegroundColor DarkGray
            $keyTitle = "Windows ($KeyName)"
            $absoluteKeyPath = (Resolve-Path $PublicKeyPath -ErrorAction SilentlyContinue).Path
            if (-not $absoluteKeyPath) {
                $absoluteKeyPath = $PublicKeyPath
            }
            
            # First, check if key already exists on GitHub by listing keys
            $publicKeyContent = Get-Content $PublicKeyPath -Raw -ErrorAction SilentlyContinue
            if ($publicKeyContent) {
                $keyParts = $publicKeyContent.Trim() -split '\s+', 3
                if ($keyParts.Count -ge 2) {
                    $keyType = $keyParts[0]
                    $keyData = $keyParts[1]
                    
                    # Get fingerprint to match against existing keys
                    $fingerprint = ssh-keygen -lf $PublicKeyPath 2>$null
                    if ($fingerprint) {
                        $fingerprintValue = ($fingerprint -split '\s+')[1]
                        
                        # List existing keys on GitHub
                        $tempListFile = [System.IO.Path]::GetTempFileName()
                        $tempListErrFile = [System.IO.Path]::GetTempFileName()
                        $listProcess = Start-Process -FilePath "gh" `
                            -ArgumentList "ssh-key", "list" `
                            -RedirectStandardOutput $tempListFile `
                            -RedirectStandardError $tempListErrFile `
                            -NoNewWindow `
                            -Wait `
                            -PassThru
                        
                        if ($listProcess.ExitCode -eq 0) {
                            $existingKeysList = Get-Content -Path $tempListFile -Raw -ErrorAction SilentlyContinue
                            Remove-Item -Path $tempListFile -Force -ErrorAction SilentlyContinue
                            Remove-Item -Path $tempListErrFile -Force -ErrorAction SilentlyContinue
                            
                            # Check if this key (by fingerprint or key data) already exists
                            if ($existingKeysList -and ($existingKeysList -match [regex]::Escape($fingerprintValue) -or $existingKeysList -match [regex]::Escape($keyData))) {
                                Write-Host "    ✓ Selected SSH key already exists on your GitHub account" -ForegroundColor DarkGreen
                                return $true
                            }
                        } else {
                            Remove-Item -Path $tempListFile -Force -ErrorAction SilentlyContinue
                            Remove-Item -Path $tempListErrFile -Force -ErrorAction SilentlyContinue
                        }
                    }
                }
            }
            
            # Capture output silently to avoid showing "Error:" messages
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
            $combinedOutput = ""
            if ($stdout) { $combinedOutput += $stdout }
            if ($stderr) { $combinedOutput += if ($combinedOutput) { "`n" + $stderr } else { $stderr } }
            Remove-Item -Path $tempStdoutFile -Force -ErrorAction SilentlyContinue
            Remove-Item -Path $tempStderrFile -Force -ErrorAction SilentlyContinue
            
            # Check if key already exists (success case)
            $alreadyExistsPatterns = @(
                "already exists",
                "already in use",
                "Public key already exists",
                "key is already in use",
                "duplicate key",
                "key already registered"
            )
            
            $keyExists = $false
            foreach ($pattern in $alreadyExistsPatterns) {
                if ($combinedOutput -match $pattern -or $stdout -match $pattern -or $stderr -match $pattern) {
                    $keyExists = $true
                    break
                }
            }
            
            if ($keyExists) {
                Write-Host "    ✓ Selected SSH key already exists on your GitHub account" -ForegroundColor DarkGreen
                return $true
            }
            
            if ($exitCode -eq 0) {
                Write-Host "    ✓ SSH key successfully added to GitHub!" -ForegroundColor DarkGreen
                Write-Host "     Title: $keyTitle" -ForegroundColor DarkGreen
                return $true
            }
        }
    }
    
    Write-Host "  Could not automatically add SSH key to GitHub" -ForegroundColor Yellow
    Write-Host ""
    return $false
} #<

function Setup-SshConfig { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$KeyName,
        
        [Parameter(Mandatory = $false)]
        [bool]$UpdateExisting = $false
    )
    
    # Check if config already uses the selected key
    if (Test-Path $SSH_CONFIG) {
        $configContent = Get-Content $SSH_CONFIG -Raw
        if ($configContent -match "Host github\.com") {
            $identityFileMatch = [regex]::Match($configContent, '(?s)Host github\.com.*?IdentityFile\s+([^\r\n]+)')
            if ($identityFileMatch.Success) {
                $existingKeyPath = $identityFileMatch.Groups[1].Value.Trim()
                $existingKeyPath = $existingKeyPath -replace '~', "$env:USERPROFILE"
                $existingKeyPath = $existingKeyPath -replace '/', '\'
                $selectedKeyPath = "$env:USERPROFILE\.ssh\$KeyName"
                $selectedKeyPath = $selectedKeyPath -replace '/', '\'
                
                if ($existingKeyPath -like "*$KeyName" -or $existingKeyPath -eq $selectedKeyPath) {
                    Write-Host "    ✓ SSH config already uses the selected key ($KeyName)" -ForegroundColor DarkGreen
                    return
                }
            }
        }
    }
    
    # Remove existing GitHub config block if updating
    if ($UpdateExisting -and (Test-Path $SSH_CONFIG)) {
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
    IdentityFile ~/.ssh/$KeyName
    IdentitiesOnly yes

"@
        Add-Content -Path $SSH_CONFIG -Value $githubConfig
        Write-Host "    ✓ SSH config updated" -ForegroundColor DarkGreen
    }
    
    # Set permissions
    if (Test-Path $SSH_CONFIG) {
        icacls $SSH_CONFIG /inheritance:r /grant "${env:USERNAME}:F" | Out-Null
    }
} #<

function Setup-Ssh { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $false)]
        [string]$KeyName,
        
        [Parameter(Mandatory = $false)]
        [bool]$CreateNew = $false,
        
        [Parameter(Mandatory = $false)]
        [bool]$AddToGitHub = $false,
        
        [Parameter(Mandatory = $false)]
        [bool]$ConfigureConfig = $false
    )
    
    $sshDir = "$env:USERPROFILE\.ssh"
    $sshKeyPath = $null
    $sshPublicKey = $null
    
    # If KeyName not provided, discover existing keys and use first one
    if (-not $KeyName) {
        if (Test-Path $sshDir) {
            $allKeys = Get-ChildItem -Path $sshDir -File -ErrorAction SilentlyContinue | 
                Where-Object { 
                    $_.Extension -ne ".pub" -and 
                    $_.Name -notmatch "^(known_hosts|config|authorized_keys)" -and
                    $_.Name -notmatch "\.bak$"
                }
            
            $existingKeys = @()
            foreach ($key in $allKeys) {
                $pubKeyPath = "$($key.FullName).pub"
                if (Test-Path $pubKeyPath) {
                    $existingKeys += @{
                        Name = $key.BaseName
                        PrivatePath = $key.FullName
                        PublicPath = $pubKeyPath
                    }
                }
            }
            
            $existingKeys = $existingKeys | Sort-Object {
                if ($_.Name.StartsWith("github_")) { "0$($_.Name)" } else { "1$($_.Name)" }
            }
            
            if ($existingKeys.Count -gt 0) {
                $KeyName = $existingKeys[0].Name
                $sshKeyPath = $existingKeys[0].PrivatePath
                $sshPublicKey = Get-Content $existingKeys[0].PublicPath -Raw
                $CreateNew = $false
            } else {
                Write-Error "No SSH keys found and KeyName not provided" -ErrorAction Stop
                return
            }
        } else {
            Write-Error "SSH directory not found and KeyName not provided" -ErrorAction Stop
            return
        }
    }
    
    # Ensure key name has github_ prefix if creating new
    if ($CreateNew -and -not $KeyName.StartsWith("github_")) {
        $KeyName = "github_$KeyName"
    }
    
    # If not creating new and key path not set, find existing key
    if (-not $CreateNew -and -not $sshKeyPath) {
        $keyPath = "$sshDir\$KeyName"
        $pubKeyPath = "$keyPath.pub"
        if (Test-Path $keyPath) {
            $sshKeyPath = $keyPath
            if (Test-Path $pubKeyPath) {
                $sshPublicKey = Get-Content $pubKeyPath -Raw
            }
        } else {
            Write-Error "SSH key not found: $KeyName" -ErrorAction Stop
            return
        }
    }
    
    # Create new key if needed
    if ($CreateNew) {
        $sshKeyPath = "$sshDir\$KeyName"
        
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
            Write-Host "    ✓ SSH key created: $sshKeyPath" -ForegroundColor DarkGreen
            
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
    
    # Add to GitHub if requested
    if ($AddToGitHub) {
        $sshKeyAdded = Add-SshKeyToGitHub -PublicKeyPath "$sshKeyPath.pub" -KeyName $KeyName
        
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
            Write-Host "   3. Title: 'Windows ($KeyName)'"
            Write-Host "   4. Key type: 'Authentication Key'"
            Write-Host "   5. Paste the public key shown above"
            Write-Host "   6. Click 'Add SSH key'"
            Write-Host ""
        }
    }
    
    # Configure SSH config if requested
    if ($ConfigureConfig) {
        Setup-SshConfig -KeyName $KeyName -UpdateExisting $false
    }
    
    # Final success message
    Write-Host ""
    Write-Host "  ✓ SSH Setup Completed" -ForegroundColor Green
    Write-Host ""
} #<

# Main execution - only run if called directly (not when dot-sourced)
if ($MyInvocation.InvocationName -ne '.') {
    try {
        Setup-Ssh -KeyName $KeyName -CreateNew $CreateNew -AddToGitHub $AddToGitHub -ConfigureConfig $ConfigureConfig
    } catch {
        Write-Host "Error: $_" -ForegroundColor Red
        exit 1
    }
}

