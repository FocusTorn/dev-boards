# ESP32-S3 Multi-Sensor & LED Controller

Arduino sketch for ESP32-S3 that reads from SHT21/HTU21 and BME680 sensors, controls SK6812 RGBW LED strips, and publishes data to MQTT.

## Features

- ✅ **SHT21/HTU21** - Temperature and humidity sensor (I2C)
- ✅ **BME680** - Temperature, humidity, pressure, and gas sensor (I2C)
- ✅ **SK6812 RGBW LED Strip** - Full color control with RGBW support
- ✅ **MQTT Publishing** - Sends sensor data to Mosquitto broker on Raspberry Pi
- ✅ **MQTT Subscribing** - Receives LED control commands via MQTT
- ✅ **WiFi Connectivity** - Automatic reconnection handling

## Hardware Connections

### ESP32-S3 Pinout (lonely binary GOLD EDITION)

```
I2C Bus (for SHT21 & BME680):
  SDA → GPIO 21
  SCL → GPIO 22
  3.3V → 3V3 pin
  GND → GND pin

SK6812 LED Strip:
  Data → GPIO 4 (configurable in code)
  VCC → 5V0 pin
  GND → GND pin
```

### I2C Sensor Addresses

- **SHT21/HTU21**: `0x40` (fixed)
- **BME680**: `0x77` (default, if SDO → 3.3V) or `0x76` (if SDO → GND)

## Required Libraries

Install these libraries via Arduino Library Manager:

1. **PubSubClient** by Nick O'Leary
   - MQTT client library
   - Install: `arduino-cli lib install "PubSubClient"`

2. **Adafruit NeoPixel** by Adafruit
   - SK6812 RGBW LED strip control
   - Install: `arduino-cli lib install "Adafruit NeoPixel"`

3. **SparkFun HTU21D** by SparkFun Electronics
   - SHT21/HTU21 sensor library
   - Install: `arduino-cli lib install "SparkFun HTU21D"`

4. **BME680_Custom** (Custom Library - Included)
   - Custom BME680 library based on official Bosch implementation
   - Includes baseline calibration and IAQ score calculation
   - Located in: `D:\_dev\projects\dev-boards\Arduino\libraries\BME680_Custom\`
   - No installation needed - Arduino IDE will find it automatically

5. **ArduinoJson** by Benoit Blanchon
   - JSON parsing for MQTT messages
   - Install: `arduino-cli lib install "ArduinoJson"`

### Install All Libraries at Once

```powershell
cd D:\_dev\projects\dev-boards\Arduino
.\arduino-cli.exe lib install "PubSubClient" "Adafruit NeoPixel" "SparkFun HTU21D" "ArduinoJson"
```

**Note:** The custom BME680 library is already included in the project and doesn't need to be installed separately.

## Configuration

Edit the configuration section in `sht21-bme680-led-mqtt.ino`:

```cpp
// WiFi Configuration
const char* ssid = "YOUR_WIFI_SSID";
const char* password = "YOUR_WIFI_PASSWORD";

// MQTT Configuration
const char* mqtt_server = "192.168.1.XXX";  // Raspberry Pi IP address
const int mqtt_port = 1883;

// LED Configuration
#define LED_PIN 4        // GPIO pin for LED data line
#define LED_COUNT 30     // Number of LEDs in strip
```

## Compilation & Upload

### Using Arduino CLI

```powershell
cd D:\_dev\projects\dev-boards\Arduino

# Compile
.\arduino-cli.exe compile --fqbn esp32:esp32:esp32s3 sketchbook\sht21-bme680-led-mqtt

# Upload (replace COM3 with your port)
.\arduino-cli.exe upload -p COM3 --fqbn esp32:esp32:esp32s3 sketchbook\sht21-bme680-led-mqtt

# Monitor serial output
.\arduino-cli.exe monitor -p COM3
```

## MQTT Topics

### Published Topics (ESP32-S3 → Raspberry Pi)

- `sensors/sht21/readings` - SHT21 temperature and humidity
- `sensors/bme680/readings` - BME680 temperature, humidity, pressure, gas
- `sensors/esp32-s3/status` - Device status (online/offline, uptime, memory)

### Subscribed Topics (Raspberry Pi → ESP32-S3)

- `sensors/esp32-s3/led/control` - LED control commands

### MQTT Message Formats

#### SHT21 Readings
```json
{
  "temperature": 24.5,
  "humidity": 52.3,
  "timestamp": 1234567890
}
```

#### BME680 Readings
```json
{
  "temperature": 24.5,
  "humidity": 52.3,
  "pressure": 1013.25,
  "gas_resistance": 12.5,
  "heat_stable": true,
  "baseline_established": true,
  "iaq_score": 85.5,
  "gas_baseline": 12500.0,
  "hum_baseline": 45.2,
  "safe_to_open": true,
  "timestamp": 1234567890
}
```

**Note:** IAQ data (iaq_score, gas_baseline, hum_baseline, safe_to_open) is only included if baseline calibration has been performed.

#### LED Control Commands

Set color:
```json
{
  "action": "set_color",
  "r": 255,
  "g": 0,
  "b": 0,
  "w": 0
}
```

Set brightness:
```json
{
  "action": "set_brightness",
  "value": 128
}
```

Clear LEDs:
```json
{
  "action": "clear"
}
```

## Testing MQTT

### Subscribe to sensor readings (on Raspberry Pi)

```bash
# SHT21 readings
mosquitto_sub -h localhost -t "sensors/sht21/readings" -v

