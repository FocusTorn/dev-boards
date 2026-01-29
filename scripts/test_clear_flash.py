import win32gui
import win32con
import win32api
import time
import ctypes
from ctypes import wintypes

# Define FLASHWINFO structure for FlashWindowEx
class FLASHWINFO(ctypes.Structure):
    _fields_ = [
        ('cbSize', wintypes.UINT),
        ('hwnd', wintypes.HWND),
        ('dwFlags', wintypes.DWORD),
        ('uCount', wintypes.UINT),
        ('dwTimeout', wintypes.DWORD)
    ]

FLASHW_STOP = 0
FLASHW_CAPTION = 1
FLASHW_TRAY = 2
FLASHW_ALL = 3
FLASHW_TIMER = 4
FLASHW_TIMERNOFG = 12

def find_orca():
    found = []
    def cb(h, _):
        if "OrcaSlicer" in win32gui.GetWindowText(h) and win32gui.GetClassName(h) == "wxWindowNR":
            found.append(h)
        return True
    win32gui.EnumWindows(cb, None)
    return found[0] if found else None

def clear_flash_v1(hwnd):
    """Simple FlashWindow(hwnd, False)"""
    print("Trying v1: FlashWindow(hwnd, False)...")
    win32gui.FlashWindow(hwnd, False)

def clear_flash_v2(hwnd):
    """FlashWindowEx with FLASHW_STOP"""
    print("Trying v2: FlashWindowEx with FLASHW_STOP...")
    info = FLASHWINFO(
        cbSize=ctypes.sizeof(FLASHWINFO),
        hwnd=hwnd,
        dwFlags=FLASHW_STOP,
        uCount=0,
        dwTimeout=0
    )
    ctypes.windll.user32.FlashWindowEx(ctypes.byref(info))

def clear_flash_v3(hwnd):
    """SetForegroundWindow + Restore"""
    print("Trying v3: SetForegroundWindow (requires focus trick)...")
    try:
        # Alt-key trick
        ctypes.windll.user32.keybd_event(0x12, 0, 0, 0) # Alt down
        win32gui.SetForegroundWindow(hwnd)
        ctypes.windll.user32.keybd_event(0x12, 0, 2, 0) # Alt up
    except Exception as e:
        print(f"Error in v3: {e}")

def find_orca_popup():
    found = []
    def cb(h, _):
        if win32gui.IsWindowVisible(h) and win32gui.GetWindowText(h) == "Error" and win32gui.GetClassName(h) == "wxWindowNR":
            found.append(h)
        return True
    win32gui.EnumWindows(cb, None)
    return found[0] if found else None

def main():
    hwnd_main = find_orca()
    if not hwnd_main:
        print("Orca Slicer (main window) not found!")
        return
    print(f"Found Orca Main HWND: {hwnd_main}")

    print("\n1. Trigger the popup in Orca Slicer so it starts flashing.")
    input("Press Enter once the popup is visible and flashing...")

    hwnd_popup = find_orca_popup()
    if hwnd_popup:
        print(f"Found Popup HWND: {hwnd_popup}. Closing it now...")
        win32gui.PostMessage(hwnd_popup, win32con.WM_CLOSE, 0, 0)
        time.sleep(1)
        print("Popup closed.")
    else:
        print("Popup not found!")

    input("\n[Test v2] Press Enter to try FlashWindowEx(FLASHW_STOP)...")
    clear_flash_v2(hwnd_main)
    
    
    input("\n[Test v1] Press Enter to try FlashWindow(hwnd, False)...")
    clear_flash_v1(hwnd_main)
    
    
    # Save current focus for v3
    prev_hwnd = win32gui.GetForegroundWindow()
    input("\n[Test v3] Press Enter to try SetForegroundWindow (Focus Swap)...")
    clear_flash_v3(hwnd_main)
    time.sleep(1)
    if prev_hwnd:
        print("Restoring focus...")
        win32gui.SetForegroundWindow(prev_hwnd)

    print("\nTesting complete.")

if __name__ == "__main__":
    main()
