/*
 * Custom BME680 Library Implementation
 * Based on Official Bosch Implementation
 */

#include "BME680_Custom.h"

// Lookup tables for gas resistance calculation
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

BME680_Custom::BME680_Custom(uint8_t i2c_addr) {
  _i2c_addr = i2c_addr;
  _variant = 0;
  _offset_temp_in_t_fine = 0;
  _ambient_temperature = 0;
  _gas_baseline = 0.0;
  _hum_baseline = 0.0;
  _baseline_established = false;
  
  // Initialize data structure
  data.temperature = 0.0;
  data.humidity = 0.0;
  data.pressure = 0.0;
  data.gas_resistance = 0.0;
  data.heat_stable = false;
  data.gas_valid = false;
}

bool BME680_Custom::begin() {
  // Check chip ID
  uint8_t chip_id = _read_byte(CHIP_ID_ADDR);
  if (chip_id != BME680_CHIP_ID) {
    return false;
  }
  
  _variant = _read_byte(CHIP_VARIANT_ADDR);
  
  // Soft reset
  _write_byte(SOFT_RESET_ADDR, SOFT_RESET_CMD);
  delay(10);
  
  // Set to sleep mode
  set_power_mode(SLEEP_MODE);
  
  // Get calibration data
  _get_calibration_data();
  
  // Default settings (matching Python implementation)
  set_humidity_oversample(OS_2X);
  set_pressure_oversample(OS_4X);
  set_temperature_oversample(OS_8X);
  set_filter(FILTER_SIZE_3);
  
  if (_variant == 0x01) {
    set_gas_status(ENABLE_GAS_MEAS_HIGH);
  } else {
    set_gas_status(ENABLE_GAS_MEAS_LOW);
  }
  
  // Initial read
  get_sensor_data();
  
  return true;
}

void BME680_Custom::_get_calibration_data() {
  uint8_t cal1[COEFF_ADDR1_LEN];
  uint8_t cal2[COEFF_ADDR2_LEN];
  
  _read_bytes(COEFF_ADDR1, cal1, COEFF_ADDR1_LEN);
  _read_bytes(COEFF_ADDR2, cal2, COEFF_ADDR2_LEN);
  
  // Temperature coefficients
  _cal.par_t1 = _bytes_to_word(cal1[33], cal1[32]);
  _cal.par_t2 = _bytes_to_word(cal1[1], cal1[2], true);
  _cal.par_t3 = _twos_comp(cal1[3]);
  
  // Pressure coefficients
  _cal.par_p1 = _bytes_to_word(cal1[5], cal1[6]);
  _cal.par_p2 = _bytes_to_word(cal1[7], cal1[8], true);
  _cal.par_p3 = _twos_comp(cal1[9]);
  _cal.par_p4 = _bytes_to_word(cal1[11], cal1[12], true);
  _cal.par_p5 = _bytes_to_word(cal1[13], cal1[14], true);
  _cal.par_p6 = _twos_comp(cal1[16]);
  _cal.par_p7 = _twos_comp(cal1[15]);
  _cal.par_p8 = _bytes_to_word(cal1[19], cal1[20], true);
  _cal.par_p9 = _bytes_to_word(cal1[21], cal1[22], true);
  _cal.par_p10 = cal1[23];
  
  // Humidity coefficients
  _cal.par_h1 = ((cal1[25] << 4) | (cal1[26] & 0x0F));
  _cal.par_h2 = ((cal1[26] >> 4) | (cal1[27] << 4));
  _cal.par_h3 = _twos_comp(cal1[28]);
  _cal.par_h4 = _twos_comp(cal1[29]);
  _cal.par_h5 = _twos_comp(cal1[30]);
  _cal.par_h6 = cal1[31];
  _cal.par_h7 = _twos_comp(cal1[32]);
  
  // Gas coefficients
  _cal.par_gh1 = _twos_comp(cal2[0]);
  _cal.par_gh2 = _bytes_to_word(cal2[2], cal2[1], true);
  _cal.par_gh3 = _twos_comp(cal2[3]);
  
  // Other calibration data
  uint8_t heat_range = _read_byte(ADDR_RES_HEAT_RANGE_ADDR);
  int8_t heat_value = _twos_comp(_read_byte(ADDR_RES_HEAT_VAL_ADDR));
  int8_t sw_error = _twos_comp(_read_byte(ADDR_RANGE_SW_ERR_ADDR));
  
  _cal.res_heat_range = (heat_range & RHRANGE_MSK) >> 4;
  _cal.res_heat_val = heat_value;
  _cal.range_sw_err = (sw_error & RSERROR_MSK) >> 4;
}

