/*
 * ESP32-S3 Multi-Sensor & LED Controller with MQTT
 * 
 * Features:
 * - SHT21/HTU21 Temperature & Humidity sensor (I2C)
 * - BME680 Temperature, Humidity, Pressure & Gas sensor (I2C)
 * - SK6812 RGBW LED strip control
 * - MQTT publishing to Raspberry Pi Mosquitto broker
 * 
 * Hardware:
 * - ESP32-S3 (lonely binary GOLD EDITION)
 * - SHT21/HTU21 on I2C (default 0x40)
 * - BME680 on I2C (default 0x76 or 0x77)
 * - SK6812 RGBW LED strip on GPIO pin
 * 
 * Libraries Required:
 * - WiFi (built-in)
 * - Wire (built-in)
 * - PubSubClient (MQTT)
 * - Adafruit NeoPixel (for SK6812)
 * - SparkFun HTU21D (for SHT21/HTU21)
 * - Adafruit BME680 (for BME680)
 */

#include <WiFi.h>
#include <Wire.h>
#include <PubSubClient.h>
#include <Adafruit_NeoPixel.h>
#include <SparkFunHTU21D.h>
#include "BME680_Custom.h"  // Custom BME680 library with IAQ support
#include <ArduinoJson.h>

// ============================================================================
// CONFIGURATION - Modify these values for your setup
// ============================================================================

// WiFi Configuration
const char* ssid = "YOUR_WIFI_SSID";
const char* password = "YOUR_WIFI_PASSWORD";

// MQTT Configuration
const char* mqtt_server = "192.168.1.XXX";  // Raspberry Pi IP address
const int mqtt_port = 1883;
const char* mqtt_client_id = "esp32-s3-sensors";
const char* mqtt_topic_sht21 = "sensors/sht21/readings";
const char* mqtt_topic_bme680 = "sensors/bme680/readings";
const char* mqtt_topic_status = "sensors/esp32-s3/status";

// I2C Configuration
#define I2C_SDA 21  // Default I2C SDA pin
#define I2C_SCL 22  // Default I2C SCL pin

// SHT21/HTU21 Configuration
HTU21D sht21;

// BME680 Configuration
#define BME680_I2C_ADDRESS BME680_I2C_ADDR_SECONDARY  // Use BME680_I2C_ADDR_PRIMARY (0x76) if SDO is connected to GND
BME680_Custom bme680(BME680_I2C_ADDRESS);

// SK6812 LED Strip Configuration
#define LED_PIN 4        // GPIO pin for LED data line (change as needed)
#define LED_COUNT 30     // Number of LEDs in strip
#define LED_BRIGHTNESS 50  // 0-255

Adafruit_NeoPixel strip(LED_COUNT, LED_PIN, NEO_GRBW + NEO_KHZ800);

// Sensor Reading Intervals
const unsigned long SENSOR_READ_INTERVAL = 5000;  // Read sensors every 5 seconds
const unsigned long MQTT_PUBLISH_INTERVAL = 30000; // Publish to MQTT every 30 seconds

// ============================================================================
// GLOBAL VARIABLES
// ============================================================================

WiFiClient espClient;
PubSubClient mqtt_client(espClient);

unsigned long last_sensor_read = 0;
unsigned long last_mqtt_publish = 0;

// Sensor data storage
struct SensorData {
  float sht21_temp = 0.0;
  float sht21_humidity = 0.0;
  bool sht21_valid = false;
  
  float bme680_temp = 0.0;
  float bme680_humidity = 0.0;
  float bme680_pressure = 0.0;
  float bme680_gas = 0.0;
  float bme680_iaq_score = 0.0;
  bool bme680_valid = false;
  bool bme680_baseline_established = false;
};

SensorData sensor_data;

// ============================================================================
// SETUP
// ============================================================================

