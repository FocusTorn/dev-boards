#!/usr/bin/env python3
"""
Orca Slicer Error Popup Handler
Finds and activates the Orca Slicer error popup when it contains the pause message,
then positions the mouse over the "Resume Printing" button.

Window Properties:
- Title: "Error"
- Class: wxWindowNR
- Executable: orca-slicer.exe
- Expected Text: "Printing paused due to the pause command added to the printing file."
- Button: "Resume Printing" (bg color: RGB(76, 76, 85))

Usage:
    python resume_orca_popup.py
    
Dependencies:
    pip install pywinauto pyautogui
"""

import sys
import re
import time
from typing import Optional

try:
    from pywinauto import Application, findwindows
    from pywinauto.findwindows import ElementNotFoundError
    PYWINAUTO_AVAILABLE = True
except ImportError:
    PYWINAUTO_AVAILABLE = False
    print("ERROR: pywinauto not installed. Install with: uv pip install pywinauto")
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

# Button text to locate
BUTTON_TEXT = "Resume Printing"

# Window properties
WINDOW_TITLE = "Error"
WINDOW_CLASS = "wxWindowNR"
WINDOW_EXE = "orca-slicer.exe"


def find_orca_error_window() -> Optional[Application]:
    """
    Find the Orca Slicer error popup window.
    
    Returns:
        Application instance if window found, None otherwise
    """
    try:
        # Strategy 1: Try to connect to orca-slicer process first, then find window
        # This is most reliable if orca-slicer is running
        try:
            app = Application(backend="win32").connect(path=WINDOW_EXE)
            try:
                # Try to find the window within this application
                window = app.window(title=WINDOW_TITLE, class_name=WINDOW_CLASS)
                # Access a property to verify it exists
                _ = window.window_text()
                return app
            except Exception:
                # Window not found in this application
                pass
        except Exception:
            # Could not connect to orca-slicer process
            pass
        
        # Strategy 2: Find by title and class name, then verify executable
        windows = findwindows.find_windows(
            title=WINDOW_TITLE,
            class_name=WINDOW_CLASS
        )
        
        if windows:
            # Try each window handle
            for handle in windows:
                try:
                    # Connect to the window
                    app = Application(backend="win32").connect(handle=handle)
                    
                    # Try to access the window to verify it exists
                    window = app.window(handle=handle)
                    window_text = window.window_text()
                    
                    # If we got here, the window is accessible
                    # We'll verify it's from orca-slicer during text verification
                    return app
                except Exception:
                    continue
        
        # Strategy 3: Find by title only (less specific)
        windows = findwindows.find_windows(title=WINDOW_TITLE)
        
        if windows:
            for handle in windows:
                try:
                    app = Application(backend="win32").connect(handle=handle)
                    window = app.window(handle=handle)
                    
                    # Check if class name matches
                    if WINDOW_CLASS.lower() in window.class_name().lower():
                        return app
                except Exception:
                    continue
        
        return None
        
    except ElementNotFoundError:
        return None
    except Exception as e:
        # Don't print full traceback unless debugging
        return None


