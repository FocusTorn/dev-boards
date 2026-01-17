#!/usr/bin/env python3
"""
OrcaSlicer Post-Processing Script - Print Flags Configuration
Prompts user to select which pre-print operations to perform.

This script should be configured in OrcaSlicer:
  Printer Settings → Post-processing scripts → Add script

Script path: [path to set_print_flags.bat] (Windows) or [path to set_print_flags.py] (Linux/Mac)
Arguments: {gcode_file}

The script will:
- Display a dialog/prompt asking which operations to enable
- Inject M1002 set_flag commands at the start of the gcode
- Flags: g29_before_print_flag, extrude_cali_flag, mech_mode_flag
"""

import sys
import os
from pathlib import Path
from typing import Dict, Optional

def prompt_for_flags_console() -> Optional[Dict[str, bool]]:
    """
    Display a console-based prompt to select which flags to enable.
    Returns a dictionary with flag names as keys and boolean values, or None if cancelled.
    """
    flags = {
        'g29_before_print_flag': False,  # Bed leveling
        'extrude_cali_flag': False,       # Flow calibration
        'mech_mode_flag': False,          # Mech mode/resonance testing
    }
    
    flag_labels = {
        'g29_before_print_flag': 'Auto Bed Leveling (G29)',
        'extrude_cali_flag': 'Flow Calibration (Extrusion)',
        'mech_mode_flag': 'Mech Mode / Resonance Testing',
    }
    
    print("\n" + "="*60, file=sys.stderr)
    print("PRE-PRINT OPERATIONS SELECTION", file=sys.stderr)
    print("="*60, file=sys.stderr)
    print("Select which pre-print operations to perform:", file=sys.stderr)
    print("", file=sys.stderr)
    
    for i, (flag_name, default) in enumerate(flags.items(), 1):
        label = flag_labels[flag_name]
        default_str = " [Y/n]" if default else " [y/N]"
        response = input(f"{i}. {label}{default_str}: ").strip().lower()
        
        if response == '':
            # Use default
            flags[flag_name] = default
        elif response in ['y', 'yes']:
            flags[flag_name] = True
        elif response in ['n', 'no']:
            flags[flag_name] = False
        else:
            print(f"Invalid response, using default: {default}", file=sys.stderr)
            flags[flag_name] = default
    
    print("", file=sys.stderr)
    print("Selected operations:", file=sys.stderr)
    for flag_name, enabled in flags.items():
        status = "ENABLED" if enabled else "DISABLED"
        print(f"  - {flag_labels[flag_name]}: {status}", file=sys.stderr)
    
    confirm = input("\nProceed with these settings? [Y/n]: ").strip().lower()
    if confirm in ['n', 'no']:
        print("Cancelled by user", file=sys.stderr)
        return None
    
    return flags

