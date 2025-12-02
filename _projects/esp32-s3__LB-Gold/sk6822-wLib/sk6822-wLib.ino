/*
 * ESP32-S3 LED Controller with MQTT (Library Version)
 * 
 * MQTT-based LED control using MQTT_RPi_Client and SK6822Controller libraries
 * Integrates with Raspberry Pi Mosquitto broker and Home Assistant
 * 
 * Features:
 * - SK6812 RGBW LED strip control (via SK6822Controller library)
 * - MQTT pub/sub for commands and status
 * - Home Assistant compatible
 * - Multiple LED patterns and effects
 * 
 * Hardware:
 * - ESP32-S3 (lonely binary GOLD EDITION)
 * - SK6812 RGBW LED strip on GPIO pin
 * 
 * Libraries Required:
 * - MQTT_RPi_Client (included in libraries folder)
 * - SK6822Controller (included in libraries folder)
 * - Adafruit NeoPixel (for SK6812)
 * - ArduinoJson (for JSON parsing)
 */

#include <MQTT_RPi_Client.h>
#include <SK6822Controller.h>
#include <ArduinoJson.h>

// ============================================================================
// CONFIGURATION - Modify these values for your setup
// ============================================================================

// MQTT Configuration
const char* mqtt_client_id = "esp32-s3-led-controller";

const char* mqtt_topic_command = "sensors/esp32-s3-led/command";  // Subscribe to commands
const char* mqtt_topic_state = "sensors/esp32-s3-led/state";      // Publish state
const char* mqtt_topic_status = "sensors/esp32-s3-led/status";    // Publish status

// SK6812 LED Strip Configuration
#define LED_PIN 6        // GPIO pin for LED data line (change as needed)
#define LED_COUNT 31     // Number of LEDs in strip
#define LED_BRIGHTNESS 50  // 0-255 (default brightness)

// Initialize LED controller
SK6822Controller leds(LED_PIN, LED_COUNT, LED_BRIGHTNESS);

// Initialize MQTT client
MQTT_RPi_Client mqtt;

// ============================================================================
// SETUP
// ============================================================================

void setup() {
  Serial.begin(115200);
  delay(1000);
  
  Serial.println("\n\n========================================");
  Serial.println("ESP32-S3 LED Controller - MQTT (Library)");
  Serial.println("========================================\n");
  
  // Initialize LED strip
  leds.begin();
  Serial.println("✓ SK6812 LED strip initialized");
  
  // Initialize MQTT client (uses default WiFi and MQTT settings from library)
  mqtt.begin(mqtt_client_id);  // Only client_id required, uses defaults for rest
  
  // Set status topic for automatic status publishing
  mqtt.setStatusTopic(mqtt_topic_status);
  
  // Set callback for incoming messages
  mqtt.setCallback(mqttCallback);
  
  // Subscribe to command topic
  if (mqtt.connected()) {
    mqtt.subscribe(mqtt_topic_command);
    Serial.print("✓ Subscribed to: ");
    Serial.println(mqtt_topic_command);
  }
  
  // Initial status LED indication
  showStatusLED();
  
  // Publish initial state
  publishState();
  
  Serial.println("\n✓ Setup complete! Ready for MQTT commands...\n");
}

// ============================================================================
// MAIN LOOP
// ============================================================================

void loop() {
  // Maintain MQTT connection and process messages
  mqtt.loop();
  
  // Update running patterns (library handles this)
  leds.update();
  
  delay(10); // Small delay to prevent watchdog issues
}

// ============================================================================
// MQTT CALLBACK
// ============================================================================

void mqttCallback(char* topic, byte* payload, unsigned int length) {
  // Convert payload to string
  String message = "";
  for (int i = 0; i < length; i++) {
    message += (char)payload[i];
  }
  
  Serial.print("MQTT message received on topic: ");
  Serial.print(topic);
  Serial.print(" - Message: ");
  Serial.println(message);
  
  // Parse JSON command
  StaticJsonDocument<300> doc;
  DeserializationError error = deserializeJson(doc, message);
  
  if (error) {
    Serial.print("JSON parse error: ");
    Serial.println(error.c_str());
    return;
  }
  
  // Handle different command types
  if (doc.containsKey("action")) {
    String action = doc["action"];
    handleCommand(action, doc);
  } else if (doc.containsKey("state")) {
    // Home Assistant style state update
    handleHAState(doc);
  } else {
    Serial.println("Unknown command format");
  }
}

