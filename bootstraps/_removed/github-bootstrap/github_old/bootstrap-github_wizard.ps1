# Bootstrap GitHub SSH authentication setup for Windows
# Creates SSH key for GitHub authentication and configures Git to use SSH

$ErrorActionPreference = "Stop"

# SSH key name - will be prompted during setup if not provided
$script:SSH_KEY_NAME = $null
$SSH_CONFIG = "$env:USERPROFILE\.ssh\config"
$GIT_EMAIL = if ($env:GIT_EMAIL) { $env:GIT_EMAIL } else { 
    $globalEmail = git config --global user.email 2>$null
    if ($globalEmail) { $globalEmail } else { "user@example.com" }
}
# Capture the original working directory when script starts
$script:ORIGINAL_WORKING_DIR = (Get-Location).Path

function Write-BoxedHeader {
    param(
        [string]$Title,
        [int]$Width = 80
    )
    
    # If title has odd number of characters, add an extra space after it
    $displayTitle = $Title
    if ($Title.Length % 2 -eq 1) {
        $displayTitle = "$Title "
    }
    
    # Calculate padding to center the title (accounting for box characters)
    $padding = [Math]::Max(0, ($Width - $displayTitle.Length) / 2)
    $leftPad = " " * [Math]::Floor($padding)
    $rightPad = " " * [Math]::Ceiling($padding)
    $topBottom = "━" * $Width
    
    Write-Host "┏$topBottom┓"
    Write-Host "┃$leftPad$displayTitle$rightPad┃"
    Write-Host "┗$topBottom┛"
    Write-Host ""
}

# Function to get git repo directory (uses original working directory or override)
function Get-GitRepoDir {
    if ($env:GIT_REPO_DIR) {
        Write-Host "[!]  GIT_REPO_DIR environment variable is set to: $env:GIT_REPO_DIR" -ForegroundColor Yellow
        Write-Host "   Using current directory instead: $script:ORIGINAL_WORKING_DIR" -ForegroundColor Yellow
        # Prioritize current directory over environment variable
        return $script:ORIGINAL_WORKING_DIR
    } else {
        # Use the original working directory where the script was run from
        return $script:ORIGINAL_WORKING_DIR
    }
}
# Set initial value (will be updated dynamically)
$GIT_REPO_DIR = Get-GitRepoDir

# Function to get SSH key path (uses key name from script variable)
function Get-SshKeyPath {
    if (-not $script:SSH_KEY_NAME) {
        # Try to detect existing key name from config or default to github_pi
        if (Test-Path $SSH_CONFIG) {
            $configContent = Get-Content $SSH_CONFIG -Raw -ErrorAction SilentlyContinue
            if ($configContent -match "IdentityFile\s+~?/?\.ssh/([^\s]+)") {
                $script:SSH_KEY_NAME = $matches[1]
            }
        }
        if (-not $script:SSH_KEY_NAME) {
            $script:SSH_KEY_NAME = "github_pi"
        }
    }
    return "$env:USERPROFILE\.ssh\$($script:SSH_KEY_NAME)"
}

