/*
 * ESP32-S3 LED Controller with MQTT
 * 
 * MQTT-based LED control using MQTT_RPi_Client library
 * Integrates with Raspberry Pi Mosquitto broker and Home Assistant
 * 
 * Features:
 * - SK6812 RGBW LED strip control
 * - MQTT pub/sub for commands and status
 * - Home Assistant compatible
 * - Multiple LED patterns and effects
 * 
 * Hardware:
 * - ESP32-S3 (lonely binary GOLD EDITION)
 * - SK6812 RGBW LED strip on GPIO pin
 * 
 * Libraries Required:
 * - MQTT_Win_Client (included in libraries folder)
 * - Adafruit NeoPixel (for SK6812)
 * - ArduinoJson (for JSON parsing)
 */

#include <MQTT_Win_Client.h>
#include <Adafruit_NeoPixel.h>
#include <ArduinoJson.h>

// ============================================================================
// CONFIGURATION - Modify these values for your setup
// ============================================================================

const char* mqtt_client_id = "esp32s3-sht21.2_sk6822";  // Unique client ID (13 chars, within 23 char limit)

const char* mqtt_6822_command = "e32s3/sk6822/command"; // Subscribe to commands
const char* mqtt_6822_state =   "e32s3/sk6822/state";   // Subscribe to state
const char* mqtt_6822_status =  "e32s3/sk6822/status";  // Subscribe to state


const char* mqtt_sht21_readings = "e32s3.1/sht21.2/readings"; // Sensor readings
const char* mqtt_sht21_status   = "e32s3.1/sht21.2/status";   // Publish status



#define COMMAND_PIN 12        // GPIO pin for LED data line (change as needed)
#define COMMAND_COUNT 31     // Number of LEDs in strip
#define COMMAND_BRIGHTNESS 50  // 0-255 (default brightness)

#define STATUS_PIN 17        // GPIO pin for LED data line (change as needed)
#define STATUS_COUNT 30     // Number of LEDs in strip
#define STATUS_BRIGHTNESS 50  // 0-255 (default brightness)





// MQTT Broker Configuration (override library defaults if needed)
// Set to nullptr to use library defaults, or specify custom values
const char* mqtt_server = nullptr;  // nullptr = use library default (Windows hostname: "Acer-Lappy")
const int mqtt_port = -1;            // -1 = use library default (1883)
const char* mqtt_username = nullptr; // nullptr = use library default ("mqtt")
const char* mqtt_password = nullptr; // nullptr = use library default ("mqtt")
const char* wifi_ssid = nullptr;     // nullptr = use library default
const char* wifi_password = nullptr; // nullptr = use library default

// MQTT Topics (unique per device - no conflict with SHT21 topics)
const char* mqtt_topic_command = "controller/esp32-s3-led/command";  // Subscribe to commands
const char* mqtt_topic_state = "controller/esp32-s3-led/state";      // Publish state
const char* mqtt_topic_status = "controller/esp32-s3-led/status";    // Publish status

// SK6812 LED Strip Configuration

#define COMMAND_PIN 12        // GPIO pin for LED data line (change as needed)
#define COMMAND_COUNT 31     // Number of LEDs in strip
#define COMMAND_BRIGHTNESS 50  // 0-255 (default brightness)

#define STATUS_PIN 17        // GPIO pin for LED data line (change as needed)
#define STATUS_COUNT 30     // Number of LEDs in strip
#define STATUS_BRIGHTNESS 50  // 0-255 (default brightness)








Adafruit_NeoPixel strip(LED_COUNT, LED_PIN, NEO_GRBW + NEO_KHZ800);
Adafruit_NeoPixel stateStrip(STATE_LED_COUNT, STATE_LED_PIN, NEO_GRBW + NEO_KHZ800);

// ============================================================================
// GLOBAL VARIABLES
// ============================================================================

MQTT_Win_Client mqtt;

// LED state
uint8_t current_brightness = LED_BRIGHTNESS;
bool pattern_running = false;
unsigned long pattern_start_time = 0;
String current_pattern = "";

// State tracking
struct LEDState {
  uint8_t r = 0;
  uint8_t g = 0;
  uint8_t b = 0;
  uint8_t w = 0;
  uint8_t brightness = LED_BRIGHTNESS;
  String pattern = "";
  bool pattern_active = false;
};

