# GitHub Setup Orchestrator
# Unified wizard collects all information, then orchestrates SSH, local repo, and remote setup

[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [switch]$SkipSsh,
    
    [Parameter(Mandatory = $false)]
    [switch]$SkipLocalRepo,
    
    [Parameter(Mandatory = $false)]
    [switch]$SkipRemote
)

$ErrorActionPreference = "Stop"

# Script variables
$script:ORIGINAL_WORKING_DIR = (Get-Location).Path
$scriptDir = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.PSCommandPath }


function Write-BoxedHeader { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$Title,
        
        [Parameter(Mandatory = $false)]
        [int]$Width = 80
    )
    
    $displayTitle = if ($Title.Length % 2 -eq 1) { "$Title " } else { $Title }
    
    $padding = [Math]::Max(0, (($Width - $displayTitle.Length) / 2) - 1)
    
    $leftPad = " " * [Math]::Floor($padding)
    $rightPad = " " * [Math]::Floor($padding)
    $topBottom = "━" * ($Width - 2)
    
    Write-Host "┏$topBottom┓"
    Write-Host "┃$leftPad$displayTitle$rightPad┃"
    Write-Host "┗$topBottom┛"
    Write-Host ""

} #<

function Write-Header { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$Title,
        
        [Parameter(Mandatory = $false)]
        [int]$Width = 65
    )
    
    $taiLines = [Math]::Max(0, $Width - ($Title.Length + 4))
    $tail = "─" * ($taiLines)
    
    
    Write-Host "┌─ $Title $tail"
} #<



function Get-LocalRepoPath { #>
    $gitDir = git rev-parse --git-dir 2>$null
    if ($gitDir) {
        $repoPath = (Resolve-Path (Split-Path $gitDir -Parent)).Path
        return $repoPath
    }
    return $null
} #<



function Test-SshConnection { #>
    # Test SSH connection non-interactively to avoid host key prompts
    # First check if github.com is in known_hosts to avoid prompts
    $knownHostsPath = "$env:USERPROFILE\.ssh\known_hosts"
    $hasKnownHost = $false
    if (Test-Path $knownHostsPath) {
        $knownHostsContent = Get-Content $knownHostsPath -Raw -ErrorAction SilentlyContinue
        if ($knownHostsContent -and $knownHostsContent -match "github\.com") {
            $hasKnownHost = $true
        }
    }
    
    # If host key is not known, assume SSH is not configured (to avoid prompts)
    if (-not $hasKnownHost) {
        return $false
    }
    
    # Test connection with non-interactive options, completely suppress output
    $oldErrorAction = $ErrorActionPreference
    $ErrorActionPreference = "SilentlyContinue"
    
    $tempOut = [System.IO.Path]::GetTempFileName()
    $tempErr = [System.IO.Path]::GetTempFileName()
    
    $process = Start-Process -FilePath "ssh" `
        -ArgumentList "-o", "BatchMode=yes", "-o", "ConnectTimeout=5", "-T", "git@github.com" `
        -RedirectStandardOutput $tempOut `
        -RedirectStandardError $tempErr `
        -NoNewWindow `
        -Wait `
        -PassThru
    
    $exitCode = $process.ExitCode
    
    Remove-Item -Path $tempOut -Force -ErrorAction SilentlyContinue
    Remove-Item -Path $tempErr -Force -ErrorAction SilentlyContinue
    
    $ErrorActionPreference = $oldErrorAction
    
    # Exit code 1 means authentication succeeded (GitHub returns "Hi username! You've successfully authenticated...")
    # Exit code 255 means connection failed or not authenticated
    return $exitCode -eq 1
} #<

function Test-LocalRepo { #>
    $null = git rev-parse --git-dir 2>$null
    return $LASTEXITCODE -eq 0
} #<