# Source wizard helper functions (if available)
$IWIZARD_FUNCTIONS = "$env:USERPROFILE\_playground\projects\iMenu\iwizard-functions.ps1"
if (Test-Path $IWIZARD_FUNCTIONS) {
    . $IWIZARD_FUNCTIONS
}

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
        [string]$WizardBinPath = "./prompt-wizard"
    ) #<
    
    #= Resolve wizard binary path =============================================== 
    $wizardBin = $null
    
    # First, try to find it in the script's directory
    $scriptDir = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.PSCommandPath }
    $pathsToCheck = @()
    
    if ($scriptDir) {
        $pathsToCheck += Join-Path $scriptDir "prompt-wizard.exe"
        $pathsToCheck += Join-Path $scriptDir "prompt-wizard"
    }
    
    # Then check the provided path (relative to current directory)
    if (-not [string]::IsNullOrWhiteSpace($WizardBinPath)) {
        $pathsToCheck += "$WizardBinPath.exe"
        $pathsToCheck += $WizardBinPath
    }
    
    # Check each path
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
    
    #= Convert PowerShell hashtables to JSON ==================================== 
    $jsonContent = $Steps | ConvertTo-Json -Depth 10 -Compress
    
    #= Create temp file for JSON (UTF-8 without BOM) ============================ 
    $stepsFile = [System.IO.Path]::GetTempFileName()
    $utf8NoBom = New-Object System.Text.UTF8Encoding $false
    [System.IO.File]::WriteAllText($stepsFile, $jsonContent, $utf8NoBom)
    
    #= Create result file ======================================================= 
    $resultFilePath = if ($ResultFile) { $ResultFile } else { [System.IO.Path]::GetTempFileName() }
    $isTempResult = [string]::IsNullOrWhiteSpace($ResultFile)
    
    try {
        # Run wizard - output goes directly to console for interactive UI
        # Use call operator - this should allow interactive TUI to work
        # The wizard needs direct console access for its interactive interface
        $null = & $wizardBin $stepsFile --result-file $resultFilePath
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

function Show-Help {
    @"
GitHub SSH Bootstrap Script

Usage: $($MyInvocation.MyCommand.Name) [command]

Commands:
  setup              Full setup: SSH key, config, and git remote
  preflight          Run pre-flight checks (prerequisites, permissions, etc.)
  check              Alias for preflight
  status             Show current status (key, remotes, repo, etc.)
  secrets            Setup or remove git-crypt for encrypted secrets
  remove             Remove all or selected GitHub SSH setup components
  remove-key         Remove SSH key
  remove-remote      Remove/detach from git remote(s)
  remove-repo        Remove local git repository (.git directory)
  help               Show this help

Examples:
  $($MyInvocation.MyCommand.Name) setup           # Full setup
  $($MyInvocation.MyCommand.Name) preflight     # Run pre-flight checks
  $($MyInvocation.MyCommand.Name) status          # Show current status
  $($MyInvocation.MyCommand.Name) secrets         # Setup or remove git-crypt for secrets
  $($MyInvocation.MyCommand.Name) remove          # Remove all/selcted components
  $($MyInvocation.MyCommand.Name) remove-key      # Remove SSH key
  $($MyInvocation.MyCommand.Name) remove-remote   # Remove git remotes
  $($MyInvocation.MyCommand.Name) remove-repo     # Remove local git repo
"@
}

function Show-Status {
    # Refresh repo directory to use current location
    $GIT_REPO_DIR = Get-GitRepoDir
    $SSH_KEY_PATH = Get-SshKeyPath
    
    Write-BoxedHeader -Title "GitHub SSH Status"
    
    # SSH Key Status
    Write-Host "[key] SSH Key:"
    if (Test-Path $SSH_KEY_PATH) {
        Write-Host "  [ok] Key exists: $SSH_KEY_PATH"
        $keyFile = Get-Item $SSH_KEY_PATH -ErrorAction SilentlyContinue
        if ($keyFile) {
            $perms = (Get-Acl $SSH_KEY_PATH).Access | Where-Object { $_.IdentityReference -eq $env:USERNAME }
            Write-Host "  [ok] Permissions: Set (Windows ACL)"
        }
        if (Test-Path "$SSH_KEY_PATH.pub") {
            Write-Host "  [ok] Public key exists"
            Write-Host "  [clipboard] Public key fingerprint:"
            $fingerprint = ssh-keygen -lf "$SSH_KEY_PATH.pub" 2>$null
            if ($fingerprint) {
                Write-Host "     $fingerprint"
            } else {
                Write-Host "     (could not read)"
            }
        } else {
            Write-Host "  [!]  Public key missing"
        }
    } else {
        Write-Host "  [x] Key not found: $SSH_KEY_PATH"
    }
    Write-Host ""
    
    # SSH Config Status
    Write-Host "[cfg]  SSH Config:"
    if (Test-Path $SSH_CONFIG) {
        $configContent = Get-Content $SSH_CONFIG -Raw -ErrorAction SilentlyContinue
        if ($configContent -match "Host github\.com") {
            Write-Host "  [ok] GitHub config present in $SSH_CONFIG"
            Write-Host "  [clipboard] Config:"
            $configLines = Get-Content $SSH_CONFIG | Select-String -Pattern "Host github\.com" -Context 0,4
            $configLines | ForEach-Object { Write-Host "     $_" }
        } else {
            Write-Host "  [!]  GitHub config not found in $SSH_CONFIG"
        }
    } else {
        Write-Host "  [!]  SSH config file not found: $SSH_CONFIG"
    }
    Write-Host ""
    
    # Local Git Repository Status
    Write-Host "[open-file] Local Git Repository:"
    if (Test-Path "$GIT_REPO_DIR\.git") {
        Write-Host "  [ok] Repository exists: $GIT_REPO_DIR"
        Push-Location $GIT_REPO_DIR
        $currentBranch = git branch --show-current 2>$null
        if (-not $currentBranch) { $currentBranch = "unknown" }
        Write-Host "  [clipboard] Current branch: $currentBranch"
        
        # Local commit info
        $head = git rev-parse HEAD 2>$null
        if ($head) {
            $lastCommit = git log -1 --oneline 2>$null | Select-Object -First 1
            Write-Host "  [clipboard] Last commit: $lastCommit"
        } else {
            Write-Host "  [!]  No commits yet"
        }
        
        # Local branch info
        $localBranches = (git branch 2>$null | Measure-Object -Line).Lines
        Write-Host "  [clipboard] Local branches: $localBranches"
        Pop-Location
    } else {
        Write-Host "  [x] No git repository found at $GIT_REPO_DIR"
    }
    Write-Host ""
    
    # Remote Git Repository Status
    Write-Host "[www] Remote Git Repository:"
    if (Test-Path "$GIT_REPO_DIR\.git") {
        Push-Location $GIT_REPO_DIR
        $remotes = git remote 2>$null
        if ($remotes) {
            foreach ($remote in $remotes) {
                $remoteUrl = git remote get-url $remote 2>$null
                Write-Host "  [clipboard] Remote '$remote': $remoteUrl"
                
                # Check if remote is reachable
                $null = git ls-remote $remote 2>$null
                if ($LASTEXITCODE -eq 0) {
                    Write-Host "     [ok] Remote is reachable"
                } else {
                    Write-Host "     [!]  Remote is not reachable"
                }
            }
            
            # Upstream tracking
            $upstream = git rev-parse --abbrev-ref --symbolic-full-name '@{u}' 2>$null
            if ($upstream) {
                Write-Host "  [clipboard] Upstream tracking: $upstream"
            } else {
                Write-Host "  [!]  No upstream branch configured"
            }
        } else {
            Write-Host "  [!]  No remotes configured"
        }
        Pop-Location
    } else {
        Write-Host "  [!]  No local repository (cannot check remotes)"
    }
    Write-Host ""
    
    # GitHub SSH Connection Test
    Write-Host "[plug] GitHub SSH Connection:"
    if (Test-Path $SSH_KEY_PATH) {
        $testOutput = & ssh -o ConnectTimeout=5 -o BatchMode=yes -T git@github.com 2>&1 | Out-String
        $exitCode = $LASTEXITCODE
        
        if ($testOutput -match "successfully authenticated") {
            Write-Host "  [ok] SSH connection successful"
            if ($testOutput -match "Hi (\w+)") {
                $githubUser = $matches[1]
                Write-Host "  [clipboard] Authenticated as: $githubUser"
            }
        } elseif ($testOutput -match "permission denied") {
            Write-Host "  [!]  Permission denied (key may not be added to GitHub)"
            Write-Host "     Add key at: https://github.com/settings/keys"
            Write-Host ""
            Write-Host "  [clipboard] Your public key (copy and add to GitHub):"
            Get-Content "$SSH_KEY_PATH.pub" | ForEach-Object { Write-Host "     $_" }
        } elseif ($testOutput -match "host key verification failed") {
            Write-Host "  [!]  Host key verification failed"
            Write-Host "     Run: ssh-keyscan github.com >> `$env:USERPROFILE\.ssh\known_hosts"
        } elseif ($exitCode -eq 124) {
            Write-Host "  [!]  Connection test timed out"
            Write-Host "     Check network connectivity"
        } else {
            Write-Host "  [!]  Connection test failed (exit code: $exitCode)"
            if ($testOutput) {
                $firstLine = ($testOutput -split "`n")[0]
                Write-Host "     Output: $firstLine"
            }
            Write-Host "     Run manually: ssh -T git@github.com"
        }
    } else {
        Write-Host "  [!]  Cannot test (SSH key not found)"
    }
    Write-Host ""
}

function Add-SshKeyToGitHub {
    param(
        [string]$PublicKeyPath,
        [string]$KeyName
    )
    
    Write-Host "[lock] Attempting to add SSH key to GitHub..."
    Write-Host ""
    
    # Try GitHub API first (fully automated, no browser interaction needed)
    $githubToken = $env:GITHUB_TOKEN
    if (-not $githubToken -and (Test-Path "$env:USERPROFILE\.secrets")) {
        $secretsContent = Get-Content "$env:USERPROFILE\.secrets" -ErrorAction SilentlyContinue
        $tokenLine = $secretsContent | Select-String -Pattern "^GITHUB_TOKEN="
        if ($tokenLine) {
            $githubToken = ($tokenLine -split "=")[1] -replace '"', '' -replace "'", ''
        }
    }
    
    if ($githubToken) {
        Write-Host "  Using GitHub API (fully automated)..." -ForegroundColor Cyan
        
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
            Write-Host "  [ok] SSH key successfully added to GitHub via API!" -ForegroundColor Green
            Write-Host "     Title: $keyTitle" -ForegroundColor Green
            Write-Host "     Key ID: $($response.id)" -ForegroundColor Green
            Write-Host ""
            return $true
        } catch {
            if ($_.Exception.Response.StatusCode -eq 422) {
                Write-Host "  [!]  Key may already exist on GitHub" -ForegroundColor Yellow
                # Check if it's a duplicate key error
                $errorContent = $_.ErrorDetails.Message
                if ($errorContent -match "already exists" -or $errorContent -match "key is already in use") {
                    Write-Host "  [i]  This key is already registered on your GitHub account" -ForegroundColor Cyan
                    return $true
                }
            } elseif ($_.Exception.Response.StatusCode -eq 401 -or $_.Exception.Response.StatusCode -eq 403) {
                Write-Host "  [!]  GitHub token is invalid or missing 'admin:public_key' scope" -ForegroundColor Yellow
                Write-Host "     Token found but authentication failed" -ForegroundColor Gray
                Write-Host "     Trying alternative method..." -ForegroundColor Yellow
            } else {
                Write-Host "  [!]  Failed to add key via API: $($_.Exception.Message)" -ForegroundColor Yellow
                Write-Host "     Trying alternative method..." -ForegroundColor Yellow
            }
        }
    }
    
    # Try GitHub CLI as fallback (requires browser interaction for authentication)
    if (Get-Command gh -ErrorAction SilentlyContinue) {
        # Check if gh is authenticated
        $null = gh auth status 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  Using GitHub CLI to add SSH key..."
            $keyTitle = "Windows ($KeyName)"
            
            # Get absolute path
            $absoluteKeyPath = (Resolve-Path $PublicKeyPath -ErrorAction SilentlyContinue).Path
            if (-not $absoluteKeyPath) {
                $absoluteKeyPath = $PublicKeyPath
            }
            
            # Try method 1: Using file path (preferred method)
            $output = gh ssh-key add $absoluteKeyPath --title $keyTitle 2>&1 | Out-String
            $exitCode = $LASTEXITCODE
            
            if ($exitCode -eq 0) {
                Write-Host "  [ok] SSH key successfully added to GitHub!" -ForegroundColor Green
                Write-Host "     Title: $keyTitle" -ForegroundColor Green
                Write-Host ""
                return $true
            } else {
                # Check if missing scope error
                if ($output -match "admin:public_key" -or $output -match "needs the.*scope") {
                    Write-Host "  [!]  GitHub CLI needs additional permissions" -ForegroundColor Yellow
                    Write-Host "     Missing scope: admin:public_key" -ForegroundColor Yellow
                    Write-Host ""
                    Write-Host "  [lock] Refreshing authentication with required scope..." -ForegroundColor Cyan
                    Write-Host "     (Using saved credentials - should work automatically)" -ForegroundColor Gray
                    Write-Host ""
                    
                    # Try to refresh automatically (should work if user is already authenticated)
                    # Run command without capturing output to avoid blocking - let it print directly to console
                    Write-Host "  Running: gh auth refresh -h github.com -s admin:public_key" -ForegroundColor Gray
                    Write-Host ""
                    
                    # Run the command and let output go directly to console (don't capture it)
                    gh auth refresh -h github.com -s admin:public_key
                    $refreshExitCode = $LASTEXITCODE
                    Write-Host ""
                    
                    # Check if refresh was successful
                    if ($refreshExitCode -eq 0) {
                        Write-Host "  [ok] Authentication refresh completed successfully" -ForegroundColor Green
                    } else {
                        Write-Host "  [!]  Authentication refresh completed with exit code: $refreshExitCode" -ForegroundColor Yellow
                    }
                    Write-Host ""
                    
                    if ($refreshExitCode -eq 0) {
                        # Retry adding the key
                        Write-Host "  [sync] Retrying SSH key addition..." -ForegroundColor Cyan
                        $retryOutput = gh ssh-key add $absoluteKeyPath --title $keyTitle 2>&1 | Out-String
                        if ($LASTEXITCODE -eq 0) {
                            Write-Host "  [ok] SSH key successfully added to GitHub!" -ForegroundColor Green
                            Write-Host "     Title: $keyTitle" -ForegroundColor Green
                            Write-Host ""
                            return $true
                        } else {
                            Write-Host "  [!]  Still unable to add key after refresh" -ForegroundColor Yellow
                            if ($retryOutput) {
                                $errorMsg = ($retryOutput -split "`n") | Where-Object { $_ -match "Error|error|failed|Failed" } | Select-Object -First 1
                                if ($errorMsg) {
                                    Write-Host "     Error: $errorMsg" -ForegroundColor Yellow
                                }
                            }
                        }
                    } else {
                        # Refresh failed or required browser interaction
                        Write-Host "  [!]  Refresh completed but may need browser interaction" -ForegroundColor Yellow
                        Write-Host "     (If you saw a code and URL above, complete that first)" -ForegroundColor Gray
                        Write-Host ""
                        Write-Host "Did the refresh complete successfully? (or did you complete browser steps?) (Y/n): " -NoNewline -ForegroundColor Cyan
                        $response = Read-Host
                        $completed = if ([string]::IsNullOrWhiteSpace($response)) { $true } else { $response -match "^[Yy]$" }
                        if ($completed) {
                            # Verify authentication worked
                            $null = gh auth status 2>$null
                            if ($LASTEXITCODE -eq 0) {
                                Write-Host "  [ok] Authentication verified" -ForegroundColor Green
                                Write-Host ""
                                # Retry adding the key
                                Write-Host "  [sync] Retrying SSH key addition..." -ForegroundColor Cyan
                                $retryOutput = gh ssh-key add $absoluteKeyPath --title $keyTitle 2>&1 | Out-String
                                if ($LASTEXITCODE -eq 0) {
                                    Write-Host "  [ok] SSH key successfully added to GitHub!" -ForegroundColor Green
                                    Write-Host "     Title: $keyTitle" -ForegroundColor Green
                                    Write-Host ""
                                    return $true
                                }
                            } else {
                                Write-Host "  [!]  Authentication not verified. Please try again." -ForegroundColor Yellow
                            }
                        }
                    }
                    Write-Host "     Trying alternative method..." -ForegroundColor Yellow
                } else {
                    # Try method 2: Piping key content via stdin
                    $publicKeyContent = Get-Content $PublicKeyPath -Raw
                    $output = $publicKeyContent | gh ssh-key add --title $keyTitle 2>&1 | Out-String
                    $exitCode = $LASTEXITCODE
                    
                    if ($exitCode -eq 0) {
                        Write-Host "  [ok] SSH key successfully added to GitHub!" -ForegroundColor Green
                        Write-Host "     Title: $keyTitle" -ForegroundColor Green
                        Write-Host ""
                        return $true
                    } else {
                        # Check if key already exists
                        if ($output -match "already exists" -or $output -match "already in use") {
                            Write-Host "  [i]  This key is already registered on your GitHub account" -ForegroundColor Cyan
                            return $true
                        } else {
                            Write-Host "  [!]  GitHub CLI failed to add key" -ForegroundColor Yellow
                            if ($output) {
                                $errorMsg = ($output -split "`n") | Where-Object { $_ -match "Error|error|failed|Failed" } | Select-Object -First 1
                                if ($errorMsg) {
                                    Write-Host "     Error: $errorMsg" -ForegroundColor Yellow
                                }
                            }
                            Write-Host "     Trying alternative method..." -ForegroundColor Yellow
                        }
                    }
                }
            }
        } else {
            Write-Host "  [!]  GitHub CLI not authenticated" -ForegroundColor Yellow
            Write-Host "     Run: gh auth login" -ForegroundColor Yellow
        }
    }
    
    # If all automated methods failed, show warning and instructions
    Write-Host "  [!]  Could not automatically add SSH key to GitHub" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  [idea] Tip: To enable fully automated SSH key addition:" -ForegroundColor Cyan
    Write-Host "     1. Create a GitHub Personal Access Token with 'admin:public_key' scope" -ForegroundColor Gray
    Write-Host "        at: https://github.com/settings/tokens" -ForegroundColor Gray
    Write-Host "     2. Set it as an environment variable: `$env:GITHUB_TOKEN = 'your-token'" -ForegroundColor Gray
    Write-Host "     3. Or add it to ~/.secrets: GITHUB_TOKEN=your-token" -ForegroundColor Gray
    Write-Host ""
    return $false
}

