/*
 * ESP32-S3 SHT21/HTU21 Sensor with MQTT - Using MQTT_RPi_Client Library
 * 
 * Example sketch using the reusable MQTT_RPi_Client library
 * 
 * Hardware:
 * - ESP32-S3 (lonely binary GOLD EDITION)
 * - SHT21/HTU21 on I2C (address 0x40)
 */

#include <Wire.h>
#include <MQTT_RPi_Client.h>
#include <SparkFunHTU21D.h>
#include <ArduinoJson.h>

// ============================================================================
// CONFIGURATION
// ============================================================================

// WiFi Configuration
const char* wifi_ssid = "MATT_5fkj4zn";
const char* wifi_password = "8a4bi3bnw#y7";

// MQTT Configuration
const char* mqtt_server = "192.168.1.50";
const int mqtt_port = 1883;
const char* mqtt_username = "esp32";
const char* mqtt_password = "password123";
const char* mqtt_client_id = "esp32-s3-sht21";

// Topics
const char* mqtt_topic_readings = "sensors/sht21/readings";
const char* mqtt_topic_status = "sensors/esp32-s3-sht21/status";

// I2C Configuration
#define I2C_SDA 4
#define I2C_SCL 5

// Intervals
const unsigned long SENSOR_READ_INTERVAL = 5000;   // 5 seconds
const unsigned long MQTT_PUBLISH_INTERVAL = 5000;   // 5 seconds

// ============================================================================
// GLOBAL OBJECTS
// ============================================================================

MQTT_RPi_Client mqtt;
HTU21D sht21;

unsigned long last_sensor_read = 0;
unsigned long last_mqtt_publish = 0;

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
  Serial.println("ESP32-S3 SHT21/HTU21 with MQTT Library");
  Serial.println("========================================\n");
  
  // Initialize I2C
  Wire.begin(I2C_SDA, I2C_SCL);
  Serial.println("✓ I2C initialized");
  Serial.print("  SDA: GPIO ");
  Serial.println(I2C_SDA);
  Serial.print("  SCL: GPIO ");
  Serial.println(I2C_SCL);
  
  // Scan I2C bus
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
  } else {
    Serial.print("  Found ");
    Serial.print(devices_found);
    Serial.println(" device(s)");
  }
  Serial.println();
  
  // Initialize sensor
  sht21.begin();
  Serial.println("✓ SHT21/HTU21 sensor initialized");
  
  // Initialize MQTT client
  Serial.println("\n--- Initializing MQTT Client ---");
  mqtt.begin(
    wifi_ssid,
    wifi_password,
    mqtt_server,
    mqtt_port,
    mqtt_username,
    mqtt_password,
    mqtt_client_id
  );
  
  // Set status topic for automatic status publishing
  mqtt.setStatusTopic(mqtt_topic_status);
  
  Serial.println("\n✓ Setup complete! Starting main loop...\n");
}

// ============================================================================
// MAIN LOOP
// ============================================================================

void loop() {
  unsigned long current_time = millis();
  
  // Must call mqtt.loop() to maintain connection
  mqtt.loop();
  
  // Read sensor at specified interval
  if (current_time - last_sensor_read >= SENSOR_READ_INTERVAL) {
    readSensor();
    last_sensor_read = current_time;
  }
  
  // Publish to MQTT at specified interval
  if (current_time - last_mqtt_publish >= MQTT_PUBLISH_INTERVAL) {
    if (sensor_valid && mqtt.connected()) {
      publishSensorData();
    }
    last_mqtt_publish = current_time;
  }
  
  delay(100);
}

// ============================================================================
// SENSOR READING
// ============================================================================

void readSensor() {
  float temp = sht21.readTemperature();
  float humidity = sht21.readHumidity();
  
  // Check if readings are valid
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
    Serial.println("SHT21 - Read failed (invalid data)");
  }
}

// ============================================================================
// MQTT PUBLISHING
// ============================================================================

void publishSensorData() {
  StaticJsonDocument<300> doc;
  doc["temperature"] = current_temp;
  doc["humidity"] = current_humidity;
  doc["timestamp"] = millis() / 1000;
  
  // Use the library's publishJSON method
  bool published = mqtt.publishJSON(mqtt_topic_readings, doc);
  
  if (published) {
    Serial.print("✓ Published to MQTT: ");
    Serial.print(mqtt_topic_readings);
    Serial.print(" - Temp: ");
    Serial.print(current_temp, 2);
    Serial.print("°C, Hum: ");
    Serial.print(current_humidity, 2);
    Serial.println("%");
  } else {
    Serial.println("✗ MQTT publish failed");
  }
}