def verify_popup_text(app: Application) -> bool:
    """
    Verify that the popup contains the required text.
    
    Args:
        app: Application instance connected to the window
        
    Returns:
        True if required text is found, False otherwise
    """
    try:
        # Get the window dialog - try multiple ways to find it
        try:
            window = app.window(title=WINDOW_TITLE, class_name=WINDOW_CLASS)
        except:
            # Try to get any window from the app
            windows = app.windows()
            if windows:
                window = windows[0]
            else:
                print("WARNING: Could not get window object")
                return False
        
        # Collect all text from all controls in the window
        all_text_parts = []
        
        # Get window text itself
        try:
            window_text = window.window_text()
            if window_text:
                all_text_parts.append(window_text)
        except:
            pass
        
        # Get text from all descendants (all child controls)
        try:
            all_controls = window.descendants()
            for control in all_controls:
                try:
                    control_text = control.window_text()
                    if control_text:
                        all_text_parts.append(control_text)
                except:
                    pass
        except:
            pass
        
        # Also try specific control types that might contain text
        for control_type in ["Text", "Static", "Edit", "Button"]:
            try:
                controls = window.descendants(control_type=control_type)
                for control in controls:
                    try:
                        control_text = control.window_text()
                        if control_text:
                            all_text_parts.append(control_text)
                    except:
                        pass
            except:
                pass
        
        # Combine all text and normalize whitespace (replace newlines/tabs with spaces)
        combined_text = " ".join(all_text_parts)
        # Normalize whitespace: replace all whitespace sequences with single space
        combined_text = re.sub(r'\s+', ' ', combined_text)
        
        # Also normalize the required text for comparison
        normalized_required = re.sub(r'\s+', ' ', REQUIRED_TEXT)
        
        # Debug output - show what text we found
        if normalized_required not in combined_text:
            print("DEBUG: Collected text from popup (first 200 chars):")
            print(f"  {combined_text[:200]}...")
            print()
            print("DEBUG: All text parts found:")
            for i, part in enumerate(all_text_parts[:10]):  # Limit to first 10 parts
                if part.strip():
                    print(f"  [{i}] {part[:100]}")
            print()
            print(f"DEBUG: Normalized required text: '{normalized_required}'")
            print(f"DEBUG: Normalized combined text contains required: {normalized_required in combined_text}")
            print()
        
        # Check if required text is present in combined text (normalized)
        if normalized_required in combined_text:
            return True
        
        # Also check case-insensitive match
        if normalized_required.lower() in combined_text.lower():
            print("NOTE: Found text with different casing")
            return True
        
        return False
        
    except Exception as e:
        print(f"WARNING: Could not verify popup text: {e}")
        import traceback
        traceback.print_exc()
        return False


