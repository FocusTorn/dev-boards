/*
 * BME680_Bosch - Official Bosch BME680 Library Port for Arduino/ESP32
 * 
 * Ported from the official Bosch BME680 Python implementation
 * Original source: RPi bme680-service/data/monitor/__init__.py
 * 
 * This library provides complete low-level access to the BME680 sensor including:
 * - Temperature, humidity, pressure measurements
 * - VOC (Volatile Organic Compound) / gas resistance measurements
 * - Internal gas heater control with temperature regulation
 * - Calibration data management
 * - Baseline establishment for air quality monitoring
 */

#ifndef BME680_BOSCH_H
#define BME680_BOSCH_H

#include <Wire.h>
#include <Arduino.h>

// Forward declarations
struct FieldData;
struct CalibrationData;
struct TPHSettings;
struct GasSettings;

// ============================================================================
// CONSTANTS (from constants.py)
// ============================================================================

// BME680 General config
#define POLL_PERIOD_MS 10

// BME680 I2C addresses
#define I2C_ADDR_PRIMARY 0x76
#define I2C_ADDR_SECONDARY 0x77

// BME680 unique chip identifier
#define CHIP_ID 0x61

// BME680 coefficients related defines
#define COEFF_SIZE 41
#define COEFF_ADDR1_LEN 25
#define COEFF_ADDR2_LEN 16

// BME680 field_x related defines
#define FIELD_LENGTH 17
#define FIELD_ADDR_OFFSET 17

// Soft reset command
#define SOFT_RESET_CMD 0xb6

// Register map
#define ADDR_RES_HEAT_VAL_ADDR 0x00
#define ADDR_RES_HEAT_RANGE_ADDR 0x02
#define ADDR_RANGE_SW_ERR_ADDR 0x04
#define ADDR_SENS_CONF_START 0x5A
#define ADDR_GAS_CONF_START 0x64
#define FIELD0_ADDR 0x1d
#define RES_HEAT0_ADDR 0x5a
#define GAS_WAIT0_ADDR 0x64
#define CONF_HEAT_CTRL_ADDR 0x70
#define CONF_ODR_RUN_GAS_NBC_ADDR 0x71
#define CONF_OS_H_ADDR 0x72
#define MEM_PAGE_ADDR 0xf3
#define CONF_T_P_MODE_ADDR 0x74
#define CONF_ODR_FILT_ADDR 0x75
#define COEFF_ADDR1 0x89
#define COEFF_ADDR2 0xe1
#define CHIP_ID_ADDR 0xd0
#define CHIP_VARIANT_ADDR 0xf0
#define SOFT_RESET_ADDR 0xe0

// Variant
#define VARIANT_LOW 0x00
#define VARIANT_HIGH 0x01

// Heater control settings
#define ENABLE_HEATER 0x00
#define DISABLE_HEATER 0x08

// Gas measurement settings
#define DISABLE_GAS_MEAS 0x00
#define ENABLE_GAS_MEAS -1
#define ENABLE_GAS_MEAS_LOW 0x01
#define ENABLE_GAS_MEAS_HIGH 0x02

// Over-sampling settings
#define OS_NONE 0
#define OS_1X 1
#define OS_2X 2
#define OS_4X 3
#define OS_8X 4
#define OS_16X 5

// IIR filter settings
#define FILTER_SIZE_0 0
#define FILTER_SIZE_1 1
#define FILTER_SIZE_3 2
#define FILTER_SIZE_7 3
#define FILTER_SIZE_15 4
#define FILTER_SIZE_31 5
#define FILTER_SIZE_63 6
#define FILTER_SIZE_127 7

// Power mode settings
#define SLEEP_MODE 0
#define FORCED_MODE 1

// Delay related
#define RESET_PERIOD 10

// Mask definitions
#define GAS_MEAS_MSK 0x30
#define NBCONV_MSK 0x0F
#define FILTER_MSK 0x1C
#define OST_MSK 0xE0
#define OSP_MSK 0x1C
#define OSH_MSK 0x07
#define HCTRL_MSK 0x08
#define RUN_GAS_MSK 0x30
#define MODE_MSK 0x03
#define RHRANGE_MSK 0x30
#define RSERROR_MSK 0xf0
#define NEW_DATA_MSK 0x80
#define GAS_INDEX_MSK 0x0f
#define GAS_RANGE_MSK 0x0f
#define GASM_VALID_MSK 0x20
#define HEAT_STAB_MSK 0x10
#define MEM_PAGE_MSK 0x10
#define BIT_H1_DATA_MSK 0x0F

// Bit position definitions
#define GAS_MEAS_POS 4
#define FILTER_POS 2
#define OST_POS 5
#define OSP_POS 2
#define OSH_POS 0
#define HCTRL_POS 3
#define RUN_GAS_POS 4
#define MODE_POS 0
#define NBCONV_POS 0

// Number of conversion settings
#define NBCONV_MIN 0
#define NBCONV_MAX 9

// Array Index to Field data mapping for Calibration Data
#define T2_LSB_REG 1
#define T2_MSB_REG 2
#define T3_REG 3
#define P1_LSB_REG 5
#define P1_MSB_REG 6
#define P2_LSB_REG 7
#define P2_MSB_REG 8
#define P3_REG 9
#define P4_LSB_REG 11
#define P4_MSB_REG 12
#define P5_LSB_REG 13
#define P5_MSB_REG 14
#define P7_REG 15
#define P6_REG 16
#define P8_LSB_REG 19
#define P8_MSB_REG 20
#define P9_LSB_REG 21
#define P9_MSB_REG 22
#define P10_REG 23
#define H2_MSB_REG 25
#define H2_LSB_REG 26
#define H1_LSB_REG 26
#define H1_MSB_REG 27
#define H3_REG 28
#define H4_REG 29
#define H5_REG 30
#define H6_REG 31
#define H7_REG 32
#define T1_LSB_REG 33
#define T1_MSB_REG 34
#define GH2_LSB_REG 35
#define GH2_MSB_REG 36
#define GH1_REG 37
#define GH3_REG 38