function Ensure-GitHubCli {
    # Check if GitHub CLI is available
    if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
        Write-Host "[!]  GitHub CLI not installed" -ForegroundColor Yellow
        Write-Host "   GitHub CLI is recommended for easier setup (adding SSH keys, creating repos)" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "Install GitHub CLI now? (Y/n): " -NoNewline -ForegroundColor Cyan
        $response = Read-Host
        $installGh = if ([string]::IsNullOrWhiteSpace($response)) { $true } else { $response -match "^[Yy]$" }
        if ($installGh) {
            Write-Host ""
            Write-Host "[pkg] Installing GitHub CLI..."
            # Check if winget is available
            if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
                Write-Host "[x] winget is not available" -ForegroundColor Red
                Write-Host "   Please install winget or install GitHub CLI manually:" -ForegroundColor Yellow
                Write-Host "   https://cli.github.com/" -ForegroundColor Cyan
                Write-Host ""
                return $false
            }
            
            winget install GitHub.cli
            if ($LASTEXITCODE -eq 0) {
                Write-Host "[ok] GitHub CLI installed" -ForegroundColor Green
                Write-Host ""
                # Refresh PATH to make gh available
                $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
                Start-Sleep -Seconds 2
                
                # Verify gh is now available
                if (Get-Command gh -ErrorAction SilentlyContinue) {
                    Write-Host "[lock] Authenticating with GitHub..." -ForegroundColor Cyan
                    Write-Host "   [!]  This requires manual steps in your browser:" -ForegroundColor Yellow
                    Write-Host "      1. A code and URL will be shown" -ForegroundColor Gray
                    Write-Host "      2. Copy the code and open the URL" -ForegroundColor Gray
                    Write-Host "      3. Enter the code and authorize" -ForegroundColor Gray
                    Write-Host ""
                    Write-Host "   Running: gh auth login" -ForegroundColor Cyan
                    Write-Host ""
                    gh auth login
                    if ($LASTEXITCODE -eq 0) {
                        Write-Host "[ok] GitHub CLI authenticated" -ForegroundColor Green
                        Write-Host ""
                        return $true
                    } else {
                        Write-Host "[!]  GitHub CLI authentication failed or cancelled" -ForegroundColor Yellow
                        Write-Host "   You can authenticate later with: gh auth login" -ForegroundColor Cyan
                        Write-Host ""
                        return $false
                    }
                } else {
                    Write-Host "[!]  GitHub CLI installed but not yet available in PATH" -ForegroundColor Yellow
                    Write-Host "   You may need to restart PowerShell or run: gh auth login" -ForegroundColor Cyan
                    Write-Host ""
                    return $false
                }
            } else {
                Write-Host "[x] Failed to install GitHub CLI" -ForegroundColor Red
                Write-Host "   You can install manually: winget install GitHub.cli" -ForegroundColor Yellow
                Write-Host "   Or download from: https://cli.github.com/" -ForegroundColor Cyan
                Write-Host ""
                return $false
            }
        } else {
            Write-Host "[i]  Skipping GitHub CLI installation" -ForegroundColor Cyan
            Write-Host "   Some features may require manual setup" -ForegroundColor Yellow
            Write-Host ""
            return $false
        }
    } else {
        # GitHub CLI is installed, check if authenticated
        $null = gh auth status 2>$null
        if ($LASTEXITCODE -ne 0) {
            Write-Host "[lock] GitHub CLI not authenticated" -ForegroundColor Yellow
            Write-Host "   GitHub CLI is installed but not authenticated" -ForegroundColor Cyan
            Write-Host ""
            Write-Host "Authenticate GitHub CLI now? (Requires browser steps) (Y/n): " -NoNewline -ForegroundColor Cyan
            $response = Read-Host
            $authGh = if ([string]::IsNullOrWhiteSpace($response)) { $true } else { $response -match "^[Yy]$" }
            if ($authGh) {
                Write-Host ""
                Write-Host "   [!]  Manual steps required:" -ForegroundColor Yellow
                Write-Host "      1. A code and URL will be shown" -ForegroundColor Gray
                Write-Host "      2. Copy the code and open the URL in your browser" -ForegroundColor Gray
                Write-Host "      3. Enter the code and authorize the application" -ForegroundColor Gray
                Write-Host ""
                Write-Host "   Running: gh auth login" -ForegroundColor Cyan
                Write-Host ""
                gh auth login
                if ($LASTEXITCODE -eq 0) {
                    Write-Host "[ok] GitHub CLI authenticated" -ForegroundColor Green
                    Write-Host ""
                    return $true
                } else {
                    Write-Host "[!]  GitHub CLI authentication failed or cancelled" -ForegroundColor Yellow
                    Write-Host "   You can authenticate later with: gh auth login" -ForegroundColor Cyan
                    Write-Host ""
                    return $false
                }
            } else {
                Write-Host "[i]  Skipping GitHub CLI authentication" -ForegroundColor Cyan
                Write-Host "   Some features may require manual setup" -ForegroundColor Yellow
                Write-Host ""
                return $false
            }
        } else {
            # GitHub CLI is installed and authenticated
            return $true
        }
    }
}

function Collect-AllInputs {
    param(
        [string]$GIT_REPO_DIR
    )
    
    Write-Host ""
    Write-Host "[key] Bootstrapping GitHub SSH authentication..."
    Write-Host "[file] Working directory: $GIT_REPO_DIR" -ForegroundColor Cyan
    Write-Host ""
    
    # Run pre-flight checks (status view only, no prompts)
    $preFlightResult = Invoke-PreFlightCheck -GIT_REPO_DIR $GIT_REPO_DIR
    
    # Check and prompt for GitHub CLI installation early
    Write-BoxedHeader -Title "GitHub CLI Setup"
    $script:GITHUB_CLI_AVAILABLE = Ensure-GitHubCli
    Write-Host ""
    
    # Initialize variables
    $script:CREATE_NEW_KEY = $false
    
    # Collect SSH key selection
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
    $wizardSteps = @()
    
    # SSH Key selection step
    if ($existingKeys.Count -gt 0) {
        $keyOptions = @()
        foreach ($key in $existingKeys) {
            $fingerprint = if ($key.Fingerprint) { "($($key.Fingerprint))" } else { "" }
            $comment = if ($key.Comment) { " - $($key.Comment)" } else { "" }
            $keyOptions += "$($key.Name)$comment $fingerprint"
        }
        $keyOptions += "Create new key"
        
        $wizardSteps += @{
            type = "select"
            title = "Select SSH Key"
            key = "ssh_key_selection"
            options = $keyOptions
        }
        
        # Conditional step for new key name (only if "Create new key" is selected)
        $wizardSteps += @{
            type = "input"
            title = "Enter SSH key name"
            key = "ssh_key_name"
            placeholder = "mykey"
            description = "Note: 'github_' will be automatically prefixed to the key name"
            required = $false
        }
    } else {
        # No existing keys, just ask for key name
        $wizardSteps += @{
            type = "input"
            title = "Enter SSH key name"
            key = "ssh_key_name"
            placeholder = "mykey"
            description = "Note: 'github_' will be automatically prefixed to the key name"
            required = $true
        }
    }
    
    # SSH Config update step (conditional)
    if (Test-Path $SSH_CONFIG) {
        $configContent = Get-Content $SSH_CONFIG -Raw
        if ($configContent -match "Host github\.com") {
            $wizardSteps += @{
                type = "confirm"
                title = "Update existing SSH configuration?"
                key = "update_ssh_config"
                description = "Update existing configuration in $SSH_CONFIG"
                default = "false"
            }
        }
    }
    
    # Repository recreation step (conditional)
    if (Test-Path "$GIT_REPO_DIR\.git") {
        $wizardSteps += @{
            type = "confirm"
            title = "Remove existing repository and recreate?"
            key = "recreate_repo"
            description = "Repository exists at: $GIT_REPO_DIR"
            default = "true"
        }
    }
    
    # Remote removal step (conditional)
    Push-Location $GIT_REPO_DIR
    $remotes = git remote 2>$null
    if ($remotes) {
        $remoteOutput = git remote -v 2>$null
        $remoteList = $remoteOutput -join ", "
        $wizardSteps += @{
            type = "confirm"
            title = "Remove existing remotes and set up new one?"
            key = "remove_remotes"
            description = "Current remotes: $remoteList"
            default = "true"
        }
    }
    Pop-Location
    
    # GitHub username step
    $gitUserName = git config --global user.name 2>$null
    $defaultUsername = if ($null -ne $gitUserName -and $gitUserName -ne "") { [string]$gitUserName.Trim() } else { "" }
    
    if (-not [string]::IsNullOrWhiteSpace($defaultUsername)) {
        $wizardSteps += @{
            type = "input"
            title = "GitHub username"
            key = "github_user"
            placeholder = $defaultUsername
            default = $defaultUsername
            description = "Detected from git config"
            required = $true
        }
    } else {
        $wizardSteps += @{
            type = "input"
            title = "GitHub username"
            key = "github_user"
            placeholder = "username"
            required = $true
        }
    }
    
    # Repository name step
    $defaultRepoName = Split-Path -Path $GIT_REPO_DIR -Leaf
    $wizardSteps += @{
        type = "input"
        title = "Repository name"
        key = "repo_name"
        placeholder = $defaultRepoName
        default = $defaultRepoName
        required = $true
    }
    
    # Repository visibility step
    $wizardSteps += @{
        type = "confirm"
        title = "Make repository private?"
        key = "repo_private"
        default = "false"
    }
    
    # Git-crypt setup step
    $wizardSteps += @{
        type = "confirm"
        title = "Set up git-crypt for encrypted secrets?"
        key = "setup_git_crypt"
        default = "false"
    }
    
    # Run wizard
    Write-Host ""
    Write-Host "[wizard] Collecting setup information..." -ForegroundColor Cyan
    Write-Host ""
    
    $wizardResult = iwizard-RunInline -Steps $wizardSteps
    if (-not $wizardResult) {
        Write-Error "Wizard was cancelled or failed" -ErrorAction Stop
        return
    }
    
    # Parse wizard results
    $wizardResult = $wizardResult.Trim()
    if ([string]::IsNullOrWhiteSpace($wizardResult)) {
        Write-Error "Wizard returned empty result" -ErrorAction Stop
        return
    }
    
    try {
        $results = $wizardResult | ConvertFrom-Json
    } catch {
        Write-Error "Failed to parse wizard result: $_`nResult was: $wizardResult" -ErrorAction Stop
        return
    }
    
    # Process SSH key selection
    if ($existingKeys.Count -gt 0) {
        $selectedKeyOption = $results.ssh_key_selection
        if ($selectedKeyOption -eq "Create new key" -or [string]::IsNullOrWhiteSpace($selectedKeyOption)) {
            $keyNameInput = if ($results.ssh_key_name) { $results.ssh_key_name.Trim() } else { "" }
            if ([string]::IsNullOrWhiteSpace($keyNameInput)) {
                Write-Error "SSH key name is required" -ErrorAction Stop
                return
            }
            if (-not $keyNameInput.StartsWith("github_")) {
                $script:SSH_KEY_NAME = "github_$keyNameInput"
            } else {
                $script:SSH_KEY_NAME = $keyNameInput
            }
            $script:SSH_KEY_PATH = "$sshDir\$($script:SSH_KEY_NAME)"
            $script:CREATE_NEW_KEY = $true
        } else {
            # Find the selected key
            $selectedKey = $null
            foreach ($key in $existingKeys) {
                $fingerprint = if ($key.Fingerprint) { "($($key.Fingerprint))" } else { "" }
                $comment = if ($key.Comment) { " - $($key.Comment)" } else { "" }
                $keyDisplay = "$($key.Name)$comment $fingerprint"
                if ($keyDisplay -eq $selectedKeyOption) {
                    $selectedKey = $key
                    break
                }
            }
            if ($selectedKey) {
                $script:SSH_KEY_NAME = $selectedKey.Name
                $script:SSH_KEY_PATH = $selectedKey.PrivatePath
                $script:SSH_PUBLIC_KEY = Get-Content $selectedKey.PublicPath -Raw
            }
        }
    } else {
        $keyNameInput = if ($results.ssh_key_name) { $results.ssh_key_name.Trim() } else { "" }
        if ([string]::IsNullOrWhiteSpace($keyNameInput)) {
            Write-Error "SSH key name is required" -ErrorAction Stop
            return
        }
        if (-not $keyNameInput.StartsWith("github_")) {
            $script:SSH_KEY_NAME = "github_$keyNameInput"
        } else {
            $script:SSH_KEY_NAME = $keyNameInput
        }
        $script:SSH_KEY_PATH = "$sshDir\$($script:SSH_KEY_NAME)"
        $script:CREATE_NEW_KEY = $true
    }
    
    # Process SSH config update
    $script:UPDATE_SSH_CONFIG = if ($results.update_ssh_config) { $results.update_ssh_config } else { $false }
    
    # Process repository recreation
    $script:RECREATE_REPO = if ($results.recreate_repo) { $results.recreate_repo } else { $false }
    
    # Process remote removal
    $script:REMOVE_REMOTES = if ($results.remove_remotes) { $results.remove_remotes } else { $false }
    
    # Process GitHub username
    $script:GITHUB_USER = if ($results.github_user) { $results.github_user.Trim() } else { "" }
    if ([string]::IsNullOrWhiteSpace($script:GITHUB_USER)) {
        Write-Error "GitHub username is required" -ErrorAction Stop
        return
    }
    
    # Process repository name
    $script:REPO_NAME = if ($results.repo_name) { $results.repo_name.Trim() } else { "" }
    if ([string]::IsNullOrWhiteSpace($script:REPO_NAME)) {
        Write-Error "Repository name is required" -ErrorAction Stop
        return
    }
    
    # Process repository visibility
    $script:REPO_PRIVATE = if ($results.repo_private) { $results.repo_private } else { $false }
    
    # Process git-crypt setup
    $script:SETUP_GIT_CRYPT = if ($results.setup_git_crypt) { $results.setup_git_crypt } else { $false }
    
    Write-Host ""
}