def find_resume_button(app: Application) -> Optional[tuple[int, int]]:
    """
    Find the "Resume Printing" button and return its center coordinates.
    
    Args:
        app: Application instance connected to the window
        
    Returns:
        Tuple of (x, y) center coordinates if button found, None otherwise
    """
    try:
        # Get the window dialog - try multiple ways to find it
        try:
            window = app.window(title=WINDOW_TITLE, class_name=WINDOW_CLASS)
        except:
            # Try to get any window from the app
            windows = app.windows()
            if windows:
                window = windows[0]
            else:
                print("ERROR: Could not get window object")
                return None
        
        # Strategy 1: Try exact match first (preferred) - with Button type
        try:
            button = window.child_window(title=BUTTON_TEXT, control_type="Button")
            rect = button.rectangle()
            center_x = (rect.left + rect.right) // 2
            center_y = (rect.top + rect.bottom) // 2
            print(f"DEBUG: Found button with exact match '{BUTTON_TEXT}' (Button type)")
            return (center_x, center_y)
        except Exception as e:
            print(f"DEBUG: Strategy 1 failed (Button type): {e}")
        
        # Strategy 1b: Try without control type restriction
        try:
            button = window.child_window(title=BUTTON_TEXT)
            rect = button.rectangle()
            center_x = (rect.left + rect.right) // 2
            center_y = (rect.top + rect.bottom) // 2
            print(f"DEBUG: Found button with exact match '{BUTTON_TEXT}' (any type)")
            return (center_x, center_y)
        except Exception as e:
            print(f"DEBUG: Strategy 1b failed (any type): {e}")
        
        # Strategy 2: Search all controls (not just Button type) and prefer exact match
        print("DEBUG: Strategy 2: Searching all controls for button text...")
        try:
            # First try Button type
            buttons = list(window.descendants(control_type="Button"))
            print(f"DEBUG: Found {len(buttons)} Button-type controls")
            
            exact_match = None
            partial_matches = []
            
            for button in buttons:
                try:
                    button_text = button.window_text()
                    if button_text == BUTTON_TEXT:
                        # Exact match - prefer this one
                        exact_match = button
                        print(f"DEBUG: Found exact match in Button-type control: '{button_text}'")
                    elif BUTTON_TEXT in button_text:
                        # Partial match - keep as backup
                        partial_matches.append((button, button_text))
                except:
                    continue
            
            # If no Button-type matches, search ALL controls
            if not exact_match and not partial_matches:
                print("DEBUG: No matches in Button-type controls, searching all controls...")
                all_controls = list(window.descendants())
                print(f"DEBUG: Searching through {len(all_controls)} total controls")
                
                for control in all_controls:
                    try:
                        control_text = control.window_text()
                        if control_text == BUTTON_TEXT:
                            # Exact match
                            exact_match = control
                            print(f"DEBUG: Found exact match in control: '{control_text}'")
                            break
                        elif BUTTON_TEXT in control_text and not any(BUTTON_TEXT in text for _, text in partial_matches):
                            # Partial match - avoid duplicates
                            partial_matches.append((control, control_text))
                    except:
                        continue
            
            # Use exact match if found
            if exact_match:
                rect = exact_match.rectangle()
                center_x = (rect.left + rect.right) // 2
                center_y = (rect.top + rect.bottom) // 2
                print(f"DEBUG: ✓ Found button with exact match from descendants")
                print(f"DEBUG: Button coordinates: ({center_x}, {center_y})")
                return (center_x, center_y)
            
            # Use first partial match if no exact match
            if partial_matches:
                button, button_text = partial_matches[0]
                rect = button.rectangle()
                center_x = (rect.left + rect.right) // 2
                center_y = (rect.top + rect.bottom) // 2
                print(f"DEBUG: ✓ Found button with partial match: '{button_text}'")
                print(f"DEBUG: Button coordinates: ({center_x}, {center_y})")
                return (center_x, center_y)
            
            print("DEBUG: No matches found in Strategy 2")
        except Exception as e:
            print(f"DEBUG: Exception searching controls: {e}")
            import traceback
            traceback.print_exc()
        
        # Strategy 3: Try different control type names
        for control_type in ["Button", "PushButton", "CommandButton"]:
            try:
                buttons = window.descendants(control_type=control_type)
                for button in buttons:
                    try:
                        button_text = button.window_text()
                        if button_text == BUTTON_TEXT or BUTTON_TEXT in button_text:
                            rect = button.rectangle()
                            center_x = (rect.left + rect.right) // 2
                            center_y = (rect.top + rect.bottom) // 2
                            print(f"DEBUG: Found button using control_type '{control_type}': '{button_text}'")
                            return (center_x, center_y)
                    except:
                        continue
            except:
                continue
        
        # Strategy 4: List all buttons for debugging
        print("DEBUG: Listing all buttons found in window:")
        print("DEBUG: Attempting to get all Button controls...")
        try:
            all_buttons = list(window.descendants(control_type="Button"))
            button_count = len(all_buttons)
            print(f"DEBUG: Found {button_count} Button controls")
            
            if button_count == 0:
                print("DEBUG: No Button controls found, trying to list all controls...")
                try:
                    all_controls = list(window.descendants())
                    control_count = len(all_controls)
                    print(f"DEBUG: Found {control_count} total controls")
                    print("DEBUG: Listing first 20 controls:")
                    for i, control in enumerate(all_controls[:20]):
                        try:
                            control_text = control.window_text()
                            control_type = control.element_info.control_type_name if hasattr(control.element_info, 'control_type_name') else 'Unknown'
                            try:
                                rect = control.rectangle()
                            except:
                                rect = None
                            print(f"  [{i}] Type: {control_type} | Text: '{control_text[:50]}' | Rect: {rect}")
                        except Exception as e:
                            print(f"  [{i}] Could not get control info: {e}")
                except Exception as e:
                    print(f"DEBUG: Could not list all controls: {e}")
                    import traceback
                    traceback.print_exc()
            else:
                print("DEBUG: Listing Button controls:")
                for i, button in enumerate(all_buttons):
                    try:
                        button_text = button.window_text()
                        rect = button.rectangle()
                        print(f"  [{i}] Text: '{button_text}' | Rect: {rect}")
                    except Exception as e:
                        print(f"  [{i}] Could not get button info: {e}")
                        import traceback
                        traceback.print_exc()
        except Exception as e:
            print(f"DEBUG: Exception while listing buttons: {e}")
            import traceback
            traceback.print_exc()
        
        print("DEBUG: Button search completed - no matching button found")
        return None
        
    except Exception as e:
        print(f"ERROR: Could not find resume button: {e}")
        import traceback
        traceback.print_exc()
        return None


