/*
 * BME680_Bosch Implementation
 * Ported from official Bosch BME680 Python library
 */

#include "BME680_Bosch.h"

// Look up tables for the possible gas range values
const uint32_t lookupTable1[16] = {
  2147483647, 2147483647, 2147483647, 2147483647,
  2147483647, 2126008810, 2147483647, 2130303777, 2147483647,
  2147483647, 2143188679, 2136746228, 2147483647, 2126008810,
  2147483647, 2147483647
};

const uint32_t lookupTable2[16] = {
  4096000000, 2048000000, 1024000000, 512000000,
  255744255, 127110228, 64000000, 32258064,
  16016016, 8000000, 4000000, 2000000,
  1000000, 500000, 250000, 125000
};

// ============================================================================
// CALIBRATION DATA METHODS
// ============================================================================

void CalibrationData::set_from_array(uint8_t* calibration) {
  // Temperature related coefficients
  par_t1 = (calibration[T1_MSB_REG] << 8) | calibration[T1_LSB_REG];
  par_t2 = twos_comp((calibration[T2_MSB_REG] << 8) | calibration[T2_LSB_REG], 16);
  par_t3 = twos_comp(calibration[T3_REG], 8);

  // Pressure related coefficients
  par_p1 = (calibration[P1_MSB_REG] << 8) | calibration[P1_LSB_REG];
  par_p2 = twos_comp((calibration[P2_MSB_REG] << 8) | calibration[P2_LSB_REG], 16);
  par_p3 = twos_comp(calibration[P3_REG], 8);
  par_p4 = twos_comp((calibration[P4_MSB_REG] << 8) | calibration[P4_LSB_REG], 16);
  par_p5 = twos_comp((calibration[P5_MSB_REG] << 8) | calibration[P5_LSB_REG], 16);
  par_p6 = twos_comp(calibration[P6_REG], 8);
  par_p7 = twos_comp(calibration[P7_REG], 8);
  par_p8 = twos_comp((calibration[P8_MSB_REG] << 8) | calibration[P8_LSB_REG], 16);
  par_p9 = twos_comp((calibration[P9_MSB_REG] << 8) | calibration[P9_LSB_REG], 16);
  par_p10 = calibration[P10_REG];

  // Humidity related coefficients
  par_h1 = (calibration[H1_MSB_REG] << HUM_REG_SHIFT_VAL) | (calibration[H1_LSB_REG] & BIT_H1_DATA_MSK);
  par_h2 = (calibration[H2_MSB_REG] << HUM_REG_SHIFT_VAL) | (calibration[H2_LSB_REG] >> HUM_REG_SHIFT_VAL);
  par_h3 = twos_comp(calibration[H3_REG], 8);
  par_h4 = twos_comp(calibration[H4_REG], 8);
  par_h5 = twos_comp(calibration[H5_REG], 8);
  par_h6 = calibration[H6_REG];
  par_h7 = twos_comp(calibration[H7_REG], 8);

  // Gas heater related coefficients
  par_gh1 = twos_comp(calibration[GH1_REG], 8);
  par_gh2 = twos_comp((calibration[GH2_MSB_REG] << 8) | calibration[GH2_LSB_REG], 16);
  par_gh3 = twos_comp(calibration[GH3_REG], 8);
}

void CalibrationData::set_other(uint8_t heat_range, int8_t heat_value, int8_t sw_error) {
  res_heat_range = (heat_range & RHRANGE_MSK) >> 4;
  res_heat_val = heat_value;
  range_sw_err = (sw_error & RSERROR_MSK) >> 4;
}

// Helper function for two's complement (standalone)
static int16_t twos_comp(uint16_t val, uint8_t bits) {
  if (val & (1 << (bits - 1))) {
    val = val - (1 << bits);
  }
  return (int16_t)val;
}

// ============================================================================
// BME680_BOSCH CONSTRUCTOR
// ============================================================================

