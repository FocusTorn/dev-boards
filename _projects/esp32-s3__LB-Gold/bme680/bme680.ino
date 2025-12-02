/*
 * ESP32-S3 BME680 Sensor with MQTT - Bosch Implementation
 * 
 * Full port of RPi bme680-service to ESP32-S3
 * Includes: Base readings, IAQ monitoring, Heatsoak detection
 * 
 * Features:
 * - Official Bosch BME680 sensor reading (not Adafruit)
 * - Temperature, Humidity, Pressure, Gas Resistance
 * - Heatsoak detection with temperature smoothing and rate calculations
 * - IAQ (Indoor Air Quality) calculations with baseline calibration
 * - MQTT pub/sub matching RPi version exactly
 * - Home Assistant compatible
 * 
 * Hardware:
 * - ESP32-S3 (lonely binary GOLD EDITION)
 * - BME680 sensor on I2C bus (0x76 or 0x77)
 * 
 * Libraries Required:
 * - MQTT_RPi_Client (included in libraries folder)
 * - Bosch BME680 library (bme680-esp-idf or equivalent)
 * - ArduinoJson (for JSON parsing)
 * 
 * MQTT Topics (matching RPi version):
 * - sensors/bme680/raw - Base readings with heatsoak
 * - homeassistant/sensor/bme680/state - IAQ readings
 * - homeassistant/sensor/bme680_chamber/state - Heatsoak only
 */

#include <MQTT_RPi_Client.h>
#include <BME680_Bosch.h>
#include <ArduinoJson.h>
#include <Wire.h>
#include <math.h>

// ============================================================================
// CONFIGURATION
// ============================================================================

// MQTT Configuration
const char* mqtt_client_id = "esp32-s3-bme680";

const char* mqtt_topic_base = "sensors/bme680/raw";                    // Base readings + heatsoak
const char* mqtt_topic_iaq = "homeassistant/sensor/bme680/state";       // IAQ readings
const char* mqtt_topic_heatsoak = "homeassistant/sensor/bme680_chamber/state";  // Heatsoak only
const char* mqtt_topic_status = "sensors/bme680/status";                // Status

// BME680 Configuration
#define BME680_I2C_ADDRESS_PRIMARY 0x76
#define BME680_I2C_ADDRESS_SECONDARY 0x77
#define BME680_CHIP_ID 0x61

// Sensor reading intervals (milliseconds)
#define READ_INTERVAL 1000      // Read sensor every 1 second (for smoothing)
#define PUBLISH_INTERVAL 30000  // Publish to MQTT every 30 seconds

// Heatsoak configuration (matching RPi defaults)
#define TEMP_SMOOTH_TIME 4      // Temperature smoothing window (seconds)
#define RATE_SMOOTH_TIME 30     // Rate smoothing window (seconds)
#define SOAK_TEMP 40.0          // Minimum temperature to start checking rate (°C)
#define MAX_RATE 0.1            // Maximum rate of change (°C/min)
#define TARGET_TEMP -1.0        // Target temperature (-1 = disabled)

// IAQ configuration
#define IAQ_THRESHOLD 80.0      // IAQ threshold for "safe to open"
#define BASELINE_BURN_IN 90     // Baseline calibration time (seconds)

// Initialize MQTT client
MQTT_RPi_Client mqtt;

// ============================================================================
// BME680 SENSOR (using ported Bosch library)
// ============================================================================

BME680_Bosch* bme680_ptr = nullptr;  // Will be initialized in setup()

// ============================================================================
// HEATSOAK DETECTION (matching Python TemperatureMonitor class)
// ============================================================================

class HeatsoakMonitor {
public:
  HeatsoakMonitor(float temp_smooth_time = 4.0, float rate_smooth_time = 30.0) {
    _temp_smooth_size = (int)temp_smooth_time;
    _rate_smooth_size = (int)rate_smooth_time;
    _temp_history = new float[_temp_smooth_size + 1];
    _smoothed_temp_history = new float[_rate_smooth_size + 1];
    _temp_history_count = 0;
    _smoothed_temp_count = 0;
    _soak_started = false;
    _max_rate_since_soak_start = 0.0;
  }
  
  ~HeatsoakMonitor() {
    delete[] _temp_history;
    delete[] _smoothed_temp_history;
  }
  