LEDState led_state;

// ============================================================================
// SETUP
// ============================================================================

void setup() {
  Serial.begin(115200);
  delay(1000);
  
  Serial.println("\n\n========================================");
  Serial.println("ESP32-S3 LED Controller - MQTT");
  Serial.println("========================================\n");
  
  // Initialize LED strip
  strip.begin();
  strip.setBrightness(current_brightness);
  strip.clear();
  strip.show();
  Serial.println("✓ SK6812 LED strip initialized");
  
  // Initialize state LED strip
  stateStrip.begin();
  stateStrip.setBrightness(STATE_LED_BRIGHTNESS);
  stateStrip.clear();
  stateStrip.show();
  Serial.println("✓ State LED strip initialized");
  
  // Initialize MQTT client
  // Pass nullptr for parameters to use library defaults
  mqtt.begin(mqtt_client_id, wifi_ssid, wifi_password, mqtt_server, mqtt_port, mqtt_username, mqtt_password);
  
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
  
  // Re-subscribe if connection was reset
  static bool last_connected = false;
  bool now_connected = mqtt.connected();
  if (now_connected && !last_connected) {
    mqtt.subscribe(mqtt_topic_command);
    Serial.println("✓ Re-subscribed to command topic");
  }
  last_connected = now_connected;
  
  // Update running patterns
  if (pattern_running) {
    updatePattern();
  }
  
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
      // Convert percentage to brightness value (0-255)
      current_brightness = (intensity_percent * 255) / 100;
      strip.setBrightness(current_brightness);
    }
    
    pattern_running = false;
    setLEDColor(r, g, b, w);
    
    // Update state
    led_state.r = r;
    led_state.g = g;
    led_state.b = b;
    led_state.w = w;
    led_state.pattern = "";
    led_state.pattern_active = false;
    
    publishState();
    Serial.printf("LED color set: R=%d G=%d B=%d W=%d (brightness: %d%%)\n", r, g, b, w, (current_brightness * 100) / 255);
    
  } else if (action == "set_brightness") {
    uint8_t brightness = doc["value"] | current_brightness;
    if (brightness > 255) brightness = 255;
    
    current_brightness = brightness;
    led_state.brightness = brightness;
    strip.setBrightness(brightness);
    strip.show();
    
    publishState();
    Serial.printf("LED brightness set: %d\n", brightness);
    
  } else if (action == "clear") {
    pattern_running = false;
    strip.clear();
    strip.show();
    
    led_state.r = 0;
    led_state.g = 0;
    led_state.b = 0;
    led_state.w = 0;
    led_state.pattern = "";
    led_state.pattern_active = false;
    
    publishState();
    Serial.println("LED strip cleared");
    
  } else if (action == "pattern") {
    String pattern_name = doc["name"] | "";
    int speed = doc["speed"] | 50;
    
    if (pattern_name == "") {
      Serial.println("Pattern name required");
      return;
    }
    
    // Start pattern
    pattern_running = true;
    current_pattern = pattern_name;
    pattern_start_time = millis();
    
    led_state.pattern = pattern_name;
    led_state.pattern_active = true;
    
    publishState();
    Serial.printf("Pattern started: %s (speed: %d)\n", pattern_name.c_str(), speed);
    
  } else if (action == "stop") {
    pattern_running = false;
    current_pattern = "";
    
    led_state.pattern = "";
    led_state.pattern_active = false;
    
    publishState();
    Serial.println("Pattern stopped");
    
  } else if (action == "progress") {
    uint8_t percent = doc["value"] | 0;
    if (percent > 100) percent = 100;
    
    // Get color from command (default to white if not specified)
    uint8_t r = doc["r"] | 255;
    uint8_t g = doc["g"] | 255;
    uint8_t b = doc["b"] | 255;
    uint8_t w = doc["w"] | 255;
    
    setProgress(percent, r, g, b, w);
    
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
  StaticJsonDocument<300> doc;
  doc["r"] = led_state.r;
  doc["g"] = led_state.g;
  doc["b"] = led_state.b;
  doc["w"] = led_state.w;
  doc["brightness"] = led_state.brightness;
  doc["pattern"] = led_state.pattern;
  doc["pattern_active"] = led_state.pattern_active;
  
  mqtt.publishJSON(mqtt_topic_state, doc);
}

