/*
 * MQTT_RPi_Client - Reusable MQTT Client Library for Raspberry Pi Mosquitto
 * 
 * This library provides a simple interface for ESP32 devices to connect
 * to a Raspberry Pi MQTT broker with authentication.
 * 
 * Features:
 * - WiFi connection management
 * - MQTT connection with username/password authentication
 * - Automatic reconnection handling
 * - Status publishing
 * - Connection state monitoring
 * 
 * Usage:
 *   #include <MQTT_RPi_Client.h>
 *   
 *   MQTT_RPi_Client mqtt;
 *   
 *   void setup() {
 *     mqtt.begin("WiFi_SSID", "WiFi_Password", "192.168.1.50", 1883, "esp32", "password123", "client-id");
 *   }
 *   
 *   void loop() {
 *     mqtt.loop();
 *     if (mqtt.connected()) {
 *       mqtt.publish("topic", "message");
 *     }
 *   }
 */

#ifndef MQTT_RPI_CLIENT_H
#define MQTT_RPI_CLIENT_H

#include <WiFi.h>
#include <PubSubClient.h>
#include <ArduinoJson.h>

class MQTT_RPi_Client {
public:
  // Default configuration values (can be overridden)
  static const char* DEFAULT_WIFI_SSID;
  static const char* DEFAULT_WIFI_PASSWORD;
  static const char* DEFAULT_MQTT_SERVER;
  static const int DEFAULT_MQTT_PORT;
  static const char* DEFAULT_MQTT_USERNAME;
  static const char* DEFAULT_MQTT_PASSWORD;

public:
public:
  MQTT_RPi_Client();
  
  // Initialize and connect (with all parameters)
  void begin(const char* wifi_ssid, const char* wifi_password,
             const char* mqtt_server, int mqtt_port,
             const char* mqtt_username, const char* mqtt_password,
             const char* mqtt_client_id);
  
  // Initialize and connect (using defaults, only client_id required)
  // Pass nullptr for any parameter to use default value
  void begin(const char* mqtt_client_id,
             const char* wifi_ssid = nullptr,
             const char* wifi_password = nullptr,
             const char* mqtt_server = nullptr,
             int mqtt_port = -1,
             const char* mqtt_username = nullptr,
             const char* mqtt_password = nullptr);
  
  // Main loop - call this in your loop()
  void loop();
  
  // Connection status
  bool connected();
  bool wifiConnected();
  
  // Publishing
  bool publish(const char* topic, const char* payload, bool retain = false);
  bool publish(const char* topic, const String& payload, bool retain = false);
  bool publishJSON(const char* topic, JsonDocument& doc, bool retain = false);
  
  // Status publishing (automatic)
  void publishStatus(const char* status_topic = nullptr);
  
  // Callback for incoming messages
  void setCallback(void (*callback)(char*, uint8_t*, unsigned int));
  
  // Subscribe to topic
  bool subscribe(const char* topic);
  
  // Get WiFi and MQTT info
  String getIPAddress();
  int getRSSI();
  unsigned long getUptime();
  
  // Manual reconnection
  void reconnect();
  
  // Configuration
  void setStatusTopic(const char* topic);
  void setStatusInterval(unsigned long interval_ms);
  void enableSerialDebug(bool enable);

private:
  WiFiClient _wifiClient;
  PubSubClient _mqttClient;
  
  // Configuration
  const char* _wifi_ssid;
  const char* _wifi_password;
  const char* _mqtt_server;
  int _mqtt_port;
  const char* _mqtt_username;
  const char* _mqtt_password;
  const char* _mqtt_client_id;
  const char* _status_topic;
  
  // State
  bool _initialized;
  bool _serial_debug;
  unsigned long _last_status_publish;
  unsigned long _status_interval;
  unsigned long _start_time;
  
  // Internal methods
  void connectWiFi();
  void connectMQTT();
  void publishDeviceStatus();
};

#endif

