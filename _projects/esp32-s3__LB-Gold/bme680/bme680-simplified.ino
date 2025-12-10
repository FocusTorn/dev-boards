/*
 * ESP32-S3 BME680 Sensor - Simplified (Raw Data Only)
 * 
 * This version publishes ONLY raw sensor readings to MQTT.
 * All calculations (heatsoak, IAQ) are done on the Raspberry Pi.
 * 
 * Architecture:
 * - ESP32: Collects raw sensor data using Bosch library
 * - RPi: Subscribes to raw data, performs all calculations
 * - Home Assistant: Displays calculated values from RPi
 * 
 * MQTT Topics:
 * - sensors/bme680/raw - Raw sensor readings (ESP32 → RPi)
 * - sensors/bme680/status - Device status
 * 
 * Hardware:
 * - ESP32-S3 (lonely binary GOLD EDITION)
 * - BME680 sensor on I2C bus (0x76 or 0x77)
 * 
 * Libraries Required:
 * - MQTT_RPi_Client (included in libraries folder)
 * - BME680_Bosch (Bosch implementation)
 * - ArduinoJson (for JSON parsing)
 */

#include <MQTT_RPi_Client.h>
#include <BME680_Bosch.h>
#include <ArduinoJson.h>
#include <Wire.h>

// ============================================================================
// CONFIGURATION
// ============================================================================

// MQTT Configuration
const char* mqtt_client_id = "esp32-s3-bme680";
const char* mqtt_topic_raw = "sensors/bme680/raw";        // Raw readings only
const char* mqtt_topic_status = "sensors/bme680/status";   // Status

// BME680 Configuration
#define BME680_I2C_ADDRESS_PRIMARY 0x76
#define BME680_I2C_ADDRESS_SECONDARY 0x77

// Sensor reading intervals (milliseconds)
#define READ_INTERVAL 1000      // Read sensor every 1 second
#define PUBLISH_INTERVAL 5000   // Publish to MQTT every 5 seconds (faster for RPi processing)

// Initialize MQTT client
MQTT_RPi_Client mqtt;

// ============================================================================
// BME680 SENSOR
// ============================================================================

BME680_Bosch* bme680_ptr = nullptr;  // Will be initialized in setup()

// ============================================================================
// GLOBAL STATE
// ============================================================================

unsigned long last_read_time = 0;
unsigned long last_publish_time = 0;
bool sensor_initialized = false;

// ============================================================================
// SETUP
// ============================================================================

void setup() {
  Serial.begin(115200);
  delay(1000);
  
  Serial.println("\n\n========================================");
  Serial.println("ESP32-S3 BME680 - Raw Data Publisher");
  Serial.println("========================================\n");
  Serial.println("Architecture: ESP32 → Raw Data → RPi → Calculations");
  Serial.println();
  
  // Initialize I2C
  Wire.begin();
  
  // Initialize BME680 sensor (try both addresses)
  Serial.println("Initializing BME680 sensor...");
  
  // Try secondary address first (0x77)
  static BME680_Bosch bme680_secondary(BME680_I2C_ADDRESS_SECONDARY);
  if (bme680_secondary.begin()) {
    bme680_ptr = &bme680_secondary;
    Serial.println("✓ BME680 found at address 0x77");
  } else {
    // Try primary address (0x76)
    Serial.println("Trying primary address 0x76...");
    static BME680_Bosch bme680_primary(BME680_I2C_ADDRESS_PRIMARY);
    if (bme680_primary.begin()) {
      bme680_ptr = &bme680_primary;
      Serial.println("✓ BME680 found at address 0x76");
    } else {
      Serial.println("✗ BME680 sensor not found!");
      Serial.println("  Check I2C connections and sensor power.");
      while (1) delay(10);
    }
  }
  
  // Configure sensor (matching Python defaults)
  bme680_ptr->set_gas_heater_temperature(320);  // 320°C
  bme680_ptr->set_gas_heater_duration(150);     // 150ms
  bme680_ptr->select_gas_heater_profile(0);
  
  sensor_initialized = true;
  
  // Initialize MQTT client
  mqtt.begin(mqtt_client_id);
  mqtt.setStatusTopic(mqtt_topic_status);
  
  Serial.println("\n✓ Setup complete!");
  Serial.println("  Publishing raw sensor data to: sensors/bme680/raw");
  Serial.println("  RPi will subscribe and perform all calculations\n");
}

// ============================================================================
// MAIN LOOP
// ============================================================================

void loop() {
  // Maintain MQTT connection
  mqtt.loop();
  
  unsigned long current_time = millis();
  
  // Read sensor at specified interval
  if (current_time - last_read_time >= READ_INTERVAL) {
    if (readSensor()) {
      // Sensor read successfully (data available in bme680_ptr->data)
      // No calculations here - just read and store
    }
    last_read_time = current_time;
  }
  
  // Publish to MQTT at specified interval
  if (current_time - last_publish_time >= PUBLISH_INTERVAL) {
    publishRawReadings();
    last_publish_time = current_time;
  }
  
  delay(10);
}

// ============================================================================
// SENSOR READING
// ============================================================================

bool readSensor() {
  if (!sensor_initialized || !bme680_ptr) return false;
  
  // Read sensor using Bosch library
  // This applies sensor calibration internally
  if (!bme680_ptr->get_sensor_data()) {
    return false;
  }
  
  // Only return true if heat stable (valid reading)
  return bme680_ptr->data.heat_stable;
}

// ============================================================================
// MQTT PUBLISHING (Raw Data Only)
// ============================================================================

void publishRawReadings() {
  if (!mqtt.connected() || !bme680_ptr) return;
  
  // Only publish if we have a valid, heat-stable reading
  if (!bme680_ptr->data.heat_stable) {
    Serial.println("⚠ Skipping publish - sensor not heat stable");
    return;
  }
  
  // Publish ONLY raw sensor readings
  // All calculations (heatsoak, IAQ) will be done on RPi
  StaticJsonDocument<300> doc;
  
  // Raw sensor values (already calibrated by Bosch library)
  doc["temperature"] = round(bme680_ptr->data.temperature * 100.0) / 100.0;
  doc["humidity"] = round(bme680_ptr->data.humidity * 100.0) / 100.0;
  doc["pressure"] = round(bme680_ptr->data.pressure * 100.0) / 100.0;
  doc["gas_resistance"] = round(bme680_ptr->data.gas_resistance * 100.0) / 100.0;
  
  // Metadata
  doc["heat_stable"] = bme680_ptr->data.heat_stable;
  doc["gas_valid"] = bme680_ptr->data.gas_valid;
  doc["timestamp"] = millis() / 1000.0;
  
  // Publish to MQTT
  mqtt.publishJSON(mqtt_topic_raw, doc);
  
  Serial.print("✓ Published raw readings: ");
  Serial.print("T=");
  Serial.print(doc["temperature"].as<float>());
  Serial.print("°C, H=");
  Serial.print(doc["humidity"].as<float>());
  Serial.print("%, P=");
  Serial.print(doc["pressure"].as<float>());
  Serial.print("hPa, G=");
  Serial.print(doc["gas_resistance"].as<float>());
  Serial.println("Ω");
}

