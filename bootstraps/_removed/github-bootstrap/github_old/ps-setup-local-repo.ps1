
[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$WorkingDirectory,
    
    [Parameter(Mandatory = $false)]
    [bool]$Recreate = $false,
    
    [Parameter(Mandatory = $false)]
    [string]$GitName,
    
    [Parameter(Mandatory = $false)]
    [string]$GitEmail
)

$ErrorActionPreference = "Stop"

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

function Get-GitConfig { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$ConfigKey,
        
        [Parameter(Mandatory = $false)]
        [switch]$Global,
        
        [Parameter(Mandatory = $true)]
        [string]$WorkingDirectory
    )
    
    Push-Location $WorkingDirectory
    try {
        $scope = if ($Global) { "--global" } else { "--local" }
        $value = git config $scope $ConfigKey 2>$null
        return $value
    } finally {
        Pop-Location
    }
} #<

function Initialize-GitRepository { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$RepositoryPath,
        
        [Parameter(Mandatory = $false)]
        [string]$UserName,
        
        [Parameter(Mandatory = $false)]
        [string]$UserEmail
    )
    
    # Ensure directory exists
    if (-not (Test-Path $RepositoryPath)) {
        New-Item -ItemType Directory -Path $RepositoryPath -Force | Out-Null
    }
    
    Push-Location $RepositoryPath
    
    try {
        # Initialize repository
        Write-Host ""
        Write-Host "  Initializing Git repository:" -ForegroundColor DarkGray
        git init | Out-Null
        
        if ($LASTEXITCODE -ne 0) {
            throw "Failed to initialize Git repository"
        }
        
        # Set default branch
        git branch -M main 2>$null
        if ($LASTEXITCODE -ne 0) {
            git checkout -b main 2>$null | Out-Null
        }
        
        # Get existing config for comparison (before setting)
        $localName = Get-GitConfig -ConfigKey "user.name" -WorkingDirectory $RepositoryPath
        $localEmail = Get-GitConfig -ConfigKey "user.email" -WorkingDirectory $RepositoryPath
        $globalName = Get-GitConfig -ConfigKey "user.name" -Global -WorkingDirectory $RepositoryPath
        $globalEmail = Get-GitConfig -ConfigKey "user.email" -Global -WorkingDirectory $RepositoryPath
        
        # Configure user name
        $nameToSet = $null
        if ($UserName) {
            $nameToSet = $UserName
            if (-not $globalName) {
                git config --global user.name $UserName
            }
        } elseif ($globalName) {
            $nameToSet = $globalName
        }
        
        # Show local path (always)
        Write-Host "    ✓ Set local path:     $RepositoryPath" -ForegroundColor DarkGreen
        
        if ($nameToSet) {
            # Check if local config exists and matches (before setting)
            $wasAlreadySet = $localName -and $localName -eq $nameToSet
            git config user.name $nameToSet
            if ($wasAlreadySet) {
                Write-Host "    ✓ Set Git user name:  $nameToSet" -ForegroundColor DarkGreen
            } else {
                Write-Host "    ✓ Set Git user name:  $nameToSet" -ForegroundColor DarkGreen
            }
        }
        
        # Configure user email
        $emailToSet = $null
        if ($UserEmail) {
            $emailToSet = $UserEmail
            if (-not $globalEmail) {
                git config --global user.email $UserEmail
            }
        } elseif ($globalEmail) {
            $emailToSet = $globalEmail
        }
        
        if ($emailToSet) {
            # Check if local config exists and matches (before setting)
            $wasAlreadySet = $localEmail -and $localEmail -eq $emailToSet
            git config user.email $emailToSet
            if ($wasAlreadySet) {
                Write-Host "    ✓ Set Git user email: $emailToSet" -ForegroundColor DarkGreen
            } else {
                Write-Host "    ✓ Set Git user email: $emailToSet" -ForegroundColor DarkGreen
            }
        }
        
        Write-Host ""
        Write-Host "  ✓  Git repository initialized successfully" -ForegroundColor Green
        Write-Host ""
        
    } finally {
        Pop-Location
    }
} #<

function Setup-LocalRepository { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$WorkingDirectory,
        
        [Parameter(Mandatory = $false)]
        [bool]$Recreate = $false,
        
        [Parameter(Mandatory = $false)]
        [string]$GitName,
        
        [Parameter(Mandatory = $false)]
        [string]$GitEmail
    )
    
    # Ensure working directory exists
    if (-not (Test-Path $WorkingDirectory)) {
        Write-Error "Working directory does not exist: $WorkingDirectory" -ErrorAction Stop
        return
    }
    
    $gitRepoPath = Join-Path $WorkingDirectory ".git"
    $repositoryExists = Test-Path $gitRepoPath
    
    # Handle existing repository
    if ($repositoryExists) {
        if ($Recreate) {
            Remove-Item $gitRepoPath -Recurse -Force -ErrorAction SilentlyContinue
            Write-Host "  Removed existing repository" -ForegroundColor Cyan
        } else {
            Write-Host "Keeping existing repository" -ForegroundColor Cyan
            Write-Host ""
            return
        }
    }
    
    # Initialize repository
    Initialize-GitRepository `
        -RepositoryPath $WorkingDirectory `
        -UserName $GitName `
        -UserEmail $GitEmail
} #<

# Main execution - only run if called directly (not when dot-sourced)
if ($MyInvocation.InvocationName -ne '.') {
    try {
        $workingDir = if ($WorkingDirectory) { $WorkingDirectory } else { (Get-Location).Path }
        Setup-LocalRepository -WorkingDirectory $workingDir -Recreate $Recreate -GitName $GitName -GitEmail $GitEmail
    } catch {
        Write-Host "Error: $_" -ForegroundColor Red
        exit 1
    }
}