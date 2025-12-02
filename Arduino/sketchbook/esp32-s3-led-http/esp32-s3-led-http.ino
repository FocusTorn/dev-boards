/*
 * ESP32-S3 LED Controller with HTTP REST API
 * 
 * Direct WiFi communication - NO POLLING REQUIRED
 * Raspberry Pi sends HTTP POST requests directly to ESP32-S3
 * 
 * Features:
 * - SK6812 RGBW LED strip control
 * - HTTP REST API server (port 80)
 * - Direct WiFi communication (no MQTT broker needed)
 * - Multiple LED patterns and effects
 * 
 * Hardware:
 * - ESP32-S3 (lonely binary GOLD EDITION)
 * - SK6812 RGBW LED strip on GPIO pin
 * 
 * Libraries Required:
 * - WiFi (built-in)
 * - WebServer (built-in ESP32)
 * - Adafruit NeoPixel (for SK6812)
 * - ArduinoJson (for JSON parsing)
 */

#include <WiFi.h>
#include <WebServer.h>
#include <Adafruit_NeoPixel.h>
#include <ArduinoJson.h>

// ============================================================================
// CONFIGURATION - Modify these values for your setup
// ============================================================================

// WiFi Configuration
const char* ssid = "YOUR_WIFI_SSID";
const char* password = "YOUR_WIFI_PASSWORD";

// HTTP Server Configuration
const int http_port = 80;

// SK6812 LED Strip Configuration
#define LED_PIN 4        // GPIO pin for LED data line (change as needed)
#define LED_COUNT 30     // Number of LEDs in strip
#define LED_BRIGHTNESS 50  // 0-255 (default brightness)

Adafruit_NeoPixel strip(LED_COUNT, LED_PIN, NEO_GRBW + NEO_KHZ800);

// ============================================================================
// GLOBAL VARIABLES
// ============================================================================

WebServer server(http_port);

// LED state
uint8_t current_brightness = LED_BRIGHTNESS;
bool pattern_running = false;
unsigned long pattern_start_time = 0;
String current_pattern = "";

// ============================================================================
// SETUP
// ============================================================================

void setup() {
  Serial.begin(115200);
  delay(1000);
  
  Serial.println("\n\n========================================");
  Serial.println("ESP32-S3 LED Controller - HTTP API");
  Serial.println("========================================\n");
  
  // Initialize LED strip
  strip.begin();
  strip.setBrightness(current_brightness);
  strip.clear();
  strip.show();
  Serial.println("✓ SK6812 LED strip initialized");
  
  // Connect to WiFi
  connectWiFi();
  
  // Setup HTTP server routes
  setupHTTPServer();
  
  // Start HTTP server
  server.begin();
  Serial.print("✓ HTTP server started on port ");
  Serial.println(http_port);
  Serial.print("  Access at: http://");
  Serial.println(WiFi.localIP());
  
  // Initial status LED indication
  showStatusLED();
  
  Serial.println("\n✓ Setup complete! Ready for HTTP commands...\n");
}

// ============================================================================
// MAIN LOOP
// ============================================================================

void loop() {
  server.handleClient();  // Handle HTTP requests (non-blocking)
  
  // Update running patterns
  if (pattern_running) {
    updatePattern();
  }
  
  delay(10); // Small delay to prevent watchdog issues
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
    Serial.print("  MAC address: ");
    Serial.println(WiFi.macAddress());
  } else {
    Serial.println("\n✗ WiFi connection failed!");
  }
}

// ============================================================================
// HTTP SERVER SETUP
// ============================================================================

void setupHTTPServer() {
  // Root endpoint - API info
  server.on("/", HTTP_GET, handleRoot);
  
  // LED control endpoints
  server.on("/api/led/color", HTTP_POST, handleSetColor);
  server.on("/api/led/brightness", HTTP_POST, handleSetBrightness);
  server.on("/api/led/clear", HTTP_POST, handleClear);
  server.on("/api/led/pattern", HTTP_POST, handlePattern);
  server.on("/api/led/stop", HTTP_POST, handleStopPattern);
  
  // Status endpoint
  server.on("/api/status", HTTP_GET, handleStatus);
  
  // 404 handler
  server.onNotFound(handleNotFound);
}

// ============================================================================
// HTTP REQUEST HANDLERS
// ============================================================================

void handleRoot() {
  String html = "<!DOCTYPE html><html><head><title>ESP32-S3 LED Controller</title></head><body>";
  html += "<h1>ESP32-S3 LED Controller API</h1>";
  html += "<h2>Endpoints:</h2>";
  html += "<ul>";
  html += "<li><b>POST /api/led/color</b> - Set LED color (JSON: {\"r\":255,\"g\":0,\"b\":0,\"w\":0})</li>";
  html += "<li><b>POST /api/led/brightness</b> - Set brightness (JSON: {\"value\":128})</li>";
  html += "<li><b>POST /api/led/clear</b> - Clear all LEDs</li>";
  html += "<li><b>POST /api/led/pattern</b> - Start pattern (JSON: {\"name\":\"rainbow\",\"speed\":50})</li>";
  html += "<li><b>POST /api/led/stop</b> - Stop current pattern</li>";
  html += "<li><b>GET /api/status</b> - Get device status</li>";
  html += "</ul>";
  html += "<h2>Patterns:</h2>";
  html += "<ul>";
  html += "<li>rainbow - Rainbow cycle</li>";
  html += "<li>chase - Color chase effect</li>";
  html += "<li>fade - Fade in/out</li>";
  html += "<li>wave - Wave effect</li>";
  html += "<li>sparkle - Random sparkles</li>";
  html += "</ul>";
  html += "</body></html>";
  
  server.send(200, "text/html", html);
}

