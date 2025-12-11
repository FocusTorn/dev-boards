# ESP32-S3 SK6812 LED Controller - MQTT

MQTT-based LED control using MQTT_RPi_Client library. Integrates with Raspberry Pi Mosquitto broker and Home Assistant.

## Overview

This Arduino sketch implements MQTT pub/sub for controlling SK6812 RGBW LED strips from a Raspberry Pi or Home Assistant. Uses the `MQTT_RPi_Client` library for simplified MQTT connection management.

## Features

- ✅ **MQTT Pub/Sub** - Subscribe to commands, publish state
- ✅ **Home Assistant Compatible** - Supports HA MQTT Light format
- ✅ **Multiple LED Patterns** - Rainbow, chase, fade, wave, sparkle
- ✅ **State Publishing** - Publishes current LED state
- ✅ **Status Publishing** - Automatic device status (via library)
- ✅ **Event-Driven** - No polling, immediate response to commands

## Hardware

- ESP32-S3 (lonely binary GOLD EDITION)
- SK6812 RGBW LED strip
- 5V power supply for LED strip

## Connections

```
SK6812 LED Strip:
  Data (Blue wire) → GPIO 6 (configurable)
  VCC (Red wire) → 5V0 pin
  GND (Black wire) → GND pin
```

## Required Libraries

Install via Arduino Library Manager:

1. **Adafruit NeoPixel** by Adafruit
   - For SK6812 LED control
   - Install: `arduino-cli lib install "Adafruit NeoPixel"`

2. **ArduinoJson** by Benoit Blanchon
   - For JSON parsing
   - Install: `arduino-cli lib install "ArduinoJson"`

3. **PubSubClient** by Nick O'Leary
   - MQTT client (dependency of MQTT_RPi_Client)
   - Install: `arduino-cli lib install "PubSubClient"`

**Included library (no installation needed):**
- **MQTT_RPi_Client** - Located in `D:\_dev\projects\dev-boards\lib\esp32-s3\MQTT_RPi_Client\`

## Configuration

Edit these values in `sk6822-solo.ino`:

```cpp
// WiFi Configuration
const char* wifi_ssid = "YOUR_WIFI_SSID";
const char* wifi_password = "YOUR_WIFI_PASSWORD";

// MQTT Configuration
const char* mqtt_server = "192.168.1.50";  // Raspberry Pi IP
const char* mqtt_username = "esp32";
const char* mqtt_password = "password123";
const char* mqtt_client_id = "esp32-s3-led-controller";

// SK6812 LED Strip Configuration
#define LED_PIN 6        // GPIO pin for LED data line (blue wire)
#define LED_COUNT 31     // Number of LEDs in strip
#define LED_BRIGHTNESS 50  // Default brightness (0-255)
```

## Compilation & Upload

### Using Arduino CLI

```powershell
cd D:\_dev\projects\dev-boards\projects\esp32-s3(LB-Gold)\sk6822-solo

# Compile
arduino-cli compile --fqbn esp32:esp32:esp32s3 .

# Upload (replace COM9 with your port)
arduino-cli upload -p COM9 --fqbn esp32:esp32:esp32s3 .

