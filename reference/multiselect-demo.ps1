# Multiselect Demo Script
# Demonstrates all three PowerShell menu modules with multiselect capability

$menuOptions = @("SSH Key", "SSH Config", "Git Remotes", "Local Git Repository", "Git-Crypt")
$title = "Select items to remove (all selected by default)"

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "Multiselect Menu Module Demo" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

# Check for installed modules and import them proactively
Write-Host "Checking for installed menu modules..." -ForegroundColor Gray

# Check and import PSMenu
$psMenuModule = Get-Module -ListAvailable -Name PSMenu -ErrorAction SilentlyContinue
if ($psMenuModule) {
    Write-Host "  - PSMenu module found (version $($psMenuModule.Version))" -ForegroundColor Green
    if (-not (Get-Module -Name PSMenu -ErrorAction SilentlyContinue)) {
        Import-Module PSMenu -Force -ErrorAction SilentlyContinue
        Write-Host "    Imported PSMenu module" -ForegroundColor Gray
    }
} else {
    Write-Host "  - PSMenu module not found" -ForegroundColor Yellow
}

# Check and import InteractiveMenu
$interactiveMenuModule = Get-Module -ListAvailable -Name InteractiveMenu -ErrorAction SilentlyContinue
if ($interactiveMenuModule) {
    Write-Host "  - InteractiveMenu module found (version $($interactiveMenuModule.Version))" -ForegroundColor Green
    if (-not (Get-Module -Name InteractiveMenu -ErrorAction SilentlyContinue)) {
        Import-Module InteractiveMenu -Force -ErrorAction SilentlyContinue
        Write-Host "    Imported InteractiveMenu module" -ForegroundColor Gray
    }
} else {
    Write-Host "  - InteractiveMenu module not found" -ForegroundColor Yellow
}

# Check and import ps-menu
$psMenuModuleModule = Get-Module -ListAvailable -Name ps-menu -ErrorAction SilentlyContinue
if ($psMenuModuleModule) {
    Write-Host "  - ps-menu module found (version $($psMenuModuleModule.Version))" -ForegroundColor Green
    if (-not (Get-Module -Name ps-menu -ErrorAction SilentlyContinue)) {
        Import-Module ps-menu -Force -ErrorAction SilentlyContinue
        Write-Host "    Imported ps-menu module" -ForegroundColor Gray
    }
} else {
    Write-Host "  - ps-menu module not found" -ForegroundColor Yellow
}

Write-Host ""

# Build initial selection (all selected by default)
$initialSelection = @(0, 1, 2, 3, 4)

# ============================================================================
# Demo 1: PSMenu (Sebazzz/PSMenu)
# https://github.com/Sebazzz/PSMenu
# ============================================================================
Write-Host "============================================================" -ForegroundColor Yellow
Write-Host "Demo 1: PSMenu (Show-Menu -MultiSelect)" -ForegroundColor Yellow
Write-Host "============================================================" -ForegroundColor Yellow
Write-Host ""

# Check if PSMenu is available
$psMenuAvailable = $false
$cmdCheck = Get-Command Show-Menu -ErrorAction SilentlyContinue
if ($cmdCheck) {
    $psMenuAvailable = $true
    Write-Host "  PSMenu command found." -ForegroundColor Green
} else {
    # Try to import if not already imported
    $moduleLoaded = Get-Module -Name PSMenu -ErrorAction SilentlyContinue
    if (-not $moduleLoaded) {
        $moduleInstalled = Get-Module -ListAvailable -Name PSMenu -ErrorAction SilentlyContinue
        if ($moduleInstalled) {
            try {
                Import-Module PSMenu -ErrorAction Stop -Force
                $cmdCheck = Get-Command Show-Menu -ErrorAction SilentlyContinue
                if ($cmdCheck) {
                    $psMenuAvailable = $true
                    Write-Host "  PSMenu imported and command found." -ForegroundColor Green
                } else {
                    Write-Host "  Warning: PSMenu module imported but command not found." -ForegroundColor Yellow
                }
            } catch {
                Write-Host "  Warning: PSMenu module found but failed to import: $($_.Exception.Message)" -ForegroundColor Yellow
            }
        } else {
            # Module not installed - offer to install
            Write-Host "  PSMenu module is not installed." -ForegroundColor Yellow
            $install = Read-Host "  Would you like to install it now? (Y/N)"
            if ($install -eq 'Y' -or $install -eq 'y') {
                try {
                    Write-Host "  Installing PSMenu..." -ForegroundColor Gray
                    Install-Module -Name PSMenu -Scope CurrentUser -Force -AllowClobber
                    Import-Module PSMenu -Force
                    $cmdCheck = Get-Command Show-Menu -ErrorAction SilentlyContinue
                    if ($cmdCheck) {
                        $psMenuAvailable = $true
                        Write-Host "  PSMenu installed and imported successfully!" -ForegroundColor Green
                    }
                } catch {
                    Write-Host "  Failed to install PSMenu: $($_.Exception.Message)" -ForegroundColor Red
                }
            }
        }
    } else {
        Write-Host "  Warning: PSMenu module is loaded but command not found." -ForegroundColor Yellow
    }
}

