/*
 * ESP32-S3 SHT21/HTU21 Sensor with MQTT
 * 
 * Simple sketch to read SHT21/HTU21 temperature and humidity
 * and publish to MQTT broker on Windows
 * 
 * Hardware:
 * - ESP32-S3 (lonely binary GOLD EDITION)
 * - SHT21/HTU21 on I2C (address 0x40)
 * 
 * Libraries Required:
 * - WiFi (built-in)
 * - Wire (built-in)
 * - MQTT_Win_Client (for MQTT connection to Windows Mosquitto)
 * - SparkFun HTU21D (for SHT21/HTU21)
 * - ArduinoJson (for JSON messages)
 */

#include <Wire.h>
#include <MQTT_Win_Client.h>
#include <SparkFunHTU21D.h>
#include <ArduinoJson.h>

// ============================================================================
// CONFIGURATION - Modify these values for your setup
// ============================================================================

// MQTT Configuration
// Note: MQTT server, port, username, and password are handled by MQTT_Win_Client library defaults
// Defaults: Server=192.168.1.10, Port=1883, Username=mqtt, Password=mqtt
const char* mqtt_client_id = "esp32-s3-sht21.2";

const char* mqtt_topic = "sensors/sht21.2/readings";
const char* mqtt_topic_status = "sensors/esp32-s3-sht21.2/status";

// I2C Configuration
// Note: ESP32-S3 doesn't have GPIO 22-25. Using GPIO 4 and 5 (both on left side)
// Any available GPIO pins can be used for I2C on ESP32-S3
#define I2C_SDA 4   // I2C SDA pin (left side, GPIO4 - also TOUCH4)
#define I2C_SCL 5   // I2C SCL pin (left side, GPIO5 - also TOUCH5)

// Sensor Configuration
HTU21D sht21;

// Reading Intervals
const unsigned long SENSOR_READ_INTERVAL = 5000;   // Read sensor every 5 seconds
const unsigned long MQTT_PUBLISH_INTERVAL = 5000;  // Publish to MQTT every 5 seconds

// ============================================================================
// GLOBAL VARIABLES
// ============================================================================

MQTT_Win_Client mqtt;

unsigned long last_sensor_read = 0;
unsigned long last_mqtt_publish = 0;

// Sensor data storage
float current_temp = 0.0;
float current_humidity = 0.0;
bool sensor_valid = false;

// ============================================================================
// SETUP
// ============================================================================

void setup() {
  Serial.begin(115200);
  delay(1000);
  
  Serial.println("\n\n========================================");
  Serial.println("ESP32-S3 SHT21/HTU21 Sensor");
  Serial.println("========================================\n");
  
  // Initialize I2C
  Wire.begin(I2C_SDA, I2C_SCL);
  Serial.println("✓ I2C initialized");
  Serial.print("  SDA: GPIO ");
  Serial.println(I2C_SDA);
  Serial.print("  SCL: GPIO ");
  Serial.println(I2C_SCL);
  
  // Scan I2C bus to find connected devices
  Serial.println("\nScanning I2C bus...");
  byte devices_found = 0;
  for (byte addr = 1; addr < 127; addr++) {
    Wire.beginTransmission(addr);
    byte error = Wire.endTransmission();
    if (error == 0) {
      Serial.print("  I2C device found at address 0x");
      if (addr < 16) Serial.print("0");
      Serial.println(addr, HEX);
      devices_found++;
      if (addr == 0x40) {
        Serial.println("    ✓ This is likely SHT21/HTU21!");
      }
    }
  }
  if (devices_found == 0) {
    Serial.println("  ✗ No I2C devices found!");
    Serial.println("  Check wiring:");
    Serial.print("    - SDA → GPIO ");
    Serial.println(I2C_SDA);
    Serial.print("    - SCL → GPIO ");
    Serial.println(I2C_SCL);
    Serial.println("    - 3.3V → 3V3 pin");
    Serial.println("    - GND → GND pin");
  } else {
    Serial.print("  Found ");
    Serial.print(devices_found);
    Serial.println(" device(s)");
  }
  Serial.println();
  
  // Initialize SHT21/HTU21
  // Note: begin() returns void, so we'll test by reading the sensor
  sht21.begin();
  Serial.println("✓ SHT21/HTU21 sensor initialized");
  Serial.println("  (Sensor will be verified on first read)");
  
  // Initialize MQTT (handles WiFi and MQTT connection)
  // Uses all defaults from MQTT_Win_Client library:
  // - WiFi: MATT_5fkj4zn / 8a4bi3bnw#y7
  // - MQTT Server: 192.168.1.10:1883
  // - MQTT Username/Password: mqtt/mqtt
  Serial.println("\n--- MQTT_Win_Client Initialization ---");
  mqtt.begin(mqtt_client_id);  // Only client_id required - everything else uses defaults
  mqtt.setStatusTopic(mqtt_topic_status);
  
  Serial.println("\n========================================");
  Serial.println("✓ Setup complete! Starting main loop...");
  Serial.println("========================================\n");
}