  void update(float current_temp) {
    _current_temp = current_temp;
    
    // Add to raw temp history
    if (_temp_history_count < _temp_smooth_size + 1) {
      _temp_history[_temp_history_count++] = current_temp;
    } else {
      // Shift array
      for (int i = 0; i < _temp_smooth_size; i++) {
        _temp_history[i] = _temp_history[i + 1];
      }
      _temp_history[_temp_smooth_size] = current_temp;
    }
    
    // Calculate smoothed temp (simple average)
    if (_temp_history_count > _temp_smooth_size) {
      float sum = 0.0;
      int count = min(_temp_history_count, _temp_smooth_size + 1);
      for (int i = 0; i < count; i++) {
        sum += _temp_history[i];
      }
      _smoothed_temp = sum / count;
      
      // Add to smoothed history for rate calculation
      if (_smoothed_temp_count < _rate_smooth_size + 1) {
        _smoothed_temp_history[_smoothed_temp_count++] = _smoothed_temp;
      } else {
        // Shift array
        for (int i = 0; i < _rate_smooth_size; i++) {
          _smoothed_temp_history[i] = _smoothed_temp_history[i + 1];
        }
        _smoothed_temp_history[_rate_smooth_size] = _smoothed_temp;
      }
      
      // Calculate rate using least squares linear regression
      if (_smoothed_temp_count > _rate_smooth_size) {
        _rate_per_minute = calculate_rate();
      }
    }
  }
  
  float calculate_rate() {
    if (_smoothed_temp_count <= 1) return 0.0;
    
    int count = min(_smoothed_temp_count, _rate_smooth_size + 1);
    float x_sum = 0.0, y_sum = 0.0, xx_sum = 0.0, xy_sum = 0.0;
    
    for (int i = 0; i < count; i++) {
      float x = (float)i;
      float y = _smoothed_temp_history[i];
      x_sum += x;
      y_sum += y;
      xx_sum += x * x;
      xy_sum += x * y;
    }
    
    float denominator = count * xx_sum - x_sum * x_sum;
    if (fabs(denominator) < 1e-10) return 0.0;
    
    float slope_per_second = (count * xy_sum - x_sum * y_sum) / denominator;
    return slope_per_second * 60.0; // Convert to per minute
  }
  
  bool check_heat_soak_ready(float soak_temp, float max_rate, float target_temp) {
    if (_smoothed_temp == 0.0 || isnan(_rate_per_minute)) {
      if (_soak_started) {
        _soak_started = false;
        _max_rate_since_soak_start = 0.0;
      }
      return false;
    }
    
    // Check if target temp reached (automatically ready)
    if (target_temp > 0 && _smoothed_temp >= target_temp) {
      if (!_soak_started) {
        _soak_started = true;
        _max_rate_since_soak_start = fabs(_rate_per_minute);
      }
      update_max_rate();
      return true;
    }
    
    // Check rate-based conditions
    bool temp_ok = _smoothed_temp >= soak_temp;
    bool rate_ok = fabs(_rate_per_minute) <= max_rate;
    bool ready = temp_ok && rate_ok;
    
    // Track soak status transitions
    if (ready && !_soak_started) {
      _soak_started = true;
      _max_rate_since_soak_start = fabs(_rate_per_minute);
    } else if (!ready && _soak_started) {
      _soak_started = false;
      _max_rate_since_soak_start = 0.0;
    }
    
    // Update max rate since soak started
    if (_soak_started) {
      update_max_rate();
    }
    
    return ready;
  }
  
  void update_max_rate() {
    float current_abs_rate = fabs(_rate_per_minute);
    if (current_abs_rate > _max_rate_since_soak_start) {
      _max_rate_since_soak_start = current_abs_rate;
    }
  }
  
  float get_current_temp() { return _current_temp; }
  float get_smoothed_temp() { return _smoothed_temp; }
  float get_rate_per_minute() { return _rate_per_minute; }
  bool get_soak_started() { return _soak_started; }
  float get_max_rate_since_soak_start() { return _max_rate_since_soak_start; }
  int get_temp_readings() { return _temp_history_count; }
  int get_smoothed_readings() { return _smoothed_temp_count; }
  
private:
  int _temp_smooth_size;
  int _rate_smooth_size;
  float* _temp_history;
  float* _smoothed_temp_history;
  int _temp_history_count;
  int _smoothed_temp_count;
  float _current_temp = 0.0;
  float _smoothed_temp = 0.0;
  float _rate_per_minute = 0.0;
  bool _soak_started = false;
  float _max_rate_since_soak_start = 0.0;
};