BME680_Bosch::BME680_Bosch(uint8_t i2c_addr) :
  i2c_addr(i2c_addr),
  chip_id(0),
  variant(0),
  power_mode(SLEEP_MODE),
  offset_temp_in_t_fine(0),
  ambient_temperature(0),
  baseline_status(-1),
  baseline_gas(0.0),
  baseline_hum(0.0)
{
}

bool BME680_Bosch::begin() {
  // Check chip ID
  chip_id = _get_regs(CHIP_ID_ADDR);
  if (chip_id != CHIP_ID) {
    return false;
  }
  
  variant = _get_regs(CHIP_VARIANT_ADDR);
  
  soft_reset();
  set_power_mode(SLEEP_MODE);
  
  _get_calibration_data();
  
  // Set default configuration (matching Python defaults)
  set_humidity_oversample(OS_2X);
  set_pressure_oversample(OS_4X);
  set_temperature_oversample(OS_8X);
  set_filter(FILTER_SIZE_3);
  
  if (variant == VARIANT_HIGH) {
    set_gas_status(ENABLE_GAS_MEAS_HIGH);
  } else {
    set_gas_status(ENABLE_GAS_MEAS_LOW);
  }
  
  set_temp_offset(0);
  get_sensor_data();
  
  return true;
}

// ============================================================================
// I2C COMMUNICATION
// ============================================================================

void BME680_Bosch::_set_regs(uint8_t register_addr, uint8_t value) {
  Wire.beginTransmission(i2c_addr);
  Wire.write(register_addr);
  Wire.write(value);
  Wire.endTransmission();
}

void BME680_Bosch::_set_regs(uint8_t register_addr, uint8_t* values, uint8_t length) {
  Wire.beginTransmission(i2c_addr);
  Wire.write(register_addr);
  Wire.write(values, length);
  Wire.endTransmission();
}

uint8_t BME680_Bosch::_get_regs(uint8_t register_addr) {
  uint8_t data;
  _get_regs(register_addr, 1, &data);
  return data;
}

uint8_t BME680_Bosch::_get_regs(uint8_t register_addr, uint8_t length, uint8_t* data) {
  Wire.beginTransmission(i2c_addr);
  Wire.write(register_addr);
  if (Wire.endTransmission() != 0) {
    return 0;
  }
  
  Wire.requestFrom(i2c_addr, length);
  uint8_t i = 0;
  while (Wire.available() && i < length) {
    data[i++] = Wire.read();
  }
  return i;
}

void BME680_Bosch::_set_bits(uint8_t register_addr, uint8_t mask, uint8_t position, uint8_t value) {
  uint8_t temp = _get_regs(register_addr);
  temp &= ~mask;
  temp |= value << position;
  _set_regs(register_addr, temp);
}

// ============================================================================
// CALIBRATION DATA RETRIEVAL
// ============================================================================

void BME680_Bosch::_get_calibration_data() {
  uint8_t calibration[COEFF_SIZE];
  
  // Read coefficient address 1
  _get_regs(COEFF_ADDR1, COEFF_ADDR1_LEN, calibration);
  
  // Read coefficient address 2
  _get_regs(COEFF_ADDR2, COEFF_ADDR2_LEN, &calibration[COEFF_ADDR1_LEN]);
  
  // Read heat range, heat value, and sw error
  uint8_t heat_range = _get_regs(ADDR_RES_HEAT_RANGE_ADDR);
  int8_t heat_value = twos_comp(_get_regs(ADDR_RES_HEAT_VAL_ADDR), 8);
  int8_t sw_error = twos_comp(_get_regs(ADDR_RANGE_SW_ERR_ADDR), 8);
  
  calibration_data.set_from_array(calibration);
  calibration_data.set_other(heat_range, heat_value, sw_error);
}