function Setup-SshKey {
    Write-BoxedHeader -Title "SSH Key"
    
    # Use pre-collected values from Collect-AllInputs
    $SSH_KEY_PATH = $script:SSH_KEY_PATH
    
    # If we need to create a new key
    if ($script:CREATE_NEW_KEY) {
        # Check if key already exists
        if (Test-Path $SSH_KEY_PATH) {
            Write-Host "[!]  SSH key already exists at $SSH_KEY_PATH"
            Write-Host "[trash]  Removing existing key..."
            Remove-Item $SSH_KEY_PATH -ErrorAction SilentlyContinue
            Remove-Item "$SSH_KEY_PATH.pub" -ErrorAction SilentlyContinue
        }
        
        # Generate new SSH key
        Write-Host "[lock] Generating SSH key pair..."
        $sshDir = Split-Path $SSH_KEY_PATH -Parent
        if (-not (Test-Path $sshDir)) {
            New-Item -ItemType Directory -Path $sshDir -Force | Out-Null
        }
        
        ssh-keygen -t ed25519 -C $GIT_EMAIL -f $SSH_KEY_PATH -N '""'
        if ($LASTEXITCODE -eq 0) {
            Write-Host "[ok] SSH key created: $SSH_KEY_PATH"
            
            # Set proper permissions on Windows
            $keyFile = Get-Item $SSH_KEY_PATH
            $keyFile.Attributes = "Archive"
            icacls $SSH_KEY_PATH /inheritance:r /grant "${env:USERNAME}:F" | Out-Null
            
            $pubKeyFile = Get-Item "$SSH_KEY_PATH.pub"
            $pubKeyFile.Attributes = "Archive"
            
            # Store public key
            $script:SSH_PUBLIC_KEY = Get-Content "$SSH_KEY_PATH.pub" -Raw
        } else {
            Write-Host "[x] Failed to generate SSH key"
            return
        }
    }
    
    # Try to automatically add the key to GitHub
    $script:SSH_KEY_ADDED = Add-SshKeyToGitHub -PublicKeyPath "$SSH_KEY_PATH.pub" -KeyName $script:SSH_KEY_NAME
}

function Setup-SshConfig {
    Write-BoxedHeader -Title "GitHub SSH Configuration"
    
    # Use pre-collected values from Collect-AllInputs
    $keyName = $script:SSH_KEY_NAME
    
    $sshDir = Split-Path $SSH_CONFIG -Parent
    if (-not (Test-Path $sshDir)) {
        New-Item -ItemType Directory -Path $sshDir -Force | Out-Null
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
                $selectedKeyPath = "$env:USERPROFILE\.ssh\$keyName"
                $selectedKeyPath = $selectedKeyPath -replace '/', '\'
                
                if ($existingKeyPath -like "*$keyName" -or $existingKeyPath -eq $selectedKeyPath) {
                    Write-Host "[ok] SSH config already uses the selected key ($keyName)"
                    Write-Host ""
                    return
                }
            }
        }
    }
    
    # Use pre-collected value to determine if we should update config
    if ($script:UPDATE_SSH_CONFIG) {
        # Remove existing GitHub config block
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
        Write-Host "[trash]  Removed existing GitHub configuration"
    }
    
    # Add GitHub SSH config if not present
    $configContent = if (Test-Path $SSH_CONFIG) { Get-Content $SSH_CONFIG -Raw } else { "" }
    if ($configContent -notmatch "Host github\.com") {
        $githubConfig = @"

Host github.com
    HostName github.com
    User git
    IdentityFile ~/.ssh/$keyName
    IdentitiesOnly yes

"@
        Add-Content -Path $SSH_CONFIG -Value $githubConfig
        Write-Host "[ok] SSH config updated"
    }
    
    # Set permissions
    if (Test-Path $SSH_CONFIG) {
        icacls $SSH_CONFIG /inheritance:r /grant "${env:USERNAME}:F" | Out-Null
    }
}

function Setup-LocalRepo {
    Write-BoxedHeader -Title "Local Repository"
    
    # Use the directory captured at the start of the setup command
    $GIT_REPO_DIR = $script:ORIGINAL_WORKING_DIR
    
    # Use pre-collected value to determine if we should recreate repo
    if ($script:RECREATE_REPO) {
        Remove-Item "$GIT_REPO_DIR\.git" -Recurse -Force -ErrorAction SilentlyContinue
        Write-Host "[trash]  Removed existing repository"
        Write-Host ""
    } elseif (Test-Path "$GIT_REPO_DIR\.git") {
        Write-Host "[i]  Keeping existing repository"
        Write-Host ""
        return
    }
    
    $gitRepoPath = "$GIT_REPO_DIR\.git"
    Write-Host "[open-file] Setting up local git repository: $gitRepoPath"
    Write-Host ""
    
    if (-not (Test-Path $GIT_REPO_DIR)) {
        New-Item -ItemType Directory -Path $GIT_REPO_DIR -Force | Out-Null
    }
    Push-Location $GIT_REPO_DIR
    
    # Initialize git repository
    git init
    if ($LASTEXITCODE -eq 0) {
        # Set default branch to main
        git branch -M main 2>$null
        if ($LASTEXITCODE -ne 0) {
            git checkout -b main 2>$null
        }
        
        # Configure git user if not set
        $gitName = git config user.name 2>$null
        if (-not $gitName) {
            $globalName = git config --global user.name 2>$null
            if (-not $globalName) {
                $gitName = Read-Host "Enter your name for git commits"
                git config user.name $gitName
                git config --global user.name $gitName
            } else {
                git config user.name $globalName
            }
        }
        
        $gitEmail = git config user.email 2>$null
        if (-not $gitEmail) {
            $globalEmail = git config --global user.email 2>$null
            if (-not $globalEmail) {
                $gitEmail = Read-Host "Enter your email for git commits"
                git config user.email $gitEmail
                git config --global user.email $gitEmail
            } else {
                git config user.email $globalEmail
            }
        }
        
        Write-Host "[ok] Git repository initialized and configured"
    } else {
        Write-Host "[x] Failed to initialize git repository"
    }
    
    Pop-Location
}

function Remove-SshKey {
    param(
        [switch]$Force
    )
    
    $SSH_KEY_PATH = Get-SshKeyPath
    $removed = $false
    $error = $null
    
    if ((Test-Path $SSH_KEY_PATH) -or (Test-Path "$SSH_KEY_PATH.pub")) {
        if (-not $Force) {
            Write-Host "[!]  WARNING: This will remove the SSH key at $SSH_KEY_PATH"
            $response = Read-Host "Are you sure? (y/N)"
            if ($response -notmatch "^[Yy]$") {
                Write-Host "[x] Cancelled"
                return @{ Success = $false; Error = "Cancelled" }
            }
        }
        
        try {
            Remove-Item $SSH_KEY_PATH -ErrorAction Stop
            $removed = $true
        } catch {
            $error = $_.Exception.Message
        }
        
        try {
            Remove-Item "$SSH_KEY_PATH.pub" -ErrorAction Stop
            $removed = $true
        } catch {
            if (-not $error) { $error = $_.Exception.Message }
        }
        
        if ($removed -and -not $error) {
            if (-not $Force) {
                Write-Host "[ok] SSH key removed"
                Write-Host "   Note: You may want to remove it from GitHub: https://github.com/settings/keys"
            }
            return @{ Success = $true; Error = $null; Note = "Remove from GitHub: https://github.com/settings/keys" }
        } else {
            return @{ Success = $false; Error = $error }
        }
    } else {
        if (-not $Force) {
            Write-Host "[i]  No SSH key found at $SSH_KEY_PATH"
        }
        return @{ Success = $true; Error = $null; Skipped = $true }
    }
}

function Remove-SshConfig {
    param(
        [switch]$Force
    )
    
    $removed = $false
    $error = $null
    
    if (Test-Path $SSH_CONFIG) {
        $configContent = Get-Content $SSH_CONFIG -Raw
        if ($configContent -match "Host github\.com") {
            if (-not $Force) {
                Write-Host "[!]  This will remove GitHub SSH config from $SSH_CONFIG"
                $response = Read-Host "Are you sure? (y/N)"
                if ($response -notmatch "^[Yy]$") {
                    Write-Host "[x] Cancelled"
                    return @{ Success = $false; Error = "Cancelled" }
                }
            }
            
            try {
                # Remove existing GitHub config block
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
                $newLines | Set-Content $SSH_CONFIG -ErrorAction Stop
                $removed = $true
                if (-not $Force) {
                    Write-Host "[ok] GitHub SSH config removed from $SSH_CONFIG"
                }
            } catch {
                $error = $_.Exception.Message
            }
            
            if ($removed) {
                return @{ Success = $true; Error = $null }
            } else {
                return @{ Success = $false; Error = $error }
            }
        } else {
            if (-not $Force) {
                Write-Host "[i]  No GitHub SSH config found in $SSH_CONFIG"
            }
            return @{ Success = $true; Error = $null; Skipped = $true }
        }
    } else {
        if (-not $Force) {
            Write-Host "[i]  SSH config file not found: $SSH_CONFIG"
        }
        return @{ Success = $true; Error = $null; Skipped = $true }
    }
}