// ============================================================================
// MAIN LOOP
// ============================================================================

void loop() {
  unsigned long current_time = millis();
  
  // Maintain MQTT connection (library handles reconnection automatically)
  mqtt.loop();
  
  // Read sensor at specified interval
  if (current_time - last_sensor_read >= SENSOR_READ_INTERVAL) {
    readSensor();
    last_sensor_read = current_time;
  }
  
  // Publish to MQTT at specified interval
  if (current_time - last_mqtt_publish >= MQTT_PUBLISH_INTERVAL) {
    Serial.println("\n--- MQTT Publish Cycle ---");
    Serial.print("MQTT Connected: ");
    Serial.println(mqtt.connected() ? "Yes" : "No");
    Serial.print("Sensor Valid: ");
    Serial.println(sensor_valid ? "Yes" : "No");
    
    if (sensor_valid) {
      publishSensorData();
    } else {
      Serial.println("⚠ Skipping sensor data publish - sensor data invalid");
      Serial.println("  Check I2C wiring and sensor power");
    }
    
    // Always publish status (even if sensor is invalid)
    publishStatus();
    last_mqtt_publish = current_time;
  }
  
  delay(100); // Small delay to prevent watchdog issues
}

// ============================================================================
// MQTT FUNCTIONS
// ============================================================================
// Note: WiFi and MQTT connection are handled by MQTT_Win_Client library

void publishStatus() {
  // Status is automatically published by MQTT_Win_Client library
  // This function is kept for compatibility but can be removed if not needed
  StaticJsonDocument<200> doc;
  doc["status"] = mqtt.connected() ? "online" : "offline";
  doc["uptime"] = mqtt.getUptime();
  doc["free_heap"] = ESP.getFreeHeap();
  doc["sensor_valid"] = sensor_valid;
  
  String payload;
  serializeJson(doc, payload);
  
  mqtt.publish(mqtt_topic_status, payload.c_str());
}

void publishSensorData() {
  if (!mqtt.connected()) {
    Serial.println("⚠ Cannot publish - MQTT not connected");
    return;
  }
  
  StaticJsonDocument<300> doc;
  doc["temperature"] = current_temp;
  doc["humidity"] = current_humidity;
  doc["timestamp"] = millis() / 1000;
  
  String payload;
  serializeJson(doc, payload);
  
  Serial.println("\n--- Publishing to MQTT ---");
  Serial.print("  Topic: ");
  Serial.println(mqtt_topic);
  Serial.print("  Payload: ");
  Serial.println(payload);
  
  bool published = mqtt.publish(mqtt_topic, payload.c_str(), false); // false = not retained
  
  if (published) {
    Serial.println("  ✓ Published successfully");
  } else {
    Serial.println("  ✗ Publish failed!");
    Serial.print("  Connected: ");
    Serial.println(mqtt.connected() ? "Yes" : "No");
  }
  Serial.println();
}

// ============================================================================
// SENSOR READING FUNCTIONS
// ============================================================================

void readSensor() {
  float temp = sht21.readTemperature();
  float humidity = sht21.readHumidity();
  
  // Check if readings are valid (not NaN and within reasonable ranges)
  // SHT21 valid ranges: Temp -40 to 125°C, Humidity 0-100%
  if (!isnan(temp) && !isnan(humidity) && 
      temp > -40.0 && temp < 125.0 && 
      humidity >= 0.0 && humidity <= 100.0) {
    current_temp = temp;
    current_humidity = humidity;
    sensor_valid = true;
    
    Serial.print("SHT21 - Temp: ");
    Serial.print(temp, 2);
    Serial.print("°C, Humidity: ");
    Serial.print(humidity, 2);
    Serial.println("%");
  } else {
    sensor_valid = false;
    if (isnan(temp) || isnan(humidity)) {
      Serial.println("SHT21 - Read failed (NaN - sensor not responding)");
      Serial.println("  Troubleshooting:");
      Serial.println("    1. Check I2C wiring (SDA, SCL, 3.3V, GND)");
      Serial.println("    2. Verify sensor is powered (3.3V, not 5V)");
      Serial.print("    3. Check I2C pins: SDA=GPIO");
      Serial.print(I2C_SDA);
      Serial.print(", SCL=GPIO");
      Serial.println(I2C_SCL);
      Serial.println("    4. Run I2C scan on startup to verify sensor at 0x40");
    } else {
      Serial.print("SHT21 - Read failed (out of range: T=");
      Serial.print(temp);
      Serial.print(", H=");
      Serial.print(humidity);
      Serial.println(")");
      Serial.println("  This usually means sensor is not responding correctly");
      Serial.println("  Check I2C connection and power supply");
    }
  }
}