void BME680_Bosch::soft_reset() {
  _set_regs(SOFT_RESET_ADDR, SOFT_RESET_CMD);
  delay(RESET_PERIOD);
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

uint16_t BME680_Bosch::bytes_to_word(uint8_t msb, uint8_t lsb, bool signed_val) {
  uint16_t word = (msb << 8) | lsb;
  if (signed_val) {
    word = twos_comp(word, 16);
  }
  return word;
}

int16_t BME680_Bosch::_twos_comp(uint16_t val, uint8_t bits) {
  if (val & (1 << (bits - 1))) {
    val = val - (1 << bits);
  }
  return (int16_t)val;
}

int16_t BME680_Bosch::twos_comp(uint16_t val, uint8_t bits) {
  return _twos_comp(val, bits);
}

// ============================================================================
// SENSOR CONFIGURATION
// ============================================================================

void BME680_Bosch::set_humidity_oversample(uint8_t value) {
  tph_settings.os_hum = value;
  _set_bits(CONF_OS_H_ADDR, OSH_MSK, OSH_POS, value);
}

void BME680_Bosch::set_pressure_oversample(uint8_t value) {
  tph_settings.os_pres = value;
  _set_bits(CONF_T_P_MODE_ADDR, OSP_MSK, OSP_POS, value);
}

void BME680_Bosch::set_temperature_oversample(uint8_t value) {
  tph_settings.os_temp = value;
  _set_bits(CONF_T_P_MODE_ADDR, OST_MSK, OST_POS, value);
}

void BME680_Bosch::set_filter(uint8_t value) {
  tph_settings.filter = value;
  _set_bits(CONF_ODR_FILT_ADDR, FILTER_MSK, FILTER_POS, value);
}

void BME680_Bosch::select_gas_heater_profile(uint8_t value) {
  if (value > NBCONV_MAX || value < NBCONV_MIN) {
    return; // Error - should throw but Arduino doesn't support exceptions well
  }
  gas_settings.nb_conv = value;
  _set_bits(CONF_ODR_RUN_GAS_NBC_ADDR, NBCONV_MSK, NBCONV_POS, value);
}

void BME680_Bosch::set_gas_heater_temperature(uint16_t temperature, uint8_t nb_profile) {
  if (nb_profile > NBCONV_MAX) return;
  if (temperature < 200) temperature = 200;
  if (temperature > 400) temperature = 400;
  
  gas_settings.heatr_temp = temperature;
  uint8_t temp = _calc_heater_resistance(temperature);
  _set_regs(RES_HEAT0_ADDR + nb_profile, temp);
}

void BME680_Bosch::set_gas_heater_duration(uint16_t duration, uint8_t nb_profile) {
  if (nb_profile > NBCONV_MAX) return;
  
  gas_settings.heatr_dur = duration;
  uint8_t temp = _calc_heater_duration(duration);
  _set_regs(GAS_WAIT0_ADDR + nb_profile, temp);
}

void BME680_Bosch::set_gas_status(int8_t value) {
  if (value == -1) {
    if (variant == VARIANT_HIGH) {
      value = ENABLE_GAS_MEAS_HIGH;
    } else {
      value = ENABLE_GAS_MEAS_LOW;
    }
  }
  gas_settings.run_gas = value;
  _set_bits(CONF_ODR_RUN_GAS_NBC_ADDR, RUN_GAS_MSK, RUN_GAS_POS, value);
}

void BME680_Bosch::set_temp_offset(float value) {
  if (value == 0) {
    offset_temp_in_t_fine = 0;
  } else {
    offset_temp_in_t_fine = (int32_t)((value > 0 ? 1 : -1) * ((((int)(fabs(value) * 100)) << 8) - 128) / 5));
  }
}

uint8_t BME680_Bosch::get_power_mode() {
  power_mode = _get_regs(CONF_T_P_MODE_ADDR) & MODE_MSK;
  return power_mode;
}

