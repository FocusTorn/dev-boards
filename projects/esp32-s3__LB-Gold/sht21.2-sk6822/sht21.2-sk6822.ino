/*
 * ESP32-S3 SHT21 Sensor + SK6812 RGBW LED Controller
 *
 * Combines SHT21 temperature/humidity sensing with dual-channel LED strip control.
 *
 * Hardware:
 * - ESP32-S3 (Lonely Binary Gold Edition)
 * - SHT21/HTU21 on I2C (SDA: GPIO 4, SCL: GPIO 5)
 * - Command LED Strip (SK6812 RGBW) on GPIO 12 (31 LEDs)
 * - Status LED Strip (SK6812 RGBW) on GPIO 17 (30 LEDs)
 *
 * MQTT Topics:
 * - e32s3/sk6822/command (Subscribe)
 * - e32s3/sk6822/state (Subscribe/Publish)
 * - e32s3/sk6822/status (Publish)
 * - e32s3.1/sht21.2/readings (Publish)
 * - e32s3.1/sht21.2/status (Publish)
 */

// TODO Add compatability for an effects engine 
 
#include <Wire.h>
#include <MQTT_Win_Client.h>
#include <SparkFunHTU21D.h>
#include <Adafruit_NeoPixel.h>
#include <ArduinoJson.h>

// ============================================================================
// CONFIGURATION
// ============================================================================

const char* mqtt_client_id = "esp32s3-sht21.2_sk6822";

// MQTT Topics
const char* mqtt_6822_command = "e32s3/sk6822/command"; 
const char* mqtt_6822_state   = "e32s3/sk6822/state";   
const char* mqtt_6822_status  = "e32s3/sk6822/status";  

const char* mqtt_sht21_readings = "e32s3.1/sht21.2/readings"; 
const char* mqtt_sht21_status   = "e32s3.1/sht21.2/status";   

// I2C Pins (ESP32-S3 specific for this board)
#define I2C_SDA 4
#define I2C_SCL 5

// LED Configuration
#define COMMAND_PIN 12
#define COMMAND_COUNT 31
#define COMMAND_BRIGHTNESS 50

#define STATUS_PIN 17
#define STATUS_COUNT 30
#define STATUS_BRIGHTNESS 50

// Objects
HTU21D sht21;
Adafruit_NeoPixel commandStrip(COMMAND_COUNT, COMMAND_PIN, NEO_GRBW + NEO_KHZ800);
Adafruit_NeoPixel statusStrip(STATUS_COUNT, STATUS_PIN, NEO_GRBW + NEO_KHZ800);
MQTT_Win_Client mqtt;

// State Variables
unsigned long last_sensor_read = 0;
unsigned long last_mqtt_publish = 0;
const unsigned long SENSOR_READ_INTERVAL = 5000;
const unsigned long MQTT_PUBLISH_INTERVAL = 5000;

float current_temp = 0.0;
float current_humidity = 0.0;
bool sensor_valid = false;

// LED State Tracking
struct StripState {
  uint8_t r = 0;
  uint8_t g = 0;
  uint8_t b = 0;
  uint8_t w = 0;
  uint8_t brightness = 50;
  String effect = "";
};

StripState commandState;
StripState statusState;

// ============================================================================
// SETUP
// ============================================================================

void setup() {
  Serial.begin(115200);
  delay(1000);
  
  Serial.println("\n\n========================================");
  Serial.println("ESP32-S3 SHT21 + SK6812 Controller");
  Serial.println("========================================\n");

  // --- I2C Init ---
  Wire.begin(I2C_SDA, I2C_SCL);
  sht21.begin();
  Serial.println("✓ I2C & SHT21 Initialized");

  // --- LED Init ---
  commandStrip.begin();
  commandStrip.setBrightness(COMMAND_BRIGHTNESS);
  commandStrip.clear();
  commandStrip.show();
  
  statusStrip.begin();
  statusStrip.setBrightness(STATUS_BRIGHTNESS);
  statusStrip.clear();
  statusStrip.show();
  Serial.println("✓ LED Strips Initialized");

  // --- MQTT Init ---
  // Using library defaults for credentials/wifi as per project standard
  mqtt.begin(mqtt_client_id);
  mqtt.setStatusTopic(mqtt_6822_status); // Primary status topic
  mqtt.setCallback(mqttCallback);
  
  // Initial Connection
  if (mqtt.connected()) {
    mqtt.subscribe(mqtt_6822_command);
    mqtt.subscribe(mqtt_6822_state); 
    Serial.println("✓ MQTT Connected & Subscribed");
  }
}

