# ESP32-S3 SHT21/HTU21 Sensor with MQTT

Simple Arduino sketch for ESP32-S3 that reads from SHT21/HTU21 sensor and publishes data to MQTT.

## Features

- ✅ **SHT21/HTU21** - Temperature and humidity sensor (I2C)
- ✅ **MQTT Publishing** - Sends sensor data to Mosquitto broker on Raspberry Pi
- ✅ **WiFi Connectivity** - Automatic reconnection handling
- ✅ **Status Monitoring** - Publishes device status and health

## Hardware Connections

### ESP32-S3 Pinout (lonely binary GOLD EDITION)

```
I2C Bus (for SHT21):
  SDA → GPIO 4
  SCL → GPIO 5
  3.3V → 3V3 pin
  GND → GND pin
```

### I2C Sensor Address

- **SHT21/HTU21**: `0x40` (fixed address)

## Required Libraries

Install these libraries via Arduino Library Manager:

1. **PubSubClient** by Nick O'Leary
   - MQTT client library
   - Install: `arduino-cli lib install "PubSubClient"`

2. **SparkFun HTU21D Humidity and Temperature Sensor Breakout** by SparkFun Electronics
   - SHT21/HTU21 sensor library
   - Install: `arduino-cli lib install "SparkFun HTU21D Humidity and Temperature Sensor Breakout"`

3. **ArduinoJson** by Benoit Blanchon
   - JSON parsing for MQTT messages
   - Install: `arduino-cli lib install "ArduinoJson"`

### Install All Libraries at Once

```powershell
cd D:\_dev\projects\dev-boards\Arduino
.\arduino-cli.exe lib install "PubSubClient" "SparkFun HTU21D Humidity and Temperature Sensor Breakout" "ArduinoJson"
```

## Configuration

Edit the configuration section in `sht21-solo.ino`:

```cpp
// WiFi Configuration
const char* ssid = "YOUR_WIFI_SSID";
const char* password = "YOUR_WIFI_PASSWORD";

// MQTT Configuration
const char* mqtt_server = "192.168.1.XXX";  // Raspberry Pi IP address
const int mqtt_port = 1883;
```

## Compilation & Upload

### Using Arduino CLI

```powershell
cd D:\_dev\projects\dev-boards\projects\esp32-s3(LB-Gold)\sht21-solo

# Compile
arduino-cli compile --fqbn esp32:esp32:esp32s3 .

# Upload (replace COM9 with your port)
arduino-cli upload -p COM9 --fqbn esp32:esp32:esp32s3 .

# Monitor serial output
arduino-cli monitor -p COM9 --config baudrate=115200
```

## MQTT Topics

### Published Topics (ESP32-S3 → Raspberry Pi)

- `sensors/sht21/readings` - SHT21 temperature and humidity readings
- `sensors/esp32-s3-sht21/status` - Device status (online/offline, uptime, memory, WiFi signal)

### MQTT Message Formats

#### SHT21 Readings
```json
{
  "temperature": 24.5,
  "humidity": 52.3,
  "timestamp": 1234567890
}
```

#### Device Status
```json
{
  "status": "online",
  "uptime": 3600,
  "free_heap": 250000,
  "wifi_rssi": -45,
  "wifi_connected": true,
  "sensor_valid": true
}
```

## Testing MQTT

### Subscribe to sensor readings (on Raspberry Pi)

```bash
# SHT21 readings
mosquitto_sub -h localhost -t "sensors/sht21/readings" -v

# Device status
mosquitto_sub -h localhost -t "sensors/esp32-s3-sht21/status" -v
```

### Test MQTT connection

```bash
# Publish a test message
mosquitto_pub -h localhost -t "test" -m "hello"
```

## Troubleshooting

### Sensor Not Detected

1. **Check I2C wiring:**
   - SDA → GPIO 4
   - SCL → GPIO 5
   - 3.3V and GND connected

2. **Check I2C scan:**
   The sketch automatically scans I2C bus on startup and reports found devices.

3. **Check sensor power:**
   - Verify 3.3V connection
   - Check GND connection

### WiFi Connection Issues

- Check SSID and password are correct
- Ensure 2.4GHz WiFi (ESP32-S3 doesn't support 5GHz)
- Check signal strength (RSSI in status messages)

### MQTT Connection Issues

- Verify Raspberry Pi IP address is correct
- Check Mosquitto is running: `systemctl status mosquitto`
- Test MQTT from Pi: `mosquitto_pub -h localhost -t "test" -m "hello"`
- Check firewall settings

### Invalid Sensor Readings

- Sensor may need time to stabilize after power-on
- Check I2C pull-up resistors (ESP32-S3 has internal pull-ups, but external 4.7kΩ may help)
- Verify sensor is not damaged

## Serial Monitor Output

Expected output:
```
========================================
ESP32-S3 SHT21/HTU21 Sensor
========================================

✓ I2C initialized
  SDA: GPIO 4
  SCL: GPIO 5
✓ SHT21/HTU21 sensor found
Connecting to WiFi: YourNetwork
........
✓ WiFi connected!
  IP address: 192.168.1.100
  Signal strength (RSSI): -45 dBm
Connecting to MQTT broker: 192.168.1.50:1883
✓ Connected to MQTT broker

✓ Setup complete! Starting main loop...

SHT21 - Temp: 24.50°C, Humidity: 52.30%
SHT21 - Temp: 24.52°C, Humidity: 52.28%
✓ Published to MQTT: {"temperature":24.50,"humidity":52.30,"timestamp":1234567890}
```

## Notes

- Sensor readings are taken every 5 seconds
- MQTT publishes every 5 seconds (configurable)
- Status is published every 5 seconds along with sensor data
- I2C pins: GPIO 4 (SDA) and GPIO 5 (SCL)
- Serial output shows all sensor readings for debugging

