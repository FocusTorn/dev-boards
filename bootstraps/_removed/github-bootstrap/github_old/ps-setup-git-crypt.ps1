# Setup Git-Crypt for Encrypted Secrets
# Standalone script to set up git-crypt for encrypted file storage

$ErrorActionPreference = "Stop"

# Script variables
$script:ORIGINAL_WORKING_DIR = (Get-Location).Path

# Helper Functions
function Write-BoxedHeader {
    param(
        [string]$Title,
        [int]$Width = 80
    )
    
    $displayTitle = $Title
    if ($Title.Length % 2 -eq 1) {
        $displayTitle = "$Title "
    }
    
    $padding = [Math]::Max(0, ($Width - $displayTitle.Length) / 2)
    $leftPad = " " * [Math]::Floor($padding)
    $rightPad = " " * [Math]::Ceiling($padding)
    $topBottom = "-" * $Width
    
    Write-Host "+$topBottom+"
    Write-Host "|$leftPad$displayTitle$rightPad|"
    Write-Host "+$topBottom+"
    Write-Host ""
}

function Get-GitRepoDir {
    if ($env:GIT_REPO_DIR) {
        Write-Host "[!]  GIT_REPO_DIR environment variable is set to: $env:GIT_REPO_DIR" -ForegroundColor Yellow
        Write-Host "   Using current directory instead: $script:ORIGINAL_WORKING_DIR" -ForegroundColor Yellow
        return $script:ORIGINAL_WORKING_DIR
    } else {
        return $script:ORIGINAL_WORKING_DIR
    }
}

function Show-YesNoPrompt {
    param(
        [string]$Prompt,
        [bool]$DefaultNo = $true,
        [switch]$NoBlankLine
    )
    
    $defaultChar = if ($DefaultNo) { "N" } else { "Y" }
    $otherChar = if ($DefaultNo) { "y" } else { "n" }
    $promptText = "$Prompt ($defaultChar/$otherChar)"
    $actualDefault = if ($DefaultNo) { "n" } else { "y" }
    
    Write-Host -NoNewline "${promptText}: " -ForegroundColor Cyan
    $response = Read-Host
    
    if ([string]::IsNullOrWhiteSpace($response)) {
        $response = $actualDefault
        [Console]::CursorTop = [Console]::CursorTop - 1
        [Console]::CursorLeft = 0
        Write-Host -NoNewline "${promptText}: " -ForegroundColor Cyan
        Write-Host $actualDefault
    }
    
    if (-not $NoBlankLine) {
        Write-Host ""
    }
    
    return $response -match "^[Yy]$"
}

function Show-TextInput {
    param(
        [string]$Prompt,
        [string]$DefaultValue = "",
        [bool]$Required = $false
    )
    
    do {
        if ($DefaultValue -and -not [string]::IsNullOrWhiteSpace($DefaultValue)) {
            Write-Host -NoNewline "$Prompt [$DefaultValue]: " -ForegroundColor Cyan
            $input = Read-Host
            if ([string]::IsNullOrWhiteSpace($input)) {
                $input = $DefaultValue
                [Console]::CursorTop = [Console]::CursorTop - 1
                [Console]::CursorLeft = 0
                Write-Host -NoNewline "$Prompt [$DefaultValue]: " -ForegroundColor Cyan
                Write-Host $DefaultValue
            }
        } else {
            Write-Host -NoNewline "${Prompt}: " -ForegroundColor Cyan
            $input = Read-Host
        }
        
        if ($Required -and [string]::IsNullOrWhiteSpace($input)) {
            Write-Host "[!]  This field is required. Please enter a value." -ForegroundColor Yellow
        }
    } while ($Required -and [string]::IsNullOrWhiteSpace($input))
    
    Write-Host ""
    return $input.Trim()
}

function Initialize-GitCrypt {
    $GIT_REPO_DIR = Get-GitRepoDir
    Push-Location $GIT_REPO_DIR
    if (-not (Test-Path ".git")) {
        Write-Host "[x] Not a git repository"
        Pop-Location
        return 1
    }
    
    $null = git-crypt status 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "[!]  git-crypt already initialized"
        Pop-Location
        return
    }
    
    Write-Host "[lock] Initializing git-crypt..."
    git-crypt init
    if ($LASTEXITCODE -eq 0) {
        Write-Host "[ok] git-crypt initialized"
    }
    Pop-Location
}