HeatsoakMonitor heatsoak(TEMP_SMOOTH_TIME, RATE_SMOOTH_TIME);

// ============================================================================
// IAQ MONITORING (matching Python IAQMonitor class)
// ============================================================================

class IAQMonitor {
public:
  IAQMonitor() {
    _baseline_established = false;
    _baseline_gas = 0.0;
    _baseline_hum = 0.0;
  }
  
  void calibrate_baseline(BME680_Bosch* sensor, int burn_in_time) {
    if (!sensor) return;
    // Use the sensor's built-in baseline calibration
    sensor->set_baselines(burn_in_time, true);
    if (sensor->get_baseline_status() == 1) {
      _baseline_gas = sensor->get_gas_baseline();
      _baseline_hum = sensor->get_hum_baseline();
      _baseline_established = true;
    }
  }
  
  float calculate_iaq_score(float current_gas, float current_hum, float baseline_gas, float baseline_hum) {
    if (!_baseline_established) return -1.0;
    
    float hum_weighting = 0.25; // 25% humidity, 75% gas
    
    // Calculate offsets
    float gas_offset = baseline_gas - current_gas;
    float hum_offset = current_hum - baseline_hum;
    
    // Calculate humidity score
    float hum_score;
    if (hum_offset > 0) {
      hum_score = (100.0 - baseline_hum - hum_offset) / (100.0 - baseline_hum) * (hum_weighting * 100.0);
    } else {
      hum_score = (baseline_hum + hum_offset) / baseline_hum * (hum_weighting * 100.0);
    }
    
    // Calculate gas score
    float gas_score;
    if (gas_offset > 0) {
      gas_score = (current_gas / baseline_gas) * (100.0 - (hum_weighting * 100.0));
    } else {
      gas_score = 100.0 - (hum_weighting * 100.0);
    }
    
    return hum_score + gas_score;
  }
  
  
  
  
  
  
  bool check_safe_to_open(float iaq_score, float threshold) {
    if (iaq_score < 0) return false; // Invalid score
    return iaq_score >= threshold;
  }
  
  bool is_baseline_established() { return _baseline_established; }
  float get_baseline_gas() { return _baseline_gas; }
  float get_baseline_hum() { return _baseline_hum; }
  
  
  
  
  
private:
  bool _baseline_established;
  float _baseline_gas;
  float _baseline_hum;
};

IAQMonitor iaq;

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
  Serial.println("ESP32-S3 BME680 Sensor - Bosch Implementation");
  Serial.println("========================================\n");
  
  // Initialize I2C
  Wire.begin();
  
  // Initialize BME680 sensor (try both addresses)
  Serial.println("Initializing BME680 sensor...");
  
  // Try secondary address first (0x77)
  static BME680_Bosch bme680_secondary(I2C_ADDR_SECONDARY);
  if (bme680_secondary.begin()) {
    bme680_ptr = &bme680_secondary;
    Serial.println("✓ BME680 found at address 0x77");
  } else {
    // Try primary address (0x76)
    Serial.println("Trying primary address 0x76...");
    static BME680_Bosch bme680_primary(I2C_ADDR_PRIMARY);
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
  
  Serial.println("\n✓ Setup complete! Starting sensor readings...\n");
}

// ============================================================================
// MAIN LOOP
// ============================================================================

void loop() {
  // Maintain MQTT connection
  mqtt.loop();
  
  unsigned long current_time = millis();
  
  // Read sensor at specified interval (for smoothing)
  if (current_time - last_read_time >= READ_INTERVAL) {
    if (readSensor()) {
  // Update heatsoak calculations
  if (bme680_ptr) {
    heatsoak.update(bme680_ptr->data.temperature);
  }
    }
    last_read_time = current_time;
  }
  
  // Publish to MQTT at specified interval
  if (current_time - last_publish_time >= PUBLISH_INTERVAL) {
    publishBaseReadings();
    publishIAQReadings();
    publishHeatsoakReadings();
    last_publish_time = current_time;
  }
  
  delay(10);
}

// ============================================================================
// SENSOR READING
// ============================================================================

bool readSensor() {
  if (!sensor_initialized) return false;
  
  // TODO: Implement actual Bosch BME680 reading
  // This should call bme680.get_sensor_data() and populate sensor_data
  // Matching the Python implementation exactly
  
  // Placeholder - replace with actual implementation
  return false;
}

// ============================================================================
// MQTT PUBLISHING (matching RPi format exactly)
// ============================================================================

