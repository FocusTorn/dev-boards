# VGA Display Diagnostic and Fix Script
# This script helps diagnose and fix VGA port issues

Write-Host "=== VGA Display Diagnostic ===" -ForegroundColor Cyan
Write-Host ""

# Check all monitors
Write-Host "Detected Monitors:" -ForegroundColor Yellow
$monitors = Get-PnpDevice -Class Monitor
foreach ($monitor in $monitors) {
    $statusColor = if ($monitor.Status -eq "OK") { "Green" } else { "Red" }
    Write-Host "  $($monitor.FriendlyName) - Status: " -NoNewline
    Write-Host $monitor.Status -ForegroundColor $statusColor
    if ($monitor.InstanceId -like "*PLL2410*") {
        Write-Host "    ^ This is your VGA monitor (PLL2410W)" -ForegroundColor Yellow
    }
}

Write-Host "`nActive Displays:" -ForegroundColor Yellow
Add-Type -AssemblyName System.Windows.Forms
$screens = [System.Windows.Forms.Screen]::AllScreens
foreach ($screen in $screens) {
    Write-Host "  $($screen.DeviceName): $($screen.Bounds.Width)x$($screen.Bounds.Height)"
}

Write-Host "`nGraphics Adapters:" -ForegroundColor Yellow
Get-CimInstance -ClassName Win32_VideoController | ForEach-Object {
    Write-Host "  $($_.Name): $($_.VideoModeDescription)"
}

Write-Host "`n=== Attempting to Detect VGA Monitor ===" -ForegroundColor Cyan
Write-Host "Scanning for hardware changes..."

# Try to rescan for monitors
try {
    $null = Start-Process "rundll32.exe" -ArgumentList "setupapi.dll,InstallHinfSection DefaultInstall 132 $env:WINDIR\inf\monitor.inf" -Wait -WindowStyle Hidden -ErrorAction SilentlyContinue
    Write-Host "Hardware scan initiated" -ForegroundColor Green
} catch {
    Write-Host "Could not initiate hardware scan: $_" -ForegroundColor Red
}

Write-Host "`n=== Recommendations ===" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. MANUALLY DETECT DISPLAY:" -ForegroundColor Cyan
Write-Host "   - Press Windows Key + P"
Write-Host "   - Select 'Extend' or 'Second screen only'"
Write-Host "   - Or: Right-click desktop > Display settings > Multiple displays > Detect"
Write-Host ""
Write-Host "2. CHECK VGA CABLE CONNECTION:" -ForegroundColor Cyan
Write-Host "   - Ensure VGA cable is firmly connected to both laptop and monitor"
Write-Host "   - Try unplugging and replugging the VGA cable"
Write-Host "   - Check if monitor needs to be powered on"
Write-Host ""
Write-Host "3. CHECK BIOS/GRAPHICS SETTINGS:" -ForegroundColor Cyan
Write-Host "   - Restart and enter BIOS/UEFI settings"
Write-Host "   - Look for 'Graphics Configuration', 'Optimus', or 'Switchable Graphics'"
Write-Host "   - Try switching between Optimus and Discrete Graphics mode"
Write-Host ""
Write-Host "4. UPDATE GRAPHICS DRIVERS:" -ForegroundColor Cyan
Write-Host "   - Update NVIDIA drivers from: https://www.nvidia.com/drivers"
Write-Host "   - Update Intel Graphics drivers from Device Manager"
Write-Host ""
Write-Host "5. TO USE NVIDIA FOR HDMI (render through NVIDIA):" -ForegroundColor Cyan
Write-Host "   - Settings > System > Display > Graphics Settings"
Write-Host "   - Add your apps and select 'High performance' (NVIDIA)"
Write-Host "   - Or use NVIDIA Control Panel > Manage 3D Settings"

Write-Host "`nPress any key to open Display Settings..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
Start-Process "ms-settings:display"