function Set-GitCryptGitAttributes {
    $GIT_REPO_DIR = Get-GitRepoDir
    Push-Location $GIT_REPO_DIR
    
    if (-not (Test-Path ".gitattributes")) {
        @"
# Encrypted files with git-crypt
.secrets filter=git-crypt diff=git-crypt
*.secrets filter=git-crypt diff=git-crypt
"@ | Set-Content ".gitattributes"
        Write-Host "[ok] Created .gitattributes"
    } else {
        $content = Get-Content ".gitattributes" -Raw
        if ($content -match "^\.secrets") {
            Write-Host "[ok] .secrets already in .gitattributes"
        } else {
            Add-Content ".gitattributes" @"

# Encrypted files with git-crypt
.secrets filter=git-crypt diff=git-crypt
*.secrets filter=git-crypt diff=git-crypt
"@
            Write-Host "[ok] Added .secrets to .gitattributes"
        }
    }
    
    Pop-Location
}

function Update-GitCryptGitIgnore {
    $GIT_REPO_DIR = Get-GitRepoDir
    Push-Location $GIT_REPO_DIR
    
    if (-not (Test-Path ".gitignore")) {
        Write-Host "[x] .gitignore not found"
        Pop-Location
        return 1
    }
    
    $content = Get-Content ".gitignore"
    if ($content -match "^\.secrets$") {
        Copy-Item ".gitignore" ".gitignore.bak"
        $newContent = $content | Where-Object { $_ -ne ".secrets" }
        $newContent | Set-Content ".gitignore"
        Write-Host "[ok] Removed .secrets from .gitignore"
        Write-Host "   Backup saved to .gitignore.bak"
    } else {
        Write-Host "[ok] .secrets not in .gitignore (or already removed)"
    }
    
    $content = Get-Content ".gitignore" -Raw
    if ($content -notmatch "# Encrypted .secrets is tracked") {
        Add-Content ".gitignore" @"

# Note: .secrets is encrypted with git-crypt and IS tracked in git
# The encrypted version is safe to commit
"@
        Write-Host "[ok] Added note about encrypted .secrets"
    }
    
    Pop-Location
}

function Copy-GitCryptSecrets {
    $GIT_REPO_DIR = Get-GitRepoDir
    $secretsFile = "$env:USERPROFILE\.secrets"
    $repoSecrets = "$GIT_REPO_DIR\.secrets"
    
    if (Test-Path $secretsFile) {
        Copy-Item $secretsFile $repoSecrets
        $file = Get-Item $repoSecrets
        $file.Attributes = "Archive"
        icacls $repoSecrets /inheritance:r /grant "${env:USERNAME}:F" | Out-Null
        Write-Host "[ok] Copied secrets file to repository"
    } else {
        Write-Host "[!]  Secrets file not found at $secretsFile"
        Write-Host "   Creating template..."
        @"
# Unified Secrets File
# This file is encrypted with git-crypt
# Add your passwords/keys here
# 
# Format: KEY=VALUE (one per line)
# Comments start with #

# ============================================
# MQTT Broker
# ============================================
MQTT_PASSWORD=
MQTT_USERNAME=mqtt

# ============================================
# GitHub (if needed)
# ============================================
# GITHUB_TOKEN=

# ============================================
# Other APIs / Services
# ============================================
# API_KEY_SERVICE1=
# API_KEY_SERVICE2=
"@ | Set-Content $repoSecrets
        $file = Get-Item $repoSecrets
        $file.Attributes = "Archive"
        icacls $repoSecrets /inheritance:r /grant "${env:USERNAME}:F" | Out-Null
        Write-Host "[ok] Created template .secrets file"
    }
}

