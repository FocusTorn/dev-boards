# MQTT_RPi_Client Library

Reusable MQTT client library for ESP32 devices to connect to Raspberry Pi Mosquitto broker with authentication.

## Features

- ✅ WiFi connection management
- ✅ MQTT connection with username/password authentication
- ✅ Automatic reconnection handling
- ✅ Status publishing (uptime, memory, WiFi signal, etc.)
- ✅ Simple publish/subscribe interface
- ✅ JSON message support
- ✅ Connection state monitoring

## Installation

This library is located in `D:\_dev\_Projects\dev-boards\_libs\esp32-s3\MQTT_RPi_Client\`.

To use with Arduino CLI, add the library path to your `arduino-cli.yaml` or use the `--library` flag:

```powershell
arduino-cli compile --fqbn esp32:esp32:esp32s3 --library "D:\_dev\_Projects\dev-boards\_libs\esp32-s3" your_sketch
```

Or add it to your Arduino IDE library path in Preferences.

## Dependencies

- **WiFi** (built-in ESP32)
- **PubSubClient** - Install via: `arduino-cli lib install "PubSubClient"`
- **ArduinoJson** - Install via: `arduino-cli lib install "ArduinoJson"`

## Basic Usage

```cpp
#include <MQTT_RPi_Client.h>
#include <ArduinoJson.h>

MQTT_RPi_Client mqtt;

void setup() {
  Serial.begin(115200);
  
  // Initialize MQTT client
  // Parameters: WiFi SSID, WiFi Password, MQTT Server, MQTT Port, 
  //             MQTT Username, MQTT Password, Client ID
  mqtt.begin(
    "YOUR_WIFI_SSID",
    "YOUR_WIFI_PASSWORD",
    "192.168.1.50",      // Raspberry Pi IP
    1883,                // MQTT port
    "esp32",             // MQTT username
    "password123",       // MQTT password
    "esp32-s3-client"    // Client ID
  );
  
  // Set status topic (optional - for device status publishing)
  mqtt.setStatusTopic("sensors/esp32-s3/status");
  
  // Subscribe to topics (optional)
  mqtt.subscribe("sensors/esp32-s3/commands");
}

void loop() {
  // Must call loop() to maintain connection and process messages
  mqtt.loop();
  
  // Publish messages
  if (mqtt.connected()) {
    mqtt.publish("sensors/temperature", "24.5");
  }
}
```

## Advanced Usage

### Publishing JSON Messages

```cpp
void loop() {
  mqtt.loop();
  
  if (mqtt.connected()) {
    StaticJsonDocument<200> doc;
    doc["temperature"] = 24.5;
    doc["humidity"] = 52.3;
    doc["timestamp"] = millis() / 1000;
    
    mqtt.publishJSON("sensors/readings", doc);
  }
}
```

### Receiving Messages (Callbacks)

```cpp
void mqttCallback(char* topic, byte* payload, unsigned int length) {
  Serial.print("Message received on topic: ");
  Serial.println(topic);
  
  String message = "";
  for (int i = 0; i < length; i++) {
    message += (char)payload[i];
  }
  Serial.print("Message: ");
  Serial.println(message);
  
  // Process message here
}

void setup() {
  // ... other setup code ...
  
  mqtt.setCallback(mqttCallback);
  mqtt.subscribe("sensors/esp32-s3/commands");
}
```

### Configuration Options

```cpp
void setup() {
  mqtt.begin(/* ... parameters ... */);
  
  // Set status publish interval (default: 30 seconds)
  mqtt.setStatusInterval(60000);  // 60 seconds
  
  // Disable serial debug output
  mqtt.enableSerialDebug(false);
}
```

## API Reference

### Methods

#### `begin(wifi_ssid, wifi_password, mqtt_server, mqtt_port, mqtt_username, mqtt_password, mqtt_client_id)`
Initialize and connect to WiFi and MQTT broker.

#### `loop()`
Must be called in your `loop()` function. Maintains connections and processes messages.

#### `connected()`
Returns `true` if MQTT is connected.

#### `wifiConnected()`
Returns `true` if WiFi is connected.

#### `publish(topic, payload, retain)`
Publish a message to MQTT topic. Returns `true` on success.

#### `publishJSON(topic, doc, retain)`
Publish a JSON document to MQTT topic.

#### `subscribe(topic)`
Subscribe to an MQTT topic.

#### `setCallback(callback)`
Set callback function for incoming messages.

#### `setStatusTopic(topic)`
Set topic for automatic status publishing.

#### `setStatusInterval(interval_ms)`
Set interval for status publishing (default: 30000ms).

#### `enableSerialDebug(enable)`
Enable/disable serial debug output (default: true).

#### `getIPAddress()`
Get current WiFi IP address as String.

#### `getRSSI()`
Get WiFi signal strength (RSSI).

#### `getUptime()`
Get device uptime in seconds.

#### `reconnect()`
Manually trigger MQTT reconnection.

## Example: SHT21 Sensor with MQTT

```cpp
#include <MQTT_RPi_Client.h>
#include <SparkFunHTU21D.h>
#include <ArduinoJson.h>

MQTT_RPi_Client mqtt;
HTU21D sht21;

void setup() {
  Serial.begin(115200);
  
  // Initialize sensor
  Wire.begin(4, 5);  // SDA, SCL
  sht21.begin();
  
  // Initialize MQTT
  mqtt.begin(
    "YOUR_WIFI_SSID",
    "YOUR_WIFI_PASSWORD",
    "192.168.1.50",
    1883,
    "esp32",
    "password123",
    "esp32-s3-sht21"
  );
  
  mqtt.setStatusTopic("sensors/esp32-s3-sht21/status");
}

void loop() {
  mqtt.loop();
  
  // Read sensor
  float temp = sht21.readTemperature();
  float humidity = sht21.readHumidity();
  
  // Publish to MQTT
  if (mqtt.connected() && !isnan(temp) && !isnan(humidity)) {
    StaticJsonDocument<200> doc;
    doc["temperature"] = temp;
    doc["humidity"] = humidity;
    doc["timestamp"] = millis() / 1000;
    
    mqtt.publishJSON("sensors/sht21/readings", doc);
  }
  
  delay(5000);
}
```

## Status Message Format

When status topic is set, the library automatically publishes:

```json
{
  "status": "online",
  "uptime": 3600,
  "free_heap": 250000,
  "wifi_rssi": -45,
  "wifi_connected": true,
  "ip_address": "192.168.1.100"
}
```

## Notes

- The library handles automatic reconnection for both WiFi and MQTT
- Status publishing happens automatically at the configured interval
- All connection details are printed to Serial (can be disabled)
- Designed specifically for Raspberry Pi Mosquitto broker with authentication

