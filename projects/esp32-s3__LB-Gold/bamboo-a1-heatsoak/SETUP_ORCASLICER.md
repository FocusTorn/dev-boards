# OrcaSlicer Post-Processing Script Setup

## Configuration for Single Text Entry Box

OrcaSlicer has a single text entry box for post-processing scripts. Use one of these methods:

### Method 1: Direct Python Command (Recommended)

In OrcaSlicer's post-processing script field, enter:

```
python "D:\_dev\_Projects\dev-boards\projects\esp32-s3__LB-Gold\bamboo-a1-heatsoak\set_print_flags.py" "{gcode_file}"
```

**Note**: Replace `python` with the full path to Python if it's not in your PATH:
- Find Python path: Open PowerShell and run `where.exe python`
- Example: `C:\Python312\python.exe "D:\_dev\_Projects\dev-boards\projects\esp32-s3__LB-Gold\bamboo-a1-heatsoak\set_print_flags.py" "{gcode_file}"`

### Method 2: Batch File (Alternative)

If Method 1 doesn't work, try the batch file:

```
"D:\_dev\_Projects\dev-boards\projects\esp32-s3__LB-Gold\bamboo-a1-heatsoak\set_print_flags.bat" "{gcode_file}"
```

### Method 3: PowerShell Wrapper

If both methods fail, use PowerShell:

```
powershell.exe -ExecutionPolicy Bypass -File "D:\_dev\_Projects\dev-boards\projects\esp32-s3__LB-Gold\bamboo-a1-heatsoak\set_print_flags.ps1" "{gcode_file}"
```

## Setup Steps

1. Open OrcaSlicer
2. Go to: **Printer Settings** → **Machine G-code** → **Post-processing scripts**
3. Click **Add script**
4. In the single text entry box, paste one of the commands above
5. Click **OK**

## How It Works

When you slice a model:
1. OrcaSlicer processes the gcode
2. The script runs and shows a dialog/prompt
3. You select which operations to enable:
   - ✓ Auto Bed Leveling (G29)
   - ✓ Flow Calibration (Extrusion)
   - ✓ Mech Mode / Resonance Testing
4. The script injects `M1002 set_flag` commands into your gcode
5. Your optimized start gcode checks these flags and runs operations conditionally

## Troubleshooting

### Error: "python is not recognized"
- Use the full path to Python.exe instead of just `python`
- Find Python path: `where.exe python` in PowerShell
- Example: `C:\Python312\python.exe "..."`

### Error: "Win32 error: 193" or "-1073741510"
- The script file might not be executable
- Try Method 1 (direct Python) instead of batch file
- Make sure Python is installed and in PATH

### Dialog doesn't appear
- Check if tkinter is installed: `python -c "import tkinter"`
- The script will fall back to console prompts if GUI is unavailable
- Make sure you're watching the console/terminal output

### Script runs but flags aren't set
- Check the gcode file for `M1002 set_flag` commands at the top
- Verify the script has write permissions to the temp directory
- Check OrcaSlicer's console output for error messages