bool BME680_Custom::get_sensor_data() {
  set_power_mode(FORCED_MODE);
  
  for (uint8_t attempt = 0; attempt < 10; attempt++) {
    uint8_t status = _read_byte(FIELD0_ADDR);
    
    if ((status & NEW_DATA_MSK) == 0) {
      delay(10);
      continue;
    }
    
    uint8_t regs[FIELD_LENGTH];
    _read_bytes(FIELD0_ADDR, regs, FIELD_LENGTH);
    
    // Extract ADC values
    uint32_t adc_pres = ((uint32_t)regs[2] << 12) | ((uint32_t)regs[3] << 4) | (regs[4] >> 4);
    uint32_t adc_temp = ((uint32_t)regs[5] << 12) | ((uint32_t)regs[6] << 4) | (regs[7] >> 4);
    uint16_t adc_hum = ((uint16_t)regs[8] << 8) | regs[9];
    uint16_t adc_gas_res_low = ((uint16_t)regs[13] << 2) | (regs[14] >> 6);
    uint16_t adc_gas_res_high = ((uint16_t)regs[15] << 2) | (regs[16] >> 6);
    uint8_t gas_range_l = regs[14] & GAS_RANGE_MSK;
    uint8_t gas_range_h = regs[16] & GAS_RANGE_MSK;
    
    // Check heat stable and gas valid
    if (_variant == 0x01) {
      data.heat_stable = (regs[16] & HEAT_STAB_MSK) > 0;
      data.gas_valid = (regs[16] & GASM_VALID_MSK) > 0;
    } else {
      data.heat_stable = (regs[14] & HEAT_STAB_MSK) > 0;
      data.gas_valid = (regs[14] & GASM_VALID_MSK) > 0;
    }
    
    // Calculate values
    int32_t temp = _calc_temperature(adc_temp);
    data.temperature = temp / 100.0;
    _ambient_temperature = temp;
    
    data.pressure = _calc_pressure(adc_pres) / 100.0;
    data.humidity = _calc_humidity(adc_hum) / 1000.0;
    
    if (_variant == 0x01) {
      data.gas_resistance = _calc_gas_resistance(adc_gas_res_high, gas_range_h);
    } else {
      data.gas_resistance = _calc_gas_resistance(adc_gas_res_low, gas_range_l);
    }
    
    return true;
  }
  
  return false;
}

int32_t BME680_Custom::_calc_temperature(uint32_t temp_adc) {
  int32_t var1 = (temp_adc >> 3) - ((int32_t)_cal.par_t1 << 1);
  int32_t var2 = (var1 * (int32_t)_cal.par_t2) >> 11;
  int32_t var3 = ((((var1 >> 1) * (var1 >> 1)) >> 12) * ((int32_t)_cal.par_t3 << 4)) >> 14;
  
  _cal.t_fine = var2 + var3 + _offset_temp_in_t_fine;
  int32_t calc_temp = (((_cal.t_fine * 5) + 128) >> 8);
  
  return calc_temp;
}