// ============================================================================
// COMMAND HANDLERS
// ============================================================================

void handleCommand(String action, JsonDocument& doc) {
  if (action == "set_color") {
    uint8_t r = doc["r"] | 0;
    uint8_t g = doc["g"] | 0;
    uint8_t b = doc["b"] | 0;
    uint8_t w = doc["w"] | 0;
    
    // Check for intensity parameter (0-100 percent)
    if (doc.containsKey("i")) {
      int intensity_percent = doc["i"] | 50;
      if (intensity_percent < 0) intensity_percent = 0;
      if (intensity_percent > 100) intensity_percent = 100;
      leds.setBrightnessPercent(intensity_percent);
      Serial.printf("LED intensity set: %d%%\n", intensity_percent);
    }
    
    leds.stopPattern();
    leds.setColor(r, g, b, w);
    
    publishState();
    Serial.printf("LED color set: R=%d G=%d B=%d W=%d (brightness: %d%%)\n", 
                  r, g, b, w, (leds.getBrightness() * 100) / 255);
    
  } else if (action == "set_brightness") {
    uint8_t brightness = doc["value"] | leds.getBrightness();
    if (brightness > 255) brightness = 255;
    
    leds.setBrightness(brightness);
    publishState();
    Serial.printf("LED brightness set: %d\n", brightness);
    
  } else if (action == "clear") {
    leds.stopPattern();
    leds.clear();
    publishState();
    Serial.println("LED strip cleared");
    
  } else if (action == "pattern") {
    String pattern_name = doc["name"] | "";
    int speed = doc["speed"] | 50;
    
    if (pattern_name == "") {
      Serial.println("Pattern name required");
      return;
    }
    
    // Start pattern using library
    leds.startPattern(pattern_name, speed);
    publishState();
    Serial.printf("Pattern started: %s (speed: %d)\n", pattern_name.c_str(), speed);
    
  } else if (action == "stop") {
    leds.stopPattern();
    publishState();
    Serial.println("Pattern stopped");
    
  } else {
    Serial.print("Unknown action: ");
    Serial.println(action);
  }
}

void handleHAState(JsonDocument& doc) {
  // Home Assistant MQTT Light format
  // {"state":"ON","brightness":128,"color":{"r":255,"g":0,"b":0}}
  
  if (doc.containsKey("state")) {
    String state = doc["state"];
    if (state == "OFF") {
      handleCommand("clear", doc);
      return;
    }
  }
  
  if (doc.containsKey("brightness")) {
    uint8_t brightness = doc["brightness"];
    doc["action"] = "set_brightness";
    doc["value"] = brightness;
    handleCommand("set_brightness", doc);
  }
  
  if (doc.containsKey("color")) {
    JsonObject color = doc["color"];
    uint8_t r = color["r"] | 0;
    uint8_t g = color["g"] | 0;
    uint8_t b = color["b"] | 0;
    uint8_t w = color["w"] | 0;
    
    doc["action"] = "set_color";
    doc["r"] = r;
    doc["g"] = g;
    doc["b"] = b;
    doc["w"] = w;
    handleCommand("set_color", doc);
  }
}

// ============================================================================
// STATE PUBLISHING
// ============================================================================

void publishState() {
  LEDState state = leds.getState();
  
  StaticJsonDocument<300> doc;
  doc["r"] = state.r;
  doc["g"] = state.g;
  doc["b"] = state.b;
  doc["w"] = state.w;
  doc["brightness"] = state.brightness;
  doc["pattern"] = state.pattern;
  doc["pattern_active"] = state.pattern_active;
  
  mqtt.publishJSON(mqtt_topic_state, doc);
}

// ============================================================================
// STATUS LED
// ============================================================================

void showStatusLED() {
  // Show status with LED colors
  if (mqtt.connected() && mqtt.wifiConnected()) {
    leds.showStatus(0, 255, 0, 0); // Green
  } else if (mqtt.wifiConnected()) {
    leds.showStatus(0, 0, 255, 0); // Blue
  } else {
    leds.showStatus(255, 0, 0, 0); // Red
  }
}

