/*
 * MQTT_RPi_Client Implementation
 */

#include "MQTT_RPi_Client.h"

// Default configuration values
const char* MQTT_RPi_Client::DEFAULT_WIFI_SSID = "MATT_5fkj4zn";
const char* MQTT_RPi_Client::DEFAULT_WIFI_PASSWORD = "8a4bi3bnw#y7";
const char* MQTT_RPi_Client::DEFAULT_MQTT_SERVER = "192.168.1.50";
const int MQTT_RPi_Client::DEFAULT_MQTT_PORT = 1883;
const char* MQTT_RPi_Client::DEFAULT_MQTT_USERNAME = "esp32";
const char* MQTT_RPi_Client::DEFAULT_MQTT_PASSWORD = "password123";

MQTT_RPi_Client::MQTT_RPi_Client() :
  _mqttClient(_wifiClient),
  _wifi_ssid(nullptr),
  _wifi_password(nullptr),
  _mqtt_server(nullptr),
  _mqtt_port(1883),
  _mqtt_username(nullptr),
  _mqtt_password(nullptr),
  _mqtt_client_id(nullptr),
  _status_topic(nullptr),
  _initialized(false),
  _serial_debug(true),
  _last_status_publish(0),
  _status_interval(30000),  // 30 seconds default
  _start_time(0)
{
}

void MQTT_RPi_Client::begin(const char* wifi_ssid, const char* wifi_password,
                            const char* mqtt_server, int mqtt_port,
                            const char* mqtt_username, const char* mqtt_password,
                            const char* mqtt_client_id) {
  _wifi_ssid = wifi_ssid ? wifi_ssid : DEFAULT_WIFI_SSID;
  _wifi_password = wifi_password ? wifi_password : DEFAULT_WIFI_PASSWORD;
  _mqtt_server = mqtt_server ? mqtt_server : DEFAULT_MQTT_SERVER;
  _mqtt_port = (mqtt_port > 0) ? mqtt_port : DEFAULT_MQTT_PORT;
  _mqtt_username = mqtt_username ? mqtt_username : DEFAULT_MQTT_USERNAME;
  _mqtt_password = mqtt_password ? mqtt_password : DEFAULT_MQTT_PASSWORD;
  _mqtt_client_id = mqtt_client_id;
  _start_time = millis();
  
  if (_serial_debug) {
    Serial.println("\n--- MQTT_RPi_Client Initialization ---");
  }
  
  // Connect WiFi
  connectWiFi();
  
  // Setup MQTT
  _mqttClient.setServer(_mqtt_server, _mqtt_port);
  connectMQTT();
  
  _initialized = true;
  
  if (_serial_debug) {
    Serial.println("✓ MQTT_RPi_Client initialized");
    Serial.println();
  }
}

// Overloaded begin() with defaults - only client_id required
void MQTT_RPi_Client::begin(const char* mqtt_client_id,
                            const char* wifi_ssid,
                            const char* wifi_password,
                            const char* mqtt_server,
                            int mqtt_port,
                            const char* mqtt_username,
                            const char* mqtt_password) {
  // Validate client ID
  if (mqtt_client_id == nullptr) {
    if (_serial_debug) {
      Serial.println("✗ ERROR: MQTT client ID cannot be null!");
    }
    return;
  }
  
  size_t client_id_len = strlen(mqtt_client_id);
  if (client_id_len == 0) {
    if (_serial_debug) {
      Serial.println("✗ ERROR: MQTT client ID cannot be empty!");
    }
    return;
  }
  
  // MQTT 3.1 client ID limit is 23 characters
  if (client_id_len > 23) {
    if (_serial_debug) {
      Serial.print("✗ ERROR: MQTT client ID too long (");
      Serial.print(client_id_len);
      Serial.println(" chars, max 23)!");
    }
    return;
  }
  
  _mqtt_client_id = mqtt_client_id;
  
  // Use defaults for nullptr parameters
  _wifi_ssid = wifi_ssid ? wifi_ssid : DEFAULT_WIFI_SSID;
  _wifi_password = wifi_password ? wifi_password : DEFAULT_WIFI_PASSWORD;
  _mqtt_server = mqtt_server ? mqtt_server : DEFAULT_MQTT_SERVER;
  _mqtt_port = (mqtt_port > 0) ? mqtt_port : DEFAULT_MQTT_PORT;
  _mqtt_username = mqtt_username ? mqtt_username : DEFAULT_MQTT_USERNAME;
  _mqtt_password = mqtt_password ? mqtt_password : DEFAULT_MQTT_PASSWORD;
  _start_time = millis();
  
  if (_serial_debug) {
    Serial.println("\n--- MQTT_RPi_Client Initialization ---");
  }
  
  // Connect WiFi
  connectWiFi();
  
  // Setup MQTT
  _mqttClient.setServer(_mqtt_server, _mqtt_port);
  connectMQTT();
  
  _initialized = true;
  
  if (_serial_debug) {
    Serial.println("✓ MQTT_RPi_Client initialized");
    Serial.println();
  }
}

