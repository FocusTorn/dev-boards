# SHT21 MQTT Example - Using MQTT_RPi_Client Library

Example sketch demonstrating how to use the `MQTT_RPi_Client` library for easy MQTT connectivity.

## Features

- Uses the reusable `MQTT_RPi_Client` library
- Much simpler code - library handles all WiFi/MQTT connection details
- Automatic reconnection
- Status publishing handled automatically

## Comparison

**Before (without library):** ~350 lines of code with WiFi and MQTT management mixed in

**After (with library):** ~200 lines - just sensor reading and publishing logic

## Usage

This example shows how simple it is to use the library:

```cpp
#include <MQTT_RPi_Client.h>

MQTT_RPi_Client mqtt;

void setup() {
  mqtt.begin(
    "WiFi_SSID",
    "WiFi_Password",
    "192.168.1.50",  // MQTT server
    1883,            // MQTT port
    "esp32",         // MQTT username
    "password123",   // MQTT password
    "client-id"      // Client ID
  );
  
  mqtt.setStatusTopic("sensors/esp32-s3/status");
}

void loop() {
  mqtt.loop();  // Maintains connection
  
  if (mqtt.connected()) {
    mqtt.publishJSON("sensors/readings", jsonDoc);
  }
}
```

That's it! The library handles all the connection management.

