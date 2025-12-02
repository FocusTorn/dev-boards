"""BME680 constants, structures and utilities (vendored)."""

# BME680 General config
POLL_PERIOD_MS = 10

# BME680 I2C addresses
I2C_ADDR_PRIMARY = 0x76
I2C_ADDR_SECONDARY = 0x77

# BME680 unique chip identifier
CHIP_ID = 0x61

# BME680 coefficients related defines
COEFF_SIZE = 41
COEFF_ADDR1_LEN = 25
COEFF_ADDR2_LEN = 16

# BME680 field_x related defines
FIELD_LENGTH = 17
FIELD_ADDR_OFFSET = 17

# Soft reset command
SOFT_RESET_CMD = 0xb6

# Error code definitions
OK = 0
# Errors
E_NULL_PTR = -1
E_COM_FAIL = -2
E_DEV_NOT_FOUND = -3
E_INVALID_LENGTH = -4

# Warnings
W_DEFINE_PWR_MODE = 1
W_NO_NEW_DATA = 2

# Info's
I_MIN_CORRECTION = 1
I_MAX_CORRECTION = 2

# Register map
# Other coefficient's address
ADDR_RES_HEAT_VAL_ADDR = 0x00
ADDR_RES_HEAT_RANGE_ADDR = 0x02
ADDR_RANGE_SW_ERR_ADDR = 0x04
ADDR_SENS_CONF_START = 0x5A
ADDR_GAS_CONF_START = 0x64

# Field settings
FIELD0_ADDR = 0x1d

# Heater settings
RES_HEAT0_ADDR = 0x5a
GAS_WAIT0_ADDR = 0x64

# Sensor configuration registers
CONF_HEAT_CTRL_ADDR = 0x70
CONF_ODR_RUN_GAS_NBC_ADDR = 0x71
CONF_OS_H_ADDR = 0x72
MEM_PAGE_ADDR = 0xf3
CONF_T_P_MODE_ADDR = 0x74
CONF_ODR_FILT_ADDR = 0x75

# Coefficient's address
COEFF_ADDR1 = 0x89
COEFF_ADDR2 = 0xe1

# Chip identifier
CHIP_ID_ADDR = 0xd0
CHIP_VARIANT_ADDR = 0xf0

VARIANT_LOW = 0x00
VARIANT_HIGH = 0x01

# Soft reset register
SOFT_RESET_ADDR = 0xe0

# Heater control settings
ENABLE_HEATER = 0x00
DISABLE_HEATER = 0x08

# Gas measurement settings
DISABLE_GAS_MEAS = 0x00
ENABLE_GAS_MEAS = -1  # Now used as auto-select
ENABLE_GAS_MEAS_LOW = 0x01
ENABLE_GAS_MEAS_HIGH = 0x02

# Over-sampling settings
OS_NONE = 0
OS_1X = 1
OS_2X = 2
OS_4X = 3
OS_8X = 4
OS_16X = 5

# IIR filter settings
FILTER_SIZE_0 = 0
FILTER_SIZE_1 = 1
FILTER_SIZE_3 = 2
FILTER_SIZE_7 = 3
FILTER_SIZE_15 = 4
FILTER_SIZE_31 = 5
FILTER_SIZE_63 = 6
FILTER_SIZE_127 = 7

# Power mode settings
SLEEP_MODE = 0
FORCED_MODE = 1

# Delay related macro declaration
RESET_PERIOD = 10

# SPI memory page settings
MEM_PAGE0 = 0x10
MEM_PAGE1 = 0x00

# Ambient humidity shift value for compensation
HUM_REG_SHIFT_VAL = 4

# Run gas enable and disable settings
RUN_GAS_DISABLE = 0
RUN_GAS_ENABLE = 1

# Gas heater enable and disable settings
GAS_HEAT_ENABLE = 0
GAS_HEAT_DISABLE = 1

# Buffer length macro declaration
TMP_BUFFER_LENGTH = 40
REG_BUFFER_LENGTH = 6
FIELD_DATA_LENGTH = 3
GAS_REG_BUF_LENGTH = 20
GAS_HEATER_PROF_LEN_MAX = 10

# Settings selector
OST_SEL = 1
OSP_SEL = 2
OSH_SEL = 4
GAS_MEAS_SEL = 8
FILTER_SEL = 16
HCNTRL_SEL = 32
RUN_GAS_SEL = 64
NBCONV_SEL = 128
GAS_SENSOR_SEL = GAS_MEAS_SEL | RUN_GAS_SEL | NBCONV_SEL

# Number of conversion settings
NBCONV_MIN = 0
NBCONV_MAX = 9