void MQTT_RPi_Client::loop() {
  if (!_initialized) return;
  
  // Maintain WiFi connection
  if (WiFi.status() != WL_CONNECTED) {
    if (_serial_debug) {
      Serial.println("WiFi disconnected, reconnecting...");
    }
    connectWiFi();
  }
  
  // Maintain MQTT connection
  if (!_mqttClient.connected()) {
    connectMQTT();
  }
  
  // Process MQTT messages
  _mqttClient.loop();
  
  // Publish status periodically
  unsigned long current_time = millis();
  if (current_time - _last_status_publish >= _status_interval) {
    publishDeviceStatus();
    _last_status_publish = current_time;
  }
}

void MQTT_RPi_Client::connectWiFi() {
  if (_serial_debug) {
    Serial.print("Connecting to WiFi: ");
    Serial.println(_wifi_ssid);
  }
  
  WiFi.mode(WIFI_STA);
  WiFi.begin(_wifi_ssid, _wifi_password);
  
  int attempts = 0;
  while (WiFi.status() != WL_CONNECTED && attempts < 20) {
    delay(500);
    if (_serial_debug) {
      Serial.print(".");
    }
    attempts++;
  }
  
  if (WiFi.status() == WL_CONNECTED) {
    if (_serial_debug) {
      Serial.println("\n✓ WiFi connected!");
      Serial.print("  SSID: ");
      Serial.println(_wifi_ssid);
      Serial.print("  IP address: ");
      Serial.println(WiFi.localIP());
      Serial.print("  Signal strength (RSSI): ");
      Serial.print(WiFi.RSSI());
      Serial.println(" dBm");
    }
  } else {
    if (_serial_debug) {
      Serial.println("\n✗ WiFi connection failed!");
      Serial.println("  Check SSID and password");
    }
  }
}

void MQTT_RPi_Client::connectMQTT() {
  // Validate client ID
  if (_mqtt_client_id == nullptr) {
    if (_serial_debug) {
      Serial.println("✗ ERROR: MQTT client ID is null!");
    }
    return;
  }
  
  size_t client_id_len = strlen(_mqtt_client_id);
  if (client_id_len == 0) {
    if (_serial_debug) {
      Serial.println("✗ ERROR: MQTT client ID is empty!");
    }
    return;
  }
  
  // MQTT 3.1 client ID limit is 23 characters
  if (client_id_len > 23) {
    if (_serial_debug) {
      Serial.print("✗ ERROR: MQTT client ID too long (");
      Serial.print(client_id_len);
      Serial.println(" chars, max 23)!");
    }
    return;
  }
  
  while (!_mqttClient.connected()) {
    if (_serial_debug) {
      Serial.print("Connecting to MQTT broker: ");
      Serial.print(_mqtt_server);
      Serial.print(":");
      Serial.println(_mqtt_port);
      Serial.print("  Using client ID: ");
      Serial.println(_mqtt_client_id);
    }
    
    // Connect with username and password
    if (_mqttClient.connect(_mqtt_client_id, _mqtt_username, _mqtt_password)) {
      if (_serial_debug) {
        Serial.println("✓ Connected to MQTT broker");
        Serial.print("  Broker: ");
        Serial.print(_mqtt_server);
        Serial.print(":");
        Serial.println(_mqtt_port);
        Serial.print("  Username: ");
        Serial.println(_mqtt_username);
        Serial.print("  Client ID: ");
        Serial.println(_mqtt_client_id);
      }
      
      // Publish online status
      publishDeviceStatus();
    } else {
      if (_serial_debug) {
        Serial.print("✗ MQTT connection failed, rc=");
        Serial.print(_mqttClient.state());
        Serial.println(" - retrying in 5 seconds");
      }
      delay(5000);
    }
  }
}

