# One-liner to show PowerShell profile paths for both pwsh and powershell
# Copy and paste this entire line:

Write-Host "=== PowerShell Core (pwsh) Profiles ===" -ForegroundColor Cyan; 'AllUsersAllHosts','AllUsersCurrentHost','CurrentUserAllHosts','CurrentUserCurrentHost' | ForEach-Object { Write-Host "$_`: $($PROFILE.$_)" -ForegroundColor Yellow }; Write-Host ""; Write-Host "=== Windows PowerShell Profiles ===" -ForegroundColor Cyan; $psCmd = '@(''AllUsersAllHosts'',''AllUsersCurrentHost'',''CurrentUserAllHosts'',''CurrentUserCurrentHost'') | ForEach-Object { $t = $_; $p = $PROFILE.$_; Write-Host ([string]::Format(''{0}: {1}'', $t, $p)) -ForegroundColor Yellow }'; powershell -NoProfile -Command $psCmd