// ============================================================================
// LED CONTROL FUNCTIONS
// ============================================================================

void showStatusLED() {
  // Show status with LED colors
  if (mqtt.connected() && mqtt.wifiConnected()) {
    setLEDColor(0, 255, 0, 0); // Green
  } else if (mqtt.wifiConnected()) {
    setLEDColor(0, 0, 255, 0); // Blue
  } else {
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

void setLEDPixel(int index, uint8_t r, uint8_t g, uint8_t b, uint8_t w) {
  if (index >= 0 && index < LED_COUNT) {
    strip.setPixelColor(index, strip.Color(r, g, b, w));
  }
}

void setProgress(uint8_t percent, uint8_t r, uint8_t g, uint8_t b, uint8_t w) {
  // Clamp percent to 0-100
  if (percent > 100) percent = 100;
  
  // Calculate how many LEDs should be lit
  int ledsToLight = (percent * STATE_LED_COUNT) / 100;
  
  // Clear the strip first
  stateStrip.clear();
  
  // Light up the calculated number of LEDs
  for (int i = 0; i < ledsToLight; i++) {
    stateStrip.setPixelColor(i, stateStrip.Color(r, g, b, w));
  }
  
  stateStrip.show();
  
  Serial.printf("Progress set: %d%% (%d/%d LEDs)\n", percent, ledsToLight, STATE_LED_COUNT);
}

// ============================================================================
// LED PATTERN FUNCTIONS
// ============================================================================

void updatePattern() {
  if (!pattern_running) return;
  
  unsigned long elapsed = millis() - pattern_start_time;
  
  if (current_pattern == "rainbow") {
    rainbowPattern(elapsed);
  } else if (current_pattern == "chase") {
    chasePattern(elapsed);
  } else if (current_pattern == "fade") {
    fadePattern(elapsed);
  } else if (current_pattern == "wave") {
    wavePattern(elapsed);
  } else if (current_pattern == "sparkle") {
    sparklePattern(elapsed);
  }
}

void rainbowPattern(unsigned long elapsed) {
  for (int i = 0; i < LED_COUNT; i++) {
    int hue = (elapsed / 10 + i * 256 / LED_COUNT) % 256;
    uint32_t color = Wheel(hue);
    strip.setPixelColor(i, color);
  }
  strip.show();
}

void chasePattern(unsigned long elapsed) {
  strip.clear();
  int pos = (elapsed / 50) % (LED_COUNT * 2);
  if (pos < LED_COUNT) {
    setLEDPixel(pos, 255, 0, 0, 0); // Red
  } else {
    setLEDPixel(LED_COUNT * 2 - pos - 1, 0, 0, 255, 0); // Blue
  }
  strip.show();
}

void fadePattern(unsigned long elapsed) {
  int brightness = (sin(elapsed / 50.0) + 1.0) * 127.5;
  strip.setBrightness(brightness);
  setLEDColor(255, 255, 255, 0); // White
  strip.show();
  strip.setBrightness(current_brightness); // Restore brightness
}

void wavePattern(unsigned long elapsed) {
  for (int i = 0; i < LED_COUNT; i++) {
    float wave = (sin((elapsed / 20.0) + (i * 0.5)) + 1.0) / 2.0;
    uint8_t r = wave * 255;
    uint8_t g = wave * 128;
    uint8_t b = wave * 64;
    setLEDPixel(i, r, g, b, 0);
  }
  strip.show();
}

void sparklePattern(unsigned long elapsed) {
  static unsigned long last_sparkle = 0;
  if (elapsed - last_sparkle > 100) {
    strip.clear();
    for (int i = 0; i < 5; i++) {
      int pos = random(LED_COUNT);
      setLEDPixel(pos, 255, 255, 255, 0);
    }
    strip.show();
    last_sparkle = elapsed;
  }
}

// Helper function for rainbow effect
uint32_t Wheel(byte WheelPos) {
  WheelPos = 255 - WheelPos;
  if (WheelPos < 85) {
    return strip.Color(255 - WheelPos * 3, 0, WheelPos * 3, 0);
  }
  if (WheelPos < 170) {
    WheelPos -= 85;
    return strip.Color(0, WheelPos * 3, 255 - WheelPos * 3, 0);
  }
  WheelPos -= 170;
  return strip.Color(WheelPos * 3, 255 - WheelPos * 3, 0, 0);
}

