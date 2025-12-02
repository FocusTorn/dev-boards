# Fix VGA Refresh Rate - Change NVIDIA output from 119Hz to 60Hz
# This script changes the refresh rate on DISPLAY2 (VGA port) to 60Hz

Write-Host "=== Fixing VGA Refresh Rate ===" -ForegroundColor Cyan
Write-Host ""

# Get current display info
Write-Host "Current Display Configuration:" -ForegroundColor Yellow
$nvidia = Get-CimInstance -ClassName Win32_VideoController | Where-Object {$_.Name -like "*NVIDIA*"}
Write-Host "  NVIDIA GPU: $($nvidia.VideoModeDescription) @ $($nvidia.CurrentRefreshRate) Hz" -ForegroundColor Red
Write-Host "  (This is too high for your VGA monitor!)" -ForegroundColor Red
Write-Host ""

# Create a C# class to change display settings using Win32 API
$changeDisplayCode = @"
using System;
using System.Runtime.InteropServices;

public class DisplaySettings {
    [DllImport("user32.dll")]
    public static extern int EnumDisplaySettings(string deviceName, int modeNum, ref DEVMODE devMode);
    
    [DllImport("user32.dll")]
    public static extern int ChangeDisplaySettings(ref DEVMODE devMode, int flags);
    
    public const int ENUM_CURRENT_SETTINGS = -1;
    public const int CDS_UPDATEREGISTRY = 0x01;
    public const int CDS_TEST = 0x02;
    public const int DISP_CHANGE_SUCCESSFUL = 0;
    public const int DISP_CHANGE_RESTART = 1;
    
    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Auto)]
    public struct DEVMODE {
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 32)]
        public string dmDeviceName;
        public short dmSpecVersion;
        public short dmDriverVersion;
        public short dmSize;
        public short dmDriverExtra;
        public int dmFields;
        public int dmPositionX;
        public int dmPositionY;
        public int dmDisplayOrientation;
        public int dmDisplayFixedOutput;
        public short dmColor;
        public short dmDuplex;
        public short dmYResolution;
        public short dmTTOption;
        public short dmCollate;
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 32)]
        public string dmFormName;
        public short dmLogPixels;
        public int dmBitsPerPel;
        public int dmPelsWidth;
        public int dmPelsHeight;
        public int dmDisplayFlags;
        public int dmDisplayFrequency;
        public int dmICMMethod;
        public int dmICMIntent;
        public int dmMediaType;
        public int dmDitherType;
        public int dmReserved1;
        public int dmReserved2;
        public int dmPanningWidth;
        public int dmPanningHeight;
    }
    
    public static bool SetRefreshRate(string deviceName, int width, int height, int refreshRate) {
        DEVMODE dm = new DEVMODE();
        dm.dmSize = (short)Marshal.SizeOf(typeof(DEVMODE));
        
        if (EnumDisplaySettings(deviceName, ENUM_CURRENT_SETTINGS, ref dm) != 0) {
            dm.dmFields = 0x80000 | 0x400000 | 0x1000000; // DM_PELSWIDTH | DM_PELSHEIGHT | DM_DISPLAYFREQUENCY
            dm.dmPelsWidth = width;
            dm.dmPelsHeight = height;
            dm.dmDisplayFrequency = refreshRate;
            
            int result = ChangeDisplaySettings(ref dm, CDS_TEST);
            if (result == DISP_CHANGE_SUCCESSFUL) {
                result = ChangeDisplaySettings(ref dm, CDS_UPDATEREGISTRY);
                return result == DISP_CHANGE_SUCCESSFUL || result == DISP_CHANGE_RESTART;
            }
        }
        return false;
    }
}
"@

try {
    Add-Type -TypeDefinition $changeDisplayCode -Language CSharp
    Write-Host "Successfully loaded display change functions" -ForegroundColor Green
    
    # Target: DISPLAY2 (VGA port) - Change from 119Hz to 60Hz
    # Resolution: 1024x768 @ 60Hz
    Write-Host "`nAttempting to change DISPLAY2 (VGA) to 60Hz..." -ForegroundColor Yellow
    
    $success = [DisplaySettings]::SetRefreshRate("\\.\DISPLAY2", 1024, 768, 60)
    
    if ($success) {
        Write-Host "✓ Successfully changed refresh rate to 60Hz!" -ForegroundColor Green
        Write-Host "  The VGA monitor should now work properly." -ForegroundColor Green
        Write-Host "`nIf the change doesn't appear, try:" -ForegroundColor Yellow
        Write-Host "  1. Disconnect and reconnect the VGA cable" -ForegroundColor Yellow
        Write-Host "  2. Power cycle the monitor" -ForegroundColor Yellow
        Write-Host "  3. Restart your computer" -ForegroundColor Yellow
    } else {
        Write-Host "✗ Failed to change refresh rate programmatically" -ForegroundColor Red
        Write-Host "`nPlease try manually:" -ForegroundColor Yellow
        Write-Host "  1. Right-click desktop > Display settings" -ForegroundColor Yellow
        Write-Host "  2. Click on Display 2" -ForegroundColor Yellow
        Write-Host "  3. Scroll down and click 'Advanced display settings'" -ForegroundColor Yellow
        Write-Host "  4. Click 'Display adapter properties for Display 2'" -ForegroundColor Yellow
        Write-Host "  5. Click 'List All Modes'" -ForegroundColor Yellow
        Write-Host "  6. Select '1024 by 768, True Color (32 bit), 60 hertz'" -ForegroundColor Yellow
        Write-Host "  7. Click OK and Apply" -ForegroundColor Yellow
    }
} catch {
    Write-Host "Error: $_" -ForegroundColor Red
    Write-Host "`nFalling back to manual instructions..." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "MANUAL FIX INSTRUCTIONS:" -ForegroundColor Cyan
    Write-Host "1. Right-click on desktop > Display settings" -ForegroundColor White
    Write-Host "2. Click on the second display (Display 2)" -ForegroundColor White
    Write-Host "3. Scroll down and click 'Advanced display settings'" -ForegroundColor White
    Write-Host "4. Click 'Display adapter properties for Display 2'" -ForegroundColor White
    Write-Host "5. Go to 'Monitor' tab" -ForegroundColor White
    Write-Host "6. Change 'Screen refresh rate' from 119 Hz to 60 Hz" -ForegroundColor White
    Write-Host "7. Click Apply and OK" -ForegroundColor White
}

Write-Host "`nPress any key to open Display Settings..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
Start-Process "ms-settings:display"