def activate_window_and_position_mouse(app: Application, coordinates: tuple[int, int]) -> bool:
    """
    Activate the window and position mouse over the button.
    
    Args:
        app: Application instance connected to the window
        coordinates: Tuple of (x, y) button center coordinates
        
    Returns:
        True if successful, False otherwise
    """
    # print("DEBUG: Starting window activation process...")
    # print()
    
    try:
        # print("DEBUG: Getting window object...")
        window = app.window(title=WINDOW_TITLE, class_name=WINDOW_CLASS)
        # print(f"DEBUG: Window handle: {window.handle}")
        # print()
        
        # Get initial window state
        # try:
        #     rect = window.rectangle()
        #     print(f"DEBUG: Initial window position: {rect}")
        #     print(f"DEBUG: Initial window size: {rect.width()}x{rect.height()}")
        # except Exception as e:
        #     print(f"DEBUG: Could not get initial window rect: {e}")
        # print()
        
        # Activate window - Strategy 3 (set_focus) was found to be the working method
        # print("DEBUG: Activating window using set_focus (pywinauto)...")
        try:
            window.set_focus()
            # print("DEBUG:   ✓ Window.set_focus() called successfully")
            time.sleep(0.2)  # Brief delay for focus to take effect
        except Exception as e:
            # print(f"DEBUG:   ✗ Window.set_focus() failed: {e}")
            pass
        # print()
        
        # Resize window: width +200, height +500
        # print("DEBUG: Resizing window (width +200, height +500)...")
        # try:
        #     rect = window.rectangle()
        #     current_width = rect.width()
        #     current_height = rect.height()
        #     new_width = current_width + 200
        #     new_height = current_height + 500
        #     
        #     print(f"DEBUG:   Current size: {current_width}x{current_height}")
        #     print(f"DEBUG:   New size: {new_width}x{new_height}")
        #     
        #     window.move_window(rect.left, rect.top, new_width, new_height)
        #     print("DEBUG:   ✓ Window resized successfully")
        #     time.sleep(0.1)  # Brief delay for resize to take effect
        # except Exception as e:
        #     print(f"DEBUG:   ✗ Window resize failed: {e}")
        # print()
        
        # Commented out - other strategies that didn't work:
        # 
        # print("DEBUG: Strategy 1: Restore if minimized...")
        # try:
        #     window.restore()
        #     print("DEBUG:   ✓ Window.restore() called successfully")
        # except Exception as e:
        #     print(f"DEBUG:   ✗ Window.restore() failed: {e}")
        # print()
        # 
        # print("DEBUG: Strategy 2: Move window to force to front...")
        # try:
        #     rect = window.rectangle()
        #     window.move_window(rect.left, rect.top, rect.width(), rect.height())
        #     print("DEBUG:   ✓ Window.move_window() called successfully")
        # except Exception as e:
        #     print(f"DEBUG:   ✗ Window.move_window() failed: {e}")
        # print()
        # 
        # print("DEBUG: Strategy 4: Set foreground (pywinauto)...")
        # try:
        #     window.set_foreground()
        #     print("DEBUG:   ✓ Window.set_foreground() called successfully")
        # except Exception as e:
        #     print(f"DEBUG:   ✗ Window.set_foreground() failed: {e}")
        # print()
        # 
        # print("DEBUG: Strategy 5: Windows API activation...")
        # try:
        #     import win32gui
        #     import win32con
        #     handle = window.handle
        #     
        #     is_iconic = win32gui.IsIconic(handle)
        #     if is_iconic:
        #         win32gui.ShowWindow(handle, win32con.SW_RESTORE)
        #     
        #     win32gui.SetForegroundWindow(handle)
        #     win32gui.BringWindowToTop(handle)
        #     win32gui.SetActiveWindow(handle)
        #     win32gui.SetFocus(handle)
        #     print("DEBUG:   ✓ Windows API activation completed")
        # except ImportError:
        #     print("DEBUG:   ✗ win32gui not available")
        # except Exception as e:
        #     print(f"DEBUG:   ✗ Windows API activation failed: {e}")
        # print()
        
        # Final check: verify window is actually in foreground
        # print("DEBUG: Verifying window is in foreground...")
        # try:
        #     import win32gui
        #     current_foreground = win32gui.GetForegroundWindow()
        #     print(f"DEBUG:   Current foreground window handle: {current_foreground}")
        #     print(f"DEBUG:   Target window handle: {window.handle}")
        #     
        #     if current_foreground == window.handle:
        #         print("DEBUG:   ✓ Window is in foreground (verified)")
        #     else:
        #         print("DEBUG:   ⚠ WARNING: Window may not be fully in foreground")
        #         print("DEBUG:   Handle mismatch - window may be behind another window")
        # except ImportError:
        #     print("DEBUG:   Cannot verify (win32gui not available)")
        # except Exception as e:
        #     print(f"DEBUG:   Could not verify foreground: {e}")
        # print()
        
        # Get final window state
        # try:
        #     rect = window.rectangle()
        #     print(f"DEBUG: Final window position: {rect}")
        # except Exception as e:
        #     print(f"DEBUG: Could not get final window rect: {e}")
        # print()
        
        # Move mouse to button center
        # print(f"DEBUG: Moving mouse to button coordinates: ({coordinates[0]}, {coordinates[1]})")
        pyautogui.moveTo(coordinates[0], coordinates[1], duration=0.3)
        # print("DEBUG: ✓ Mouse positioned over button")
        # print()
        
        return True
        
    except Exception as e:
        print(f"ERROR: Failed to activate window and position mouse: {e}")
        import traceback
        traceback.print_exc()
        return False


