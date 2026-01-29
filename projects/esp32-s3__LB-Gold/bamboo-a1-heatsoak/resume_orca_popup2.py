#!/usr/bin/env python3
"""
Orca Slicer Error Popup Handler (Optimized v2)
Uses pywin32 for high-performance window and button detection.

Finds and activates the Orca Slicer error popup when it contains the pause message,
then positions the mouse over the "Resume Printing" button.

Dependencies:
    pip install pywin32 pyautogui
"""

import sys
import time
import re
from typing import Optional, List, Tuple

try:
    import win32gui
    import win32con
    import win32api
    import win32process
    PYWIN32_AVAILABLE = True
except ImportError:
    PYWIN32_AVAILABLE = False
    print("ERROR: pywin32 not installed. Install with: uv pip install pywin32")
    sys.exit(1)

try:
    import pyautogui
    PYAUTOGUI_AVAILABLE = True
except ImportError:
    PYAUTOGUI_AVAILABLE = False
    print("ERROR: pyautogui not installed. Install with: uv pip install pyautogui")
    sys.exit(1)

# Expected text that must be present in the popup
REQUIRED_TEXT = "Printing paused due to the pause command added to the printing file."
BUTTON_TEXT = "Resume Printing"
WINDOW_TITLE = "Error"
WINDOW_CLASS = "wxWindowNR"
WINDOW_EXE = "orca-slicer.exe"

def get_window_text(hwnd: int) -> str:
    """Get the text of a window or control."""
    return win32gui.GetWindowText(hwnd)

def get_window_class(hwnd: int) -> str:
    """Get the class name of a window or control."""
    return win32gui.GetClassName(hwnd)

def find_orca_error_window() -> Optional[int]:
    """Find the Orca Slicer error popup window handle."""
    found_hwnd = None

    def enum_windows_callback(hwnd, _):
        nonlocal found_hwnd
        if win32gui.IsWindowVisible(hwnd):
            title = get_window_text(hwnd)
            cls = get_window_class(hwnd)
            
            if title == WINDOW_TITLE and cls == WINDOW_CLASS:
                # We'll use the title and class as primary matchers
                # We can't easily verify the process name without psutil, 
                # but wxWindowNR + "Error" title is very specific to this popup.
                found_hwnd = hwnd
                return False # Stop enumerating
        return True

    win32gui.EnumWindows(enum_windows_callback, None)
    return found_hwnd

def get_all_child_info(hwnd: int) -> List[Tuple[int, str, str]]:
    """Get a list of all child window handles, their text, and classes."""
    children = []

    def enum_child_callback(child_hwnd, _):
        children.append((
            child_hwnd,
            get_window_text(child_hwnd),
            get_window_class(child_hwnd)
        ))
        return True

    try:
        win32gui.EnumChildWindows(hwnd, enum_child_callback, None)
    except Exception:
        pass
    return children

def main() -> int:
    print(f"Searching for Orca Slicer error popup (Optimized)...")
    
    start_time = time.perf_counter()
    hwnd = find_orca_error_window()
    
    if not hwnd:
        print("NOT FOUND: Orca Slicer error popup window is not open.")
        return 1

    print(f"FOUND: Window handle {hwnd} detected.")
    
    # Get all children once for text verification and button finding
    children = get_all_child_info(hwnd)
    
    # 1. Verify Text
    # Collect all text from window and children
    text_parts = [get_window_text(hwnd)]
    text_parts.extend([text for _, text, _ in children if text])
    
    combined_text = " ".join(text_parts)
    normalized_combined = re.sub(r'\s+', ' ', combined_text)
    
    if REQUIRED_TEXT not in normalized_combined:
        # Try a more fuzzy match in case of newline differences
        normalized_required = re.sub(r'\s+', ' ', REQUIRED_TEXT)
        if normalized_required not in normalized_combined:
            print("VERIFICATION FAILED: Popup does not contain the required text.")
            print(f"DEBUG: Found text: {normalized_combined[:200]}...")
            return 1
    
    print("VERIFIED: Popup contains the required text.")

    # 2. Find Button
    button_hwnd = None
    for c_hwnd, c_text, c_cls in children:
        if c_text == BUTTON_TEXT:
            button_hwnd = c_hwnd
            break
    
    if not button_hwnd:
        # Fallback partial match or case-insensitive
        for c_hwnd, c_text, c_cls in children:
            if BUTTON_TEXT.lower() in c_text.lower():
                button_hwnd = c_hwnd
                break

    if not button_hwnd:
        print(f"ERROR: Could not locate the '{BUTTON_TEXT}' button.")
        # List buttons we DID find for debugging
        button_classes = ["Button", "wxWindowClassNR"]
        found_buttons = [t for _, t, cl in children if any(bc in cl for bc in button_classes)]
        if found_buttons:
            print(f"DEBUG: Found other buttons: {found_buttons}")
        return 1

    # 3. Close the popup
    try:
        # Bring the main window to front (optional, but helps verify it's the right one)
        win32gui.ShowWindow(hwnd, win32con.SW_RESTORE)
        win32gui.SetForegroundWindow(hwnd)
        
        # Position Mouse (Commented out as requested)
        # rect = win32gui.GetWindowRect(button_hwnd)
        # center_x = (rect[0] + rect[2]) // 2
        # center_y = (rect[1] + rect[3]) // 2
        # pyautogui.moveTo(center_x, center_y, duration=0.1)
        
        # Close the window directly
        win32gui.PostMessage(hwnd, win32con.WM_CLOSE, 0, 0)
        
        end_time = time.perf_counter()
        print(f"SUCCESS: Popup window (handle {hwnd}) closed.")
        print(f"Total processing time: {(end_time - start_time)*1000:.2f}ms")
        return 0
    except Exception as e:
        print(f"ERROR during window closure: {e}")
        return 1

if __name__ == '__main__':
    sys.exit(main())