void setup() {
  Serial.begin(115200);
  delay(1000);
  
  Serial.println("\n\n========================================");
  Serial.println("ESP32-S3 Multi-Sensor & LED Controller");
  Serial.println("========================================\n");
  
  // Initialize I2C
  Wire.begin(I2C_SDA, I2C_SCL);
  Serial.println("✓ I2C initialized");
  
  // Initialize SHT21/HTU21
  if (sht21.begin()) {
    Serial.println("✓ SHT21/HTU21 sensor found");
  } else {
    Serial.println("✗ SHT21/HTU21 sensor not found!");
  }
  
  // Initialize BME680
  if (bme680.begin()) {
    Serial.println("✓ BME680 sensor found");
    
    // Configure heater (matching Python implementation)
    bme680.set_gas_heater_temperature(320);  // 320°C
    bme680.set_gas_heater_duration(150);     // 150ms
    bme680.select_gas_heater_profile(0);
    
    Serial.println("  Note: Run baseline calibration for accurate IAQ readings");
    Serial.println("  Use: bme680.set_baselines(300) in setup() or via MQTT");
  } else {
    Serial.println("✗ BME680 sensor not found!");
    Serial.println("  Trying alternate address 0x76...");
    BME680_Custom bme680_alt(BME680_I2C_ADDR_PRIMARY);
    if (bme680_alt.begin()) {
      Serial.println("✓ BME680 found at 0x76");
      bme680 = bme680_alt;
      bme680.set_gas_heater_temperature(320);
      bme680.set_gas_heater_duration(150);
      bme680.select_gas_heater_profile(0);
    }
  }
  
  // Initialize LED strip
  strip.begin();
  strip.setBrightness(LED_BRIGHTNESS);
  strip.clear();
  strip.show();
  Serial.println("✓ SK6812 LED strip initialized");
  
  // Connect to WiFi
  connectWiFi();
  
  // Connect to MQTT
  mqtt_client.setServer(mqtt_server, mqtt_port);
  mqtt_client.setCallback(mqttCallback);
  connectMQTT();
  
  // Initial status LED indication
  showStatusLED();
  
  Serial.println("\n✓ Setup complete! Starting main loop...\n");
}

// ============================================================================
// MAIN LOOP
// ============================================================================

void loop() {
  unsigned long current_time = millis();
  
  // Maintain MQTT connection
  if (!mqtt_client.connected()) {
    connectMQTT();
  }
  mqtt_client.loop();
  
  // Read sensors at specified interval
  if (current_time - last_sensor_read >= SENSOR_READ_INTERVAL) {
    readSensors();
    last_sensor_read = current_time;
  }
  
  // Publish to MQTT at specified interval
  if (current_time - last_mqtt_publish >= MQTT_PUBLISH_INTERVAL) {
    publishSensorData();
    last_mqtt_publish = current_time;
  }
  
  delay(100); // Small delay to prevent watchdog issues
}

// ============================================================================
// WIFI FUNCTIONS
// ============================================================================

void connectWiFi() {
  Serial.print("Connecting to WiFi: ");
  Serial.println(ssid);
  
  WiFi.mode(WIFI_STA);
  WiFi.begin(ssid, password);
  
  int attempts = 0;
  while (WiFi.status() != WL_CONNECTED && attempts < 20) {
    delay(500);
    Serial.print(".");
    attempts++;
  }
  
  if (WiFi.status() == WL_CONNECTED) {
    Serial.println("\n✓ WiFi connected!");
    Serial.print("  IP address: ");
    Serial.println(WiFi.localIP());
  } else {
    Serial.println("\n✗ WiFi connection failed!");
  }
}

// ============================================================================
// MQTT FUNCTIONS
// ============================================================================

void connectMQTT() {
  while (!mqtt_client.connected()) {
    Serial.print("Connecting to MQTT broker: ");
    Serial.print(mqtt_server);
    Serial.print(":");
    Serial.println(mqtt_port);
    
    if (mqtt_client.connect(mqtt_client_id)) {
      Serial.println("✓ Connected to MQTT broker");
      
      // Subscribe to LED control topic
      mqtt_client.subscribe("sensors/esp32-s3/led/control");
      // Subscribe to BME680 calibration topic
      mqtt_client.subscribe("sensors/esp32-s3/bme680/calibrate");
      
      // Publish online status
      publishStatus("online");
    } else {
      Serial.print("✗ MQTT connection failed, rc=");
      Serial.print(mqtt_client.state());
      Serial.println(" - retrying in 5 seconds");
      delay(5000);
    }
  }
}