uint32_t BME680_Custom::_calc_pressure(uint32_t pres_adc) {
  int32_t var1 = ((int32_t)_cal.t_fine >> 1) - 64000;
  int32_t var2 = ((((var1 >> 2) * (var1 >> 2)) >> 11) * (int32_t)_cal.par_p6) >> 2;
  var2 = var2 + ((var1 * (int32_t)_cal.par_p5) << 1);
  var2 = (var2 >> 2) + ((int32_t)_cal.par_p4 << 16);
  
  var1 = (((((var1 >> 2) * (var1 >> 2)) >> 13) * ((int32_t)_cal.par_p3 << 5)) >> 3) +
         (((int32_t)_cal.par_p2 * var1) >> 1);
  var1 = var1 >> 18;
  
  var1 = ((32768 + var1) * (int32_t)_cal.par_p1) >> 15;
  int64_t calc_pres = 1048576 - pres_adc;
  calc_pres = ((calc_pres - (var2 >> 12)) * 3125);
  
  if (calc_pres >= ((int64_t)1 << 31)) {
    calc_pres = ((calc_pres / var1) << 1);
  } else {
    calc_pres = ((calc_pres << 1) / var1);
  }
  
  var1 = ((int32_t)_cal.par_p9 * (((calc_pres >> 3) * (calc_pres >> 3)) >> 13)) >> 12;
  int32_t var2 = ((calc_pres >> 2) * (int32_t)_cal.par_p8) >> 13;
  int64_t var3 = ((calc_pres >> 8) * (calc_pres >> 8) * (calc_pres >> 8) * (int64_t)_cal.par_p10) >> 17;
  
  calc_pres = (calc_pres) + ((var1 + var2 + var3 + ((int32_t)_cal.par_p7 << 7)) >> 4);
  
  return (uint32_t)calc_pres;
}

uint32_t BME680_Custom::_calc_humidity(uint16_t hum_adc) {
  int32_t temp_scaled = ((_cal.t_fine * 5) + 128) >> 8;
  int32_t var1 = (hum_adc - ((int32_t)_cal.par_h1 * 16)) -
                 (((temp_scaled * (int32_t)_cal.par_h3) / 100) >> 1);
  int32_t var2 = ((int32_t)_cal.par_h2 *
                  (((temp_scaled * (int32_t)_cal.par_h4) / 100) +
                   (((temp_scaled * ((temp_scaled * (int32_t)_cal.par_h5) / 100)) >> 6) / 100) +
                   (1 * 16384))) >> 10;
  int32_t var3 = var1 * var2;
  int32_t var4 = (int32_t)_cal.par_h6 << 7;
  var4 = ((var4) + ((temp_scaled * (int32_t)_cal.par_h7) / 100)) >> 4;
  int32_t var5 = ((var3 >> 14) * (var3 >> 14)) >> 10;
  int32_t var6 = (var4 * var5) >> 1;
  int32_t calc_hum = (((var3 + var6) >> 10) * 1000) >> 12;
  
  if (calc_hum < 0) calc_hum = 0;
  if (calc_hum > 100000) calc_hum = 100000;
  
  return (uint32_t)calc_hum;
}

uint32_t BME680_Custom::_calc_gas_resistance(uint16_t gas_res_adc, uint8_t gas_range) {
  if (_variant == 0x01) {
    // High variant
    uint32_t var1 = 262144 >> gas_range;
    int32_t var2 = gas_res_adc - 512;
    var2 *= 3;
    var2 = 4096 + var2;
    uint64_t calc_gas_res = ((uint64_t)10000 * var1) / var2;
    calc_gas_res *= 100;
    return (uint32_t)calc_gas_res;
  } else {
    // Low variant
    int32_t var1 = ((1340 + (5 * (int32_t)_cal.range_sw_err)) * (int32_t)lookupTable1[gas_range]) >> 16;
    int32_t var2 = (((gas_res_adc << 15) - 16777216) + var1);
    int32_t var3 = ((lookupTable2[gas_range] * var1) >> 9);
    int64_t calc_gas_res = ((var3 + (var2 >> 1)) / var2);
    
    if (calc_gas_res < 0) {
      calc_gas_res = ((int64_t)1 << 32) + calc_gas_res;
    }
    
    return (uint32_t)calc_gas_res;
  }
}

