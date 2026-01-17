#!/usr/bin/env python3
"""
OrcaSlicer Post-Processing Script
Reads chamber temperature from bridge output and injects it into G-code.

This script should be configured in OrcaSlicer:
  Printer Settings → Post-processing scripts → Add script

Script path: [path to this script]
Arguments: {gcode_file} {chamber_temp_file}

The script will:
- Read chamber temperature from the bridge output file
- Inject M155 temperature reporting commands into G-code
- Add comments with chamber temperature values
"""

import sys
import re
from pathlib import Path
from typing import Optional

def read_chamber_temp(temp_file: Path) -> Optional[int]:
    """Read chamber temperature from bridge output file."""
    try:
        if temp_file.exists():
            with open(temp_file, 'r') as f:
                content = f.read().strip()
                # File contains just the temperature as integer
                if content:
                    return int(float(content))
    except (ValueError, IOError) as e:
        print(f"Warning: Could not read chamber temperature from {temp_file}: {e}", file=sys.stderr)
    except Exception as e:
        print(f"Warning: Unexpected error reading temperature file: {e}", file=sys.stderr)
    
    return None

def inject_chamber_temp(gcode_file: Path, chamber_temp: int, output_file: Path):
    """Inject chamber temperature into G-code file."""
    if chamber_temp is None:
        print(f"Warning: No chamber temperature available, skipping injection", file=sys.stderr)
        # Still copy the file even if temp is unavailable
        with open(gcode_file, 'r', encoding='utf-8') as f:
            content = f.read()
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(content)
        return
    
    with open(gcode_file, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    # Find machine_start_gcode section or beginning of file
    # We'll add chamber temperature reporting after the initial setup commands
    output_lines = []
    inserted = False
    
    # Look for a good insertion point (after initial setup, before actual printing)
    insertion_keywords = [
        ';===== start to heat',
        'M104',  # Hotend temp
        'M140',  # Bed temp
        'G28',   # Home
    ]
    
    for i, line in enumerate(lines):
        output_lines.append(line)
        
        # Insert chamber temp reporting after bed/hotend heating commands
        if not inserted:
            # Check if we've passed the initial setup section
            if any(keyword in line.upper() for keyword in insertion_keywords):
                # Look ahead to find a good spot (after a few more setup lines)
                if i < len(lines) - 5:
                    # Check next few lines
                    next_lines = ''.join(lines[i+1:i+6]).upper()
                    if 'G28' in next_lines or 'M109' in next_lines or 'M190' in next_lines:
                        # We're in the heating section, insert after this block
                        continue
                    else:
                        # Insert chamber temp reporting here
                        output_lines.append(f"\n;===== Chamber Temperature Reporting (from SHT21) =====\n")
                        output_lines.append(f"; Current chamber temperature: {chamber_temp}°C\n")
                        output_lines.append(f"M155 S30 ; Request temperature report every 30 seconds\n")
                        output_lines.append(f"; Note: Bamboo Lab printers may not support M155, but OrcaSlicer may read this\n")
                        output_lines.append(f"; For display purposes, temperature is {chamber_temp}°C\n")
                        inserted = True
    
    # If we haven't inserted yet, add at the beginning after comments
    if not inserted:
        # Find first non-comment, non-empty line
        for i, line in enumerate(output_lines):
            stripped = line.strip()
            if stripped and not stripped.startswith(';'):
                # Insert before this line
                output_lines.insert(i, f"\n;===== Chamber Temperature Reporting (from SHT21) =====\n")
                output_lines.insert(i+1, f"; Current chamber temperature: {chamber_temp}°C\n")
                output_lines.insert(i+2, f"M155 S30 ; Request temperature report every 30 seconds\n")
                output_lines.insert(i+3, f"; Note: Temperature reading from SHT21 sensor via MQTT\n")
                inserted = True
                break
        
        # If still not inserted, add at the very beginning
        if not inserted:
            output_lines.insert(0, f";===== Chamber Temperature Reporting (from SHT21) =====\n")
            output_lines.insert(1, f"; Current chamber temperature: {chamber_temp}°C\n")
            output_lines.insert(2, f"M155 S30 ; Request temperature report every 30 seconds\n")
            output_lines.insert(3, f"\n")
    
    # Write output
    with open(output_file, 'w', encoding='utf-8') as f:
        f.writelines(output_lines)
    
    print(f"Injected chamber temperature {chamber_temp}°C into G-code", file=sys.stderr)

def main():
    if len(sys.argv) < 2:
        print("Usage: orca_post_process.py <gcode_file> [chamber_temp_file]", file=sys.stderr)
        print("  This script is called by OrcaSlicer during post-processing", file=sys.stderr)
        print("  If chamber_temp_file is not provided, looks for chamber_temp.txt in script directory", file=sys.stderr)
        sys.exit(1)
    
    gcode_file = Path(sys.argv[1])
    
    # Try to find temperature file
    if len(sys.argv) > 2:
        temp_file = Path(sys.argv[2])
    else:
        # Default: look for chamber_temp.txt in the same directory as this script
        script_dir = Path(__file__).parent
        temp_file = script_dir / "chamber_temp.txt"
    
    # OrcaSlicer provides the output file as the same as input (it will be replaced)
    output_file = gcode_file
    
    if not gcode_file.exists():
        print(f"Error: G-code file not found: {gcode_file}", file=sys.stderr)
        sys.exit(1)
    
    # Read chamber temperature
    chamber_temp = read_chamber_temp(temp_file)
    
    # Inject into G-code
    inject_chamber_temp(gcode_file, chamber_temp, output_file)

if __name__ == '__main__':
    main()