void BME680_Bosch::set_power_mode(uint8_t value, bool blocking) {
  if (value != SLEEP_MODE && value != FORCED_MODE) {
    return;
  }
  
  power_mode = value;
  _set_bits(CONF_T_P_MODE_ADDR, MODE_MSK, MODE_POS, value);
  
  if (blocking) {
    while (get_power_mode() != power_mode) {
      delay(POLL_PERIOD_MS);
    }
  }
}

// ============================================================================
// CALCULATION FUNCTIONS
// ============================================================================

int32_t BME680_Bosch::_calc_temperature(uint32_t temperature_adc) {
  int32_t var1 = (temperature_adc >> 3) - ((int32_t)calibration_data.par_t1 << 1);
  int32_t var2 = (var1 * (int32_t)calibration_data.par_t2) >> 11;
  int32_t var3 = ((var1 >> 1) * (var1 >> 1)) >> 12;
  var3 = ((var3) * ((int32_t)calibration_data.par_t3 << 4)) >> 14;
  
  calibration_data.t_fine = (var2 + var3) + offset_temp_in_t_fine;
  int32_t calc_temp = (((calibration_data.t_fine * 5) + 128) >> 8);
  
  ambient_temperature = calc_temp; // Saved for heater calc
  return calc_temp;
}

float BME680_Bosch::_calc_pressure(uint32_t pressure_adc) {
  int64_t var1 = ((int64_t)calibration_data.t_fine >> 1) - 64000;
  int64_t var2 = ((((var1 >> 2) * (var1 >> 2)) >> 11) * (int64_t)calibration_data.par_p6) >> 2;
  var2 = var2 + ((var1 * (int64_t)calibration_data.par_p5) << 1);
  var2 = (var2 >> 2) + ((int64_t)calibration_data.par_p4 << 16);
  int64_t var1_2 = (((((var1 >> 2) * (var1 >> 2)) >> 13) * ((int64_t)calibration_data.par_p3 << 5)) >> 3) +
                    ((var1 * (int64_t)calibration_data.par_p2) >> 1);
  var1_2 = var1_2 >> 18;
  
  var1_2 = ((32768 + var1_2) * (int64_t)calibration_data.par_p1) >> 15;
  int64_t calc_pressure = 1048576 - pressure_adc;
  calc_pressure = ((calc_pressure - (var2 >> 12)) * 3125);
  
  if (calc_pressure >= (1LL << 31)) {
    calc_pressure = ((calc_pressure / var1_2) << 1);
  } else {
    calc_pressure = ((calc_pressure << 1) / var1_2);
  }
  
  int64_t var1_3 = ((int64_t)calibration_data.par_p9 * (((calc_pressure >> 3) * (calc_pressure >> 3)) >> 13)) >> 12;
  int64_t var2_2 = ((calc_pressure >> 2) * (int64_t)calibration_data.par_p8) >> 13;
  int64_t var3 = ((calc_pressure >> 8) * (calc_pressure >> 8) * (calc_pressure >> 8) *
                  (int64_t)calibration_data.par_p10) >> 17;
  
  calc_pressure = (calc_pressure) + ((var1_3 + var2_2 + var3 + ((int64_t)calibration_data.par_p7 << 7)) >> 4);
  
  return (float)calc_pressure / 100.0; // Convert Pa to hPa
}

float BME680_Bosch::_calc_humidity(uint16_t humidity_adc) {
  int32_t temp_scaled = ((calibration_data.t_fine * 5) + 128) >> 8;
  int32_t var1 = (humidity_adc - ((calibration_data.par_h1 * 16))) -
                 (((temp_scaled * calibration_data.par_h3) / 100) >> 1);
  int32_t var2 = (calibration_data.par_h2 *
                  (((temp_scaled * calibration_data.par_h4) / 100) +
                   (((temp_scaled * ((temp_scaled * calibration_data.par_h5) / 100)) >> 6) / 100) +
                   (1 * 16384))) >> 10;
  int32_t var3 = var1 * var2;
  int32_t var4 = calibration_data.par_h6 << 7;
  var4 = ((var4) + ((temp_scaled * calibration_data.par_h7) / 100)) >> 4;
  int32_t var5 = ((var3 >> 14) * (var3 >> 14)) >> 10;
  int32_t var6 = (var4 * var5) >> 1;
  int32_t calc_hum = (((var3 + var6) >> 10) * 1000) >> 12;
  
  if (calc_hum < 0) calc_hum = 0;
  if (calc_hum > 100000) calc_hum = 100000;
  
  return (float)calc_hum / 1000.0; // Convert from per-mille to percent
}