# BME680 readings
mosquitto_sub -h localhost -t "sensors/bme680/readings" -v

# Device status
mosquitto_sub -h localhost -t "sensors/esp32-s3/status" -v
```

### Control LED from Raspberry Pi

```bash
# Set LED to red
mosquitto_pub -h localhost -t "sensors/esp32-s3/led/control" -m '{"action":"set_color","r":255,"g":0,"b":0,"w":0}'

# Set brightness
mosquitto_pub -h localhost -t "sensors/esp32-s3/led/control" -m '{"action":"set_brightness","value":128}'

# Clear LEDs
mosquitto_pub -h localhost -t "sensors/esp32-s3/led/control" -m '{"action":"clear"}'
```

### Calibrate BME680 Baseline (for IAQ)

The BME680 requires baseline calibration for accurate IAQ readings. This should be done once when the environment is in a normal state (not printing, no unusual odors).

```bash
# Start baseline calibration (300 seconds = 5 minutes)
mosquitto_pub -h localhost -t "sensors/esp32-s3/bme680/calibrate" -m '{"action":"calibrate","duration":300}'
```

The calibration will run for the specified duration and establish baseline values for gas resistance and humidity. After calibration, IAQ scores will be calculated and included in sensor readings.

## Troubleshooting

### Sensors Not Detected

1. **Check I2C wiring:**
   - SDA → GPIO 21
   - SCL → GPIO 22
   - 3.3V and GND connected

2. **Check I2C addresses:**
   ```cpp
   // Scan I2C bus (add to setup())
   Wire.begin();
   for (byte addr = 1; addr < 127; addr++) {
     Wire.beginTransmission(addr);
     if (Wire.endTransmission() == 0) {
       Serial.printf("I2C device found at 0x%02X\n", addr);
     }
   }
   ```

3. **BME680 address:**
   - Try both `0x76` and `0x77` in code
   - Check SDO pin connection (GND = 0x76, 3.3V = 0x77)

### WiFi Connection Issues

- Check SSID and password are correct
- Ensure 2.4GHz WiFi (ESP32-S3 doesn't support 5GHz)
- Check signal strength

### MQTT Connection Issues

- Verify Raspberry Pi IP address is correct
- Check Mosquitto is running: `systemctl status mosquitto`
- Test MQTT from Pi: `mosquitto_pub -h localhost -t "test" -m "hello"`

### LED Strip Not Working

- Check data pin connection (default GPIO 4)
- Verify 5V power supply is adequate (SK6812 needs 5V, not 3.3V)
- Check LED count matches `LED_COUNT` in code
- Try different GPIO pin if needed

## Integration with Home Assistant

The MQTT messages are compatible with Home Assistant MQTT sensors. Add to your `configuration.yaml`:

```yaml
mqtt:
  sensor:
    - name: "SHT21 Temperature"
      state_topic: "sensors/sht21/readings"
      value_template: "{{ value_json.temperature }}"
      unit_of_measurement: "°C"
      
    - name: "SHT21 Humidity"
      state_topic: "sensors/sht21/readings"
      value_template: "{{ value_json.humidity }}"
      unit_of_measurement: "%"
      
    - name: "BME680 Temperature"
      state_topic: "sensors/bme680/readings"
      value_template: "{{ value_json.temperature }}"
      unit_of_measurement: "°C"
      
    - name: "BME680 Humidity"
      state_topic: "sensors/bme680/readings"
      value_template: "{{ value_json.humidity }}"
      unit_of_measurement: "%"
      
    - name: "BME680 Pressure"
      state_topic: "sensors/bme680/readings"
      value_template: "{{ value_json.pressure }}"
      unit_of_measurement: "hPa"
      
    - name: "BME680 Gas Resistance"
      state_topic: "sensors/bme680/readings"
      value_template: "{{ value_json.gas_resistance }}"
      unit_of_measurement: "kOhm"
```

## BME680 IAQ (Indoor Air Quality)

The custom BME680 library includes IAQ calculation based on your Python implementation:

- **Baseline Calibration**: Required before IAQ scores are calculated. Run calibration when environment is in normal state.
- **IAQ Score**: Calculated from gas resistance and humidity, weighted 75% gas / 25% humidity
- **Safe to Open**: Threshold check (default 80.0) to determine if enclosure is safe to open
- **Heat Stable Detection**: Only publishes readings when sensor is heat stable

### IAQ Calculation Details

The IAQ score uses the same algorithm as your Python implementation:
- Gas score: `(gas_resistance / gas_baseline) * 75`
- Humidity score: Calculated based on humidity offset from baseline * 25
- Total IAQ: `gas_score + humidity_score` (higher = better air quality)

## Notes

- Sensor readings are taken every 5 seconds
- MQTT publishes every 30 seconds (configurable)
- BME680 requires heat stabilization - readings may fail if sensor is not stable
- **BME680 baseline calibration is required for IAQ** - run calibration via MQTT or in setup()
- LED strip uses 5V power - ensure adequate power supply for long strips
- Default I2C pins can be changed in code if needed
- Custom BME680 library uses official Bosch calibration algorithms for maximum accuracy

