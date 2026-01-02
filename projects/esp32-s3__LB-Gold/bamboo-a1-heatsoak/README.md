# Bamboo Labs A1 Heat Soak Monitor

Automated heat soak monitoring for Bamboo Labs A1 3D printer using SHT21 temperature sensor via MQTT.

## Overview

This system monitors chamber temperature from an SHT21 sensor (published via MQTT) and automatically resumes the printer when heatsoak conditions are met. The monitoring uses the same smoothing and threshold logic as the BME680 project.

## Components

1. **Start G-code** (`start_gcode.gcode`) - Pauses printer after initial prep, maintains temperatures
2. **Python Monitor** (`monitor_heatsoak.py`) - Monitors MQTT temperature, applies heatsoak calculations, resumes printer

## Setup

### 1. Install Dependencies

```bash
uv pip install paho-mqtt requests
```

### 2. Configure Start G-code

1. Open Bambu Studio or OrcaSlicer
2. Go to Printer Settings → Machine → Start G-code
3. Replace the start G-code with the contents of `start_gcode.gcode`

### 3. Configure Printer API Access

The script attempts to connect to the printer via HTTP API. You may need to:

- Enable LAN mode on your Bamboo Labs A1 printer
- Note the printer's IP address (found in printer settings)
- Some printers may require API key or authentication - adjust the `BambooLabPrinter` class accordingly

### 4. Run the Monitor

```bash
# Basic usage
python monitor_heatsoak.py --printer-ip 192.168.1.100

# Custom thresholds (matching BME680 defaults)
python monitor_heatsoak.py --printer-ip 192.168.1.100 \
    --rate-start-temp 40.0 \
    --rate-change-plateau 0.1

# With target temperature
python monitor_heatsoak.py --printer-ip 192.168.1.100 \
    --target-temp 45.0 \
    --rate-start-temp 40.0 \
    --rate-change-plateau 0.1

# Custom MQTT settings
python monitor_heatsoak.py --printer-ip 192.168.1.100 \
    --mqtt-host 192.168.1.50 \
    --mqtt-topic sensors/sht21/readings
```

## How It Works

### Start G-code Sequence

1. Sets hotend to 150°C
2. Sets bed to initial layer temperature
3. Homes all axes (G28)
4. Waits for bed temperature (M190)
5. Waits for hotend temperature (M109)
6. Finishes all moves (M400)
7. **Pauses print (M25)** - printer maintains temperatures while paused

### Monitoring Script

1. **Subscribes to MQTT** - Listens to `sensors/sht21/readings` topic
2. **Applies Smoothing** - Uses BME680-style calculations:
   - Temperature smoothing: 4-second window (configurable)
   - Rate smoothing: 30-second window (configurable)
   - Least squares linear regression for rate calculation
3. **Detects Heatsoak** - Checks if conditions are met:
   - **Target temp mode**: `smoothed_temp > target_temp` → Ready
   - **Rate mode**: `smoothed_temp > rate_start_temp AND abs(rate) < rate_change_plateau` → Ready
4. **Resumes Printer** - Sends resume command via HTTP API when ready

## Heatsoak Logic (BME680-style)

The monitoring uses the same logic as the BME680 project:

- **Temperature Smoothing**: Simple average over 4-second window
- **Rate Calculation**: Least squares linear regression over 30-second window
- **Threshold Detection**:
  - `rate_start_temp`: Minimum temperature to start checking rate (prevents false positives)
  - `rate_change_plateau`: Maximum rate of change (°C/min) indicating diminishing returns
  - `target_temp`: Optional target temperature (if reached, automatically ready)

### Rate Start Types

- **absolute**: `rate_start_temp` is used as-is (e.g., 40°C)
- **offset**: `rate_start_temp` is added to initial soak temperature when soaking starts

## Configuration

### Default Settings (matching BME680)

```yaml
temp_smooth: 4.0          # Temperature smoothing window (seconds)
rate_smooth: 30.0         # Rate smoothing window (seconds)
rate_start_type: "absolute"
rate_start_temp: 40.0     # °C
rate_change_plateau: 0.1  # °C/min
target_temp: None         # Optional
```

### MQTT Topic Format

The script expects SHT21 readings in this format:

```json
{
  "temperature": 24.5,
  "humidity": 52.3,
  "timestamp": 1234567890
}
```

## Troubleshooting

### Printer Not Resuming

1. **Check printer IP address** - Verify the printer is accessible on the network
2. **Check API endpoints** - The script tries multiple common endpoints. You may need to adjust based on your printer's API
3. **Manual resume** - If automatic resume fails, you can manually resume from the printer interface

### MQTT Connection Issues

1. **Verify MQTT broker** - Ensure the broker is running and accessible
2. **Check topic name** - Verify the SHT21 sensor is publishing to the expected topic
3. **Check message format** - Ensure messages contain a `temperature` field

### Temperature Not Stabilizing

1. **Adjust thresholds** - Increase `rate_start_temp` or `rate_change_plateau` if getting false positives
2. **Check sensor** - Verify SHT21 sensor is reading correctly
3. **Check smoothing** - Adjust `temp_smooth` or `rate_smooth` windows if needed

## API Endpoints

The script attempts these HTTP endpoints (in order):

1. `/api/v1/printer/print/resume`
2. `/api/printer/print/resume`
3. `/command`

If none work, you may need to:
- Check your printer's API documentation
- Use a different method (e.g., serial connection, Moonraker API)
- Modify the `BambooLabPrinter` class to match your printer's API

## Testing on Windows (Before Raspberry Pi Setup)

If you need to test on Windows before your Raspberry Pi with Mosquitto is ready:

1. **Install MQTT Broker on Windows**:
   - See `WINDOWS_MQTT_SETUP.md` for detailed instructions
   - Recommended: Mosquitto for Windows (same as Raspberry Pi)

2. **Update ESP32-S3 IP Address**:
   - Change MQTT server IP in `sht21-solo.ino` to your Windows machine IP
   - Re-upload sketch to ESP32-S3

3. **Test with Simulator** (optional):
   ```bash
   # Simulate heatsoak scenario
   python test_mqtt_publisher.py --simulate-heatsoak
   
   # Run monitor in another terminal
   python monitor_heatsoak.py --printer-ip 192.168.1.100 --mqtt-host localhost
   ```

4. **Run Monitor**:
   ```bash
   python monitor_heatsoak.py --printer-ip 192.168.1.100 --mqtt-host localhost
   ```

See `WINDOWS_MQTT_SETUP.md` for complete Windows setup guide.

## References

- BME680 heatsoak logic: `reference/bme680-service/mqtt/data/base-readings.py`
- SHT21 MQTT topic: `sensors/sht21/readings` (from `sht21-solo` project)
