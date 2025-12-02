/*
 * Custom BME680 Library - Based on Official Bosch Implementation
 * 
 * This library provides accurate BME680 sensor readings with:
 * - Official Bosch calibration algorithms
 * - Baseline calibration for IAQ
 * - IAQ score calculation
 * - Heat stable detection
 * 
 * Ported from Python implementation to Arduino C++
 */

#ifndef BME680_CUSTOM_H
#define BME680_CUSTOM_H

#include <Wire.h>
#include <Arduino.h>

// I2C Addresses
#define BME680_I2C_ADDR_PRIMARY   0x76
#define BME680_I2C_ADDR_SECONDARY 0x77

// Chip ID
#define BME680_CHIP_ID 0x61

// Oversampling settings
#define OS_NONE  0
#define OS_1X    1
#define OS_2X    2
#define OS_4X    3
#define OS_8X    4
#define OS_16X   5

// Filter settings
#define FILTER_SIZE_0   0
#define FILTER_SIZE_1   1
#define FILTER_SIZE_3   2
#define FILTER_SIZE_7   3
#define FILTER_SIZE_15  4
#define FILTER_SIZE_31  5
#define FILTER_SIZE_63  6
#define FILTER_SIZE_127 7

// Power modes
#define SLEEP_MODE  0
#define FORCED_MODE 1

// Gas measurement
#define ENABLE_GAS_MEAS_LOW  0x01
#define ENABLE_GAS_MEAS_HIGH 0x02

// Register addresses
#define CHIP_ID_ADDR          0xD0
#define CHIP_VARIANT_ADDR     0xF0
#define SOFT_RESET_ADDR       0xE0
#define SOFT_RESET_CMD        0xB6
#define FIELD0_ADDR           0x1D
#define CONF_OS_H_ADDR        0x72
#define CONF_T_P_MODE_ADDR    0x74
#define CONF_ODR_FILT_ADDR    0x75
#define CONF_HEAT_CTRL_ADDR   0x70
#define CONF_ODR_RUN_GAS_NBC_ADDR 0x71
#define RES_HEAT0_ADDR        0x5A
#define GAS_WAIT0_ADDR        0x64
#define COEFF_ADDR1           0x89
#define COEFF_ADDR1_LEN       25
#define COEFF_ADDR2           0xE1
#define COEFF_ADDR2_LEN       16
#define ADDR_RES_HEAT_RANGE_ADDR 0x02
#define ADDR_RES_HEAT_VAL_ADDR   0x00
#define ADDR_RANGE_SW_ERR_ADDR   0x04

// Masks
#define NEW_DATA_MSK    0x80
#define GAS_INDEX_MSK   0x0F
#define GAS_RANGE_MSK   0x0F
#define GASM_VALID_MSK  0x20
#define HEAT_STAB_MSK   0x10
#define OSH_MSK         0x07
#define OSP_MSK         0x1C
#define OST_MSK         0xE0
#define FILTER_MSK      0x1C
#define HCTRL_MSK       0x08
#define RUN_GAS_MSK     0x30
#define MODE_MSK         0x03
#define NBCONV_MSK      0x0F
#define RHRANGE_MSK     0x30
#define RSERROR_MSK     0xF0

// Positions
#define OSH_POS         0
#define OSP_POS         2
#define OST_POS         5
#define FILTER_POS      2
#define HCTRL_POS       3
#define RUN_GAS_POS     4
#define MODE_POS        0
#define NBCONV_POS      0

// Field length
#define FIELD_LENGTH    17

// Lookup tables for gas resistance calculation
extern const uint32_t lookupTable1[16];
extern const uint32_t lookupTable2[16];

// Calibration data structure
struct CalibrationData {
  uint16_t par_h1;
  uint16_t par_h2;
  int8_t par_h3;
  int8_t par_h4;
  int8_t par_h5;
  uint8_t par_h6;
  int8_t par_h7;
  int8_t par_gh1;
  int16_t par_gh2;
  int8_t par_gh3;
  uint16_t par_t1;
  int16_t par_t2;
  int8_t par_t3;
  uint16_t par_p1;
  int16_t par_p2;
  int8_t par_p3;
  int16_t par_p4;
  int16_t par_p5;
  int8_t par_p6;
  int8_t par_p7;
  int16_t par_p8;
  int16_t par_p9;
  uint8_t par_p10;
  int32_t t_fine;
  uint8_t res_heat_range;
  int8_t res_heat_val;
  int8_t range_sw_err;
};

// Sensor data structure
struct SensorData {
  float temperature;
  float humidity;
  float pressure;
  float gas_resistance;
  bool heat_stable;
  bool gas_valid;
};

class BME680_Custom {
public:
  BME680_Custom(uint8_t i2c_addr = BME680_I2C_ADDR_PRIMARY);
  bool begin();
  
  // Configuration
  void set_humidity_oversample(uint8_t value);
  void set_pressure_oversample(uint8_t value);
  void set_temperature_oversample(uint8_t value);
  void set_filter(uint8_t value);
  void set_gas_status(uint8_t value);
  void set_gas_heater_temperature(uint16_t temperature, uint8_t nb_profile = 0);
  void set_gas_heater_duration(uint16_t duration, uint8_t nb_profile = 0);
  void select_gas_heater_profile(uint8_t profile);
  void set_power_mode(uint8_t mode);
  
  // Reading
  bool get_sensor_data();
  SensorData data;
  
  // Baseline calibration for IAQ
  bool set_baselines(uint16_t burn_in_time_seconds = 300, bool verbose = false);
  float get_gas_baseline();
  float get_hum_baseline();
  bool is_baseline_established();
  
  // IAQ calculation
  float calculate_iaq_score(float hum_weighting = 0.25);
  bool check_safe_to_open(float threshold = 80.0);
  
private:
  uint8_t _i2c_addr;
  uint8_t _variant;
  CalibrationData _cal;
  int32_t _offset_temp_in_t_fine;
  int32_t _ambient_temperature;
  
  // Baseline data
  float _gas_baseline;
  float _hum_baseline;
  bool _baseline_established;
  
  // IAQ calculation
  float _calculate_humidity_score(float hum_weighting);
  float _calculate_gas_score(float hum_weighting);
  
  // Low-level I2C
  void _write_byte(uint8_t reg, uint8_t value);
  uint8_t _read_byte(uint8_t reg);
  void _read_bytes(uint8_t reg, uint8_t* data, uint8_t len);
  
  // Register manipulation
  void _set_bits(uint8_t reg, uint8_t mask, uint8_t position, uint8_t value);
  
  // Calibration
  void _get_calibration_data();
  
  // Calculations
  int32_t _calc_temperature(uint32_t temp_adc);
  uint32_t _calc_pressure(uint32_t pres_adc);
  uint32_t _calc_humidity(uint16_t hum_adc);
  uint32_t _calc_gas_resistance(uint16_t gas_res_adc, uint8_t gas_range);
  uint8_t _calc_heater_resistance(uint16_t temperature);
  uint8_t _calc_heater_duration(uint16_t duration);
  
  // Helper functions
  int16_t _bytes_to_word(uint8_t msb, uint8_t lsb, bool signed_val = false);
  int8_t _twos_comp(uint8_t val);
};

#endif