function Test-GitCrypt { #>
    # Check if git-crypt is installed
    if (-not (Get-Command git-crypt -ErrorAction SilentlyContinue)) {
        return @{
            Installed = $false
            Configured = $false
            Locked = $false
        }
    }
    
    # Check if git-crypt is initialized in the repo
    $gitCryptPath = Join-Path (Get-LocalRepoPath) ".git-crypt"
    $isConfigured = Test-Path $gitCryptPath
    
    # Check if repo is locked (no key available)
    $isLocked = $false
    if ($isConfigured) {
        $tempOut = [System.IO.Path]::GetTempFileName()
        $tempErr = [System.IO.Path]::GetTempFileName()
        $process = Start-Process -FilePath "git-crypt" `
            -ArgumentList "status" `
            -RedirectStandardOutput $tempOut `
            -RedirectStandardError $tempErr `
            -NoNewWindow `
            -Wait `
            -PassThru
        $output = Get-Content -Path $tempOut -Raw -ErrorAction SilentlyContinue
        Remove-Item -Path $tempOut -Force -ErrorAction SilentlyContinue
        Remove-Item -Path $tempErr -Force -ErrorAction SilentlyContinue
        
        # If status shows "not unlocked" or similar, it's locked
        if ($output -match "not unlocked|locked|no key") {
            $isLocked = $true
        }
    }
    
    return @{
        Installed = $true
        Configured = $isConfigured
        Locked = $isLocked
    }
} #<

function Test-GitHubCli { #>
    $installed = $false
    $authenticated = $false
    
    if (Get-Command gh -ErrorAction SilentlyContinue) {
        $installed = $true
        
        # Check authentication
        $tempOut = [System.IO.Path]::GetTempFileName()
        $tempErr = [System.IO.Path]::GetTempFileName()
        $process = Start-Process -FilePath "gh" `
            -ArgumentList "auth", "status" `
            -RedirectStandardOutput $tempOut `
            -RedirectStandardError $tempErr `
            -NoNewWindow `
            -Wait `
            -PassThru
        $authStatus = $process.ExitCode
        Remove-Item -Path $tempOut -Force -ErrorAction SilentlyContinue
        Remove-Item -Path $tempErr -Force -ErrorAction SilentlyContinue
        
        $authenticated = $authStatus -eq 0
    }
    
    return @{
        Installed = $installed
        Authenticated = $authenticated
    }
} #<

function Test-RepoExists { #>
    param(
        [string]$GitHubUser,
        [string]$RepoName
    )
    
    if (-not $GitHubUser -or -not $RepoName) {
        return $false
    }
    
    Push-Location $script:ORIGINAL_WORKING_DIR
    
    # Use GIT_SSH_COMMAND to pass SSH options that suppress host key verification prompts
    # Also suppress permission denied errors (SSH not configured yet)
    $env:GIT_SSH_COMMAND = "ssh -o BatchMode=yes -o StrictHostKeyChecking=no -o ConnectTimeout=5 -o LogLevel=ERROR"
    
    # Redirect both stdout and stderr to suppress all output
    $tempOut = [System.IO.Path]::GetTempFileName()
    $tempErr = [System.IO.Path]::GetTempFileName()
    
    try {
        $process = Start-Process -FilePath "git" `
            -ArgumentList "ls-remote", "git@github.com:${GitHubUser}/${RepoName}.git" `
            -RedirectStandardOutput $tempOut `
            -RedirectStandardError $tempErr `
            -NoNewWindow `
            -Wait `
            -PassThru
        
        $exitCode = $process.ExitCode
        $errorOutput = Get-Content -Path $tempErr -Raw -ErrorAction SilentlyContinue
    } finally {
        Remove-Item -Path $tempOut -Force -ErrorAction SilentlyContinue
        Remove-Item -Path $tempErr -Force -ErrorAction SilentlyContinue
        Remove-Item Env:\GIT_SSH_COMMAND -ErrorAction SilentlyContinue
    }
    
    Pop-Location
    
    # If we get permission denied, SSH isn't configured yet - this is expected
    if ($errorOutput -match "Permission denied|publickey") {
        return $false
    }
    
    # Exit code 0 means repo exists
    if ($exitCode -eq 0) {
        return $true
    }
    
    # Exit code 128 with "not found" means repo doesn't exist (this is fine)
    if ($exitCode -eq 128 -and ($errorOutput -match "not found" -or $errorOutput -match "Repository not found")) {
        return $false
    }
    
    # Other errors - assume repo doesn't exist
    return $false
} #<



