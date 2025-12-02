# ESP32-S3 LED Control Guide

## Direct WiFi Communication - HTTP REST API

This guide explains how to control the ESP32-S3 LED strip directly from the Raspberry Pi using HTTP REST API. **No polling required** - the RPi sends commands directly to the ESP32-S3 over WiFi.

## Architecture

```
Raspberry Pi                    ESP32-S3
     |                              |
     |  HTTP POST Request           |
     |--------------------------->  |
     |                              |  Process command
     |                              |  Update LEDs
     |  HTTP Response (JSON)        |
     |<---------------------------  |
```

**Key Benefits:**
- ✅ **No polling** - ESP32-S3 doesn't need to check for messages
- ✅ **Direct communication** - No MQTT broker required
- ✅ **Low latency** - Immediate response
- ✅ **Simple** - Standard HTTP requests
- ✅ **Push-based** - RPi sends commands when needed

## Setup

### 1. Upload Arduino Sketch

Upload `esp32-s3-led-http.ino` to your ESP32-S3. The sketch will:
- Connect to WiFi
- Start an HTTP server on port 80
- Print the IP address to Serial Monitor

### 2. Find ESP32-S3 IP Address

After upload, check the Serial Monitor. You'll see:
```
✓ WiFi connected!
  IP address: 192.168.1.100
```

**Note the IP address** - you'll need it for the RPi script.

### 3. Install Python Dependencies on Raspberry Pi

```bash
pip3 install requests
```

## Usage

### Python Script (Recommended)

Use the provided `rpi_led_controller.py` script:

```bash
# Set LED to red
python3 rpi_led_controller.py --ip 192.168.1.100 --color 255 0 0

# Set LED to white (using white channel)
python3 rpi_led_controller.py --ip 192.168.1.100 --color 0 0 0 255

# Set brightness
python3 rpi_led_controller.py --ip 192.168.1.100 --brightness 128

# Start rainbow pattern
python3 rpi_led_controller.py --ip 192.168.1.100 --pattern rainbow --speed 50

# Start chase pattern
python3 rpi_led_controller.py --ip 192.168.1.100 --pattern chase --speed 30

# Clear LEDs
python3 rpi_led_controller.py --ip 192.168.1.100 --clear

# Stop current pattern
python3 rpi_led_controller.py --ip 192.168.1.100 --stop

# Get device status
python3 rpi_led_controller.py --ip 192.168.1.100 --status
```

### Direct HTTP Requests (curl)

You can also use `curl` directly:

```bash
# Set color (red)
curl -X POST http://192.168.1.100/api/led/color \
  -H "Content-Type: application/json" \
  -d '{"r":255,"g":0,"b":0,"w":0}'

# Set brightness
curl -X POST http://192.168.1.100/api/led/brightness \
  -H "Content-Type: application/json" \
  -d '{"value":128}'

# Start pattern
curl -X POST http://192.168.1.100/api/led/pattern \
  -H "Content-Type: application/json" \
  -d '{"name":"rainbow","speed":50}'

# Clear LEDs
curl -X POST http://192.168.1.100/api/led/clear

# Get status
curl http://192.168.1.100/api/status
```

### Python Code Example

```python
import requests

# Set LED to red
response = requests.post(
    "http://192.168.1.100/api/led/color",
    json={"r": 255, "g": 0, "b": 0, "w": 0}
)
print(response.json())

# Start rainbow pattern
response = requests.post(
    "http://192.168.1.100/api/led/pattern",
    json={"name": "rainbow", "speed": 50}
)
print(response.json())
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

**Response:**
```json
{
  "status": "ok",
  "color": {
    "r": 255,
    "g": 0,
    "b": 0,
    "w": 0
  }
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

**Response:**
```json
{
  "status": "ok",
  "brightness": 128
}
```

### POST `/api/led/clear`
Clear all LEDs (turn off).

**Response:**
```json
{
  "status": "ok",
  "message": "LEDs cleared"
}
```

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

**Response:**
```json
{
  "status": "ok",
  "pattern": "rainbow",
  "speed": 50
}
```

### POST `/api/led/stop`
Stop current pattern.

**Response:**
```json
{
  "status": "ok",
  "message": "Pattern stopped"
}
```

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

## Comparison: HTTP vs MQTT

| Feature | HTTP REST API | MQTT |
|---------|---------------|------|
| **Polling Required** | ❌ No | ❌ No (but ESP32 must subscribe) |
| **Broker Required** | ❌ No | ✅ Yes (Mosquitto) |
| **Latency** | Very low | Low |
| **Complexity** | Simple | Moderate |
| **Direct Communication** | ✅ Yes | ❌ No (via broker) |
| **Best For** | Direct control | Multiple devices, pub/sub |

**HTTP REST API is better for:**
- Direct RPi → ESP32-S3 communication
- Simple command/response
- No broker setup needed
- Immediate response

**MQTT is better for:**
- Multiple devices
- Pub/sub patterns
- Integration with Home Assistant
- When you already have MQTT infrastructure

## Troubleshooting

### ESP32-S3 Not Responding

1. **Check IP address** - Verify ESP32-S3 is connected to WiFi
2. **Check Serial Monitor** - Look for connection errors
3. **Ping test** - `ping 192.168.1.100` from RPi
4. **Firewall** - Ensure port 80 is not blocked

### Connection Timeout

1. **Check WiFi** - Ensure both devices on same network
2. **Check IP** - Verify IP address is correct
3. **Check Port** - Default is 80, change if needed

### LEDs Not Updating

1. **Check GPIO pin** - Verify LED_PIN in code matches hardware
2. **Check LED count** - Verify LED_COUNT matches your strip
3. **Check power** - Ensure adequate 5V power supply

## Advanced Usage

### Continuous Pattern Updates

```python
import requests
import time

ip = "192.168.1.100"

# Start rainbow pattern
requests.post(f"http://{ip}/api/led/pattern", 
              json={"name": "rainbow", "speed": 50})

# Let it run for 10 seconds
time.sleep(10)

# Change to chase pattern
requests.post(f"http://{ip}/api/led/pattern", 
              json={"name": "chase", "speed": 30})

# Stop after 5 seconds
time.sleep(5)
requests.post(f"http://{ip}/api/led/stop")
```

### Color Transitions

```python
import requests
import time

ip = "192.168.1.100"

# Smooth transition from red to blue
for i in range(256):
    r = 255 - i
    b = i
    requests.post(f"http://{ip}/api/led/color",
                  json={"r": r, "g": 0, "b": b, "w": 0})
    time.sleep(0.01)  # 10ms delay
```

## Notes

- HTTP server runs on port 80 by default (changeable in code)
- ESP32-S3 handles requests non-blocking (doesn't block main loop)
- Patterns run continuously until stopped or new command received
- Multiple commands can be sent rapidly
- No authentication by default (add if needed for security)