if ($psMenuAvailable) {
    Write-Host "PSMenu is available. Press any key to try it..." -ForegroundColor Green
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
    Write-Host ""
    
    try {
        $selected = Show-Menu -MenuItems $menuOptions -MultiSelect -InitialSelection $initialSelection
        Write-Host ""
        Write-Host "Selected items:" -ForegroundColor Green
        $selected | ForEach-Object { Write-Host "  [X] $_" -ForegroundColor Green }
    } catch {
        Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
    }
} else {
    Write-Host "PSMenu is not available." -ForegroundColor Red
    Write-Host "Install with: Install-Module -Name PSMenu" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Press any key to continue to next demo..." -ForegroundColor Cyan
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
Write-Host ""

# ============================================================================
# Demo 2: PowerShell Interactive Menu (bibistroc/powershell-interactive-menu)
# https://github.com/bibistroc/powershell-interactive-menu
# ============================================================================
Write-Host "============================================================" -ForegroundColor Yellow
Write-Host "Demo 2: PowerShell Interactive Menu" -ForegroundColor Yellow
Write-Host "============================================================" -ForegroundColor Yellow
Write-Host ""

# Check if InteractiveMenu is available
$hasInteractiveMenu = $false
$cmdCheck1 = Get-Command Get-InteractiveMenuUserSelection -ErrorAction SilentlyContinue
$cmdCheck2 = Get-Command Get-InteractiveMultiMenuOption -ErrorAction SilentlyContinue
if ($cmdCheck1 -and $cmdCheck2) {
    $hasInteractiveMenu = $true
    Write-Host "  InteractiveMenu commands found." -ForegroundColor Green
} else {
    # Try to import if not already imported
    $moduleLoaded = Get-Module -Name InteractiveMenu -ErrorAction SilentlyContinue
    if (-not $moduleLoaded) {
        $moduleInstalled = Get-Module -ListAvailable -Name InteractiveMenu -ErrorAction SilentlyContinue
        if ($moduleInstalled) {
            try {
                Import-Module InteractiveMenu -ErrorAction Stop -Force
                $cmdCheck1 = Get-Command Get-InteractiveMenuUserSelection -ErrorAction SilentlyContinue
                $cmdCheck2 = Get-Command Get-InteractiveMultiMenuOption -ErrorAction SilentlyContinue
                if ($cmdCheck1 -and $cmdCheck2) {
                    $hasInteractiveMenu = $true
                    Write-Host "  InteractiveMenu imported and commands found." -ForegroundColor Green
                } else {
                    Write-Host "  Warning: InteractiveMenu module imported but commands not found." -ForegroundColor Yellow
                }
            } catch {
                Write-Host "  Warning: InteractiveMenu module found but failed to import: $($_.Exception.Message)" -ForegroundColor Yellow
            }
        } else {
            # Module not installed - offer to install
            Write-Host "  InteractiveMenu module is not installed." -ForegroundColor Yellow
            $install = Read-Host "  Would you like to install it now? (Y/N)"
            if ($install -eq 'Y' -or $install -eq 'y') {
                try {
                    Write-Host "  Installing InteractiveMenu..." -ForegroundColor Gray
                    Install-Module -Name InteractiveMenu -Scope CurrentUser -Force -AllowClobber
                    Import-Module InteractiveMenu -Force
                    $cmdCheck1 = Get-Command Get-InteractiveMenuUserSelection -ErrorAction SilentlyContinue
                    $cmdCheck2 = Get-Command Get-InteractiveMultiMenuOption -ErrorAction SilentlyContinue
                    if ($cmdCheck1 -and $cmdCheck2) {
                        $hasInteractiveMenu = $true
                        Write-Host "  InteractiveMenu installed and imported successfully!" -ForegroundColor Green
                    }
                } catch {
                    Write-Host "  Failed to install InteractiveMenu: $($_.Exception.Message)" -ForegroundColor Red
                }
            }
        }
    } else {
        Write-Host "  Warning: InteractiveMenu module is loaded but commands not found." -ForegroundColor Yellow
    }
}

if ($hasInteractiveMenu) {
    Write-Host "PowerShell Interactive Menu is available. Press any key to try it..." -ForegroundColor Green
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
    Write-Host ""
    
    try {
        # Build menu items with all selected by default
        $menuItems = @()
        for ($i = 0; $i -lt $menuOptions.Count; $i++) {
            $params = @{
                Item = $menuOptions[$i]
                Label = $menuOptions[$i]
                Order = $i
            }
            # All items selected by default
            $params['Selected'] = $true
            $menuItems += Get-InteractiveMultiMenuOption @params
        }
        
        $selectedOptions = Get-InteractiveMenuUserSelection -Header $title -Items $menuItems
        Write-Host ""
        Write-Host "Selected items:" -ForegroundColor Green
        $selectedOptions | ForEach-Object { Write-Host "  [X] $_" -ForegroundColor Green }
    } catch {
        Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
    }
} else {
    Write-Host "PowerShell Interactive Menu is not available." -ForegroundColor Red
    Write-Host "Install with: Install-Module -Name InteractiveMenu" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Press any key to continue to next demo..." -ForegroundColor Cyan
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
Write-Host ""

# ============================================================================
# Demo 3: ps-menu (chrisseroka/ps-menu)
# https://github.com/chrisseroka/ps-menu
# ============================================================================
Write-Host "============================================================" -ForegroundColor Yellow
Write-Host "Demo 3: ps-menu (Menu -Multiselect)" -ForegroundColor Yellow
Write-Host "============================================================" -ForegroundColor Yellow
Write-Host ""

# Check if ps-menu is available
$psMenuModuleAvailable = $false
$cmdCheck = Get-Command Menu -ErrorAction SilentlyContinue
if ($cmdCheck) {
    $psMenuModuleAvailable = $true
    Write-Host "  ps-menu command found." -ForegroundColor Green
} else {
    # Try to import if not already imported
    $moduleLoaded = Get-Module -Name ps-menu -ErrorAction SilentlyContinue
    if (-not $moduleLoaded) {
        $moduleInstalled = Get-Module -ListAvailable -Name ps-menu -ErrorAction SilentlyContinue
        if ($moduleInstalled) {
            try {
                Import-Module ps-menu -ErrorAction Stop -Force
                $cmdCheck = Get-Command Menu -ErrorAction SilentlyContinue
                if ($cmdCheck) {
                    $psMenuModuleAvailable = $true
                    Write-Host "  ps-menu imported and command found." -ForegroundColor Green
                } else {
                    Write-Host "  Warning: ps-menu module imported but command not found." -ForegroundColor Yellow
                }
            } catch {
                Write-Host "  Warning: ps-menu module found but failed to import: $($_.Exception.Message)" -ForegroundColor Yellow
            }
        } else {
            # Module not installed - offer to install
            Write-Host "  ps-menu module is not installed." -ForegroundColor Yellow
            $install = Read-Host "  Would you like to install it now? (Y/N)"
            if ($install -eq 'Y' -or $install -eq 'y') {
                try {
                    Write-Host "  Installing ps-menu..." -ForegroundColor Gray
                    Install-Module -Name ps-menu -Scope CurrentUser -Force -AllowClobber
                    Import-Module ps-menu -Force
                    $cmdCheck = Get-Command Menu -ErrorAction SilentlyContinue
                    if ($cmdCheck) {
                        $psMenuModuleAvailable = $true
                        Write-Host "  ps-menu installed and imported successfully!" -ForegroundColor Green
                    }
                } catch {
                    Write-Host "  Failed to install ps-menu: $($_.Exception.Message)" -ForegroundColor Red
                }
            }
        }
    } else {
        Write-Host "  Warning: ps-menu module is loaded but command not found." -ForegroundColor Yellow
    }
}

if ($psMenuModuleAvailable) {
    Write-Host "ps-menu is available. Press any key to try it..." -ForegroundColor Green
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
    Write-Host ""
    
    try {
        # ps-menu uses -MenuItems (PascalCase) and may support -Title
        $selected = Menu -MenuItems $menuOptions -Multiselect -Title $title
        Write-Host ""
        Write-Host "Selected items:" -ForegroundColor Green
        $selected | ForEach-Object { Write-Host "  [X] $_" -ForegroundColor Green }
    } catch {
        Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
        Write-Host "Trying without Title parameter..." -ForegroundColor Yellow
        try {
            $selected = Menu -MenuItems $menuOptions -Multiselect
            Write-Host ""
            Write-Host "Selected items:" -ForegroundColor Green
            $selected | ForEach-Object { Write-Host "  [X] $_" -ForegroundColor Green }
        } catch {
            Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
        }
    }
} else {
    Write-Host "ps-menu is not available." -ForegroundColor Red
    Write-Host "Install with: Install-Module -Name ps-menu" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "Demo Complete!" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""