def list_all_error_windows() -> None:
    """List all windows with 'Error' in the title for debugging."""
    try:
        all_windows = findwindows.find_windows(title_re=WINDOW_TITLE)
        if not all_windows:
            # Try exact match
            all_windows = findwindows.find_windows(title=WINDOW_TITLE)
        
        if all_windows:
            print("DEBUG: Found windows with 'Error' in title:")
            for handle in all_windows[:5]:  # Limit to first 5
                try:
                    elements = findwindows.find_elements(handle=handle)
                    if elements:
                        element = elements[0]
                        print(f"  Handle: {handle}")
                        print(f"    Title: {element.get('name', 'N/A')}")
                        print(f"    Class: {element.get('class_name', 'N/A')}")
                        print(f"    Process: {element.get('process', 'N/A')}")
                        print()
                except Exception:
                    pass
    except Exception:
        pass


def main() -> int:
    """
    Main function to find, verify, and handle the Orca Slicer error popup.
    
    Returns:
        Exit code (0 for success, 1 for failure)
    """
    print(f"Searching for Orca Slicer error popup...")
    print(f"  Window: {WINDOW_TITLE}")
    print(f"  Class: {WINDOW_CLASS}")
    print(f"  Executable: {WINDOW_EXE}")
    print()
    
    # Step 1: Find the window
    app = find_orca_error_window()
    if not app:
        print("NOT FOUND: Orca Slicer error popup window is not open.")
        print()
        print("DEBUG: Listing all 'Error' windows found on system:")
        list_all_error_windows()
        print()
        print("TROUBLESHOOTING:")
        print("  1. Make sure Orca Slicer is running")
        print("  2. Make sure the error popup is actually open and visible")
        print("  3. Try running this script again when the popup is showing")
        return 1
    
    print("FOUND: Orca Slicer error popup window detected.")
    print()
    
    # Step 2: Verify the required text is present
    print(f"Verifying popup contains required text:")
    print(f"  '{REQUIRED_TEXT}'")
    print()
    
    if not verify_popup_text(app):
        print("VERIFICATION FAILED: Popup does not contain the required text.")
        print("  The popup may be for a different error, or the text has changed.")
        return 1
    
    print("VERIFIED: Popup contains the required text.")
    print()
    
    # Step 3: Find the Resume Printing button
    print(f"Searching for button: '{BUTTON_TEXT}'")
    coordinates = find_resume_button(app)
    
    if not coordinates:
        print("ERROR: Could not locate the 'Resume Printing' button.")
        print("  Window found and text verified, but button detection failed.")
        print("  This may require manual intervention or additional debugging.")
        return 1
    
    print(f"FOUND: Button located at coordinates ({coordinates[0]}, {coordinates[1]})")
    print()
    
    # Step 4: Activate window and position mouse
    print("Activating window and positioning mouse over button...")
    if activate_window_and_position_mouse(app, coordinates):
        print("SUCCESS: Window activated and mouse positioned over 'Resume Printing' button.")
        print()
        print("You can now click the button or press Enter to resume printing.")
        return 0
    else:
        print("ERROR: Failed to activate window or position mouse.")
        return 1


if __name__ == '__main__':
    try:
        exit_code = main()
        sys.exit(exit_code)
    except KeyboardInterrupt:
        print("\n\nInterrupted by user.")
        sys.exit(1)
    except Exception as e:
        print(f"\nERROR: Unexpected error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