void handleSetColor() {
  if (server.hasArg("plain")) {
    String body = server.arg("plain");
    
    StaticJsonDocument<200> doc;
    DeserializationError error = deserializeJson(doc, body);
    
    if (error) {
      server.send(400, "application/json", "{\"error\":\"Invalid JSON\"}");
      return;
    }
    
    // Stop any running pattern
    pattern_running = false;
    
    uint8_t r = doc["r"] | 0;
    uint8_t g = doc["g"] | 0;
    uint8_t b = doc["b"] | 0;
    uint8_t w = doc["w"] | 0;
    
    setLEDColor(r, g, b, w);
    
    Serial.printf("LED color set via HTTP: R=%d G=%d B=%d W=%d\n", r, g, b, w);
    
    server.send(200, "application/json", "{\"status\":\"ok\",\"color\":{\"r\":" + String(r) + ",\"g\":" + String(g) + ",\"b\":" + String(b) + ",\"w\":" + String(w) + "}}");
  } else {
    server.send(400, "application/json", "{\"error\":\"No data provided\"}");
  }
}

void handleSetBrightness() {
  if (server.hasArg("plain")) {
    String body = server.arg("plain");
    
    StaticJsonDocument<200> doc;
    DeserializationError error = deserializeJson(doc, body);
    
    if (error) {
      server.send(400, "application/json", "{\"error\":\"Invalid JSON\"}");
      return;
    }
    
    uint8_t brightness = doc["value"] | current_brightness;
    if (brightness > 255) brightness = 255;
    
    current_brightness = brightness;
    strip.setBrightness(brightness);
    strip.show();
    
    Serial.printf("LED brightness set via HTTP: %d\n", brightness);
    
    server.send(200, "application/json", "{\"status\":\"ok\",\"brightness\":" + String(brightness) + "}");
  } else {
    server.send(400, "application/json", "{\"error\":\"No data provided\"}");
  }
}

void handleClear() {
  pattern_running = false;
  strip.clear();
  strip.show();
  
  Serial.println("LED strip cleared via HTTP");
  
  server.send(200, "application/json", "{\"status\":\"ok\",\"message\":\"LEDs cleared\"}");
}

void handlePattern() {
  if (server.hasArg("plain")) {
    String body = server.arg("plain");
    
    StaticJsonDocument<200> doc;
    DeserializationError error = deserializeJson(doc, body);
    
    if (error) {
      server.send(400, "application/json", "{\"error\":\"Invalid JSON\"}");
      return;
    }
    
    String pattern_name = doc["name"] | "";
    int speed = doc["speed"] | 50;
    
    if (pattern_name == "") {
      server.send(400, "application/json", "{\"error\":\"Pattern name required\"}");
      return;
    }
    
    // Start pattern
    pattern_running = true;
    current_pattern = pattern_name;
    pattern_start_time = millis();
    
    Serial.printf("Pattern started via HTTP: %s (speed: %d)\n", pattern_name.c_str(), speed);
    
    server.send(200, "application/json", "{\"status\":\"ok\",\"pattern\":\"" + pattern_name + "\",\"speed\":" + String(speed) + "}");
  } else {
    server.send(400, "application/json", "{\"error\":\"No data provided\"}");
  }
}

void handleStopPattern() {
  pattern_running = false;
  current_pattern = "";
  
  Serial.println("Pattern stopped via HTTP");
  
  server.send(200, "application/json", "{\"status\":\"ok\",\"message\":\"Pattern stopped\"}");
}

void handleStatus() {
  StaticJsonDocument<300> doc;
  doc["status"] = "online";
  doc["uptime"] = millis() / 1000;
  doc["free_heap"] = ESP.getFreeHeap();
  doc["wifi_rssi"] = WiFi.RSSI();
  doc["ip_address"] = WiFi.localIP().toString();
  doc["brightness"] = current_brightness;
  doc["pattern_running"] = pattern_running;
  doc["current_pattern"] = current_pattern;
  
  String response;
  serializeJson(doc, response);
  
  server.send(200, "application/json", response);
}

void handleNotFound() {
  server.send(404, "application/json", "{\"error\":\"Not found\"}");
}

// ============================================================================
// LED CONTROL FUNCTIONS
// ============================================================================

void showStatusLED() {
  // Show status with LED colors
  if (WiFi.status() == WL_CONNECTED) {
    setLEDColor(0, 255, 0, 0); // Green
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

