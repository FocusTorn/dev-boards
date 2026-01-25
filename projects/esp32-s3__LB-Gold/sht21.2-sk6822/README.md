# SHT21.2 + SK6822 Controller

This project combines SHT21 sensor readings with dual-channel SK6812 RGBW LED control for the ESP32-S3.

## Hardware Setup
*   **SDA:** GPIO 4
*   **SCL:** GPIO 5
*   **Command Strip:** GPIO 12 (31 LEDs)
*   **Status Strip:** GPIO 17 (30 LEDs)

## MQTT Topics

### Subscribed
*   `e32s3/sk6822/command`: Send JSON commands here.

### Published
*   `e32s3/sk6822/status`: Controller status.
*   `e32s3.1/sht21.2/readings`: Sensor data `{"temperature": 25.5, "humidity": 50.2, "valid": true}`.
*   `e32s3.1/sht21.2/status`: Sensor specific status.

## JSON Command Reference

Send these JSON payloads to `e32s3/sk6822/command`.

### 1. Set Color (Whole Strip)
Target defaults to "command" if omitted.
```json
{
  "target": "command", 
  "r": 255,
  "g": 0,
  "b": 0,
  "w": 50
}
```

### 2. Set Range of LEDs
Sets LEDs 0 through 10 (inclusive) to Blue.
```json
{
  "target": "status",
  "start": 0,
  "end": 10,
  "r": 0,
  "g": 0,
  "b": 255
}
```

### 3. Set Brightness
Global brightness for the strip (0-255).
```json
{
  "target": "command",
  "brightness": 200
}
```

### 4. Clear Strip
Turns off all LEDs.
```json
{
  "target": "both",
  "action": "clear"
}
```