# Monitor serial output
arduino-cli monitor -p COM9 --config baudrate=115200
```

## MQTT Topics

### Subscribe (ESP32-S3 receives commands)
- `sensors/esp32-s3-led/command` - LED control commands

### Publish (ESP32-S3 sends updates)
- `sensors/esp32-s3-led/state` - Current LED state
- `sensors/esp32-s3-led/status` - Device status (automatic, via library)

## MQTT Message Formats

### Command: Set Color

**Topic:** `sensors/esp32-s3-led/command`

**Message:**
```json
{
  "action": "set_color",
  "r": 255,
  "g": 0,
  "b": 0,
  "w": 0
}
```

### Command: Set Brightness

**Topic:** `sensors/esp32-s3-led/command`

**Message:**
```json
{
  "action": "set_brightness",
  "value": 128
}
```

### Command: Start Pattern

**Topic:** `sensors/esp32-s3-led/command`

**Message:**
```json
{
  "action": "pattern",
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

### Command: Stop Pattern

**Topic:** `sensors/esp32-s3-led/command`

**Message:**
```json
{
  "action": "stop"
}
```

### Command: Clear LEDs

**Topic:** `sensors/esp32-s3-led/command`

**Message:**
```json
{
  "action": "clear"
}
```

### State (Published by ESP32-S3)

**Topic:** `sensors/esp32-s3-led/state`

**Message:**
```json
{
  "r": 255,
  "g": 0,
  "b": 0,
  "w": 0,
  "brightness": 128,
  "pattern": "rainbow",
  "pattern_active": true
}
```

### Home Assistant Format

**Topic:** `sensors/esp32-s3-led/command`

**Message:**
```json
{
  "state": "ON",
  "brightness": 128,
  "color": {
    "r": 255,
    "g": 0,
    "b": 0,
    "w": 0
  }
}
```

## Usage from Raspberry Pi

### Using mosquitto_pub

```bash
# Set LED to red
mosquitto_pub -h localhost -t "sensors/esp32-s3-led/command" \
  -m '{"action":"set_color","r":255,"g":0,"b":0,"w":0}'

# Set brightness
mosquitto_pub -h localhost -t "sensors/esp32-s3-led/command" \
  -m '{"action":"set_brightness","value":128}'

# Start rainbow pattern
mosquitto_pub -h localhost -t "sensors/esp32-s3-led/command" \
  -m '{"action":"pattern","name":"rainbow","speed":50}'

# Stop pattern
mosquitto_pub -h localhost -t "sensors/esp32-s3-led/command" \
  -m '{"action":"stop"}'

# Clear LEDs
mosquitto_pub -h localhost -t "sensors/esp32-s3-led/command" \
  -m '{"action":"clear"}'
```

### Using Python

See `rpi_led_controller_mqtt.py` in the `reference/` directory for a complete Python script.

## Home Assistant Integration

Add to your `configuration.yaml`:

```yaml
mqtt:
  light:
    - name: "ESP32-S3 LED Strip"
      state_topic: "sensors/esp32-s3-led/state"
      command_topic: "sensors/esp32-s3-led/command"
      brightness: true
      rgb: true
      white_value: true
      schema: json
      state_value_template: "{{ value_json.state }}"
      brightness_value_template: "{{ value_json.brightness }}"
      rgb_value_template: "{{ value_json.r }},{{ value_json.g }},{{ value_json.b }}"
      white_value_template: "{{ value_json.w }}"
```

## How It Works

1. **ESP32-S3 connects to WiFi** and MQTT broker (via MQTT_RPi_Client library)
2. **Subscribes to command topic** (`sensors/esp32-s3-led/command`)
3. **Raspberry Pi publishes commands** to the command topic
4. **MQTT broker pushes message** to ESP32-S3 (event-driven, no polling)
5. **Callback function processes command** immediately
6. **LEDs update** and state is published back

## Advantages

- ✅ **Event-driven** - No polling, immediate response
- ✅ **Scalable** - Easy to add more ESP32 devices
- ✅ **Decoupled** - RPi doesn't need to know ESP32 IP
- ✅ **Home Assistant ready** - Built-in integration support
- ✅ **State tracking** - Publishes current LED state
- ✅ **Automatic reconnection** - Library handles WiFi/MQTT reconnection

## Troubleshooting

### MQTT Connection Failed

- Check MQTT server IP address
- Verify username/password are correct
- Ensure Mosquitto is running: `systemctl status mosquitto`
- Check firewall settings

### Messages Not Received

- Verify subscription: Check Serial Monitor for "✓ Subscribed to: ..."
- Check topic name matches exactly
- Test with: `mosquitto_sub -h localhost -t "sensors/esp32-s3-led/command" -v`

### LEDs Not Working

- Verify GPIO pin connection (default GPIO 6 for blue wire)
- Check LED count matches `LED_COUNT` in code
- Ensure adequate 5V power supply
- Verify SK6812 timing (code uses NEO_KHZ800)

## Serial Monitor Output

After upload, you should see:

```
========================================
ESP32-S3 LED Controller - MQTT
========================================

✓ SK6812 LED strip initialized
--- MQTT_RPi_Client Initialization ---
Connecting to WiFi: YOUR_WIFI_SSID
........
✓ WiFi connected!
  IP address: 192.168.1.100
Connecting to MQTT broker: 192.168.1.50:1883
✓ Connected to MQTT broker
✓ Subscribed to: sensors/esp32-s3-led/command

✓ Setup complete! Ready for MQTT commands...
```

## Notes

- MQTT_RPi_Client library handles automatic reconnection
- Status publishing happens automatically every 30 seconds (configurable)
- State is published after every command
- Patterns run continuously until stopped or new command received
- Home Assistant format is supported for easy integration