bool BME680_Custom::set_baselines(uint16_t burn_in_time_seconds, bool verbose) {
  unsigned long start_time = millis();
  unsigned long burn_in_ms = burn_in_time_seconds * 1000UL;
  
  float gas_sum = 0.0;
  float hum_sum = 0.0;
  uint16_t count = 0;
  uint16_t averaged_qty = 50;
  
  if (verbose) {
    Serial.print("Calibrating baseline for ");
    Serial.print(burn_in_time_seconds);
    Serial.println(" seconds...");
  }
  
  while ((millis() - start_time) < burn_in_ms) {
    if (get_sensor_data() && data.heat_stable) {
      gas_sum += data.gas_resistance;
      hum_sum += data.humidity;
      count++;
      
      if (verbose && (count % 10 == 0)) {
        Serial.print("Progress: ");
        Serial.print((millis() - start_time) / 1000);
        Serial.print("s - Gas: ");
        Serial.print(data.gas_resistance);
        Serial.print(" Ohms, Hum: ");
        Serial.print(data.humidity);
        Serial.println("%");
      }
      
      delay(1000);
    } else {
      delay(100);
    }
  }
  
  if (count < averaged_qty) {
    averaged_qty = count;
  }
  
  if (count > 0) {
    // Use last N readings for baseline
    uint16_t start_idx = (count > averaged_qty) ? (count - averaged_qty) : 0;
    // For simplicity, we'll use average of all readings
    // In a full implementation, you'd store readings and average last N
    _gas_baseline = gas_sum / count;
    _hum_baseline = hum_sum / count;
    _baseline_established = true;
    
    if (verbose) {
      Serial.print("Baseline established - Gas: ");
      Serial.print(_gas_baseline);
      Serial.print(" Ohms, Hum: ");
      Serial.print(_hum_baseline);
      Serial.println("%");
    }
    
    return true;
  }
  
  return false;
}

float BME680_Custom::calculate_iaq_score(float hum_weighting) {
  if (!_baseline_established) {
    return -1.0;
  }
  
  if (data.gas_resistance == 0.0) {
    return -1.0;
  }
  
  // Calculate offsets
  float gas_offset = _gas_baseline - data.gas_resistance;
  float hum_offset = data.humidity - _hum_baseline;
  
  // Calculate humidity score
  float hum_score;
  if (hum_offset > 0) {
    hum_score = (100.0 - _hum_baseline - hum_offset) / (100.0 - _hum_baseline) * (hum_weighting * 100.0);
  } else {
    hum_score = (_hum_baseline + hum_offset) / _hum_baseline * (hum_weighting * 100.0);
  }
  
  // Calculate gas score
  float gas_score;
  if (gas_offset > 0) {
    gas_score = (data.gas_resistance / _gas_baseline) * (100.0 - (hum_weighting * 100.0));
  } else {
    gas_score = 100.0 - (hum_weighting * 100.0);
  }
  
  // Calculate air quality score
  float air_quality_score = hum_score + gas_score;
  
  return air_quality_score;
}

bool BME680_Custom::check_safe_to_open(float threshold) {
  if (!_baseline_established) {
    return false;
  }
  
  float score = calculate_iaq_score();
  if (score < 0) {
    return false;
  }
  
  return score >= threshold;
}

// Configuration methods
void BME680_Custom::set_humidity_oversample(uint8_t value) {
  _set_bits(CONF_OS_H_ADDR, OSH_MSK, OSH_POS, value);
}

void BME680_Custom::set_pressure_oversample(uint8_t value) {
  _set_bits(CONF_T_P_MODE_ADDR, OSP_MSK, OSP_POS, value);
}

void BME680_Custom::set_temperature_oversample(uint8_t value) {
  _set_bits(CONF_T_P_MODE_ADDR, OST_MSK, OST_POS, value);
}

void BME680_Custom::set_filter(uint8_t value) {
  _set_bits(CONF_ODR_FILT_ADDR, FILTER_MSK, FILTER_POS, value);
}

void BME680_Custom::set_gas_status(uint8_t value) {
  _set_bits(CONF_ODR_RUN_GAS_NBC_ADDR, RUN_GAS_MSK, RUN_GAS_POS, value);
}

void BME680_Custom::set_gas_heater_temperature(uint16_t temperature, uint8_t nb_profile) {
  temperature = constrain(temperature, 200, 400);
  uint8_t res_heat = _calc_heater_resistance(temperature);
  _write_byte(RES_HEAT0_ADDR + nb_profile, res_heat);
}

