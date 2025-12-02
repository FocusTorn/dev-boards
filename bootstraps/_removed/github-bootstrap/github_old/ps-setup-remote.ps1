# Setup Remote Git Repository
# Standalone script to configure git remote for GitHub

[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$GitHubUser,
    
    [Parameter(Mandatory = $false)]
    [string]$RepoName,
    
    [Parameter(Mandatory = $false)]
    [bool]$IsPrivate = $false,
    
    [Parameter(Mandatory = $false)]
    [bool]$RemoveRemotes = $false,
    
    [Parameter(Mandatory = $false)]
    [bool]$CreateRepo = $false,
    
    [Parameter(Mandatory = $false)]
    [bool]$RemoveExisting = $false
)

# Suppress all errors by default - sub-commands handle their own error display
$ErrorActionPreference = "SilentlyContinue"
$PSDefaultParameterValues['*:ErrorAction'] = 'SilentlyContinue'

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

function Get-GitRepoDir {
    $workingDir = (Get-Location).Path
    if ($env:GIT_REPO_DIR) {
        Write-Host "GIT_REPO_DIR environment variable is set to: $env:GIT_REPO_DIR" -ForegroundColor DarkGray
        Write-Host "Using current directory instead: $workingDir" -ForegroundColor DarkGray
        return $workingDir
    } else {
        return $workingDir
    }
}

function Test-RepoExists {
    param(
        [string]$GitHubUser,
        [string]$RepoName
    )
    
    $GIT_REPO_DIR = Get-GitRepoDir
    Push-Location $GIT_REPO_DIR
    
    # Try to check if repo exists via SSH with non-interactive options to avoid host key prompts
    $oldErrorAction = $ErrorActionPreference
    $ErrorActionPreference = "SilentlyContinue"
    
    # Use GIT_SSH_COMMAND to pass SSH options that suppress host key verification prompts
    # Also suppress permission denied errors (SSH not configured yet)
    $env:GIT_SSH_COMMAND = "ssh -o BatchMode=yes -o StrictHostKeyChecking=no -o ConnectTimeout=5 -o LogLevel=ERROR"
    
    # Redirect both stdout and stderr to suppress all output
    $tempOut = [System.IO.Path]::GetTempFileName()
    $tempErr = [System.IO.Path]::GetTempFileName()
    
    $process = Start-Process -FilePath "git" `
        -ArgumentList "ls-remote", "git@github.com:${GitHubUser}/${RepoName}.git" `
        -RedirectStandardOutput $tempOut `
        -RedirectStandardError $tempErr `
        -NoNewWindow `
        -Wait `
        -PassThru
    
    $exitCode = $process.ExitCode
    $errorOutput = Get-Content -Path $tempErr -Raw -ErrorAction SilentlyContinue
    
    Remove-Item -Path $tempOut -Force -ErrorAction SilentlyContinue
    Remove-Item -Path $tempErr -Force -ErrorAction SilentlyContinue
    Remove-Item Env:\GIT_SSH_COMMAND -ErrorAction SilentlyContinue
    
    $ErrorActionPreference = $oldErrorAction
    
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
    
    # Other errors (like SSH connection issues, host key verification, etc.) - assume repo doesn't exist
    # The orchestrator should handle SSH setup first, but we'll be graceful here
    # Don't throw - just return false so setup can continue
    return $false
}