void MQTT_RPi_Client::publishDeviceStatus() {
  if (_status_topic == nullptr) return;
  
  StaticJsonDocument<200> doc;
  doc["status"] = _mqttClient.connected() ? "online" : "offline";
  doc["uptime"] = (millis() - _start_time) / 1000;
  doc["free_heap"] = ESP.getFreeHeap();
  doc["wifi_rssi"] = WiFi.RSSI();
  doc["wifi_connected"] = (WiFi.status() == WL_CONNECTED);
  doc["ip_address"] = WiFi.localIP().toString();
  
  String payload;
  serializeJson(doc, payload);
  
  _mqttClient.publish(_status_topic, payload.c_str());
}

bool MQTT_RPi_Client::connected() {
  return _mqttClient.connected();
}

bool MQTT_RPi_Client::wifiConnected() {
  return (WiFi.status() == WL_CONNECTED);
}

bool MQTT_RPi_Client::publish(const char* topic, const char* payload, bool retain) {
  if (!_mqttClient.connected()) {
    if (_serial_debug) {
      Serial.println("⚠ Cannot publish - MQTT not connected");
    }
    return false;
  }
  
  bool result = _mqttClient.publish(topic, payload, retain);
  
  if (_serial_debug && result) {
    Serial.print("✓ Published to ");
    Serial.print(topic);
    Serial.print(": ");
    Serial.println(payload);
  }
  
  return result;
}

bool MQTT_RPi_Client::publish(const char* topic, const String& payload, bool retain) {
  return publish(topic, payload.c_str(), retain);
}

bool MQTT_RPi_Client::publishJSON(const char* topic, JsonDocument& doc, bool retain) {
  String payload;
  serializeJson(doc, payload);
  return publish(topic, payload, retain);
}

void MQTT_RPi_Client::publishStatus(const char* status_topic) {
  if (status_topic != nullptr) {
    _status_topic = status_topic;
  }
  publishDeviceStatus();
}

void MQTT_RPi_Client::setCallback(void (*callback)(char*, uint8_t*, unsigned int)) {
  _mqttClient.setCallback(callback);
}

bool MQTT_RPi_Client::subscribe(const char* topic) {
  if (!_mqttClient.connected()) {
    return false;
  }
  
  bool result = _mqttClient.subscribe(topic);
  
  if (_serial_debug && result) {
    Serial.print("✓ Subscribed to: ");
    Serial.println(topic);
  }
  
  return result;
}

String MQTT_RPi_Client::getIPAddress() {
  return WiFi.localIP().toString();
}

int MQTT_RPi_Client::getRSSI() {
  return WiFi.RSSI();
}

unsigned long MQTT_RPi_Client::getUptime() {
  return (millis() - _start_time) / 1000;
}

void MQTT_RPi_Client::reconnect() {
  connectMQTT();
}

void MQTT_RPi_Client::setStatusTopic(const char* topic) {
  _status_topic = topic;
}

void MQTT_RPi_Client::setStatusInterval(unsigned long interval_ms) {
  _status_interval = interval_ms;
}

void MQTT_RPi_Client::enableSerialDebug(bool enable) {
  _serial_debug = enable;
}