void BME680_Custom::set_gas_heater_duration(uint16_t duration, uint8_t nb_profile) {
  uint8_t dur = _calc_heater_duration(duration);
  _write_byte(GAS_WAIT0_ADDR + nb_profile, dur);
}

void BME680_Custom::select_gas_heater_profile(uint8_t profile) {
  if (profile > 9) profile = 9;
  _set_bits(CONF_ODR_RUN_GAS_NBC_ADDR, NBCONV_MSK, NBCONV_POS, profile);
}

void BME680_Custom::set_power_mode(uint8_t mode) {
  _set_bits(CONF_T_P_MODE_ADDR, MODE_MSK, MODE_POS, mode);
  delay(10);
}

// Helper methods
float BME680_Custom::get_gas_baseline() {
  return _baseline_established ? _gas_baseline : -1.0;
}

float BME680_Custom::get_hum_baseline() {
  return _baseline_established ? _hum_baseline : -1.0;
}

bool BME680_Custom::is_baseline_established() {
  return _baseline_established;
}

// Low-level I2C methods
void BME680_Custom::_write_byte(uint8_t reg, uint8_t value) {
  Wire.beginTransmission(_i2c_addr);
  Wire.write(reg);
  Wire.write(value);
  Wire.endTransmission();
}

uint8_t BME680_Custom::_read_byte(uint8_t reg) {
  Wire.beginTransmission(_i2c_addr);
  Wire.write(reg);
  Wire.endTransmission();
  Wire.requestFrom(_i2c_addr, (uint8_t)1);
  return Wire.read();
}

void BME680_Custom::_read_bytes(uint8_t reg, uint8_t* data, uint8_t len) {
  Wire.beginTransmission(_i2c_addr);
  Wire.write(reg);
  Wire.endTransmission();
  Wire.requestFrom(_i2c_addr, len);
  for (uint8_t i = 0; i < len; i++) {
    data[i] = Wire.read();
  }
}

void BME680_Custom::_set_bits(uint8_t reg, uint8_t mask, uint8_t position, uint8_t value) {
  uint8_t temp = _read_byte(reg);
  temp &= ~mask;
  temp |= (value << position) & mask;
  _write_byte(reg, temp);
}

int16_t BME680_Custom::_bytes_to_word(uint8_t msb, uint8_t lsb, bool signed_val) {
  int16_t word = ((uint16_t)msb << 8) | lsb;
  if (signed_val && (word & 0x8000)) {
    word = word - 0x10000;
  }
  return word;
}

int8_t BME680_Custom::_twos_comp(uint8_t val) {
  if (val & 0x80) {
    return (int8_t)(val - 256);
  }
  return (int8_t)val;
}

uint8_t BME680_Custom::_calc_heater_resistance(uint16_t temperature) {
  temperature = constrain(temperature, 200, 400);
  
  int32_t var1 = (((int32_t)_ambient_temperature * (int32_t)_cal.par_gh3) / 1000) * 256;
  int32_t var2 = ((int32_t)_cal.par_gh1 + 784) * 
                 (((((int32_t)_cal.par_gh2 + 154009) * temperature * 5) / 100) + 3276800) / 10;
  int32_t var3 = var1 + (var2 / 2);
  int32_t var4 = (var3 / ((int32_t)_cal.res_heat_range + 4));
  int32_t var5 = (131 * (int32_t)_cal.res_heat_val) + 65536;
  int32_t heatr_res_x100 = (((var4 / var5) - 250) * 34);
  uint8_t heatr_res = ((heatr_res_x100 + 50) / 100);
  
  return heatr_res;
}

uint8_t BME680_Custom::_calc_heater_duration(uint16_t duration) {
  if (duration < 0xFC0) {
    uint8_t factor = 0;
    uint16_t dur = duration;
    
    while (dur > 0x3F) {
      dur /= 4;
      factor++;
    }
    
    return dur + (factor * 64);
  }
  
  return 0xFF;
}