function Remove-RepoFromGitHub {
    param(
        [string]$GitHubUser,
        [string]$RepoName
    )
    
    if (-not $GitHubUser -or -not $RepoName) {
        return $false
    }
    
    # Try GitHub CLI first
    if (Get-Command gh -ErrorAction SilentlyContinue) {
        $null = gh auth status 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "Removing repository from GitHub..." -ForegroundColor DarkGray
            Write-Host "  Repository: ${GitHubUser}/${RepoName}" -ForegroundColor DarkGray
            Write-Host ""
            
            $tempOut = [System.IO.Path]::GetTempFileName()
            $tempErr = [System.IO.Path]::GetTempFileName()
            
            try {
                $process = Start-Process -FilePath "gh" `
                    -ArgumentList "repo", "delete", "${GitHubUser}/${RepoName}", "--yes" `
                    -RedirectStandardOutput $tempOut `
                    -RedirectStandardError $tempErr `
                    -NoNewWindow `
                    -Wait `
                    -PassThru
                
                $exitCode = $process.ExitCode
                $errorOutput = Get-Content -Path $tempErr -Raw -ErrorAction SilentlyContinue
                
                Remove-Item -Path $tempOut -Force -ErrorAction SilentlyContinue
                Remove-Item -Path $tempErr -Force -ErrorAction SilentlyContinue
                
                if ($exitCode -eq 0) {
                    Write-Host "  ✓ Repository removed from GitHub" -ForegroundColor DarkGreen
                    return $true
                } else {
                    # Check if repo doesn't exist (that's fine)
                    if ($errorOutput -match "not found" -or $errorOutput -match "does not exist") {
                        Write-Host "  Repository does not exist (already removed)" -ForegroundColor DarkGray
                        return $true
                    }
                    Write-Host "  Failed to remove repository" -ForegroundColor Yellow
                    return $false
                }
            } catch {
                Write-Host "  Failed to remove repository: $_" -ForegroundColor Yellow
                return $false
            }
        }
    }
    
    # Try GitHub API as fallback
    $githubToken = $env:GITHUB_TOKEN
    if (-not $githubToken -and (Test-Path "$env:USERPROFILE\.secrets")) {
        $secretsContent = Get-Content "$env:USERPROFILE\.secrets" -ErrorAction SilentlyContinue
        $tokenLine = $secretsContent | Select-String -Pattern "^GITHUB_TOKEN="
        if ($tokenLine) {
            $githubToken = ($tokenLine -split "=")[1] -replace '"', '' -replace "'", ''
        }
    }
    
    if ($githubToken) {
        Write-Host "Removing repository from GitHub via API..." -ForegroundColor DarkGray
        Write-Host "  Repository: ${GitHubUser}/${RepoName}" -ForegroundColor DarkGray
        Write-Host ""
        
        $headers = @{
            Accept = "application/vnd.github.v3+json"
            Authorization = "token $githubToken"
        }
        
        try {
            $response = Invoke-RestMethod -Uri "https://api.github.com/repos/${GitHubUser}/${RepoName}" -Method Delete -Headers $headers -ErrorAction Stop
            Write-Host "  ✓ Repository removed from GitHub" -ForegroundColor DarkGreen
            return $true
        } catch {
            if ($_.Exception.Response.StatusCode -eq 404) {
                Write-Host "  Repository does not exist (already removed)" -ForegroundColor DarkGray
                return $true
            }
            Write-Host "  Failed to remove repository: $_" -ForegroundColor Yellow
            return $false
        }
    }
    
    Write-Host "  Cannot remove repository: GitHub CLI not authenticated and no API token found" -ForegroundColor Yellow
    return $false
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
    
    # Check auth status more reliably
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
    
    if ($authStatus -ne 0) {
        return $false
    }
    
    $GIT_REPO_DIR = Get-GitRepoDir
    Push-Location $GIT_REPO_DIR
    
    if ($IsPrivate) {
        $output = gh repo create $RepoName --private --source=. --remote=origin 2>&1 | Out-String
    } else {
        $output = gh repo create $RepoName --public --source=. --remote=origin 2>&1 | Out-String
    }
    $result = $LASTEXITCODE -eq 0
    
    if (-not $result -and $output -match "already exists|Name already exists") {
        Write-Host "  Repository already exists - will use existing repository" -ForegroundColor Cyan
        $result = $true
    }
    
    if ($result) {
        $oldErrorAction = $ErrorActionPreference
        $ErrorActionPreference = "SilentlyContinue"
        $null = git rev-parse HEAD 2>$null
        $hasCommits = $LASTEXITCODE -eq 0
        $ErrorActionPreference = $oldErrorAction
        if ($hasCommits) {
            $null = git push -u origin main 2>$null
            if ($LASTEXITCODE -ne 0) {
                Write-Host "  Push failed (you can push manually later)" -ForegroundColor Yellow
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

function Build-WizardSteps { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$DefaultGitHubUser,
        
        [Parameter(Mandatory = $true)]
        [string]$DefaultRepoName,
        
        [Parameter(Mandatory = $true)]
        [bool]$HasExistingRemotes
    )
    
    $steps = @()
    
    # Step 1: GitHub username
    if ($DefaultGitHubUser) {
        $steps += New-WizardStep `
            -Type "input" `
            -Title "GitHub username" `
            -Key "github_user" `
            -Placeholder $DefaultGitHubUser `
            -Default $DefaultGitHubUser `
            -Description "Detected from Git config"
    } else {
        $steps += New-WizardStep `
            -Type "input" `
            -Title "GitHub username" `
            -Key "github_user" `
            -Placeholder "username" `
            -Description "Enter your GitHub username"
    }
    
    # Step 2: Repository name
    $steps += New-WizardStep `
        -Type "input" `
        -Title "Repository name" `
        -Key "repo_name" `
        -Placeholder $DefaultRepoName `
        -Default $DefaultRepoName `
        -Description "Default: current directory name"
    
    # Step 3: Repository visibility
    $steps += New-WizardStep `
        -Type "confirm" `
        -Title "Make repository private?" `
        -Key "repo_private" `
        -Description "Private repositories are only visible to you and collaborators" `
        -Default "no"
    
    # Step 4: Remove existing remotes (if any)
    if ($HasExistingRemotes) {
        $steps += New-WizardStep `
            -Type "confirm" `
            -Title "Remove existing remotes?" `
            -Key "remove_remotes" `
            -Description "Remove existing remotes and set up new one" `
            -Default "yes"
    }
    
    return $steps
} #<

function Setup-Remote { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $false)]
        [string]$GitHubUser,
        
        [Parameter(Mandatory = $false)]
        [string]$RepoName,
        
        [Parameter(Mandatory = $false)]
        [bool]$IsPrivate = $false,
        
        [Parameter(Mandatory = $false)]
        [bool]$RemoveRemotes = $false,
        
        [Parameter(Mandatory = $false)]
        [bool]$CreateRepo = $false
    )
    
    $GIT_REPO_DIR = Get-GitRepoDir
    Push-Location $GIT_REPO_DIR
    
    # Use defaults if not provided
    if (-not $GitHubUser) {
        $GitHubUser = git config --global user.name 2>$null
        if ($GitHubUser) {
            $GitHubUser = $GitHubUser.Trim()
        }
    }
    
    if (-not $RepoName) {
        $RepoName = Split-Path -Path $GIT_REPO_DIR -Leaf
    }
    
    if (-not $GitHubUser -or -not $RepoName) {
        Write-Error "GitHub username and repository name are required" -ErrorAction Stop
        Pop-Location
        return
    }
    
    # Check existing remotes
    $remotes = git remote 2>$null
    $hasExistingRemotes = $remotes -and $remotes.Count -gt 0
    
    # Remove remotes if requested
    if ($hasExistingRemotes -and $RemoveRemotes) {
        foreach ($remote in $remotes) {
            git remote remove $remote
        }
        Write-Host "Removed existing remotes" -ForegroundColor Cyan
        Write-Host ""
    } elseif ($hasExistingRemotes) {
        Write-Host "Keeping existing remotes" -ForegroundColor Cyan
        Pop-Location
        return
    }
    
    # Skip repository existence check to avoid SSH errors
    # Proceed directly with setup - repository will be created if requested, or user can create manually
    $repoExists = $false
    
    # Remove existing repository if requested
    if ($RemoveExisting -and $GitHubUser -and $RepoName) {
        Remove-RepoFromGitHub -GitHubUser $GitHubUser -RepoName $RepoName
        Write-Host ""
    }
    
    # Try to create repository if requested
    if ($CreateRepo) {
            $isPrivate = $IsPrivate
                
                # Check GitHub CLI availability
                $ghAvailable = $false
                if (Get-Command gh -ErrorAction SilentlyContinue) {
                    # Check if authenticated
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
                    
                    if ($authStatus -eq 0) {
                        $ghAvailable = $true
                    } else {
                        Write-Host "  GitHub CLI not authenticated" -ForegroundColor DarkGray
                        Write-Host "  Running: gh auth login" -ForegroundColor DarkGray
                        Write-Host ""
                        gh auth login
                        if ($LASTEXITCODE -eq 0) {
                            Write-Host "    ✓ GitHub CLI authenticated" -ForegroundColor DarkGreen
                            $ghAvailable = $true
                        } else {
                            Write-Host "    GitHub CLI authentication failed or cancelled" -ForegroundColor Yellow
                        }
                    }
                } else {
                    Write-Host "  GitHub CLI not installed" -ForegroundColor DarkGray
                    Write-Host ""
                    
                    # Try to install using winget
                    if (Get-Command winget -ErrorAction SilentlyContinue) {
                        Write-Host "  Would you like to install GitHub CLI using winget?" -ForegroundColor Cyan
                        $installChoice = Read-Host "  Install GitHub CLI? (Y/n)"
                        if ($installChoice -match "^[Yy]|^$") {
                            Write-Host "  Installing GitHub CLI..." -ForegroundColor DarkGray
                            winget install --id GitHub.cli --accept-package-agreements --accept-source-agreements
                            if ($LASTEXITCODE -eq 0) {
                                Write-Host "    ✓ GitHub CLI installed successfully" -ForegroundColor DarkGreen
                                Write-Host "    Please restart your terminal and run this script again" -ForegroundColor Yellow
                                Write-Host ""
                                return
                            } else {
                                Write-Host "    Failed to install GitHub CLI via winget" -ForegroundColor Yellow
                            }
                        }
                    }
                    
                    Write-Host "  GitHub CLI is required for automatic repository creation" -ForegroundColor DarkGray
                    Write-Host "  Download from: https://cli.github.com/" -ForegroundColor DarkGray
                    Write-Host "  Or install using: winget install GitHub.cli" -ForegroundColor DarkGray
                    Write-Host ""
                }
                
                Write-Host "  Creating repository on GitHub..." -ForegroundColor DarkGray
                Write-Host "    Repository: ${GitHubUser}/${RepoName}" -ForegroundColor DarkGray
                Write-Host "    Visibility: $(if ($isPrivate) { 'Private' } else { 'Public' })" -ForegroundColor DarkGray
                Write-Host ""
                
                if ($ghAvailable) {
                    if (New-RepoWithGh $GitHubUser $RepoName $isPrivate) {
                        Write-Host "    ✓ Repository ready (created or already exists)" -ForegroundColor DarkGreen
                        $currentBranch = git branch --show-current 2>$null
                        if (-not $currentBranch) { $currentBranch = "main" }
                        # Only set upstream if there's at least one commit
                        $oldErrorAction = $ErrorActionPreference
                        $ErrorActionPreference = "SilentlyContinue"
                        $null = git rev-parse HEAD 2>$null
                        $hasCommits = $LASTEXITCODE -eq 0
                        $ErrorActionPreference = $oldErrorAction
                        if ($hasCommits) {
                            git branch --set-upstream-to=origin/$currentBranch 2>$null
                        }
                        Pop-Location
                        Write-Host ""
                        Write-Host "  ✓ Remote Repository Setup Completed" -ForegroundColor Green
                        Write-Host ""
                        return
                    } else {
                        Write-Host "    Failed to create repository using GitHub CLI" -ForegroundColor Yellow
                        Write-Host ""
                    }
                }
                
                # GitHub CLI failed or not available, try API as fallback
                $githubToken = $env:GITHUB_TOKEN
                if (-not $githubToken -and (Test-Path "$env:USERPROFILE\.secrets")) {
                    $secretsContent = Get-Content "$env:USERPROFILE\.secrets" -ErrorAction SilentlyContinue
                    $tokenLine = $secretsContent | Select-String -Pattern "^GITHUB_TOKEN="
                    if ($tokenLine) {
                        $githubToken = ($tokenLine -split "=")[1] -replace '"', '' -replace "'", ''
                    }
                }
                
                if ($githubToken) {
                    Write-Host "  Trying GitHub API..." -ForegroundColor DarkGray
                    if (New-RepoWithApi $GitHubUser $RepoName $isPrivate $githubToken) {
                        Write-Host "    ✓ Repository created using GitHub API" -ForegroundColor DarkGreen
                        $repoExists = $true
                    } else {
                        Write-Host "    Failed to create repository via API" -ForegroundColor Yellow
                        Write-Host "    Repository may already exist or API token is invalid" -ForegroundColor DarkGray
                    }
                } else {
                    Write-Host "  Cannot create repository automatically (no GitHub CLI or API token)" -ForegroundColor Yellow
                }
    } else {
        # CreateRepo is false, but we should still try to verify if it exists
        Write-Host "Repository creation not requested. Will set up remote only." -ForegroundColor DarkGray
        Write-Host ""
    }
    
    # Add remote if it doesn't exist
    # Note: git remote add/set-url don't actually connect to the remote, so they won't trigger SSH errors
    # But we suppress errors anyway to be safe
    $existingRemotes = git remote 2>$null
    if ($existingRemotes -notcontains "origin") {
        $remoteUrl = "git@github.com:${GitHubUser}/${RepoName}.git"
        # Use Start-Process to completely suppress output
        $tempOut = [System.IO.Path]::GetTempFileName()
        $tempErr = [System.IO.Path]::GetTempFileName()
        $process = Start-Process -FilePath "git" `
            -ArgumentList "remote", "add", "origin", $remoteUrl `
            -RedirectStandardOutput $tempOut `
            -RedirectStandardError $tempErr `
            -NoNewWindow `
            -Wait `
            -PassThru
        Remove-Item -Path $tempOut -Force -ErrorAction SilentlyContinue
        Remove-Item -Path $tempErr -Force -ErrorAction SilentlyContinue
        Write-Host "    ✓ Added remote 'origin': $remoteUrl" -ForegroundColor DarkGreen
    } else {
        $currentUrl = git remote get-url origin 2>$null
        $newUrl = "git@github.com:${GitHubUser}/${RepoName}.git"
        if ($currentUrl -ne $newUrl) {
            # Use Start-Process to completely suppress output
            $tempOut = [System.IO.Path]::GetTempFileName()
            $tempErr = [System.IO.Path]::GetTempFileName()
            $process = Start-Process -FilePath "git" `
                -ArgumentList "remote", "set-url", "origin", $newUrl `
                -RedirectStandardOutput $tempOut `
                -RedirectStandardError $tempErr `
                -NoNewWindow `
                -Wait `
                -PassThru
            Remove-Item -Path $tempOut -Force -ErrorAction SilentlyContinue
            Remove-Item -Path $tempErr -Force -ErrorAction SilentlyContinue
            Write-Host "    ✓ Updated remote 'origin': $newUrl" -ForegroundColor DarkGreen
        }
    }
    
    # Set upstream branch (only if there's at least one commit)
    $currentBranch = git branch --show-current 2>$null
    if (-not $currentBranch) { $currentBranch = "main" }
    
    # Check if there's at least one commit before setting upstream
    $oldErrorAction = $ErrorActionPreference
    $ErrorActionPreference = "SilentlyContinue"
    $null = git rev-parse HEAD 2>$null
    $hasCommits = $LASTEXITCODE -eq 0
    $ErrorActionPreference = $oldErrorAction
    if ($hasCommits) {
        Write-Host "Setting upstream branch to origin/$currentBranch" -ForegroundColor Cyan
        git branch --set-upstream-to=origin/$currentBranch 2>$null
        if ($LASTEXITCODE -ne 0) {
            Write-Host "  Upstream will be set on first push" -ForegroundColor Yellow
        }
    } else {
        Write-Host "  Upstream will be set on first push (no commits yet)" -ForegroundColor DarkGray
        Write-Host ""
    }
    
    Pop-Location
    Write-Host ""
    Write-Host "  ✓ Remote Repository Setup Completed" -ForegroundColor Green
    Write-Host ""
} #<

# Main execution - only run if called directly (not when dot-sourced)
if ($MyInvocation.InvocationName -ne '.') {
    try {
        Setup-Remote -GitHubUser $GitHubUser -RepoName $RepoName -IsPrivate $IsPrivate -RemoveRemotes $RemoveRemotes -CreateRepo $CreateRepo
    } catch {
        Write-Host "Error: $_" -ForegroundColor Red
        exit 1
    }
}