function Show-VerificationReport { #>
    param(
        [hashtable]$Checks
    )
    
    Write-Header -Title "Verification Report"
    
    # SSH Auth Check
    if ($Checks.ContainsKey("SshAuth")) {
        $sshAuth = $Checks["SshAuth"]
        if ($sshAuth) {
            Write-Host "  ✓ SSH Authentication: Working" -ForegroundColor DarkGreen
        } else {
            Write-Host "  ✗ SSH Authentication: Failed" -ForegroundColor Red
        }
    }
    
    # Local Repo Check
    if ($Checks.ContainsKey("LocalRepo")) {
        $localRepo = $Checks["LocalRepo"]
        if ($localRepo.Exists) {
            Write-Host "  ✓ Local Repository: Exists" -ForegroundColor DarkGreen
            Write-Host "    Path: $($localRepo.Path)" -ForegroundColor DarkGray
        } else {
            Write-Host "  ✗ Local Repository: Not found" -ForegroundColor Red
        }
    }
    
    # Remote Repo Check
    if ($Checks.ContainsKey("RemoteRepo")) {
        $remoteRepo = $Checks["RemoteRepo"]
        if ($remoteRepo.Exists) {
            Write-Host "  ✓ Remote Repository: Exists" -ForegroundColor DarkGreen
            Write-Host "    URL: $($remoteRepo.Url)" -ForegroundColor DarkGray
        } else {
            Write-Host "  ⚠ Remote Repository: Not found (may need to be created)" -ForegroundColor Yellow
        }
    }
    
    # GitHub CLI Check
    if ($Checks.ContainsKey("GitHubCli")) {
        $ghCli = $Checks["GitHubCli"]
        if ($ghCli.Installed) {
            if ($ghCli.Authenticated) {
                Write-Host "  ✓ GitHub CLI: Installed and authenticated" -ForegroundColor DarkGreen
            } else {
                Write-Host "  ⚠ GitHub CLI: Installed but not authenticated" -ForegroundColor Yellow
            }
        } else {
            Write-Host "  ⚠ GitHub CLI: Not installed" -ForegroundColor Yellow
        }
    }
    
    # Git-Crypt Check
    if ($Checks.ContainsKey("GitCrypt")) {
        $gitCrypt = $Checks["GitCrypt"]
        if ($gitCrypt.Installed) {
            if ($gitCrypt.Configured) {
                if ($gitCrypt.Locked) {
                    Write-Host "  ⚠ Git-Crypt: Configured but locked (key needed)" -ForegroundColor Yellow
                } else {
                    Write-Host "  ✓ Git-Crypt: Configured and unlocked" -ForegroundColor DarkGreen
                }
            } else {
                Write-Host "  ℹ Git-Crypt: Installed but not configured" -ForegroundColor Cyan
            }
        } else {
            Write-Host "  ℹ Git-Crypt: Not installed" -ForegroundColor Cyan
        }
    }
    
    Write-Host ""
} #<