#define HUM_REG_SHIFT_VAL 4

// Look up tables for the possible gas range values
extern const uint32_t lookupTable1[16];
extern const uint32_t lookupTable2[16];

// ============================================================================
// DATA STRUCTURES
// ============================================================================

struct FieldData {
  uint8_t status = 0;
  bool heat_stable = false;
  uint8_t gas_index = 0;
  uint8_t meas_index = 0;
  float temperature = 0.0;      // in degrees celsius
  float pressure = 0.0;          // in hPa
  float humidity = 0.0;          // in % relative humidity
  float gas_resistance = 0.0;   // in Ohms
};

struct CalibrationData {
  // Humidity parameters
  uint16_t par_h1 = 0;
  uint16_t par_h2 = 0;
  int8_t par_h3 = 0;
  int8_t par_h4 = 0;
  int8_t par_h5 = 0;
  uint8_t par_h6 = 0;
  int8_t par_h7 = 0;
  
  // Gas heater parameters
  int8_t par_gh1 = 0;
  int16_t par_gh2 = 0;
  int8_t par_gh3 = 0;
  
  // Temperature parameters
  uint16_t par_t1 = 0;
  int16_t par_t2 = 0;
  int8_t par_t3 = 0;
  
  // Pressure parameters
  uint16_t par_p1 = 0;
  int16_t par_p2 = 0;
  int8_t par_p3 = 0;
  int16_t par_p4 = 0;
  int16_t par_p5 = 0;
  int8_t par_p6 = 0;
  int8_t par_p7 = 0;
  int16_t par_p8 = 0;
  int16_t par_p9 = 0;
  uint8_t par_p10 = 0;
  
  // Other
  int32_t t_fine = 0;
  uint8_t res_heat_range = 0;
  int8_t res_heat_val = 0;
  int8_t range_sw_err = 0;
  
  void set_from_array(uint8_t* calibration);
  void set_other(uint8_t heat_range, int8_t heat_value, int8_t sw_error);
};

struct TPHSettings {
  uint8_t os_hum = OS_2X;
  uint8_t os_temp = OS_8X;
  uint8_t os_pres = OS_4X;
  uint8_t filter = FILTER_SIZE_3;
};

struct GasSettings {
  uint8_t nb_conv = 0;
  uint8_t heatr_ctrl = 0;
  uint8_t run_gas = 0;
  uint16_t heatr_temp = 320;
  uint16_t heatr_dur = 150;
};

// ============================================================================
// BME680 CLASS
// ============================================================================

class BME680_Bosch {
public:
  BME680_Bosch(uint8_t i2c_addr = I2C_ADDR_PRIMARY);
  bool begin();
  
  // Sensor configuration
  void set_humidity_oversample(uint8_t value);
  void set_pressure_oversample(uint8_t value);
  void set_temperature_oversample(uint8_t value);
  void set_filter(uint8_t value);
  void set_gas_heater_temperature(uint16_t temperature, uint8_t nb_profile = 0);
  void set_gas_heater_duration(uint16_t duration, uint8_t nb_profile = 0);
  void select_gas_heater_profile(uint8_t value);
  void set_gas_status(int8_t value);
  void set_temp_offset(float value);
  
  // Sensor reading
  bool get_sensor_data();
  
  // Baseline calibration (for IAQ)
  void set_baselines(uint16_t burn_in_time = 300, bool verbose = false);
  float get_gas_baseline();
  float get_hum_baseline();
  int8_t get_baseline_status() { return baseline_status; }
  
  // Public data
  FieldData data;
  CalibrationData calibration_data;
  TPHSettings tph_settings;
  GasSettings gas_settings;
  
private:
  uint8_t i2c_addr;
  uint8_t chip_id;
  uint8_t variant;
  uint8_t power_mode;
  int32_t offset_temp_in_t_fine;
  int32_t ambient_temperature;
  
  // Baseline data
  int8_t baseline_status;
  float baseline_gas;
  float baseline_hum;
  
  // I2C communication
  void _set_regs(uint8_t register_addr, uint8_t value);
  void _set_regs(uint8_t register_addr, uint8_t* values, uint8_t length);
  uint8_t _get_regs(uint8_t register_addr, uint8_t length, uint8_t* data);
  uint8_t _get_regs(uint8_t register_addr);
  void _set_bits(uint8_t register_addr, uint8_t mask, uint8_t position, uint8_t value);
  
  // Calibration
  void _get_calibration_data();
  void soft_reset();
  
  // Calculations
  int32_t _calc_temperature(uint32_t temperature_adc);
  float _calc_pressure(uint32_t pressure_adc);
  float _calc_humidity(uint16_t humidity_adc);
  float _calc_gas_resistance(uint16_t gas_res_adc, uint8_t gas_range);
  float _calc_gas_resistance_high(uint16_t gas_res_adc, uint8_t gas_range);
  float _calc_gas_resistance_low(uint16_t gas_res_adc, uint8_t gas_range);
  uint8_t _calc_heater_resistance(uint16_t temperature);
  uint8_t _calc_heater_duration(uint16_t duration);
  
  // Helper functions
  uint16_t bytes_to_word(uint8_t msb, uint8_t lsb, bool signed_val = false);
  int16_t twos_comp(uint16_t val, uint8_t bits = 16);
  
  uint8_t get_power_mode();
  void set_power_mode(uint8_t value, bool blocking = true);
};

#endif // BME680_BOSCH_H

