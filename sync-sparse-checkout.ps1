# Script to sync/pull changes from remote repository
# Usage: .\sync-sparse-checkout.ps1 -LocalPath "WS1-Root\local1"

param(
    [Parameter(Mandatory=$true)]
    [string]$LocalPath,
    
    [Parameter(Mandatory=$false)]
    [string]$Branch = "main"
)

if (-not (Test-Path $LocalPath)) {
    Write-Host "Error: Path $LocalPath does not exist" -ForegroundColor Red
    exit 1
}

if (-not (Test-Path (Join-Path $LocalPath ".git"))) {
    Write-Host "Error: $LocalPath is not a Git repository" -ForegroundColor Red
    exit 1
}

Push-Location $LocalPath

try {
    Write-Host "Syncing from remote repository..." -ForegroundColor Green
    Write-Host "Current sparse checkout paths:" -ForegroundColor Cyan
    git sparse-checkout list
    
    Write-Host "`nFetching latest changes..." -ForegroundColor Green
    git fetch origin
    
    Write-Host "`nPulling changes from $Branch..." -ForegroundColor Green
    git pull origin $Branch
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "`n✓ Sync complete!" -ForegroundColor Green
    } else {
        Write-Host "`n✗ Sync failed. Check for conflicts." -ForegroundColor Red
    }
    
} finally {
    Pop-Location
}

