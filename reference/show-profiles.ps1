Write-Host "=== PowerShell Core (pwsh) Profiles ===" -ForegroundColor Cyan
'AllUsersAllHosts','AllUsersCurrentHost','CurrentUserAllHosts','CurrentUserCurrentHost' | ForEach-Object {
    Write-Host "$_`: $($PROFILE.$_)" -ForegroundColor Yellow
}
Write-Host ""
Write-Host "=== Windows PowerShell Profiles ===" -ForegroundColor Cyan
powershell -NoProfile -Command 'AllUsersAllHosts,AllUsersCurrentHost,CurrentUserAllHosts,CurrentUserCurrentHost | ForEach-Object { Write-Host "$_`: $($PROFILE.$_)" -ForegroundColor Yellow }'