void mqttCallback(char* topic, byte* payload, unsigned int length) {
  // Handle incoming MQTT messages (e.g., LED control commands)
  String message = "";
  for (int i = 0; i < length; i++) {
    message += (char)payload[i];
  }
  
  Serial.print("MQTT message received on topic: ");
  Serial.print(topic);
  Serial.print(" - Message: ");
  Serial.println(message);
  
  // Parse LED control commands
  if (String(topic) == "sensors/esp32-s3/led/control") {
    handleLEDControl(message);
  }
  
  // Parse BME680 calibration commands
  if (String(topic) == "sensors/esp32-s3/bme680/calibrate") {
    handleBME680Calibration(message);
  }
}

void publishStatus(const char* status) {
  StaticJsonDocument<200> doc;
  doc["status"] = status;
  doc["uptime"] = millis() / 1000;
  doc["free_heap"] = ESP.getFreeHeap();
  doc["wifi_rssi"] = WiFi.RSSI();
  
  String payload;
  serializeJson(doc, payload);
  
  mqtt_client.publish(mqtt_topic_status, payload.c_str());
}

void publishSensorData() {
  // Publish SHT21 data
  if (sensor_data.sht21_valid) {
    StaticJsonDocument<300> doc;
    doc["temperature"] = sensor_data.sht21_temp;
    doc["humidity"] = sensor_data.sht21_humidity;
    doc["timestamp"] = millis() / 1000;
    
    String payload;
    serializeJson(doc, payload);
    mqtt_client.publish(mqtt_topic_sht21, payload.c_str());
    
    Serial.print("Published SHT21: ");
    Serial.println(payload);
  }
  
  // Publish BME680 data
  if (sensor_data.bme680_valid) {
    StaticJsonDocument<500> doc;
    doc["temperature"] = sensor_data.bme680_temp;
    doc["humidity"] = sensor_data.bme680_humidity;
    doc["pressure"] = sensor_data.bme680_pressure;
    doc["gas_resistance"] = sensor_data.bme680_gas;
    doc["heat_stable"] = true;
    doc["timestamp"] = millis() / 1000;
    
    // Add IAQ data if baseline is established
    if (sensor_data.bme680_baseline_established) {
      doc["iaq_score"] = sensor_data.bme680_iaq_score;
      doc["baseline_established"] = true;
      doc["gas_baseline"] = bme680.get_gas_baseline();
      doc["hum_baseline"] = bme680.get_hum_baseline();
      doc["safe_to_open"] = bme680.check_safe_to_open(80.0);
    } else {
      doc["baseline_established"] = false;
    }
    
    String payload;
    serializeJson(doc, payload);
    mqtt_client.publish(mqtt_topic_bme680, payload.c_str());
    
    Serial.print("Published BME680: ");
    Serial.println(payload);
  }
}

// ============================================================================
// SENSOR READING FUNCTIONS
// ============================================================================

void readSensors() {
  // Read SHT21/HTU21
  float temp = sht21.readTemperature();
  float humidity = sht21.readHumidity();
  
  if (!isnan(temp) && !isnan(humidity)) {
    sensor_data.sht21_temp = temp;
    sensor_data.sht21_humidity = humidity;
    sensor_data.sht21_valid = true;
    
    Serial.print("SHT21 - Temp: ");
    Serial.print(temp);
    Serial.print("°C, Humidity: ");
    Serial.print(humidity);
    Serial.println("%");
  } else {
    sensor_data.sht21_valid = false;
    Serial.println("SHT21 - Read failed");
  }
  
  // Read BME680
  if (bme680.performReading()) {
    sensor_data.bme680_temp = bme680.temperature;
    sensor_data.bme680_humidity = bme680.humidity;
    sensor_data.bme680_pressure = bme680.pressure / 100.0; // Convert to hPa
    sensor_data.bme680_gas = bme680.gas_resistance / 1000.0; // Convert to kOhm
    sensor_data.bme680_valid = true;
    
    Serial.print("BME680 - Temp: ");
    Serial.print(sensor_data.bme680_temp);
    Serial.print("°C, Humidity: ");
    Serial.print(sensor_data.bme680_humidity);
    Serial.print("%, Pressure: ");
    Serial.print(sensor_data.bme680_pressure);
    Serial.print(" hPa, Gas: ");
    Serial.print(sensor_data.bme680_gas);
    Serial.println(" kOhm");
  } else {
    sensor_data.bme680_valid = false;
    Serial.println("BME680 - Read failed (not heat stable)");
  }
}

