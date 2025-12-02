# Minimal re-export wrapper. The full sensor implementation is vendored
# Vendored monitor implementation (trimmed)
from .constants import *  # noqa: F401,F403
import math
import time

class BME680(BME680Data):
    def __init__(self, i2c_addr=I2C_ADDR_PRIMARY, i2c_device=None):
        BME680Data.__init__(self)
        self.i2c_addr = i2c_addr
        self._i2c = i2c_device
        if self._i2c is None:
            import smbus2
            self._i2c = smbus2.SMBus(1)
        self.chip_id = self._get_regs(CHIP_ID_ADDR, 1)
        if self.chip_id != CHIP_ID:
            raise RuntimeError("BME680 Not Found. Invalid CHIP ID: 0x{0:02x}".format(self.chip_id))
        self._variant = self._get_regs(CHIP_VARIANT_ADDR, 1)
        self.soft_reset()
        self.set_power_mode(SLEEP_MODE)
        self._get_calibration_data()
        self.set_humidity_oversample(OS_2X)
        self.set_pressure_oversample(OS_4X)
        self.set_temperature_oversample(OS_8X)
        self.set_filter(FILTER_SIZE_3)
        self.set_gas_status(ENABLE_GAS_MEAS)
        self.set_temp_offset(0)
        self.get_sensor_data()
        self.baseline_status = -1

    def _get_calibration_data(self):
        calibration = self._get_regs(COEFF_ADDR1, COEFF_ADDR1_LEN)
        calibration += self._get_regs(COEFF_ADDR2, COEFF_ADDR2_LEN)
        heat_range = self._get_regs(ADDR_RES_HEAT_RANGE_ADDR, 1)
        heat_value = twos_comp(self._get_regs(ADDR_RES_HEAT_VAL_ADDR, 1), bits=8)
        sw_error = twos_comp(self._get_regs(ADDR_RANGE_SW_ERR_ADDR, 1), bits=8)
        self.calibration_data.set_from_array(calibration)
        self.calibration_data.set_other(heat_range, heat_value, sw_error)

    def soft_reset(self):
        self._set_regs(SOFT_RESET_ADDR, SOFT_RESET_CMD)
        time.sleep(RESET_PERIOD / 1000.0)

    def set_temp_offset(self, value):
        if value == 0:
            self.offset_temp_in_t_fine = 0
        else:
            self.offset_temp_in_t_fine = int(math.copysign((((int(abs(value) * 100)) << 8) - 128) / 5, value))

    def set_humidity_oversample(self, value):
        self.tph_settings.os_hum = value
        self._set_bits(CONF_OS_H_ADDR, OSH_MSK, OSH_POS, value)

    def set_pressure_oversample(self, value):
        self.tph_settings.os_pres = value
        self._set_bits(CONF_T_P_MODE_ADDR, OSP_MSK, OSP_POS, value)

    def set_temperature_oversample(self, value):
        self.tph_settings.os_temp = value
        self._set_bits(CONF_T_P_MODE_ADDR, OST_MSK, OST_POS, value)

    def set_filter(self, value):
        self.tph_settings.filter = value
        self._set_bits(CONF_ODR_FILT_ADDR, FILTER_MSK, FILTER_POS, value)

    def set_gas_status(self, value):
        if value == -1:
            value = ENABLE_GAS_MEAS_HIGH if self._variant == VARIANT_HIGH else ENABLE_GAS_MEAS_LOW
        self.gas_settings.run_gas = value
        self._set_bits(CONF_ODR_RUN_GAS_NBC_ADDR, RUN_GAS_MSK, RUN_GAS_POS, value)

    def set_gas_heater_profile(self, temperature, duration, nb_profile=0):
        self.set_gas_heater_temperature(temperature, nb_profile=nb_profile)
        self.set_gas_heater_duration(duration, nb_profile=nb_profile)

    def set_gas_heater_temperature(self, value, nb_profile=0):
        if nb_profile > NBCONV_MAX or value < NBCONV_MIN:
            raise ValueError("Profile '{}' should be between {} and {}".format(nb_profile, NBCONV_MIN, NBCONV_MAX))
        self.gas_settings.heatr_temp = value
        temp = int(self._calc_heater_resistance(self.gas_settings.heatr_temp))
        self._set_regs(RES_HEAT0_ADDR + nb_profile, temp)

    def set_gas_heater_duration(self, value, nb_profile=0):
        if nb_profile > NBCONV_MAX or value < NBCONV_MIN:
            raise ValueError("Profile '{}' should be between {} and {}".format(nb_profile, NBCONV_MIN, NBCONV_MAX))
        self.gas_settings.heatr_dur = value
        temp = self._calc_heater_duration(self.gas_settings.heatr_dur)
        self._set_regs(GAS_WAIT0_ADDR + nb_profile, temp)

    def set_power_mode(self, value, blocking=True):
        if value not in (SLEEP_MODE, FORCED_MODE):
            raise ValueError("Power mode should be one of SLEEP_MODE or FORCED_MODE")
        self.power_mode = value
        self._set_bits(CONF_T_P_MODE_ADDR, MODE_MSK, MODE_POS, value)
        while blocking and self.get_power_mode() != self.power_mode:
            time.sleep(POLL_PERIOD_MS / 1000.0)

    def get_power_mode(self):
        self.power_mode = self._get_regs(CONF_T_P_MODE_ADDR, 1)
        return self.power_mode

    def get_sensor_data(self):
        self.set_power_mode(FORCED_MODE)
        for _ in range(10):
            status = self._get_regs(FIELD0_ADDR, 1)
            if (status & NEW_DATA_MSK) == 0:
                time.sleep(POLL_PERIOD_MS / 1000.0)
                continue
            regs = self._get_regs(FIELD0_ADDR, FIELD_LENGTH)
            self.data.status = regs[0] & NEW_DATA_MSK
            self.data.gas_index = regs[0] & GAS_INDEX_MSK
            self.data.meas_index = regs[1]
            adc_pres = (regs[2] << 12) | (regs[3] << 4) | (regs[4] >> 4)
            adc_temp = (regs[5] << 12) | (regs[6] << 4) | (regs[7] >> 4)
            adc_hum = (regs[8] << 8) | regs[9]
            adc_gas_res_low = (regs[13] << 2) | (regs[14] >> 6)
            gas_range_l = regs[14] & GAS_RANGE_MSK
            self.data.heat_stable = (regs[14] & HEAT_STAB_MSK) > 0
            temperature = self._calc_temperature(adc_temp)
            self.data.temperature = temperature / 100.0
            self.ambient_temperature = temperature
            self.data.pressure = self._calc_pressure(adc_pres) / 100.0
            self.data.humidity = self._calc_humidity(adc_hum) / 1000.0
            self.data.gas_resistance = self._calc_gas_resistance_low(adc_gas_res_low, gas_range_l)
            return True
        return False

    def _set_bits(self, register, mask, position, value):
        temp = self._get_regs(register, 1)
        temp &= ~mask
        temp |= value << position
        self._set_regs(register, temp)

    def _set_regs(self, register, value):
        if isinstance(value, int):
            self._i2c.write_byte_data(self.i2c_addr, register, value)
        else:
            self._i2c.write_i2c_block_data(self.i2c_addr, register, value)

    def _get_regs(self, register, length):
        if length == 1:
            return self._i2c.read_byte_data(self.i2c_addr, register)
        else:
            return self._i2c.read_i2c_block_data(self.i2c_addr, register, length)

    def _calc_temperature(self, temperature_adc):
        var1 = (temperature_adc >> 3) - (self.calibration_data.par_t1 << 1)
        var2 = (var1 * self.calibration_data.par_t2) >> 11
        var3 = ((var1 >> 1) * (var1 >> 1)) >> 12
        var3 = ((var3) * (self.calibration_data.par_t3 << 4)) >> 14
        self.calibration_data.t_fine = (var2 + var3)
        calc_temp = (((self.calibration_data.t_fine * 5) + 128) >> 8)
        return calc_temp

    def _calc_pressure(self, pressure_adc):
        var1 = ((self.calibration_data.t_fine) >> 1) - 64000
        var2 = ((((var1 >> 2) * (var1 >> 2)) >> 11) * self.calibration_data.par_p6) >> 2
        var2 = var2 + ((var1 * self.calibration_data.par_p5) << 1)
        var2 = (var2 >> 2) + (self.calibration_data.par_p4 << 16)
        var1 = (((((var1 >> 2) * (var1 >> 2)) >> 13) * ((self.calibration_data.par_p3 << 5)) >> 3) + ((self.calibration_data.par_p2 * var1) >> 1))
        var1 = var1 >> 18
        var1 = ((32768 + var1) * self.calibration_data.par_p1) >> 15
        calc_pressure = 1048576 - pressure_adc
        calc_pressure = ((calc_pressure - (var2 >> 12)) * (3125))
        if calc_pressure >= (1 << 31):
            calc_pressure = ((calc_pressure // var1) << 1)
        else:
            calc_pressure = ((calc_pressure << 1) // var1)
        return calc_pressure

    def _calc_humidity(self, humidity_adc):
        temp_scaled = ((self.calibration_data.t_fine * 5) + 128) >> 8
        var1 = (humidity_adc - ((self.calibration_data.par_h1 * 16)))
        var2 = (self.calibration_data.par_h2 * temp_scaled) >> 10
        var3 = (var1 * var2)
        var4 = (self.calibration_data.par_h6 << 7)
        var4 = ((var4)) >> 4
        var5 = ((var3 >> 14) * (var3 >> 14)) >> 10
        var6 = (var4 * var5) >> 1
        calc_hum = (((var3 + var6) >> 10) * (1000)) >> 12
        return min(max(calc_hum, 0), 100000)

    def _calc_gas_resistance_low(self, gas_res_adc, gas_range):
        var1 = ((1340 + (5 * self.calibration_data.range_sw_err)) * (lookupTable1[gas_range])) >> 16
        var2 = (((gas_res_adc << 15) - (16777216)) + var1)
        var3 = ((lookupTable2[gas_range] * var1) >> 9)
        calc_gas_res = ((var3 + (var2 >> 1)) / var2)
        if calc_gas_res < 0:
            calc_gas_res = (1 << 32) + calc_gas_res
        return calc_gas_res