function Build-UnifiedWizardSteps { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [bool]$SshConfigured,
        
        [Parameter(Mandatory = $true)]
        [bool]$LocalRepoExists,
        
        [Parameter(Mandatory = $true)]
        [bool]$HasExistingRemotes,
        
        [Parameter(Mandatory = $false)]
        [array]$ExistingSshKeys = @(),
        
        [Parameter(Mandatory = $false)]
        [bool]$HasExistingSshConfig = $false,
        
        [Parameter(Mandatory = $false)]
        [string]$DefaultGitHubUser = "",
        
        [Parameter(Mandatory = $false)]
        [string]$DefaultRepoName = "",
        
        [Parameter(Mandatory = $false)]
        [string]$DefaultGitName = "",
        
        [Parameter(Mandatory = $false)]
        [string]$DefaultGitEmail = "",
        
        [Parameter(Mandatory = $false)]
        [bool]$SkipSsh = $false,
        
        [Parameter(Mandatory = $false)]
        [bool]$SkipLocalRepo = $false,
        
        [Parameter(Mandatory = $false)]
        [bool]$SkipRemote = $false
    )
    
    $steps = @()
    
    # SSH Setup Section (if not configured)
    if (-not $SshConfigured -and -not $SkipSsh) {
        if ($ExistingSshKeys.Count -gt 0) {
            $menuOptions = @()
            foreach ($key in $ExistingSshKeys) {
                $fingerprint = if ($key.Fingerprint) { "($($key.Fingerprint))" } else { "" }
                $comment = if ($key.Comment) { " - $($key.Comment)" } else { "" }
                $menuOptions += "$($key.Name)$comment $fingerprint"
            }
            $menuOptions += "Create new key"
            
            $defaultIndex = if ($menuOptions.Count -gt 1) { "1" } else { "0" }
            
            $steps += New-WizardStep `
                -Type "select" `
                -Title "Select SSH Key" `
                -Key "ssh_key_selection" `
                -Description "Choose an existing key or create a new one" `
                -Options $menuOptions `
                -Default $defaultIndex
        } else {
            $steps += New-WizardStep `
                -Type "input" `
                -Title "SSH key name" `
                -Key "ssh_new_key_name" `
                -Placeholder "github_pi" `
                -Description "Note: 'github_' will be automatically prefixed if not present"
        }
        
        $steps += New-WizardStep `
            -Type "confirm" `
            -Title "Add SSH key to GitHub?" `
            -Key "ssh_add_to_github" `
            -Description "Automatically add the selected key to your GitHub account" `
            -Default "yes"
        
        if ($HasExistingSshConfig) {
            $steps += New-WizardStep `
                -Type "confirm" `
                -Title "Update SSH config?" `
                -Key "ssh_update_config" `
                -Description "SSH config already contains GitHub configuration. Update it to use the selected key?" `
                -Default "yes"
        } else {
            $steps += New-WizardStep `
                -Type "confirm" `
                -Title "Configure SSH config?" `
                -Key "ssh_configure_config" `
                -Description "Configure SSH config file to use the selected key for GitHub" `
                -Default "yes"
        }
    }
    
    # Local Repository Setup Section
    if (-not $SkipLocalRepo) {
        if ($LocalRepoExists) {
            $steps += New-WizardStep `
                -Type "confirm" `
                -Title "Remove existing repository and recreate?" `
                -Key "local_recreate_repo" `
                -Description "Repository exists at: $script:ORIGINAL_WORKING_DIR" `
                -Default "yes"
        }
        
        if (-not $DefaultGitName) {
            $steps += New-WizardStep `
                -Type "input" `
                -Title "Git user name" `
                -Key "local_git_name" `
                -Placeholder "Your Name" `
                -Description "Name for Git commits"
        }
        
        if (-not $DefaultGitEmail) {
            $steps += New-WizardStep `
                -Type "input" `
                -Title "Git user email" `
                -Key "local_git_email" `
                -Placeholder "your.email@example.com" `
                -Description "Email for Git commits"
        }
    }
    
    # Remote Repository Setup Section
    if (-not $SkipRemote) {
       
         if (-not $DefaultGitHubUser) {
            $steps += New-WizardStep `
                -Type "input" `
                -Title "GitHub username" `
                -Key "remote_github_user" `
                -Placeholder "username" `
                -Description "Enter your GitHub username"
        } else {
            $steps += New-WizardStep `
                -Type "input" `
                -Title "GitHub username" `
                -Key "remote_github_user" `
                -Placeholder $DefaultGitHubUser `
                -Default $DefaultGitHubUser `
                -Description "Detected from Git config"
        }
        
        
        
        
        $steps += New-WizardStep `
            -Type "input" `
            -Title "Repository name" `
            -Key "remote_repo_name" `
            -Placeholder $DefaultRepoName `
            -Default $DefaultRepoName `
            -Description "Default: current directory name"
        
        
        $steps += New-WizardStep `
            -Type "confirm" `
            -Title "Make repository private?" `
            -Key "remote_repo_private" `
            -Description "Private repositories are only visible to you and collaborators" `
            -Default "no"
        
        $steps += New-WizardStep `
            -Type "confirm" `
            -Title "Create repository on GitHub?" `
            -Key "remote_create_repo" `
            -Description "Automatically create the repository on GitHub if it doesn't exist" `
            -Default "yes"
        
        if ($HasExistingRemotes) {
            $steps += New-WizardStep `
                -Type "confirm" `
                -Title "Remove existing remotes?" `
                -Key "remote_remove_remotes" `
                -Description "Remove existing remotes and set up new one" `
                -Default "yes"
        }
    }
    
    return $steps
} #<



function Invoke-iWizard { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [array]$Steps,
        
        [Parameter(Mandatory = $false)]
        [string]$ResultFile
    )
    
    # Resolve wizard binary path
    $wizardPaths = @()
    if ($scriptDir) {
        $wizardPaths += Join-Path $scriptDir "prompt-wizard.exe"
        $wizardPaths += Join-Path $scriptDir "prompt-wizard"
        $parentDir = Split-Path -Parent $scriptDir
        if ($parentDir) {
            $wizardPaths += Join-Path $parentDir "prompt-wizard.exe"
            $wizardPaths += Join-Path $parentDir "prompt-wizard"
        }
    }
    
    $wizardBin = $null
    foreach ($path in $wizardPaths) {
        if (Test-Path $path) {
            $wizardBin = (Resolve-Path $path).Path
            break
        }
    }
    
    if (-not $wizardBin) {
        Write-Error "❌ Wizard executable not found. Checked:`n$($wizardPaths -join "`n")" -ErrorAction Stop
        return $null
    }
    
    # Convert steps to JSON array
    $stepsArray = @($Steps)
    $jsonContent = $stepsArray | ConvertTo-Json -Depth 10
    $jsonContent = $jsonContent.Trim()
    
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

function Invoke-SetupScript { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$ScriptName,
        
        [Parameter(Mandatory = $false)]
        [hashtable]$Parameters = @{}
    )
    
    $scriptPath = Join-Path $scriptDir $ScriptName
    if (-not (Test-Path $scriptPath)) {
        Write-Error "Script not found: $scriptPath" -ErrorAction Stop
        return $false
    }
    
    try {
        # Suppress errors from sub-scripts (they handle their own error display)
        $oldErrorAction = $ErrorActionPreference
        $ErrorActionPreference = "SilentlyContinue"
        
        # Build parameter splatting and capture both stdout and stderr
        if ($Parameters.Count -gt 0) {
            $output = & $scriptPath @Parameters 2>&1 | Out-String
        } else {
            $output = & $scriptPath 2>&1 | Out-String
        }
        
        $ErrorActionPreference = $oldErrorAction
        $success = $LASTEXITCODE -eq 0
        return $success
    } catch {
        $ErrorActionPreference = $oldErrorAction
        Write-Host "  Error running script: $_" -ForegroundColor Red
        return $false
    }
} #<

function Setup-GitHub { #>
    Write-BoxedHeader -Title "GitHub Setup"
    
    # Discover current state
    $sshConfigured = Test-SshConnection
    $localRepoExists = Test-LocalRepo
    $remotes = git remote 2>$null
    $hasExistingRemotes = $remotes -and $remotes.Count -gt 0
    
    # Discover SSH keys
    $existingSshKeys = @()
    $sshDir = "$env:USERPROFILE\.ssh"
    $hasExistingSshConfig = $false
    
    if (-not $sshConfigured -and -not $SkipSsh) {
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
                    
                    $existingSshKeys += $keyInfo
                }
            }
            
            $existingSshKeys = $existingSshKeys | Sort-Object {
                if ($_.Name.StartsWith("github_")) { "0$($_.Name)" } else { "1$($_.Name)" }
            }
        }
        
        $sshConfigPath = "$env:USERPROFILE\.ssh\config"
        if (Test-Path $sshConfigPath) {
            $configContent = Get-Content $sshConfigPath -Raw -ErrorAction SilentlyContinue
            $hasExistingSshConfig = $configContent -match "Host github\.com"
        }
    }
    
    # Get defaults
    $defaultGitHubUser = git config --global user.name 2>$null
    if ($defaultGitHubUser) { $defaultGitHubUser = $defaultGitHubUser.Trim() }
    
    $defaultRepoName = Split-Path -Path $script:ORIGINAL_WORKING_DIR -Leaf
    
    $defaultGitName = git config --global user.name 2>$null
    if ($defaultGitName) { $defaultGitName = $defaultGitName.Trim() }
    
    $defaultGitEmail = git config --global user.email 2>$null
    if ($defaultGitEmail) { $defaultGitEmail = $defaultGitEmail.Trim() }
    
    # Build unified wizard steps
    $wizardSteps = Build-UnifiedWizardSteps `
        -SshConfigured $sshConfigured `
        -LocalRepoExists $localRepoExists `
        -HasExistingRemotes $hasExistingRemotes `
        -ExistingSshKeys $existingSshKeys `
        -HasExistingSshConfig $hasExistingSshConfig `
        -DefaultGitHubUser $defaultGitHubUser `
        -DefaultRepoName $defaultRepoName `
        -DefaultGitName $defaultGitName `
        -DefaultGitEmail $defaultGitEmail `
        -SkipSsh $SkipSsh.IsPresent `
        -SkipLocalRepo $SkipLocalRepo.IsPresent `
        -SkipRemote $SkipRemote.IsPresent
    
    # Run unified wizard
    if ($wizardSteps.Count -gt 0) {
        if (-not [Environment]::UserInteractive) {
            Write-Warning "Console is not interactive. Wizard may not work properly."
        }
        
        $wizardJsonResult = Invoke-iWizard -Steps $wizardSteps
        if (-not $wizardJsonResult) {
            Write-Host "Wizard was cancelled. Exiting." -ForegroundColor Yellow
            return
        }
        
        $jsonResultString = if ($wizardJsonResult -is [array]) {
            ($wizardJsonResult | Out-String).Trim()
        } else {
            [string]$wizardJsonResult
        }
        
        $wizardResults = Get-WizardResults -JsonResult $jsonResultString
        if (-not $wizardResults) {
            Write-Host "Wizard was cancelled. Exiting." -ForegroundColor Yellow
            return
        }
    } else {
        $wizardResults = @{}
    }
    
    # Step 1: SSH Setup
    if (-not $sshConfigured -and -not $SkipSsh) {
        Write-Header -Title "Setting up Github SSH"
        
        # Extract SSH parameters from wizard results
        $sshParams = @{}
        
        if ($wizardResults.ContainsKey("ssh_key_selection")) {
            $selected = $wizardResults["ssh_key_selection"]
            if ($selected -eq "Create new key") {
                # Need to get key name
                if ($wizardResults.ContainsKey("ssh_new_key_name")) {
                    $keyName = $wizardResults["ssh_new_key_name"].ToString().Trim()
                    if (-not $keyName.StartsWith("github_")) {
                        $keyName = "github_$keyName"
                    }
                    $sshParams["KeyName"] = $keyName
                    $sshParams["CreateNew"] = $true
                }
            } else {
                # Find selected key
                foreach ($key in $existingSshKeys) {
                    $fingerprint = if ($key.Fingerprint) { "($($key.Fingerprint))" } else { "" }
                    $comment = if ($key.Comment) { " - $($key.Comment)" } else { "" }
                    $keyDisplay = "$($key.Name)$comment $fingerprint"
                    if ($keyDisplay -eq $selected) {
                        $sshParams["KeyName"] = $key.Name
                        $sshParams["CreateNew"] = $false
                        break
                    }
                }
            }
        } elseif ($wizardResults.ContainsKey("ssh_new_key_name")) {
            $keyName = $wizardResults["ssh_new_key_name"].ToString().Trim()
            if (-not $keyName.StartsWith("github_")) {
                $keyName = "github_$keyName"
            }
            $sshParams["KeyName"] = $keyName
            $sshParams["CreateNew"] = $true
        }
        
        $sshParams["AddToGitHub"] = $false
        if ($wizardResults.ContainsKey("ssh_add_to_github")) {
            $addValue = $wizardResults["ssh_add_to_github"]
            if ($addValue -is [bool]) {
                $sshParams["AddToGitHub"] = $addValue
            } elseif ($addValue -is [string]) {
                $addValueLower = $addValue.ToLower()
                $sshParams["AddToGitHub"] = $addValueLower -eq "true" -or $addValueLower -eq "yes" -or $addValueLower -eq "y"
            }
        }
        
        $sshParams["ConfigureConfig"] = $false
        if ($wizardResults.ContainsKey("ssh_configure_config")) {
            $configValue = $wizardResults["ssh_configure_config"]
            if ($configValue -is [bool]) {
                $sshParams["ConfigureConfig"] = $configValue
            } elseif ($configValue -is [string]) {
                $configValueLower = $configValue.ToLower()
                $sshParams["ConfigureConfig"] = $configValueLower -eq "true" -or $configValueLower -eq "yes" -or $configValueLower -eq "y"
            }
        }
        
        if ($wizardResults.ContainsKey("ssh_update_config")) {
            $updateValue = $wizardResults["ssh_update_config"]
            if ($updateValue -is [bool]) {
                $sshParams["ConfigureConfig"] = $updateValue
            } elseif ($updateValue -is [string]) {
                $updateValueLower = $updateValue.ToLower()
                $sshParams["ConfigureConfig"] = $updateValueLower -eq "true" -or $updateValueLower -eq "yes" -or $updateValueLower -eq "y"
            }
        }
        
        
        $sshSuccess = Invoke-SetupScript -ScriptName "ps-setup-ssh.ps1" -Parameters $sshParams
        if ($sshSuccess) {
            $sshConfigured = Test-SshConnection
            if ($sshConfigured) {
                Write-Host "  ✓ SSH connection verified" -ForegroundColor DarkGreen
            }
        }
        Write-Host ""
    }
    
    # Step 2: Local Repository Setup
    if (-not $SkipLocalRepo) {
        Write-Header -Title "Setting up local repository"
        
        $localParams = @{
            WorkingDirectory = $script:ORIGINAL_WORKING_DIR
        }
        
        if ($wizardResults.ContainsKey("local_recreate_repo")) {
            $recreateValue = $wizardResults["local_recreate_repo"]
            if ($recreateValue -is [bool]) {
                $localParams["Recreate"] = $recreateValue
            } elseif ($recreateValue -is [string]) {
                $recreateValueLower = $recreateValue.ToLower()
                $localParams["Recreate"] = $recreateValueLower -eq "true" -or $recreateValueLower -eq "yes" -or $recreateValueLower -eq "y"
            }
        }
        
        if ($wizardResults.ContainsKey("local_git_name")) {
            $localParams["GitName"] = $wizardResults["local_git_name"].ToString().Trim()
        } elseif ($defaultGitName) {
            $localParams["GitName"] = $defaultGitName
        }
        
        if ($wizardResults.ContainsKey("local_git_email")) {
            $localParams["GitEmail"] = $wizardResults["local_git_email"].ToString().Trim()
        } elseif ($defaultGitEmail) {
            $localParams["GitEmail"] = $defaultGitEmail
        }
        
        $localSuccess = Invoke-SetupScript -ScriptName "ps-setup-local-repo.ps1" -Parameters $localParams
        Write-Host ""
    }
    
    # Step 3: Remote Repository Setup
    if (-not $SkipRemote) {
        Write-Header -Title "Setting up remote repository"
        
        $remoteParams = @{}
        
        if ($wizardResults.ContainsKey("remote_github_user")) {
            $remoteParams["GitHubUser"] = $wizardResults["remote_github_user"].ToString().Trim()
        } elseif ($defaultGitHubUser) {
            $remoteParams["GitHubUser"] = $defaultGitHubUser
        }
        
        if ($wizardResults.ContainsKey("remote_repo_name")) {
            $remoteParams["RepoName"] = $wizardResults["remote_repo_name"].ToString().Trim()
        } elseif ($defaultRepoName) {
            $remoteParams["RepoName"] = $defaultRepoName
        }
        
        if ($wizardResults.ContainsKey("remote_repo_private")) {
            $privateValue = $wizardResults["remote_repo_private"]
            if ($privateValue -is [bool]) {
                $remoteParams["IsPrivate"] = $privateValue
            } elseif ($privateValue -is [string]) {
                $privateValueLower = $privateValue.ToLower()
                $remoteParams["IsPrivate"] = $privateValueLower -eq "true" -or $privateValueLower -eq "yes" -or $privateValueLower -eq "y"
            }
        }
        
        if ($wizardResults.ContainsKey("remote_remove_remotes")) {
            $removeValue = $wizardResults["remote_remove_remotes"]
            if ($removeValue -is [bool]) {
                $remoteParams["RemoveRemotes"] = $removeValue
            } elseif ($removeValue -is [string]) {
                $removeValueLower = $removeValue.ToLower()
                $remoteParams["RemoveRemotes"] = $removeValueLower -eq "true" -or $removeValueLower -eq "yes" -or $removeValueLower -eq "y"
            }
        }
        
        $shouldCreateRepo = $false
        if ($wizardResults.ContainsKey("remote_create_repo")) {
            $createValue = $wizardResults["remote_create_repo"]
            
            # Convert to string first to handle all cases
            $createValueStr = $createValue.ToString().Trim()
            $createValueLower = $createValueStr.ToLower()
            
            # Check for true/yes/y/1
            $shouldCreateRepo = $createValueLower -eq "true" -or $createValueLower -eq "yes" -or $createValueLower -eq "y" -or $createValueLower -eq "1"
            
            # Also handle boolean directly
            if ($createValue -is [bool]) {
                $shouldCreateRepo = $createValue
            }
        }
        
        # If user wants to create repo, check if it exists and ask to remove if needed
        if ($shouldCreateRepo -and $remoteParams["GitHubUser"] -and $remoteParams["RepoName"]) {
            $repoExists = Test-RepoExists -GitHubUser $remoteParams["GitHubUser"] -RepoName $remoteParams["RepoName"]
            if ($repoExists) {
                # Show a confirm step asking to remove existing repo
                $removeSteps = @(
                    New-WizardStep `
                        -Type "confirm" `
                        -Title "Repository already exists. Remove it?" `
                        -Key "remote_remove_existing" `
                        -Description "Repository '$($remoteParams["RepoName"])' already exists on GitHub. Remove it and recreate?" `
                        -Default "no"
                )
                
                $removeWizardResult = Invoke-iWizard -Steps $removeSteps
                if ($removeWizardResult) {
                    $removeJsonString = if ($removeWizardResult -is [array]) {
                        ($removeWizardResult | Out-String).Trim()
                    } else {
                        [string]$removeWizardResult
                    }
                    
                    $removeResults = Get-WizardResults -JsonResult $removeJsonString
                    if ($removeResults -and $removeResults.ContainsKey("remote_remove_existing")) {
                        $removeValue = $removeResults["remote_remove_existing"]
                        if ($removeValue -is [bool]) {
                            $remoteParams["RemoveExisting"] = $removeValue
                        } elseif ($removeValue -is [string]) {
                            $removeValueLower = $removeValue.ToLower()
                            $remoteParams["RemoveExisting"] = $removeValueLower -eq "true" -or $removeValueLower -eq "yes" -or $removeValueLower -eq "y"
                        }
                    } else {
                        # User cancelled - don't create repo
                        $shouldCreateRepo = $false
                    }
                } else {
                    # User cancelled - don't create repo
                    $shouldCreateRepo = $false
                }
            }
        }
        
        $remoteParams["CreateRepo"] = $shouldCreateRepo
        
        $remoteSuccess = Invoke-SetupScript -ScriptName "ps-setup-remote.ps1" -Parameters $remoteParams
        Write-Host ""
    }
    
    # Verification Report - Run comprehensive checks after setup
    $verificationChecks = @{}
    
    # SSH Auth Check
    $sshAuthWorking = Test-SshConnection
    $verificationChecks["SshAuth"] = $sshAuthWorking
    
    # Local Repo Check
    $localRepoPath = Get-LocalRepoPath
    $verificationChecks["LocalRepo"] = @{
        Exists = $null -ne $localRepoPath
        Path = if ($localRepoPath) { $localRepoPath } else { "Not found" }
    }
    
    # Remote Repo Check
    if ($wizardResults.ContainsKey("remote_github_user") -or $defaultGitHubUser) {
        $remoteUser = if ($wizardResults.ContainsKey("remote_github_user")) {
            $wizardResults["remote_github_user"].ToString().Trim()
        } else {
            $defaultGitHubUser
        }
        
        $remoteRepo = if ($wizardResults.ContainsKey("remote_repo_name")) {
            $wizardResults["remote_repo_name"].ToString().Trim()
        } else {
            $defaultRepoName
        }
        
        if ($remoteUser -and $remoteRepo) {
            $remoteExists = Test-RepoExists -GitHubUser $remoteUser -RepoName $remoteRepo
            $verificationChecks["RemoteRepo"] = @{
                Exists = $remoteExists
                Url = "git@github.com:${remoteUser}/${remoteRepo}.git"
            }
        }
    }
    
    # GitHub CLI Check
    $ghCliStatus = Test-GitHubCli
    $verificationChecks["GitHubCli"] = $ghCliStatus
    
    # Git-Crypt Check
    $gitCryptStatus = Test-GitCrypt
    $verificationChecks["GitCrypt"] = $gitCryptStatus
    
    # Show Verification Report
    Show-VerificationReport -Checks $verificationChecks
} #<



try {
    Setup-GitHub
} catch {
    Write-Host "Error: $_" -ForegroundColor Red
    exit 1
}