// ============================================================================
// MAIN LOOP
// ============================================================================

void loop() {
  mqtt.loop();
  unsigned long current_time = millis();

  // Re-subscribe loop protection
  static bool last_connected = false;
  if (mqtt.connected() && !last_connected) {
    mqtt.subscribe(mqtt_6822_command);
    mqtt.subscribe(mqtt_6822_state);
    Serial.println("✓ Re-subscribed to topics");
  }
  last_connected = mqtt.connected();

  // SHT21 Logic
  if (current_time - last_sensor_read >= SENSOR_READ_INTERVAL) {
    readSensor();
    last_sensor_read = current_time;
  }

  // Publish Logic
  if (current_time - last_mqtt_publish >= MQTT_PUBLISH_INTERVAL) {
    publishSensorData();
    // publishLEDState(); // Optional: periodic state push
    last_mqtt_publish = current_time;
  }
  
  // TODO: Add non-blocking effect update logic here
  
  delay(10);
}

// ============================================================================
// MQTT CALLBACK & PARSING
// ============================================================================

void mqttCallback(char* topic, byte* payload, unsigned int length) {
  String message = "";
  for (int i = 0; i < length; i++) message += (char)payload[i];
  String topicStr = String(topic);

  Serial.printf("Msg on %s: %s\n", topic, message.c_str());

  StaticJsonDocument<1024> doc;
  DeserializationError error = deserializeJson(doc, message);

  if (error) {
    Serial.print("JSON Error: ");
    Serial.println(error.c_str());
    return;
  }

  // Handle Command Topic
  if (topicStr == mqtt_6822_command) {
    // Determine target strip ("command" or "status" or "both")
    // Default to "command" strip if not specified
    String target = doc["target"] | "command"; 
    
    if (target == "command" || target == "both") {
      handleStripCommand(commandStrip, commandState, doc);
    }
    if (target == "status" || target == "both") {
      handleStripCommand(statusStrip, statusState, doc);
    }
  }
}

void handleStripCommand(Adafruit_NeoPixel &strip, StripState &state, JsonDocument &doc) {
  // 1. Brightness
  if (doc.containsKey("brightness")) {
    uint8_t b = doc["brightness"];
    state.brightness = b;
    strip.setBrightness(b);
  }

  // 2. Clear
  if (doc.containsKey("action") && doc["action"] == "clear") {
    strip.clear();
    strip.show();
    return;
  }

  // 3. Color Set (Single or Range)
  if (doc.containsKey("r") || doc.containsKey("g") || doc.containsKey("b") || doc.containsKey("w")) {
    uint8_t r = doc["r"] | 0;
    uint8_t g = doc["g"] | 0;
    uint8_t b = doc["b"] | 0;
    uint8_t w = doc["w"] | 0;

    state.r = r; state.g = g; state.b = b; state.w = w;

    // Range support: "start": 0, "end": 5
    int start = doc["start"] | 0;
    int end = doc["end"] | (strip.numPixels() - 1);

    // Sanity checks
    if (start < 0) start = 0;
    if (end >= strip.numPixels()) end = strip.numPixels() - 1;

    for (int i = start; i <= end; i++) {
      strip.setPixelColor(i, strip.Color(r, g, b, w));
    }
    strip.show();
  }
  
  // 4. Effect stub
  if (doc.containsKey("effect")) {
    String effect = doc["effect"];
    
    // TODO: Implement effects engine
    
    Serial.println("Effect requested: " + effect);
  }
}

// ============================================================================
// SENSOR LOGIC
// ============================================================================

void readSensor() {
  float temp = sht21.readTemperature();
  float hum = sht21.readHumidity();

  if (!isnan(temp) && !isnan(hum) && temp > -40 && temp < 125) {
    current_temp = temp;
    current_humidity = hum;
    sensor_valid = true;
  } else {
    sensor_valid = false;
    Serial.println("Sensor Read Failed");
  }
}

void publishSensorData() {
  if (!mqtt.connected()) return;

  StaticJsonDocument<300> doc;
  doc["temperature"] = current_temp;
  doc["humidity"] = current_humidity;
  doc["valid"] = sensor_valid;
  
  String payload;
  serializeJson(doc, payload);
  mqtt.publish(mqtt_sht21_readings, payload.c_str());
  
  // Also publish status to the specific status topic
  mqtt.publish(mqtt_sht21_status, sensor_valid ? "online" : "error");
}