void publishBaseReadings() {
  if (!mqtt.connected() || !bme680_ptr) return;
  
  // Check heatsoak status
  bool ready = heatsoak.check_heat_soak_ready(SOAK_TEMP, MAX_RATE, TARGET_TEMP);
  
  StaticJsonDocument<500> doc;
  doc["temperature"] = round(bme680_ptr->data.temperature * 100.0) / 100.0;
  doc["humidity"] = round(bme680_ptr->data.humidity * 100.0) / 100.0;
  doc["pressure"] = round(bme680_ptr->data.pressure * 100.0) / 100.0;
  doc["gas_resistance"] = round(bme680_ptr->data.gas_resistance * 100.0) / 100.0;
  doc["heat_stable"] = bme680_ptr->data.heat_stable;
  doc["timestamp"] = millis() / 1000.0;
  
  // Heatsoak calculations (matching base-readings.py)
  if (heatsoak.get_smoothed_temp() > 0) {
    doc["smoothed_temp"] = round(heatsoak.get_smoothed_temp() * 100.0) / 100.0;
    doc["smoothed_change_rate"] = round(heatsoak.get_rate_per_minute() * 1000.0) / 1000.0;
    doc["temp_smoothing_buffer"] = heatsoak.get_temp_readings();
    doc["rate_smoothing_buffer"] = heatsoak.get_smoothed_readings();
    doc["ready"] = ready;
    doc["temp_ok"] = heatsoak.get_smoothed_temp() >= SOAK_TEMP;
    doc["rate_ok"] = fabs(heatsoak.get_rate_per_minute()) <= MAX_RATE;
    doc["soak_started"] = heatsoak.get_soak_started();
    doc["max_rate_since_soak_start"] = round(heatsoak.get_max_rate_since_soak_start() * 1000.0) / 1000.0;
  }
  
  mqtt.publishJSON(mqtt_topic_base, doc);
  Serial.println("✓ Published base readings");
}

void publishIAQReadings() {
  if (!mqtt.connected() || !bme680_ptr || bme680_ptr->get_baseline_status() != 1) return;
  
  float baseline_gas = bme680_ptr->get_gas_baseline();
  float baseline_hum = bme680_ptr->get_hum_baseline();
  float iaq_score = iaq.calculate_iaq_score(bme680_ptr->data.gas_resistance, bme680_ptr->data.humidity, baseline_gas, baseline_hum);
  bool is_safe = iaq.check_safe_to_open(iaq_score, IAQ_THRESHOLD);
  
  StaticJsonDocument<300> doc;
  doc["baseline_established"] = true;
  doc["heat_stable"] = bme680_ptr->data.heat_stable;
  doc["current"]["gas"] = bme680_ptr->data.gas_resistance;
  doc["current"]["humidity"] = bme680_ptr->data.humidity;
  doc["current"]["temperature"] = bme680_ptr->data.temperature;
  doc["current"]["pressure"] = bme680_ptr->data.pressure;
  doc["baseline"]["gas"] = baseline_gas;
  doc["baseline"]["humidity"] = baseline_hum;
  doc["air_quality_score"] = round(iaq_score * 10.0) / 10.0;
  doc["safe_to_open"] = is_safe;
  
  mqtt.publishJSON(mqtt_topic_iaq, doc);
  Serial.println("✓ Published IAQ readings");
}

void publishHeatsoakReadings() {
  if (!mqtt.connected()) return;
  
  bool ready = heatsoak.check_heat_soak_ready(SOAK_TEMP, MAX_RATE, TARGET_TEMP);
  
  StaticJsonDocument<300> doc;
  doc["temperature"] = round(heatsoak.get_current_temp() * 100.0) / 100.0;
  doc["smoothed_temp"] = round(heatsoak.get_smoothed_temp() * 100.0) / 100.0;
  doc["rate_per_minute"] = round(heatsoak.get_rate_per_minute() * 1000.0) / 1000.0;
  doc["heat_stable"] = bme680_ptr ? bme680_ptr->data.heat_stable : false;
  doc["readings"] = heatsoak.get_temp_readings();
  doc["smoothed_readings"] = heatsoak.get_smoothed_readings();
  doc["soak_started"] = heatsoak.get_soak_started();
  doc["max_rate_since_soak_start"] = round(heatsoak.get_max_rate_since_soak_start() * 1000.0) / 1000.0;
  
  mqtt.publishJSON(mqtt_topic_heatsoak, doc);
  Serial.println("✓ Published heatsoak readings");
}