float BME680_Bosch::_calc_gas_resistance(uint16_t gas_res_adc, uint8_t gas_range) {
  if (variant == VARIANT_HIGH) {
    return _calc_gas_resistance_high(gas_res_adc, gas_range);
  } else {
    return _calc_gas_resistance_low(gas_res_adc, gas_range);
  }
}

float BME680_Bosch::_calc_gas_resistance_high(uint16_t gas_res_adc, uint8_t gas_range) {
  uint32_t var1 = 262144 >> gas_range;
  int32_t var2 = gas_res_adc - 512;
  
  var2 *= 3;
  var2 = 4096 + var2;
  
  float calc_gas_res = (10000.0 * var1) / var2;
  calc_gas_res *= 100;
  
  return calc_gas_res;
}

float BME680_Bosch::_calc_gas_resistance_low(uint16_t gas_res_adc, uint8_t gas_range) {
  uint32_t var1 = ((1340 + (5 * calibration_data.range_sw_err)) * lookupTable1[gas_range]) >> 16;
  int32_t var2 = (((gas_res_adc << 15) - 16777216) + var1);
  uint32_t var3 = ((lookupTable2[gas_range] * var1) >> 9);
  float calc_gas_res = ((var3 + (var2 >> 1)) / (float)var2);
  
  if (calc_gas_res < 0) {
    calc_gas_res = (1ULL << 32) + calc_gas_res;
  }
  
  return calc_gas_res;
}

uint8_t BME680_Bosch::_calc_heater_resistance(uint16_t temperature) {
  if (temperature < 200) temperature = 200;
  if (temperature > 400) temperature = 400;
  
  int32_t var1 = ((ambient_temperature * calibration_data.par_gh3) / 1000) * 256;
  int32_t var2 = (calibration_data.par_gh1 + 784) * 
                 (((((calibration_data.par_gh2 + 154009) * temperature * 5) / 100) + 3276800) / 10);
  int32_t var3 = var1 + (var2 / 2);
  int32_t var4 = (var3 / (calibration_data.res_heat_range + 4));
  int32_t var5 = (131 * calibration_data.res_heat_val) + 65536;
  int32_t heatr_res_x100 = (((var4 / var5) - 250) * 34);
  uint8_t heatr_res = ((heatr_res_x100 + 50) / 100);
  
  return heatr_res;
}

uint8_t BME680_Bosch::_calc_heater_duration(uint16_t duration) {
  if (duration < 0xfc0) {
    uint8_t factor = 0;
    uint16_t dur = duration;
    
    while (dur > 0x3f) {
      dur /= 4;
      factor++;
    }
    
    return dur + (factor * 64);
  }
  
  return 0xff;
}

// ============================================================================
// SENSOR DATA READING
// ============================================================================

