# ESP32-S3 LED Controller - HTTP REST API

Direct WiFi communication for LED control - **NO POLLING REQUIRED**

## Overview

This Arduino sketch implements an HTTP REST API server on the ESP32-S3 that allows direct control of SK6812 RGBW LED strips from a Raspberry Pi or any HTTP client. The ESP32-S3 runs a web server that responds immediately to HTTP POST requests - no polling, no MQTT broker needed.

## Features

- ✅ **Direct WiFi Communication** - HTTP REST API
- ✅ **No Polling** - ESP32-S3 responds immediately to requests
- ✅ **Multiple LED Patterns** - Rainbow, chase, fade, wave, sparkle
- ✅ **Color Control** - Full RGBW support
- ✅ **Brightness Control** - 0-255 range
- ✅ **Non-blocking** - HTTP server doesn't block main loop
- ✅ **Status Endpoint** - Get device status via GET request

## Hardware

- ESP32-S3 (lonely binary GOLD EDITION)
- SK6812 RGBW LED strip
- 5V power supply for LED strip

## Connections

```
SK6812 LED Strip:
  Data → GPIO 4 (configurable)
  VCC → 5V0 pin
  GND → GND pin
```

## Required Libraries

Install via Arduino Library Manager:

1. **Adafruit NeoPixel** by Adafruit
   - For SK6812 LED control
   - Install: `arduino-cli lib install "Adafruit NeoPixel"`

2. **ArduinoJson** by Benoit Blanchon
   - For JSON parsing
   - Install: `arduino-cli lib install "ArduinoJson"`

**Built-in libraries (no installation needed):**
- WiFi (ESP32)
- WebServer (ESP32)

## Configuration

Edit these values in `esp32-s3-led-http.ino`:

```cpp
// WiFi Configuration
const char* ssid = "YOUR_WIFI_SSID";
const char* password = "YOUR_WIFI_PASSWORD";

// SK6812 LED Strip Configuration
#define LED_PIN 4        // GPIO pin for LED data line
#define LED_COUNT 30     // Number of LEDs in strip
#define LED_BRIGHTNESS 50  // Default brightness (0-255)
```

## Compilation & Upload

### Using Arduino CLI

```powershell
cd D:\_dev\_Projects\dev-boards\Arduino

# Compile
.\arduino-cli.exe compile --fqbn esp32:esp32:esp32s3 sketchbook\esp32-s3-led-http

# Upload (replace COM3 with your port)
.\arduino-cli.exe upload -p COM3 --fqbn esp32:esp32:esp32s3 sketchbook\esp32-s3-led-http

# Monitor serial output
.\arduino-cli.exe monitor -p COM3
```

## API Endpoints

### POST `/api/led/color`
Set all LEDs to a specific color.

**Request:**
```json
{
  "r": 255,
  "g": 0,
  "b": 0,
  "w": 0
}
```

### POST `/api/led/brightness`
Set LED strip brightness.

**Request:**
```json
{
  "value": 128
}
```

### POST `/api/led/clear`
Clear all LEDs (turn off).

### POST `/api/led/pattern`
Start a LED pattern.

**Request:**
```json
{
  "name": "rainbow",
  "speed": 50
}
```

**Available patterns:**
- `rainbow` - Rainbow cycle
- `chase` - Color chase effect
- `fade` - Fade in/out
- `wave` - Wave effect
- `sparkle` - Random sparkles

### POST `/api/led/stop`
Stop current pattern.

### GET `/api/status`
Get device status.

**Response:**
```json
{
  "status": "online",
  "uptime": 12345,
  "free_heap": 250000,
  "wifi_rssi": -45,
  "ip_address": "192.168.1.100",
  "brightness": 128,
  "pattern_running": false,
  "current_pattern": ""
}
```

## Usage from Raspberry Pi

See `LED_CONTROL_GUIDE.md` in the `reference/` directory for detailed usage instructions.

### Quick Start

```bash
# Set LED to red
python3 rpi_led_controller.py --ip 192.168.1.100 --color 255 0 0

# Start rainbow pattern
python3 rpi_led_controller.py --ip 192.168.1.100 --pattern rainbow --speed 50

# Set brightness
python3 rpi_led_controller.py --ip 192.168.1.100 --brightness 128
```

## How It Works

1. **ESP32-S3 connects to WiFi** and starts HTTP server on port 80
2. **Raspberry Pi sends HTTP POST requests** directly to ESP32-S3 IP address
3. **ESP32-S3 processes request immediately** (non-blocking)
4. **LEDs update** and HTTP response sent back
5. **No polling needed** - ESP32-S3 just waits for incoming HTTP requests

## Advantages Over MQTT

- ✅ **No broker required** - Direct communication
- ✅ **Simpler setup** - Just WiFi, no MQTT server
- ✅ **Lower latency** - Direct connection
- ✅ **Standard protocol** - HTTP is universal
- ✅ **Easy debugging** - Can test with browser or curl

## Troubleshooting

### WiFi Connection Failed

- Check SSID and password
- Ensure 2.4GHz WiFi (ESP32-S3 doesn't support 5GHz)
- Check signal strength

### HTTP Server Not Responding

- Check Serial Monitor for IP address
- Verify ESP32-S3 is connected to WiFi
- Ping from RPi: `ping 192.168.1.100`
- Check firewall settings

### LEDs Not Working

- Verify GPIO pin connection (default GPIO 4)
- Check LED count matches `LED_COUNT` in code
- Ensure adequate 5V power supply
- Verify SK6812 timing (code uses NEO_KHZ800)

## Serial Monitor Output

After upload, you should see:

```
========================================
ESP32-S3 LED Controller - HTTP API
========================================

✓ SK6812 LED strip initialized
Connecting to WiFi: YOUR_WIFI_SSID
........
✓ WiFi connected!
  IP address: 192.168.1.100
  MAC address: AA:BB:CC:DD:EE:FF
✓ HTTP server started on port 80
  Access at: http://192.168.1.100

✓ Setup complete! Ready for HTTP commands...
```

## Notes

- HTTP server runs on port 80 (changeable in code)
- Server handles requests non-blocking (doesn't block main loop)
- Patterns run continuously until stopped or new command received
- Multiple commands can be sent rapidly
- No authentication by default (add if needed for security)