# Mask definitions
GAS_MEAS_MSK = 0x30
NBCONV_MSK = 0X0F
FILTER_MSK = 0X1C
OST_MSK = 0XE0
OSP_MSK = 0X1C
OSH_MSK = 0X07
HCTRL_MSK = 0x08
RUN_GAS_MSK = 0x30
MODE_MSK = 0x03
RHRANGE_MSK = 0x30
RSERROR_MSK = 0xf0
NEW_DATA_MSK = 0x80
GAS_INDEX_MSK = 0x0f
GAS_RANGE_MSK = 0x0f
GASM_VALID_MSK = 0x20
HEAT_STAB_MSK = 0x10
MEM_PAGE_MSK = 0x10
SPI_RD_MSK = 0x80
SPI_WR_MSK = 0x7f
BIT_H1_DATA_MSK = 0x0F

# Bit positions
GAS_MEAS_POS = 4
FILTER_POS = 2
OST_POS = 5
OSP_POS = 2
OSH_POS = 0
HCTRL_POS = 3
RUN_GAS_POS = 4
MODE_POS = 0
NBCONV_POS = 0

def bytes_to_word(msb, lsb, bits=16, signed=False):
    word = (msb << 8) | lsb
    if signed:
        word = twos_comp(word, bits)
    return word

def twos_comp(val, bits=16):
    if val & (1 << (bits - 1)) != 0:
        val = val - (1 << bits)
    return val

class FieldData:
    def __init__(self):
        self.status = None
        self.heat_stable = False
        self.gas_index = None
        self.meas_index = None
        self.temperature = None
        self.pressure = None
        self.humidity = None
        self.gas_resistance = None

class CalibrationData:
    def __init__(self):
        self.par_h1 = None; self.par_h2 = None; self.par_h3 = None
        self.par_h4 = None; self.par_h5 = None; self.par_h6 = None; self.par_h7 = None
        self.par_gh1 = None; self.par_gh2 = None; self.par_gh3 = None
        self.par_t1 = None; self.par_t2 = None; self.par_t3 = None
        self.par_p1 = None; self.par_p2 = None; self.par_p3 = None; self.par_p4 = None
        self.par_p5 = None; self.par_p6 = None; self.par_p7 = None; self.par_p8 = None
        self.par_p9 = None; self.par_p10 = None
        self.t_fine = None
        self.res_heat_range = None
        self.res_heat_val = None
        self.range_sw_err = None

    def set_from_array(self, calibration):
        self.par_t1 = bytes_to_word(calibration[34], calibration[33])
        self.par_t2 = bytes_to_word(calibration[36], calibration[35], bits=16, signed=True)
        self.par_t3 = twos_comp(calibration[3], bits=8)
        self.par_p1 = bytes_to_word(calibration[12], calibration[11])
        self.par_p2 = bytes_to_word(calibration[22], calibration[21], bits=16, signed=True)
        self.par_p3 = twos_comp(calibration[9], bits=8)
        self.par_p4 = bytes_to_word(calibration[12], calibration[11], bits=16, signed=True)
        self.par_p5 = bytes_to_word(calibration[14], calibration[13], bits=16, signed=True)
        self.par_p6 = twos_comp(calibration[16], bits=8)
        self.par_p7 = twos_comp(calibration[15], bits=8)
        self.par_p8 = bytes_to_word(calibration[20], calibration[19], bits=16, signed=True)
        self.par_p9 = bytes_to_word(calibration[22], calibration[21], bits=16, signed=True)
        self.par_p10 = calibration[23]
        self.par_h1 = (calibration[27] << 4) | (calibration[26] & 0x0F)
        self.par_h2 = (calibration[25] << 4) | (calibration[26] >> 4)
        self.par_h3 = twos_comp(calibration[28], bits=8)
        self.par_h4 = twos_comp(calibration[29], bits=8)
        self.par_h5 = twos_comp(calibration[30], bits=8)
        self.par_h6 = calibration[31]
        self.par_h7 = twos_comp(calibration[32], bits=8)
        self.par_gh1 = twos_comp(calibration[37], bits=8)
        self.par_gh2 = bytes_to_word(calibration[36], calibration[35], bits=16, signed=True)
        self.par_gh3 = twos_comp(calibration[38], bits=8)

    def set_other(self, heat_range, heat_value, sw_error):
        self.res_heat_range = (heat_range & 0x30) // 16
        self.res_heat_val = heat_value
        self.range_sw_err = (sw_error & 0xF0) // 16

class TPHSettings:
    def __init__(self):
        self.os_hum = None
        self.os_temp = None
        self.os_pres = None
        self.filter = None

class GasSettings:
    def __init__(self):
        self.nb_conv = None
        self.heatr_ctrl = None
        self.run_gas = None
        self.heatr_temp = None
        self.heatr_dur = None

class BME680Data:
    def __init__(self):
        self.chip_id = None
        self.dev_id = None
        self.intf = None
        self.mem_page = None
        self.ambient_temperature = None
        self.data = FieldData()
        self.calibration_data = CalibrationData()
        self.tph_settings = TPHSettings()
        self.gas_settings = GasSettings()
        self.power_mode = None
        self.new_fields = None