bool BME680_Bosch::get_sensor_data() {
  set_power_mode(FORCED_MODE);
  
  for (int attempt = 0; attempt < 10; attempt++) {
    uint8_t status = _get_regs(FIELD0_ADDR);
    
    if ((status & NEW_DATA_MSK) == 0) {
      delay(POLL_PERIOD_MS);
      continue;
    }
    
    uint8_t regs[FIELD_LENGTH];
    if (_get_regs(FIELD0_ADDR, FIELD_LENGTH, regs) != FIELD_LENGTH) {
      continue;
    }
    
    data.status = regs[0] & NEW_DATA_MSK;
    data.gas_index = regs[0] & GAS_INDEX_MSK;
    data.meas_index = regs[1];
    
    uint32_t adc_pres = (regs[2] << 12) | (regs[3] << 4) | (regs[4] >> 4);
    uint32_t adc_temp = (regs[5] << 12) | (regs[6] << 4) | (regs[7] >> 4);
    uint16_t adc_hum = (regs[8] << 8) | regs[9];
    uint16_t adc_gas_res_low = (regs[13] << 2) | (regs[14] >> 6);
    uint16_t adc_gas_res_high = (regs[15] << 2) | (regs[16] >> 6);
    uint8_t gas_range_l = regs[14] & GAS_RANGE_MSK;
    uint8_t gas_range_h = regs[16] & GAS_RANGE_MSK;
    
    if (variant == VARIANT_HIGH) {
      data.status |= regs[16] & GASM_VALID_MSK;
      data.status |= regs[16] & HEAT_STAB_MSK;
    } else {
      data.status |= regs[14] & GASM_VALID_MSK;
      data.status |= regs[14] & HEAT_STAB_MSK;
    }
    
    data.heat_stable = (data.status & HEAT_STAB_MSK) > 0;
    
    int32_t temperature = _calc_temperature(adc_temp);
    data.temperature = temperature / 100.0;
    
    data.pressure = _calc_pressure(adc_pres);
    data.humidity = _calc_humidity(adc_hum);
    
    if (variant == VARIANT_HIGH) {
      data.gas_resistance = _calc_gas_resistance_high(adc_gas_res_high, gas_range_h);
    } else {
      data.gas_resistance = _calc_gas_resistance_low(adc_gas_res_low, gas_range_l);
    }
    
    return true;
  }
  
  return false;
}

// ============================================================================
// BASELINE CALIBRATION (for IAQ)
// ============================================================================

void BME680_Bosch::set_baselines(uint16_t burn_in_time, bool verbose) {
  unsigned long start_time = millis();
  float gas_burn_data[300]; // Max 300 seconds
  float hum_burn_data[300];
  uint16_t count = 0;
  
  while ((millis() - start_time) < (burn_in_time * 1000UL)) {
    if (get_sensor_data() && data.heat_stable) {
      if (count < 300) {
        gas_burn_data[count] = data.gas_resistance;
        hum_burn_data[count] = data.humidity;
        count++;
        
        if (verbose && count > 1) {
          float change_from_prior = ((gas_burn_data[count-1] - gas_burn_data[count-2]) / gas_burn_data[count-2]) * 100.0;
          float change_from_orig = ((gas_burn_data[count-1] - gas_burn_data[1]) / gas_burn_data[1]) * 100.0;
          Serial.printf("%.2f - %.3f (P:%.2f)(O:%.2f)\n", 
                       (millis() - start_time) / 1000.0, 
                       gas_burn_data[count-1], 
                       change_from_prior, 
                       change_from_orig);
        }
      }
    }
    delay(1000);
  }
  
  if (count < 50) {
    // Average all readings
    float gas_sum = 0, hum_sum = 0;
    for (uint16_t i = 0; i < count; i++) {
      gas_sum += gas_burn_data[i];
      hum_sum += hum_burn_data[i];
    }
    baseline_gas = gas_sum / count;
    baseline_hum = hum_sum / count;
  } else {
    // Average last 50 readings
    float gas_sum = 0, hum_sum = 0;
    for (uint16_t i = count - 50; i < count; i++) {
      gas_sum += gas_burn_data[i];
      hum_sum += hum_burn_data[i];
    }
    baseline_gas = gas_sum / 50;
    baseline_hum = hum_sum / 50;
  }
  
  baseline_status = 1;
}

float BME680_Bosch::get_gas_baseline() {
  if (baseline_status == 1) {
    return baseline_gas;
  }
  return 0.0;
}

float BME680_Bosch::get_hum_baseline() {
  if (baseline_status == 1) {
    return baseline_hum;
  }
  return 0.0;
}