// ============================================================================
// LED CONTROL FUNCTIONS
// ============================================================================

void showStatusLED() {
  // Show status with LED colors
  // Green = all good, Red = error, Blue = connecting
  
  if (WiFi.status() == WL_CONNECTED && mqtt_client.connected()) {
    // All good - green
    setLEDColor(0, 255, 0, 0); // Green
  } else if (WiFi.status() == WL_CONNECTED) {
    // WiFi OK but MQTT not connected - blue
    setLEDColor(0, 0, 255, 0); // Blue
  } else {
    // Not connected - red
    setLEDColor(255, 0, 0, 0); // Red
  }
  
  delay(500);
  strip.clear();
  strip.show();
}

void setLEDColor(uint8_t r, uint8_t g, uint8_t b, uint8_t w) {
  for (int i = 0; i < LED_COUNT; i++) {
    strip.setPixelColor(i, strip.Color(r, g, b, w));
  }
  strip.show();
}

void handleBME680Calibration(String message) {
  // Parse JSON calibration commands
  // Example: {"action":"calibrate","duration":300}
  
  StaticJsonDocument<200> doc;
  DeserializationError error = deserializeJson(doc, message);
  
  if (error) {
    Serial.print("JSON parse error: ");
    Serial.println(error.c_str());
    return;
  }
  
  String action = doc["action"];
  
  if (action == "calibrate") {
    uint16_t duration = doc["duration"] | 300;
    Serial.print("Starting BME680 baseline calibration for ");
    Serial.print(duration);
    Serial.println(" seconds...");
    
    bool success = bme680.set_baselines(duration, true);
    if (success) {
      Serial.print("✓ Baseline established - Gas: ");
      Serial.print(bme680.get_gas_baseline());
      Serial.print(" Ohms, Hum: ");
      Serial.print(bme680.get_hum_baseline());
      Serial.println("%");
      
      // Publish calibration status
      StaticJsonDocument<200> status;
      status["calibration"] = "complete";
      status["gas_baseline"] = bme680.get_gas_baseline();
      status["hum_baseline"] = bme680.get_hum_baseline();
      String payload;
      serializeJson(status, payload);
      mqtt_client.publish("sensors/esp32-s3/bme680/calibration_status", payload.c_str());
    } else {
      Serial.println("✗ Baseline calibration failed");
    }
  }
}

void handleLEDControl(String message) {
  // Parse JSON LED control commands
  // Example: {"action":"set_color","r":255,"g":0,"b":0,"w":0}
  // Example: {"action":"set_brightness","value":128}
  // Example: {"action":"clear"}
  
  StaticJsonDocument<200> doc;
  DeserializationError error = deserializeJson(doc, message);
  
  if (error) {
    Serial.print("JSON parse error: ");
    Serial.println(error.c_str());
    return;
  }
  
  String action = doc["action"];
  
  if (action == "set_color") {
    uint8_t r = doc["r"] | 0;
    uint8_t g = doc["g"] | 0;
    uint8_t b = doc["b"] | 0;
    uint8_t w = doc["w"] | 0;
    setLEDColor(r, g, b, w);
    Serial.printf("LED color set: R=%d G=%d B=%d W=%d\n", r, g, b, w);
    
  } else if (action == "set_brightness") {
    uint8_t brightness = doc["value"] | 50;
    strip.setBrightness(brightness);
    strip.show();
    Serial.printf("LED brightness set: %d\n", brightness);
    
  } else if (action == "clear") {
    strip.clear();
    strip.show();
    Serial.println("LED strip cleared");
  }
}

