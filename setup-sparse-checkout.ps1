# PowerShell script to set up Git sparse checkout for multiple workspaces
# Usage: .\setup-sparse-checkout.ps1

param(
    [Parameter(Mandatory=$true)]
    [string]$RemoteRepoUrl,
    
    [Parameter(Mandatory=$true)]
    [string]$WorkspaceRoot,
    
    [Parameter(Mandatory=$true)]
    [string]$LocalFolderName,
    
    [Parameter(Mandatory=$true)]
    [string[]]$FoldersToSync,
    
    [Parameter(Mandatory=$false)]
    [string]$Branch = "main",
    
    [Parameter(Mandatory=$false)]
    [hashtable]$FolderMapping = @{}  # e.g., @{"RR-Folder1" = "L1-Folder1"}
)

Write-Host "Setting up sparse checkout..." -ForegroundColor Green
Write-Host "Remote Repo: $RemoteRepoUrl" -ForegroundColor Cyan
Write-Host "Workspace: $WorkspaceRoot" -ForegroundColor Cyan
Write-Host "Local Folder: $LocalFolderName" -ForegroundColor Cyan
Write-Host "Folders to sync: $($FoldersToSync -join ', ')" -ForegroundColor Cyan

# Create workspace directory if it doesn't exist
if (-not (Test-Path $WorkspaceRoot)) {
    New-Item -ItemType Directory -Path $WorkspaceRoot -Force | Out-Null
    Write-Host "Created workspace directory: $WorkspaceRoot" -ForegroundColor Yellow
}

$LocalPath = Join-Path $WorkspaceRoot $LocalFolderName

# Check if directory already exists
if (Test-Path $LocalPath) {
    $response = Read-Host "Directory $LocalPath already exists. Remove and re-clone? (y/N)"
    if ($response -eq 'y' -or $response -eq 'Y') {
        Remove-Item -Path $LocalPath -Recurse -Force
        Write-Host "Removed existing directory" -ForegroundColor Yellow
    } else {
        Write-Host "Aborted." -ForegroundColor Red
        exit 1
    }
}

# Change to workspace directory
Push-Location $WorkspaceRoot

try {
    # Clone without checking out
    Write-Host "`nCloning repository (no checkout)..." -ForegroundColor Green
    git clone --no-checkout $RemoteRepoUrl $LocalFolderName
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to clone repository" -ForegroundColor Red
        exit 1
    }
    
    # Change to cloned directory
    Push-Location $LocalFolderName
    
    try {
        # Enable sparse checkout (cone mode)
        Write-Host "`nEnabling sparse checkout..." -ForegroundColor Green
        git sparse-checkout init --cone
        
        if ($LASTEXITCODE -ne 0) {
            Write-Host "Failed to initialize sparse checkout" -ForegroundColor Red
            exit 1
        }
        
        # Set folders to check out
        Write-Host "`nConfiguring folders to sync..." -ForegroundColor Green
        git sparse-checkout set $FoldersToSync
        
        if ($LASTEXITCODE -ne 0) {
            Write-Host "Failed to set sparse checkout folders" -ForegroundColor Red
            exit 1
        }
        
        # Check out the branch
        Write-Host "`nChecking out branch: $Branch..." -ForegroundColor Green
        git checkout $Branch
        
        if ($LASTEXITCODE -ne 0) {
            Write-Host "Failed to checkout branch" -ForegroundColor Red
            exit 1
        }
        
        # Rename folders if mapping provided
        if ($FolderMapping.Count -gt 0) {
            Write-Host "`nRenaming folders according to mapping..." -ForegroundColor Green
            foreach ($key in $FolderMapping.Keys) {
                $source = $key
                $target = $FolderMapping[$key]
                
                if (Test-Path $source) {
                    if (Test-Path $target) {
                        Write-Host "  Warning: $target already exists, skipping $source" -ForegroundColor Yellow
                    } else {
                        Move-Item -Path $source -Destination $target
                        Write-Host "  Renamed: $source -> $target" -ForegroundColor Cyan
                    }
                } else {
                    Write-Host "  Warning: $source not found, skipping" -ForegroundColor Yellow
                }
            }
        }
        
        Write-Host "`n✓ Sparse checkout setup complete!" -ForegroundColor Green
        Write-Host "`nCurrent sparse checkout paths:" -ForegroundColor Cyan
        git sparse-checkout list
        
    } finally {
        Pop-Location
    }
    
} finally {
    Pop-Location
}

Write-Host "`nSetup complete. Navigate to $LocalPath to work with the synced folders." -ForegroundColor Green

