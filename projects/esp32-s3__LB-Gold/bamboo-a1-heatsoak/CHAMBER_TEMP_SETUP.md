# Chamber Temperature Display Setup for OrcaSlicer

This guide explains how to display chamber temperature readings from your ESP32-S3 SHT21 sensor in OrcaSlicer.

## Overview

Since Bamboo Lab printers don't have native chamber temperature sensors, we use:
1. **ESP32-S3** reads SHT21 sensor and publishes to MQTT
2. **Bridge Script** (`chamber_temp_bridge.py`) reads MQTT and writes temperature to a file
3. **OrcaSlicer Post-Processing** reads the temperature file and injects it into G-code

**Note:** OrcaSlicer may not display chamber temperature in the UI for Bamboo Lab printers (since they don't report it natively), but the temperature will be:
- Available in the G-code file (as comments)
- Can be used in your print start macros
- Can be logged/monitored via external tools

## Setup Steps

### 1. Start the Temperature Bridge

The bridge script reads temperature from MQTT and writes it to a file that OrcaSlicer can access.

```powershell
# Start the bridge (run in a separate terminal/background)
python chamber_temp_bridge.py

# Or with custom settings:
python chamber_temp_bridge.py --mqtt-host localhost --output-file C:\path\to\chamber_temp.txt
```

The bridge will:
- Subscribe to `sensors/sht21/readings` MQTT topic
- Write current temperature to `chamber_temp.txt` (default location)
- Update the file every time a new reading is received

**Important:** Keep this script running while you want chamber temperature to be available.

### 2. Configure OrcaSlicer Post-Processing Script

1. **Open OrcaSlicer** → Printer Settings
2. **Navigate to** "Post-processing scripts"
3. **Add Script:**
   - **Script Path:** Full path to `orca_post_process.py`
     - Example: `D:\_dev\_Projects\dev-boards\projects\esp32-s3__LB-Gold\bamboo-a1-heatsoak\orca_post_process.py`
   - **Arguments:** `{gcode_file} {chamber_temp_file}`
     - Replace `{chamber_temp_file}` with the full path to `chamber_temp.txt`
     - Example: `{gcode_file} D:\_dev\_Projects\dev-boards\projects\esp32-s3__LB-Gold\bamboo-a1-heatsoak\chamber_temp.txt`

**OrcaSlicer Settings:**
```
Post-processing scripts:
  Script: D:\...\orca_post_process.py
  Arguments: {gcode_file} D:\...\chamber_temp.txt
```

### 3. Alternative: Use Temperature in Start G-code

Instead of post-processing, you can also use the temperature file directly in your start G-code:

```gcode
; Read chamber temperature from file (if available)
; This is a comment - actual G-code execution depends on printer firmware support
; Chamber temp from SHT21 sensor: Read from chamber_temp.txt
```

However, since Bamboo Lab printers don't support M191/M141 natively, you'll primarily use this for:
- Documentation in G-code
- External monitoring tools
- Custom scripts that read the G-code

### 4. Verify Setup

1. **Check Bridge is Running:**
   ```powershell
   # The bridge should print:
   ✅ Connected to MQTT broker at localhost:1883
   ✅ Subscribed to topic: sensors/sht21/readings
   🌡️  Chamber temp: 35.2°C → chamber_temp.txt
   ```

2. **Check Temperature File:**
   ```powershell
   # Should contain just the temperature as integer:
   Get-Content chamber_temp.txt
   # Output: 35
   ```

3. **Slice a File in OrcaSlicer:**
   - Open a model
   - Slice it
   - Check the G-code preview - you should see comments like:
     ```gcode
     ;===== Chamber Temperature Reporting (from SHT21) =====
     ; Current chamber temperature: 35°C
     ```

## Limitations

### Bamboo Lab Printer Limitations

Bamboo Lab printers (A1, X1, P1 series) **do not natively support:**
- M191 command (set and wait for chamber temperature)
- M141 command (set chamber temperature)
- M155 command (temperature auto-report)
- Chamber temperature reporting in status messages

### What This Means

1. **OrcaSlicer UI:** May not display chamber temperature in the printer status panel (since the printer doesn't report it)

2. **G-code Comments:** Temperature will be available as comments in your G-code file

3. **External Monitoring:** You can use external tools to:
   - Monitor the temperature file
   - Display it in Home Assistant, OctoPrint, etc.
   - Use it for custom automation

4. **Future Firmware Updates:** If Bamboo Lab adds chamber temperature support in future firmware updates, this setup will be ready to use

## Alternative: Real-time Display Solutions

If you want real-time chamber temperature display, consider:

### Option 1: Home Assistant / MQTT Dashboard
- Use Home Assistant MQTT integration
- Create a sensor card showing `sensors/sht21/readings`
- Display on a dashboard or tablet near the printer

### Option 2: OctoPrint Plugin
- If using OctoPrint, create a plugin that subscribes to MQTT
- Display chamber temperature in OctoPrint UI

### Option 3: Custom Web Dashboard
- Create a simple HTML/JavaScript dashboard
- Connect to MQTT broker via WebSocket
- Display real-time temperature readings

## Troubleshooting

### Bridge Not Receiving Messages
- Check ESP32 is powered on and connected to WiFi
- Verify ESP32 is publishing to `sensors/sht21/readings` topic
- Check MQTT broker is accessible at configured host/port
- Verify MQTT credentials (username/password)

### Temperature File Not Updating
- Check bridge script is still running
- Verify file path is writable
- Check file permissions

### OrcaSlicer Post-Processing Not Working
- Verify script path is correct (use full absolute path)
- Check Python is in system PATH
- Verify chamber_temp.txt file exists and is readable
- Check OrcaSlicer console/logs for error messages

### Temperature Shows as 0 or Invalid
- Check ESP32 sensor is reading correctly (check serial output)
- Verify MQTT message format matches expected JSON structure
- Check bridge script console for error messages

## Files

- `chamber_temp_bridge.py` - Bridge script that reads MQTT and writes temperature file
- `orca_post_process.py` - OrcaSlicer post-processing script
- `monitor_heatsoak.py` - Heatsoak monitoring (separate functionality)
- `chamber_temp.txt` - Output file with current temperature (created by bridge)

## Example Workflow

1. **Before Printing:**
   ```powershell
   # Start the bridge (in background or separate terminal)
   python chamber_temp_bridge.py
   ```

2. **In OrcaSlicer:**
   - Configure filament with desired chamber temperature (even if printer doesn't support it)
   - Slice your model
   - G-code will automatically include chamber temperature comments

3. **During Printing:**
   - Temperature is continuously updated in `chamber_temp.txt`
   - You can monitor it with: `Get-Content chamber_temp.txt -Wait` (PowerShell)
   - Or use external monitoring tools

4. **After Printing:**
   - Stop the bridge script (Ctrl+C)
   - Or leave it running for continuous monitoring