def prompt_for_flags_gui() -> Optional[Dict[str, bool]]:
    """
    Display a GUI dialog to select which flags to enable.
    Falls back to console if GUI is not available.
    """
    try:
        import tkinter as tk
    except ImportError:
        # tkinter not available, use console
        return prompt_for_flags_console()
    
    flags = {
        'g29_before_print_flag': False,  # Bed leveling
        'extrude_cali_flag': False,       # Flow calibration
        'mech_mode_flag': False,          # Mech mode/resonance testing
    }
    
    flag_labels = {
        'g29_before_print_flag': '✓ Auto Bed Leveling (G29)',
        'extrude_cali_flag': '✓ Flow Calibration (Extrusion)',
        'mech_mode_flag': '✓ Mech Mode / Resonance Testing',
    }
    
    try:
        # Create root window (hidden)
        root = tk.Tk()
        root.withdraw()  # Hide main window
        
        # Create dialog window
        dialog = tk.Toplevel(root)
        dialog.title("Pre-Print Operations")
        dialog.geometry("450x280")
        dialog.resizable(False, False)
        
        # Center the window
        dialog.update_idletasks()
        x = (dialog.winfo_screenwidth() // 2) - (dialog.winfo_width() // 2)
        y = (dialog.winfo_screenheight() // 2) - (dialog.winfo_height() // 2)
        dialog.geometry(f"+{x}+{y}")
        
        # Make dialog modal
        dialog.transient(root)
        dialog.grab_set()
        
        # Label
        label = tk.Label(
            dialog,
            text="Select which pre-print operations to perform:",
            font=('Arial', 10, 'bold'),
            pady=10
        )
        label.pack()
        
        # Checkboxes
        checkboxes = {}
        for flag_name, default in flags.items():
            var = tk.BooleanVar(value=default)
            checkbox = tk.Checkbutton(
                dialog,
                text=flag_labels[flag_name],
                variable=var,
                font=('Arial', 9)
            )
            checkbox.pack(anchor='w', padx=30, pady=5)
            checkboxes[flag_name] = var
        
        # Buttons frame
        button_frame = tk.Frame(dialog)
        button_frame.pack(pady=20)
        
        result = {'confirmed': False}
        
        def on_ok():
            for flag_name, var in checkboxes.items():
                flags[flag_name] = var.get()
            result['confirmed'] = True
            dialog.destroy()
            root.destroy()
        
        def on_cancel():
            result['confirmed'] = False
            dialog.destroy()
            root.destroy()
        
        ok_button = tk.Button(
            button_frame,
            text="OK",
            command=on_ok,
            width=10,
            font=('Arial', 9)
        )
        ok_button.pack(side=tk.LEFT, padx=5)
        
        cancel_button = tk.Button(
            button_frame,
            text="Cancel",
            command=on_cancel,
            width=10,
            font=('Arial', 9)
        )
        cancel_button.pack(side=tk.LEFT, padx=5)
        
        # Set default button
        dialog.bind('<Return>', lambda e: on_ok())
        dialog.bind('<Escape>', lambda e: on_cancel())
        
        # Focus on dialog
        dialog.focus_set()
        
        # Wait for user response
        root.wait_window(dialog)
        
        if not result['confirmed']:
            return None
        
        return flags
        
    except Exception as e:
        # GUI failed, fall back to console
        print(f"GUI unavailable ({e}), using console prompt", file=sys.stderr)
        return prompt_for_flags_console()

def inject_flags(gcode_file: Path, flags: Dict[str, bool], output_file: Path):
    """Inject M1002 set_flag commands at the start of gcode file."""
    with open(gcode_file, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    # Remove any existing flag commands (to avoid duplicates)
    filtered_lines = []
    skip_flag_section = False
    for line in lines:
        stripped = line.strip()
        if 'PRE-PRINT OPERATION FLAGS' in line.upper() or 'set_flag' in stripped:
            skip_flag_section = True
            continue
        if skip_flag_section and (stripped.startswith(';') or not stripped):
            # Skip empty lines and comments in flag section
            continue
        if skip_flag_section and stripped and not stripped.startswith(';'):
            # End of flag section
            skip_flag_section = False
            filtered_lines.append(line)
        elif not skip_flag_section:
            filtered_lines.append(line)
    
    # Find insertion point (after header comments, before first command)
    insertion_index = 0
    for i, line in enumerate(filtered_lines):
        stripped = line.strip()
        # Skip empty lines and comments
        if stripped and not stripped.startswith(';'):
            insertion_index = i
            break
    
    # Build flag commands
    flag_commands = []
    flag_commands.append("; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐\n")
    flag_commands.append("; │                              PRE-PRINT OPERATION FLAGS (AUTO-SET)                                │\n")
    flag_commands.append("; └────────────────────────────────────────────────────────────────────────────────────────────────┘\n")
    flag_commands.append("; Pre-print operation flags (set by post-processing script)\n")
    flag_commands.append(";\n")
    
    for flag_name, enabled in flags.items():
        value = 1 if enabled else 0
        flag_commands.append(f"M1002 set_flag {flag_name}={value}\n")
    
    flag_commands.append("\n")
    
    # Insert flag commands
    output_lines = filtered_lines[:insertion_index] + flag_commands + filtered_lines[insertion_index:]
    
    # Write output
    with open(output_file, 'w', encoding='utf-8') as f:
        f.writelines(output_lines)
    
    enabled_flags = [name for name, enabled in flags.items() if enabled]
    if enabled_flags:
        print(f"✓ Enabled flags: {', '.join(enabled_flags)}", file=sys.stderr)
    else:
        print("✓ No flags enabled (all operations skipped)", file=sys.stderr)

def main():
    if len(sys.argv) < 2:
        print("Usage: set_print_flags.py <gcode_file>", file=sys.stderr)
        print("  This script is called by OrcaSlicer during post-processing", file=sys.stderr)
        sys.exit(1)
    
    gcode_file = Path(sys.argv[1])
    
    if not gcode_file.exists():
        print(f"Error: G-code file not found: {gcode_file}", file=sys.stderr)
        sys.exit(1)
    
    # Check if we're in a headless environment (no display)
    use_gui = os.getenv('DISPLAY') is not None or sys.platform == 'win32'
    
    # Prompt user for flags
    try:
        if use_gui:
            flags = prompt_for_flags_gui()
        else:
            flags = prompt_for_flags_console()
        
        if flags is None:
            # User cancelled - don't modify gcode
            print("User cancelled flag selection", file=sys.stderr)
            sys.exit(0)
        
        # Inject flags into gcode
        output_file = gcode_file  # OrcaSlicer expects in-place modification
        inject_flags(gcode_file, flags, output_file)
        
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)

if __name__ == '__main__':
    main()