# Main Setup Function
function Setup-GitCrypt {
    $GIT_REPO_DIR = Get-GitRepoDir
    
    # Install git-crypt if not already installed
    if (-not (Get-Command git-crypt -ErrorAction SilentlyContinue)) {
        Write-Host "[!]  git-crypt not installed"
        $installGitCrypt = Show-YesNoPrompt -Prompt "Install git-crypt now?" -DefaultNo $false
        if ($installGitCrypt) {
            Write-Host ""
            Write-Host "[pkg] Looking for git-crypt.exe..." -ForegroundColor Cyan
            Write-Host ""
            
            $installSuccess = $false
            $downloadsPath = $null
            try {
                $downloadsGuid = "{374DE290-123F-4565-9164-39C4925E467B}"
                $downloadsPath = (Get-ItemProperty "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Shell Folders").$downloadsGuid
                if ($downloadsPath) {
                    Write-Host "  Using Downloads folder from registry: $downloadsPath" -ForegroundColor Gray
                }
            } catch {
                $downloadsPath = [Environment]::GetFolderPath("UserProfile") + "\Downloads"
                Write-Host "  Using default Downloads folder: $downloadsPath" -ForegroundColor Gray
            }
            
            $gitCryptExe = $null
            
            if ($downloadsPath -and (Test-Path $downloadsPath)) {
                Write-Host "  Checking Downloads folder: $downloadsPath" -ForegroundColor Gray
                $gitCryptExe = Get-ChildItem -Path $downloadsPath -Filter "git-crypt.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
                
                if ($gitCryptExe) {
                    Write-Host "  [ok] Found: $($gitCryptExe.FullName)" -ForegroundColor Green
                } else {
                    Write-Host "  [x] Not found in: $downloadsPath" -ForegroundColor Yellow
                }
            }
            
            if (-not $gitCryptExe) {
                Write-Host "  [x] git-crypt.exe not found in Downloads" -ForegroundColor Yellow
                Write-Host ""
                Write-Host "   Download from: https://github.com/oholovko/git-crypt-windows/releases" -ForegroundColor Cyan
                Write-Host ""
                $searchPath = Show-TextInput -Prompt "Enter directory path to search for git-crypt.exe" -DefaultValue $downloadsPath -Required $false
                
                if ($searchPath -and (Test-Path $searchPath)) {
                    Write-Host "  Searching in: $searchPath" -ForegroundColor Gray
                    $gitCryptExe = Get-ChildItem -Path $searchPath -Filter "git-crypt.exe" -Recurse -ErrorAction SilentlyContinue | Select-Object -First 1
                    if ($gitCryptExe) {
                        Write-Host "  [ok] Found: $($gitCryptExe.FullName)" -ForegroundColor Green
                    } else {
                        Write-Host "  [x] git-crypt.exe not found in: $searchPath" -ForegroundColor Red
                    }
                }
            }
            
            if ($gitCryptExe) {
                Write-Host ""
                Write-Host "  [pkg] Installing git-crypt.exe..." -ForegroundColor Cyan
                
                $targetDirs = @(
                    "$env:USERPROFILE\bin",
                    "$env:USERPROFILE\.local\bin",
                    "$env:LOCALAPPDATA\Programs\git-crypt",
                    "C:\Program Files\Git\usr\bin",
                    "C:\Program Files\Git\cmd",
                    "$env:ProgramFiles\Git\usr\bin",
                    "$env:ProgramFiles\Git\cmd"
                )
                
                $targetDir = $null
                foreach ($dir in $targetDirs) {
                    if (Test-Path $dir) {
                        try {
                            $testFile = Join-Path $dir ".write-test"
                            "test" | Set-Content $testFile -ErrorAction Stop
                            Remove-Item $testFile -ErrorAction SilentlyContinue
                            $targetDir = $dir
                            break
                        } catch {
                            continue
                        }
                    } elseif ($dir -like "$env:USERPROFILE*" -or $dir -like "$env:LOCALAPPDATA*") {
                        try {
                            New-Item -ItemType Directory -Path $dir -Force -ErrorAction Stop | Out-Null
                            $targetDir = $dir
                            break
                        } catch {
                            continue
                        }
                    }
                }
                
                if (-not $targetDir) {
                    $targetDir = "$env:USERPROFILE\bin"
                    try {
                        New-Item -ItemType Directory -Path $targetDir -Force -ErrorAction Stop | Out-Null
                    } catch {
                        Write-Host "  [x] Cannot create target directory: $targetDir" -ForegroundColor Red
                        $installSuccess = $false
                        $targetDir = $null
                    }
                }
                
                if ($targetDir) {
                    try {
                        $targetPath = Join-Path $targetDir "git-crypt.exe"
                        Copy-Item -Path $gitCryptExe.FullName -Destination $targetPath -Force -ErrorAction Stop
                        Write-Host "  [ok] Copied git-crypt.exe to $targetDir" -ForegroundColor Green
                        
                        $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
                        if ($currentPath -notlike "*$targetDir*") {
                            Write-Host "  [*] Adding to user PATH..." -ForegroundColor Cyan
                            [Environment]::SetEnvironmentVariable("Path", "$currentPath;$targetDir", "User")
                            $env:Path += ";$targetDir"
                            Write-Host "  [OK] Added $targetDir to PATH" -ForegroundColor Green
                        }
                        
                        Start-Sleep -Seconds 1
                        if (Get-Command git-crypt -ErrorAction SilentlyContinue) {
                            Write-Host "  [ok] git-crypt is now available!" -ForegroundColor Green
                            $installSuccess = $true
                        } else {
                            Write-Host "  [!]  git-crypt installed but may need PowerShell restart" -ForegroundColor Yellow
                            $installSuccess = $true
                        }
                    } catch {
                        Write-Host "  [x] Failed to install git-crypt: $($_.Exception.Message)" -ForegroundColor Red
                        $installSuccess = $false
                    }
                } else {
                    $installSuccess = $false
                }
            }
            
            if (-not $installSuccess) {
                Write-Host ""
                Write-Host "   Manual installation:" -ForegroundColor Cyan
                Write-Host "   1. Download from: https://github.com/oholovko/git-crypt-windows/releases" -ForegroundColor Gray
                Write-Host "   2. Place git-crypt.exe in a directory in your PATH" -ForegroundColor Gray
                Write-Host "      (e.g., C:\\Program Files\\Git\\usr\\bin)" -ForegroundColor Gray
                Write-Host ""
                $continue = Show-YesNoPrompt -Prompt "Continue without git-crypt? (You can install it manually later)" -DefaultNo $true
                if (-not $continue) {
                    Write-Host "[x] Cannot proceed without git-crypt"
                    return 1
                }
            }
        } else {
            Write-Host "[x] Cannot proceed without git-crypt"
            return 1
        }
    }
    
    Push-Location $GIT_REPO_DIR
    if (-not (Test-Path ".git")) {
        Write-Host "[x] Not a git repository. Run 'Setup local git repository' first."
        Pop-Location
        return 1
    }
    
    Write-BoxedHeader -Title "Git-Crypt Setup"
    
    Write-Host "[lock] Setting up git-crypt..."
    Write-Host ""
    
    # Initialize git-crypt
    Initialize-GitCrypt
    Write-Host ""
    
    # Setup .gitattributes
    Set-GitCryptGitAttributes
    Write-Host ""
    
    # Update .gitignore
    Update-GitCryptGitIgnore
    Write-Host ""
    
    # Copy secrets file to repo
    Copy-GitCryptSecrets
    Write-Host ""
    
    Write-Host "------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------"
    Write-Host "[ok] git-crypt setup complete!"
    Write-Host "------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------"
    Write-Host ""
    Write-Host "[note] Next steps:"
    Write-Host "   1. Edit .secrets file if needed:"
    Write-Host "      notepad $GIT_REPO_DIR\.secrets"
    Write-Host ""
    Write-Host "   2. Add and commit:"
    Write-Host "      cd $GIT_REPO_DIR"
    Write-Host "      git add .gitattributes .secrets"
    Write-Host "      git commit -m 'Add encrypted .secrets with git-crypt'"
    Write-Host ""
    Write-Host "   3. Push to GitHub:"
    Write-Host "      git push origin main"
    Write-Host ""
    
    Pop-Location
}

# Main execution
Setup-GitCrypt