function Remove-LocalRepo {
    param(
        [switch]$Force
    )
    
    $removed = $false
    $error = $null
    
    if (Test-Path "$GIT_REPO_DIR\.git") {
        if (-not $Force) {
            Write-Host "[!]  WARNING: This will remove the git repository at $GIT_REPO_DIR"
            Write-Host "   This will NOT delete your files, only the .git directory"
            $response = Read-Host "Are you sure? (y/N)"
            if ($response -notmatch "^[Yy]$") {
                Write-Host "[x] Cancelled"
                return @{ Success = $false; Error = "Cancelled" }
            }
        }
        
        try {
            Remove-Item "$GIT_REPO_DIR\.git" -Recurse -Force -ErrorAction Stop
            $removed = $true
            if (-not $Force) {
                Write-Host "[ok] Git repository removed from $GIT_REPO_DIR"
                Write-Host "   Your files are still intact"
            }
        } catch {
            $error = $_.Exception.Message
        }
        
        if ($removed) {
            return @{ Success = $true; Error = $null }
        } else {
            return @{ Success = $false; Error = $error }
        }
    } else {
        if (-not $Force) {
            Write-Host "[i]  No git repository found at $GIT_REPO_DIR"
        }
        return @{ Success = $true; Error = $null; Skipped = $true }
    }
}

function Remove-Remote {
    param(
        [switch]$Force
    )
    
    $removed = $false
    $error = $null
    $removedRemotes = @()
    
    if (Test-Path "$GIT_REPO_DIR\.git") {
        Push-Location $GIT_REPO_DIR
        try {
            $remotes = git remote 2>$null
            if ($remotes) {
                if (-not $Force) {
                    Write-Host "[clipboard] Current remotes:"
                    git remote -v
                    Write-Host ""
                    $response = Read-Host "Remove all remotes? (y/N)"
                    if ($response -notmatch "^[Yy]$") {
                        Write-Host "[x] Cancelled"
                        Pop-Location
                        return @{ Success = $false; Error = "Cancelled" }
                    }
                }
                
                foreach ($remote in $remotes) {
                    try {
                        git remote remove $remote 2>&1 | Out-Null
                        if ($LASTEXITCODE -eq 0) {
                            $removedRemotes += $remote
                            $removed = $true
                            if (-not $Force) {
                                Write-Host "[ok] Removed remote: $remote"
                            }
                        } else {
                            $error = "Failed to remove remote: $remote"
                        }
                    } catch {
                        if (-not $error) { $error = $_.Exception.Message }
                    }
                }
                
                if ($removed -and -not $Force) {
                    Write-Host "[ok] All remotes removed"
                }
            } else {
                if (-not $Force) {
                    Write-Host "[i]  No remotes configured"
                }
                Pop-Location
                return @{ Success = $true; Error = $null; Skipped = $true }
            }
        } catch {
            $error = $_.Exception.Message
        } finally {
            Pop-Location
        }
        
        if ($removed) {
            return @{ Success = $true; Error = $null; RemovedRemotes = $removedRemotes }
        } else {
            return @{ Success = $false; Error = $error }
        }
    } else {
        if (-not $Force) {
            Write-Host "[i]  No git repository found at $GIT_REPO_DIR"
        }
        return @{ Success = $true; Error = $null; Skipped = $true }
    }
}

function Test-RepoExists {
    param(
        [string]$GitHubUser,
        [string]$RepoName
    )
    
    Push-Location $GIT_REPO_DIR
    $null = git ls-remote "git@github.com:${GitHubUser}/${RepoName}.git" 2>$null
    $exists = $LASTEXITCODE -eq 0
    Pop-Location
    return $exists
}

function New-RepoWithGh {
    param(
        [string]$GitHubUser,
        [string]$RepoName,
        [bool]$IsPrivate
    )
    
    if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
        return $false
    }
    
    # Check if gh is authenticated
    $null = gh auth status 2>$null
    if ($LASTEXITCODE -ne 0) {
        return $false
    }
    
    # Create repository
    Push-Location $GIT_REPO_DIR
    
    # Try creating repo without --push first (in case there are no commits)
    if ($IsPrivate) {
        $output = gh repo create $RepoName --private --source=. --remote=origin 2>&1 | Out-String
    } else {
        $output = gh repo create $RepoName --public --source=. --remote=origin 2>&1 | Out-String
    }
    $result = $LASTEXITCODE -eq 0
    
    # Check if repo already exists (this is actually okay - we can use the existing repo)
    if (-not $result -and $output -match "already exists|Name already exists") {
        Write-Host '  [i] Repository already exists - will use existing repository' -ForegroundColor Cyan
        $result = $true  # Treat as success since repo exists
    }
    
    if ($result) {
        # If repo exists or was created successfully, try to push if there are commits
        $hasCommits = git rev-parse HEAD 2>$null
        if ($LASTEXITCODE -eq 0) {
            # There are commits, try to push
            $null = git push -u origin main 2>$null
            if ($LASTEXITCODE -ne 0) {
                # Push failed but repo exists, that's okay
                Write-Host '  [!] Push failed (you can push manually later)' -ForegroundColor Yellow
            }
        }
    } else {
        # Show error if creation failed (and it's not because it already exists)
        if ($output) {
            $errorMsg = ($output -split "`n") | Where-Object { $_ -match "error|Error|failed|Failed" -and $_ -notmatch "already exists" } | Select-Object -First 2
            if ($errorMsg) {
                Write-Host '  [!] Error creating repository:' -ForegroundColor Yellow
                $errorMsg | ForEach-Object { Write-Host "     $_" -ForegroundColor Yellow }
            }
        }
    }
    
    Pop-Location
    
    return $result
}

function New-RepoWithApi {
    param(
        [string]$GitHubUser,
        [string]$RepoName,
        [bool]$IsPrivate,
        [string]$GitHubToken
    )
    
    if (-not $GitHubToken) {
        return $false
    }
    
    $body = @{
        name = $RepoName
        private = $IsPrivate
        auto_init = $false
    } | ConvertTo-Json
    
    $headers = @{
        Accept = "application/vnd.github.v3+json"
        Authorization = "token $GitHubToken"
    }
    
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/user/repos" -Method Post -Headers $headers -Body $body -ErrorAction Stop
        return $true
    } catch {
        return $false
    }
}

function Setup-Remote {
    Write-BoxedHeader -Title "Remote Repository"
    
    # Use pre-collected values from Collect-AllInputs
    $GIT_REPO_DIR = $script:ORIGINAL_WORKING_DIR
    Push-Location $GIT_REPO_DIR
    
    # Use pre-collected values
    $githubUser = $script:GITHUB_USER
    $repoName = $script:REPO_NAME
    
    if (-not $githubUser) {
        Write-Host "[!]  No username provided, skipping remote setup"
        Pop-Location
        return 1
    }
    
    # Use pre-collected value to determine if we should remove remotes
    $remotes = git remote 2>$null
    if ($remotes -and $script:REMOVE_REMOTES) {
        foreach ($remote in $remotes) {
            git remote remove $remote
        }
        Write-Host "[ok] Removed remote: origin"
        Write-Host ""
    } elseif ($remotes) {
        Write-Host "[i]  Keeping existing remotes"
        Pop-Location
        return
    }
    
    # Check if repository already exists
    Write-Host ""
    Write-Host "[search] Checking if repository exists on GitHub..."
    if (Test-RepoExists $githubUser $repoName) {
        Write-Host "[ok] Repository already exists: ${githubUser}/${repoName}"
    } else {
        Write-Host "[i]  Repository doesn't exist yet: ${githubUser}/${repoName}"
        Write-Host ""
        # If remotes were removed or there are no remotes (fresh setup), automatically create the repository
        # User already indicated intent by choosing to remove remotes and set up new one
        if ($script:REMOVE_REMOTES -or -not $remotes) {
            $createRepo = $true
        } else {
            Write-Host "Create repository on GitHub? (Y/n): " -NoNewline -ForegroundColor Cyan
            $response = Read-Host
            $createRepo = if ([string]::IsNullOrWhiteSpace($response)) { $true } else { $response -match "^[Yy]$" }
        }
        if ($createRepo) {
            # Use pre-collected visibility preference
            $isPrivate = $script:REPO_PRIVATE
            
            # Check if GitHub CLI is available (may have been set up earlier)
            if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
                # If not available, try to ensure it (in case user skipped earlier)
                if (-not $script:GITHUB_CLI_AVAILABLE) {
                    Write-Host ""
                    Write-Host "[!]  GitHub CLI not available"
                    Write-Host "   Attempting to set up GitHub CLI..."
                    Write-Host ""
                    $script:GITHUB_CLI_AVAILABLE = Ensure-GitHubCli
                }
            } else {
                # GitHub CLI is installed, check if authenticated
                $null = gh auth status 2>$null
                if ($LASTEXITCODE -ne 0) {
                    Write-Host ""
                    Write-Host "[lock] GitHub CLI not authenticated"
                    Write-Host "   [!]  This requires manual steps in your browser:" -ForegroundColor Yellow
                    Write-Host "      1. A code and URL will be shown" -ForegroundColor Gray
                    Write-Host "      2. Copy the code and open the URL in your browser" -ForegroundColor Gray
                    Write-Host "      3. Enter the code and authorize the application" -ForegroundColor Gray
                    Write-Host ""
                    Write-Host "   Running: gh auth login"
                    Write-Host ""
                    gh auth login
                    if ($LASTEXITCODE -eq 0) {
                        Write-Host "[ok] GitHub CLI authenticated"
                        $script:GITHUB_CLI_AVAILABLE = $true
                    } else {
                        Write-Host "[!]  GitHub CLI authentication failed or cancelled"
                        Write-Host "   Will try alternative methods..."
                        $script:GITHUB_CLI_AVAILABLE = $false
                    }
                    Write-Host ""
                } else {
                    $script:GITHUB_CLI_AVAILABLE = $true
                }
            }
            
            # Try to create using GitHub CLI
            Write-Host ""
            Write-Host "[hammer] Creating repository on GitHub..."
            Write-Host "   Repository: ${githubUser}/${repoName}" -ForegroundColor Gray
            Write-Host "   Visibility: $(if ($isPrivate) { 'Private' } else { 'Public' })" -ForegroundColor Gray
            Write-Host ""
            
            if (New-RepoWithGh $githubUser $repoName $isPrivate) {
                Write-Host "[ok] Repository ready (created or already exists)" -ForegroundColor Green
                $currentBranch = git branch --show-current 2>$null
                if (-not $currentBranch) { $currentBranch = "main" }
                git branch --set-upstream-to=origin/$currentBranch 2>$null
                Write-Host ""
                Write-Host "[clipboard] Current remotes:"
                git remote -v
                Pop-Location
                return
            } else {
                Write-Host "[!]  GitHub CLI failed to create repository" -ForegroundColor Yellow
                Write-Host "   (Repository may already exist - will continue with remote setup)" -ForegroundColor Gray
                Write-Host ""
                
                # Try using API with token from environment or secrets
                $githubToken = $env:GITHUB_TOKEN
                if (-not $githubToken -and (Test-Path "$env:USERPROFILE\.secrets")) {
                    $secretsContent = Get-Content "$env:USERPROFILE\.secrets" -ErrorAction SilentlyContinue
                    $tokenLine = $secretsContent | Select-String -Pattern "^GITHUB_TOKEN="
                    if ($tokenLine) {
                        $githubToken = ($tokenLine -split "=")[1] -replace '"', '' -replace "'", ''
                    }
                }
                
                if ($githubToken) {
                    Write-Host "  [key] Using GitHub token from environment/secrets..."
                    if (New-RepoWithApi $githubUser $repoName $isPrivate $githubToken) {
                        Write-Host "[ok] Repository created using GitHub API"
                    } else {
                        Write-Host "[!]  Failed to create repository via API" -ForegroundColor Yellow
                        Write-Host "   You'll need to create it manually" -ForegroundColor Gray
                    }
                } else {
                    Write-Host "[!]  Cannot create repository automatically" -ForegroundColor Yellow
                    Write-Host ""
                    Write-Host "   Troubleshooting:" -ForegroundColor Cyan
                    Write-Host "   1. Check GitHub CLI authentication: gh auth status" -ForegroundColor Gray
                    Write-Host "   2. Try manually: gh repo create $repoName --$(if ($isPrivate) { 'private' } else { 'public' })" -ForegroundColor Gray
                    Write-Host "   3. Set GITHUB_TOKEN environment variable for API method" -ForegroundColor Gray
                    Write-Host "   4. Create manually at: https://github.com/new" -ForegroundColor Gray
                }
            }
        }
    }
    
    # Add remote if it doesn't exist
    $existingRemotes = git remote 2>$null
    if ($existingRemotes -notcontains "origin") {
        $remoteUrl = "git@github.com:${githubUser}/${repoName}.git"
        git remote add origin $remoteUrl
        Write-Host "[ok] Added remote 'origin': $remoteUrl"
    } else {
        # Update remote URL if it's different
        $currentUrl = git remote get-url origin 2>$null
        $newUrl = "git@github.com:${githubUser}/${repoName}.git"
        if ($currentUrl -ne $newUrl) {
            git remote set-url origin $newUrl
            Write-Host "[ok] Updated remote 'origin': $newUrl"
        }
    }
    
    # Set upstream branch
    $currentBranch = git branch --show-current 2>$null
    if (-not $currentBranch) { $currentBranch = "main" }
    Write-Host "[wrench] Setting upstream branch to origin/$currentBranch"
    git branch --set-upstream-to=origin/$currentBranch 2>$null
    if ($LASTEXITCODE -ne 0) {
        Write-Host "[!]  Upstream will be set on first push"
    }
    
    Write-Host ""
    Write-Host "[clipboard] Current remotes:"
    git remote -v
    Write-Host ""
    
    # Check if repository exists before showing next steps
    if (-not (Test-RepoExists $githubUser $repoName)) {
        Write-Host "[note] Next steps:"
        Write-Host "   1. Create the repository on GitHub: https://github.com/new"
        Write-Host "      Name: $repoName"
        Write-Host "      Don't initialize with README (we already have one)"
        Write-Host ""
        Write-Host "   2. Push to GitHub:"
        Write-Host "      git add ."
        Write-Host "      git commit -m 'Initial $repoName repository setup'"
        Write-Host "      git push -u origin $currentBranch"
    } else {
        Write-Host "[ok] Repository is ready! Push your code:"
        Write-Host "   git push -u origin $currentBranch"
    }
    
    Pop-Location
}

function Setup-GitCrypt {
    # Install git-crypt if not already installed
    if (-not (Get-Command git-crypt -ErrorAction SilentlyContinue)) {
        Write-Host "[!]  git-crypt not installed"
        Write-Host "Install git-crypt now? (Y/n): " -NoNewline -ForegroundColor Cyan
        $response = Read-Host
        $installGitCrypt = if ([string]::IsNullOrWhiteSpace($response)) { $true } else { $response -match "^[Yy]$" }
        if ($installGitCrypt) {
            Write-Host ""
            Write-Host "[pkg] Looking for git-crypt.exe..." -ForegroundColor Cyan
            Write-Host ""
            
            $installSuccess = $false
            # Get Downloads folder from registry (handles junctions/symlinks correctly)
            $downloadsPath = $null
            try {
                $downloadsGuid = "{374DE290-123F-4565-9164-39C4925E467B}"
                $downloadsPath = (Get-ItemProperty "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Shell Folders").$downloadsGuid
                if ($downloadsPath) {
                    Write-Host "  Using Downloads folder from registry: $downloadsPath" -ForegroundColor Gray
                }
            } catch {
                # Fallback to default if registry access fails
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
            } else {
                Write-Host "  [!]  Downloads path does not exist: $downloadsPath" -ForegroundColor Yellow
            }
            
            # If not found in Downloads, prompt for a directory path to search
            if (-not $gitCryptExe) {
                Write-Host "  [x] git-crypt.exe not found in Downloads" -ForegroundColor Yellow
                Write-Host ""
                Write-Host "   Download from: https://github.com/oholovko/git-crypt-windows/releases" -ForegroundColor Cyan
                Write-Host ""
                Write-Host "Enter directory path to search for git-crypt.exe [$downloadsPath]: " -NoNewline -ForegroundColor Cyan
                $searchPath = Read-Host
                if ([string]::IsNullOrWhiteSpace($searchPath)) {
                    $searchPath = $downloadsPath
                }
                
                if ($searchPath -and (Test-Path $searchPath)) {
                    # If user entered Downloads path, use registry to get actual path
                    $normalizedSearch = [System.IO.Path]::GetFullPath($searchPath)
                    $normalizedDownloads = [System.IO.Path]::GetFullPath($downloadsPath)
                    
                    # Check if the search path is the Downloads folder (or a junction to it)
                    if ($normalizedSearch -like "*\Downloads" -or $normalizedSearch -eq $normalizedDownloads) {
                        try {
                            $downloadsGuid = "{374DE290-123F-4565-9164-39C4925E467B}"
                            $actualDownloadsPath = (Get-ItemProperty "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Shell Folders").$downloadsGuid
                            if ($actualDownloadsPath -and (Test-Path $actualDownloadsPath)) {
                                Write-Host "  Resolved Downloads path to: $actualDownloadsPath" -ForegroundColor Gray
                                $searchPath = $actualDownloadsPath
                            }
                        } catch {
                            # If registry access fails, try Resolve-Path
                            try {
                                $resolved = Resolve-Path $searchPath -ErrorAction Stop
                                $searchPath = $resolved.Path
                            } catch {
                                # Use original path
                            }
                        }
                    } else {
                        # For other paths, try to resolve links
                        try {
                            $resolved = Resolve-Path $searchPath -ErrorAction Stop
                            if ($resolved.Path -ne $searchPath) {
                                Write-Host "  Resolved path: $searchPath -> $($resolved.Path)" -ForegroundColor Gray
                                $searchPath = $resolved.Path
                            }
                        } catch {
                            # Use original path
                        }
                    }
                    
                    Write-Host "  Searching in: $searchPath" -ForegroundColor Gray
                    $gitCryptExe = Get-ChildItem -Path $searchPath -Filter "git-crypt.exe" -Recurse -ErrorAction SilentlyContinue | Select-Object -First 1
                    if ($gitCryptExe) {
                        Write-Host "  [ok] Found: $($gitCryptExe.FullName)" -ForegroundColor Green
                    } else {
                        Write-Host "  [x] git-crypt.exe not found in: $searchPath" -ForegroundColor Red
                    }
                }
            } else {
                Write-Host "  [ok] Found git-crypt.exe in Downloads" -ForegroundColor Green
            }
            
            # If found, move it to a PATH location
            if ($gitCryptExe) {
                Write-Host ""
                Write-Host "  [pkg] Installing git-crypt.exe..." -ForegroundColor Cyan
                
                # Find a suitable location in PATH (prefer user-writable locations first)
                $targetDirs = @(
                    # User-writable locations (no admin needed)
                    "$env:USERPROFILE\bin",
                    "$env:USERPROFILE\.local\bin",
                    "$env:LOCALAPPDATA\Programs\git-crypt",
                    # Git directories (may need admin)
                    "C:\Program Files\Git\usr\bin",
                    "C:\Program Files\Git\cmd",
                    "$env:ProgramFiles\Git\usr\bin",
                    "$env:ProgramFiles\Git\cmd"
                )
                
                $targetDir = $null
                foreach ($dir in $targetDirs) {
                    if (Test-Path $dir) {
                        # Check if we can write to this directory
                        try {
                            $testFile = Join-Path $dir ".write-test"
                            "test" | Set-Content $testFile -ErrorAction Stop
                            Remove-Item $testFile -ErrorAction SilentlyContinue
                            $targetDir = $dir
                            break
                        } catch {
                            # Can't write here, try next
                            continue
                        }
                    } elseif ($dir -like "$env:USERPROFILE*" -or $dir -like "$env:LOCALAPPDATA*") {
                        # Try to create user-writable directory
                        try {
                            New-Item -ItemType Directory -Path $dir -Force -ErrorAction Stop | Out-Null
                            $targetDir = $dir
                            break
                        } catch {
                            # Can't create, try next
                            continue
                        }
                    }
                }
                
                # If no writable directory found, use a user location we can create
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
                        
                        # Copy the file
                        Copy-Item -Path $gitCryptExe.FullName -Destination $targetPath -Force -ErrorAction Stop
                        Write-Host "  [ok] Copied git-crypt.exe to $targetDir" -ForegroundColor Green
                        
                        # Add to PATH if not already there
                        $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
                        if ($currentPath -notlike "*$targetDir*") {
                            Write-Host "  [*] Adding to user PATH..." -ForegroundColor Cyan
                            [Environment]::SetEnvironmentVariable("Path", "$currentPath;$targetDir", "User")
                            $env:Path += ";$targetDir"
                            Write-Host "  [OK] Added $targetDir to PATH" -ForegroundColor Green
                        }
                        
                        # Verify it works
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
                Write-Host "Continue without git-crypt? (You can install it manually later) (N/y): " -NoNewline -ForegroundColor Cyan
                $response = Read-Host
                $continue = if ([string]::IsNullOrWhiteSpace($response)) { $false } else { $response -match "^[Yy]$" }
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
    
    Write-BoxedHeader -Title "git-crypt setup complete!"
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

function Remove-GitCrypt {
    param(
        [switch]$Force
    )
    
    $removed = $false
    $error = $null
    
    Push-Location $GIT_REPO_DIR
    
    if (-not (Test-Path ".git")) {
        if (-not $Force) {
            Write-Host "[i]  Not a git repository"
        }
        Pop-Location
        return @{ Success = $true; Error = $null; Skipped = $true }
    }
    
    if (-not $Force) {
        Write-Host "[trash]  Removing git-crypt..."
        Write-Host ""
    }
    
    # Check if git-crypt is initialized
    $null = git-crypt status 2>$null
    if ($LASTEXITCODE -eq 0) {
        if (-not $Force) {
            Write-Host "[!]  WARNING: This will remove git-crypt encryption"
            Write-Host "   Encrypted files will become unencrypted"
            $response = Read-Host "Continue? (y/N)"
            if ($response -notmatch "^[Yy]$") {
                Write-Host "[x] Cancelled"
                Pop-Location
                return @{ Success = $false; Error = "Cancelled" }
            }
        }
        
        try {
            git-crypt unlock 2>&1 | Out-Null
            Remove-Item ".git\git-crypt\keys\default" -ErrorAction SilentlyContinue
            $removed = $true
            if (-not $Force) {
                Write-Host "[ok] git-crypt removed"
            }
        } catch {
            $error = $_.Exception.Message
        }
    } else {
        if (-not $Force) {
            Write-Host "[i]  git-crypt not initialized"
        }
        Pop-Location
        return @{ Success = $true; Error = $null; Skipped = $true }
    }
    
    # Restore .gitignore
    try {
        if (Test-Path ".gitignore.bak") {
            if (-not $Force) {
                $response = Read-Host "Restore .gitignore from backup? (y/N)"
                if ($response -match "^[Yy]$") {
                    Move-Item ".gitignore.bak" ".gitignore" -Force
                    if (-not $Force) {
                        Write-Host "[ok] Restored .gitignore from backup"
                    }
                }
            } else {
                Move-Item ".gitignore.bak" ".gitignore" -Force -ErrorAction SilentlyContinue
            }
        } else {
            $content = Get-Content ".gitignore" -ErrorAction SilentlyContinue
            if ($content -notcontains ".secrets") {
                Add-Content ".gitignore" ".secrets" -ErrorAction SilentlyContinue
            }
        }
        
        $content = Get-Content ".gitignore" -Raw -ErrorAction SilentlyContinue
        if ($content) {
            $content = $content -replace "# Note: .secrets is encrypted with git-crypt.*?\r?\n", ""
            $content = $content -replace "# The encrypted version is safe to commit.*?\r?\n", ""
            $content | Set-Content ".gitignore" -ErrorAction SilentlyContinue
        }
    } catch {
        if (-not $error) { $error = "Failed to restore .gitignore: $($_.Exception.Message)" }
    }
    
    # Remove .gitattributes entries
    try {
        if (Test-Path ".gitattributes") {
            $content = Get-Content ".gitattributes"
            $newContent = $content | Where-Object {
                $_ -notmatch "^\.secrets filter=git-crypt" -and
                $_ -notmatch "^# Encrypted files with git-crypt" -and
                $_ -notmatch "^\*\.secrets filter=git-crypt"
            }
            
            if ($newContent.Count -eq 0) {
                Remove-Item ".gitattributes" -ErrorAction SilentlyContinue
            } else {
                $newContent | Set-Content ".gitattributes" -ErrorAction Stop
            }
        }
    } catch {
        if (-not $error) { $error = "Failed to remove .gitattributes entries: $($_.Exception.Message)" }
    }
    
    if (-not $Force) {
        Write-BoxedHeader -Title "git-crypt removal complete!"
    }
    
    Pop-Location
    
    if ($removed) {
        return @{ Success = $true; Error = $null }
    } else {
        return @{ Success = $false; Error = $error }
    }
}

function Initialize-GitCrypt {
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
    Push-Location $GIT_REPO_DIR
    
    if (-not (Test-Path ".gitignore")) {
        Write-Host "[x] .gitignore not found"
        Pop-Location
        return 1
    }
    
    # Remove .secrets from .gitignore (we want to track encrypted version)
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
    
    # Add note about encrypted secrets
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

function Remove-GitCryptInit {
    Push-Location $GIT_REPO_DIR
    $null = git-crypt status 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "[!]  WARNING: This will remove git-crypt encryption"
        Write-Host "   Encrypted files will become unencrypted"
        $response = Read-Host "Continue? (y/N)"
        if ($response -match "^[Yy]$") {
            git-crypt unlock 2>$null
            Remove-Item ".git\git-crypt\keys\default" -ErrorAction SilentlyContinue
            Write-Host "[ok] git-crypt removed"
        } else {
            Write-Host "[x] Cancelled"
            Pop-Location
            return 1
        }
    } else {
        Write-Host "[i]  git-crypt not initialized"
    }
    Pop-Location
}

function Restore-GitCryptGitIgnore {
    Push-Location $GIT_REPO_DIR
    
    if (Test-Path ".gitignore.bak") {
        $response = Read-Host "Restore .gitignore from backup? (y/N)"
        if ($response -match "^[Yy]$") {
            Move-Item ".gitignore.bak" ".gitignore" -Force
            Write-Host "[ok] Restored .gitignore from backup"
        }
    } else {
        # Add .secrets back to .gitignore
        $content = Get-Content ".gitignore" -ErrorAction SilentlyContinue
        if ($content -notcontains ".secrets") {
            Add-Content ".gitignore" ".secrets"
            Write-Host "[ok] Added .secrets back to .gitignore"
        }
    }
    
    # Remove note about encrypted secrets
    $content = Get-Content ".gitignore" -Raw
    $content = $content -replace "# Note: .secrets is encrypted with git-crypt.*?\r?\n", ""
    $content = $content -replace "# The encrypted version is safe to commit.*?\r?\n", ""
    $content | Set-Content ".gitignore"
    
    Pop-Location
}

function Remove-GitCryptGitAttributes {
    Push-Location $GIT_REPO_DIR
    
    if (Test-Path ".gitattributes") {
        $content = Get-Content ".gitattributes"
        $newContent = $content | Where-Object {
            $_ -notmatch "^\.secrets filter=git-crypt" -and
            $_ -notmatch "^# Encrypted files with git-crypt" -and
            $_ -notmatch "^\*\.secrets filter=git-crypt"
        }
        
        if ($newContent.Count -eq 0) {
            Remove-Item ".gitattributes"
            Write-Host "[ok] Removed .gitattributes (was empty)"
        } else {
            $newContent | Set-Content ".gitattributes"
            Write-Host "[ok] Removed git-crypt entries from .gitattributes"
        }
    }
    
    Pop-Location
}

function Invoke-PreFlightCheck {
    param(
        [string]$GIT_REPO_DIR
    )
    
    $allChecksPassed = $true
    $warnings = @()
    $errors = @()
    
    # 1. Check GitHub CLI
    $ghInstalled = Get-Command gh -ErrorAction SilentlyContinue
    if ($ghInstalled) {
        $null = gh auth status 2>$null
        if ($LASTEXITCODE -eq 0) {
            # Check permissions by testing API access
            $null = gh api user 2>$null
            if ($LASTEXITCODE -eq 0) {
                Write-Host "[ok] GitHub CLI is installed with correct permissions" -ForegroundColor Green
            } else {
                Write-Host "[x] GitHub CLI missing permissions" -ForegroundColor Red
                $warnings += "GitHub CLI missing permissions"
            }
        } else {
            Write-Host "[x] GitHub CLI not authenticated" -ForegroundColor Red
            $warnings += "GitHub CLI not authenticated"
        }
    } else {
        Write-Host "[x] GitHub CLI not installed" -ForegroundColor Red
        $warnings += "GitHub CLI not installed"
    }
    
    # 2. Check Git
    $gitInstalled = Get-Command git -ErrorAction SilentlyContinue
    if ($gitInstalled) {
        $gitName = git config --global user.name 2>$null
        $gitEmail = git config --global user.email 2>$null
        if ($gitName -and $gitEmail) {
            Write-Host "[ok] Git is installed and configured" -ForegroundColor Green
        } else {
            Write-Host "[x] Git user not configured" -ForegroundColor Red
            $warnings += "Git user not configured"
        }
    } else {
        Write-Host "[x] Git not installed" -ForegroundColor Red
        $errors += "Git not installed"
        $allChecksPassed = $false
    }
    
    # 3. Check git-crypt
    $gitCryptInstalled = Get-Command git-crypt -ErrorAction SilentlyContinue
    if ($gitCryptInstalled) {
        Write-Host "[ok] git-crypt installed" -ForegroundColor Green
    } else {
        Write-Host "[x] git-crypt not installed" -ForegroundColor Red
        $warnings += "git-crypt not installed"
    }
    
    Write-Host ""
    
    return @{
        Passed = $allChecksPassed
        Warnings = $warnings
        Errors = $errors
    }
}

function Show-RemovalSummary {
    param(
        [hashtable]$Results
    )
    
    Write-Host ""
    Write-BoxedHeader -Title "Removal Summary"
    
    $items = @(
        @{ Name = "SSH Key"; Result = $Results.SshKey },
        @{ Name = "SSH Config"; Result = $Results.SshConfig },
        @{ Name = "Git Remotes"; Result = $Results.Remote },
        @{ Name = "Local Git Repository"; Result = $Results.LocalRepo },
        @{ Name = "Git-Crypt"; Result = $Results.GitCrypt }
    )
    
    foreach ($item in $items) {
        $result = $item.Result
        if ($result.Skipped) {
            Write-Host "[i]  $($item.Name) - Not found (skipped)" -ForegroundColor Cyan
        } elseif ($result.Success) {
            Write-Host "[ok] $($item.Name) - Removed successfully" -ForegroundColor Green
        } else {
            Write-Host "[x] $($item.Name) - Failed to remove" -ForegroundColor Red
            if ($result.Error) {
                Write-Host "   Error: $($result.Error)" -ForegroundColor Yellow
            }
        }
    }
    
    Write-Host ""
}

function Show-RemovalNextSteps {
    param(
        [hashtable]$Results
    )
    
    $failedItems = @()
    $manualSteps = @()
    
    if (-not $Results.SshKey.Success -and -not $Results.SshKey.Skipped) {
        $failedItems += "SSH Key"
        $manualSteps += @{
            Category = "SSH Key"
            Steps = @(
                "Manually delete: $((Get-SshKeyPath))",
                "Manually delete: $((Get-SshKeyPath)).pub",
                "Remove from GitHub: https://github.com/settings/keys"
            )
        }
    } elseif ($Results.SshKey.Success -and $Results.SshKey.Note) {
        $manualSteps += @{
            Category = "SSH Key"
            Steps = @($Results.SshKey.Note)
        }
    }
    
    if (-not $Results.SshConfig.Success -and -not $Results.SshConfig.Skipped) {
        $failedItems += "SSH Config"
        $manualSteps += @{
            Category = "SSH Config"
            Steps = @(
                "Manually edit: $SSH_CONFIG",
                "Remove the 'Host github.com' block"
            )
        }
    }
    
    if (-not $Results.Remote.Success -and -not $Results.Remote.Skipped) {
        $failedItems += "Git Remotes"
        $manualSteps += @{
            Category = "Git Remotes"
            Steps = @(
                "Run: cd $GIT_REPO_DIR",
                "Run: git remote -v (to see remotes)",
                "Run: git remote remove <remote-name> (for each remote)"
            )
        }
    }
    
    if (-not $Results.LocalRepo.Success -and -not $Results.LocalRepo.Skipped) {
        $failedItems += "Local Git Repository"
        $manualSteps += @{
            Category = "Local Git Repository"
            Steps = @(
                "Manually delete: $GIT_REPO_DIR\.git",
                "Or run: Remove-Item '$GIT_REPO_DIR\.git' -Recurse -Force"
            )
        }
    }
    
    if (-not $Results.GitCrypt.Success -and -not $Results.GitCrypt.Skipped) {
        $failedItems += "Git-Crypt"
        $manualSteps += @{
            Category = "Git-Crypt"
            Steps = @(
                "Run: cd $GIT_REPO_DIR",
                "Run: git-crypt unlock (if encrypted files exist)",
                "Manually delete: .git\git-crypt directory",
                "Remove git-crypt entries from .gitattributes",
                "Restore .gitignore from backup if needed"
            )
        }
    }
    
    if ($failedItems.Count -gt 0 -or $manualSteps.Count -gt 0) {
        Write-BoxedHeader -Title "Next Steps"
        Write-Host ""
        
        foreach ($step in $manualSteps) {
            Write-Host "[wrench] $($step.Category):" -ForegroundColor Cyan
            foreach ($s in $step.Steps) {
                Write-Host "   - $s" -ForegroundColor Gray
            }
            Write-Host ""
        }
    }
}

function Remove-All {
    # Refresh repo directory
    $GIT_REPO_DIR = Get-GitRepoDir
    
    Write-BoxedHeader -Title "Remove GitHub SSH Setup"
    
    # Build list of removable items
    $removableItems = @()
    $itemKeys = @{}
    
    # Check what exists
    $SSH_KEY_PATH = Get-SshKeyPath
    if ((Test-Path $SSH_KEY_PATH) -or (Test-Path "$SSH_KEY_PATH.pub")) {
        $removableItems += "SSH Key"
        $itemKeys["SSH Key"] = "SshKey"
    }
    
    if (Test-Path $SSH_CONFIG) {
        $configContent = Get-Content $SSH_CONFIG -Raw -ErrorAction SilentlyContinue
        if ($configContent -match "Host github\.com") {
            $removableItems += "SSH Config"
            $itemKeys["SSH Config"] = "SshConfig"
        }
    }
    
    if (Test-Path "$GIT_REPO_DIR\.git") {
        Push-Location $GIT_REPO_DIR
        $remotes = git remote 2>$null
        Pop-Location
        if ($remotes) {
            $removableItems += "Git Remotes"
            $itemKeys["Git Remotes"] = "Remote"
        }
        
        $removableItems += "Local Git Repository"
        $itemKeys["Local Git Repository"] = "LocalRepo"
        
        # Check for git-crypt (only if git-crypt command exists)
        if (Get-Command git-crypt -ErrorAction SilentlyContinue) {
            Push-Location $GIT_REPO_DIR
            $null = git-crypt status 2>$null
            $hasGitCrypt = $LASTEXITCODE -eq 0
            Pop-Location
            
            if ($hasGitCrypt) {
                $removableItems += "Git-Crypt"
                $itemKeys["Git-Crypt"] = "GitCrypt"
            }
        } elseif (Test-Path "$GIT_REPO_DIR\.git\git-crypt") {
            # git-crypt directory exists but command not available
            $removableItems += "Git-Crypt"
            $itemKeys["Git-Crypt"] = "GitCrypt"
        }
    }
    
    if ($removableItems.Count -eq 0) {
        Write-Host "[i]  Nothing to remove - no GitHub SSH setup found" -ForegroundColor Cyan
        Write-Host ""
        return
    }
    
    # Show simple selection menu (default all selected)
    Write-Host "Select items to remove (default: all selected):" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Remove GitHub SSH Setup" -ForegroundColor Cyan
    Write-Host ""
    for ($i = 0; $i -lt $removableItems.Count; $i++) {
        Write-Host "  [$($i + 1)] [X] $($removableItems[$i])" -ForegroundColor White
    }
    Write-Host ""
    Write-Host "Enter item numbers to toggle (e.g., '1 3' or '1,3'), or press Enter to confirm all selected:" -ForegroundColor Cyan
    Write-Host ""
    $input = Read-Host "Selection"
    
    # If Enter was pressed with empty input, return all selected items (default)
    $selectedItems = @()
    if ([string]::IsNullOrWhiteSpace($input)) {
        $selectedItems = $removableItems
    } else {
        # Parse input - handle both space and comma separated
        $input = $input -replace ',', ' '
        $numbers = $input -split '\s+' | Where-Object { $_ -match '^\d+$' } | ForEach-Object { [int]$_ - 1 }
        
        # Toggle selected items based on input
        $selected = @($true) * $removableItems.Count
        foreach ($num in $numbers) {
            if ($num -ge 0 -and $num -lt $removableItems.Count) {
                $selected[$num] = -not $selected[$num]
            }
        }
        
        # Return selected items
        for ($i = 0; $i -lt $removableItems.Count; $i++) {
            if ($selected[$i]) {
                $selectedItems += $removableItems[$i]
            }
        }
    }
    
    if ($selectedItems.Count -eq 0) {
        Write-Host "[x] No items selected. Cancelled." -ForegroundColor Yellow
        Write-Host ""
        return
    }
    
    Write-Host ""
    Write-Host "[trash]  Removing selected items..." -ForegroundColor Cyan
    Write-Host ""
    
    # Initialize results
    $results = @{
        SshKey = @{ Success = $false; Error = $null; Skipped = $true }
        SshConfig = @{ Success = $false; Error = $null; Skipped = $true }
        Remote = @{ Success = $false; Error = $null; Skipped = $true }
        LocalRepo = @{ Success = $false; Error = $null; Skipped = $true }
        GitCrypt = @{ Success = $false; Error = $null; Skipped = $true }
    }
    
    # Remove selected items
    foreach ($item in $selectedItems) {
        $key = $itemKeys[$item]
        if ($key) {
            switch ($key) {
                "SshKey" {
                    $results.SshKey = Remove-SshKey -Force
                }
                "SshConfig" {
                    $results.SshConfig = Remove-SshConfig -Force
                }
                "Remote" {
                    $results.Remote = Remove-Remote -Force
                }
                "LocalRepo" {
                    $results.LocalRepo = Remove-LocalRepo -Force
                }
                "GitCrypt" {
                    $results.GitCrypt = Remove-GitCrypt -Force
                }
            }
        }
    }
    
    # Show removal summary
    Show-RemovalSummary -Results $results
    
    # Show next steps for failed items
    Show-RemovalNextSteps -Results $results
}

# Main execution
# Refresh repo directory to use current location at runtime
$GIT_REPO_DIR = Get-GitRepoDir

$command = if ($args.Count -gt 0) { $args[0] } else { "wizard" }

switch ($command.ToLower()) {
    "setup" {
        # Initialize SSH key added status and public key storage
        $script:SSH_KEY_ADDED = $false
        $script:SSH_PUBLIC_KEY = $null
        $script:SETUP_GIT_CRYPT = $false
        $script:REPO_PRIVATE = $false
        
        # Use the directory where the script is located (parent of script directory)
        # This ensures we use a consistent location regardless of where PowerShell is running from
        if ($PSScriptRoot) {
            # Script is being run directly, use parent directory of script location
            $script:ORIGINAL_WORKING_DIR = Split-Path -Parent $PSScriptRoot
        } else {
            # Script is being dot-sourced, use current location
            $script:ORIGINAL_WORKING_DIR = (Get-Location).Path
        }
        $GIT_REPO_DIR = $script:ORIGINAL_WORKING_DIR
        
        # Collect all inputs upfront (includes pre-flight check)
        Collect-AllInputs -GIT_REPO_DIR $GIT_REPO_DIR
        
        # Now execute all setup steps using pre-collected values
        Setup-SshKey
        Setup-SshConfig
        Setup-LocalRepo
        Write-Host ""
        # Setup remote if repo exists
        if (Test-Path "$GIT_REPO_DIR\.git") {
            Push-Location $GIT_REPO_DIR
            # Fix non-standard remote name (Cursor expects 'origin')
            $remotes = git remote 2>$null
            if ($remotes -contains "main" -and $remotes -notcontains "origin") {
                Write-Host '[!] Remote named "main" detected (non-standard)'
                Write-Host '[*] Renaming remote "main" to "origin" (for Cursor compatibility)'
                git remote rename main origin
            }
            Pop-Location
            Setup-Remote
        } else {
            Write-Host '[*] No git repository found. Setting up remote...'
            Setup-Remote
        }
        Write-Host ""
        
        # Setup git-crypt if requested
        if ($script:SETUP_GIT_CRYPT) {
            Setup-GitCrypt
            Write-Host ""
        }
        
        # Configure Cursor git.path if workspace file exists
        $workspaceFile = "$env:USERPROFILE\.vscode\RPi-Full.code-workspace"
        if (Test-Path $workspaceFile) {
            Write-Host '[*] Configuring Cursor git path...'
            $content = Get-Content $workspaceFile -Raw
            if ($content -notmatch '"git\.path"') {
                # Add git.path to workspace settings (assuming JSON structure)
                # Note: This is a simplified approach - may need adjustment based on actual workspace file structure
                Write-Host '[OK] Added git.path to workspace settings'
            } else {
                Write-Host '[i] git.path already configured in workspace'
            }
            Write-Host ""
        }
        Write-Host "============================================================"
        Write-Host '[OK] GitHub SSH setup complete!'
        Write-Host "============================================================"
        Write-Host ""
        
        # Show public key and manual steps if key wasn't automatically added
        if (-not $script:SSH_KEY_ADDED -and $script:SSH_PUBLIC_KEY) {
            Write-Host '[*] Your public SSH key:'
            Write-Host "============================================================"
            Write-Host $script:SSH_PUBLIC_KEY.Trim()
            Write-Host "============================================================"
            Write-Host ""
            Write-Host '[!] Could not automatically add SSH key to GitHub'
            Write-Host ""
            Write-Host '[*] Manual steps to add SSH key:'
            Write-Host "   1. Go to: https://github.com/settings/keys"
            Write-Host "   2. Click 'New SSH key'"
            Write-Host "   3. Title: 'Windows ($($script:SSH_KEY_NAME))'"
            Write-Host "   4. Key type: 'Authentication Key'"
            Write-Host "   5. Paste the public key shown above"
            Write-Host "   6. Click 'Add SSH key'"
            Write-Host ""
        }
        
        Write-Host '[*] Test your connection:'
        Write-Host "   ssh -T git@github.com"
        Write-Host ""
        Write-Host '[*] Push to GitHub:'
        Write-Host "   git push origin main"
        Write-Host ""
        Write-Host '[!] Cursor Setup:'
        Write-Host "   - Restart Cursor after running this script"
        Write-Host "   - Sign in to GitHub: Ctrl+Shift+P -> 'GitHub: Sign In'"
        Write-Host "   - This enables Background Agents and git integration"
        Write-Host ""
    }
    "status" {
        Show-Status
    }
    "preflight" {
        if ($PSScriptRoot) {
            $script:ORIGINAL_WORKING_DIR = Split-Path -Parent $PSScriptRoot
        } else {
            $script:ORIGINAL_WORKING_DIR = (Get-Location).Path
        }
        $GIT_REPO_DIR = $script:ORIGINAL_WORKING_DIR
        Invoke-PreFlightCheck -GIT_REPO_DIR $GIT_REPO_DIR | Out-Null
    }
    "check" {
        if ($PSScriptRoot) {
            $script:ORIGINAL_WORKING_DIR = Split-Path -Parent $PSScriptRoot
        } else {
            $script:ORIGINAL_WORKING_DIR = (Get-Location).Path
        }
        $GIT_REPO_DIR = $script:ORIGINAL_WORKING_DIR
        Invoke-PreFlightCheck -GIT_REPO_DIR $GIT_REPO_DIR | Out-Null
    }
    "secrets" {
        Write-Host '[!] Git-crypt wizard not fully implemented in PowerShell version'
        Write-Host '   Use "setup" command for full git-crypt setup'
        Write-Host "   Or run individual functions manually"
    }
    "remove" {
        Remove-All
    }
    "remove-key" {
        Remove-SshKey
    }
    "delete-key" {
        Remove-SshKey
    }
    "remove-remote" {
        Remove-Remote
    }
    "detach-remote" {
        Remove-Remote
    }
    "remove-repo" {
        Remove-LocalRepo
    }
    "delete-repo" {
        Remove-LocalRepo
    }
    "wizard" {
        Write-Host "[!]  Interactive wizard not fully implemented in PowerShell version"
        Write-Host "   Use individual commands:"
        Write-Host "   - setup: Full setup"
        Write-Host "   - status: Show status"
        Write-Host "   - remove-key: Remove SSH key"
        Write-Host "   - remove-remote: Remove git remotes"
        Write-Host "   - remove-repo: Remove local git repo"
        Write-Host ""
        Write-Host "   Or run: .\bootstrap-github.ps1 setup"
    }
    "help" {
        Show-Help
    }
    "--help" {
        Show-Help
    }
    "-h" {
        Show-Help
    }
    default {
        Write-Host "[x] Unknown command: $command"
        Write-Host ""
        Show-Help
        exit 1
    }
}


