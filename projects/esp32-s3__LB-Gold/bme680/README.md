# ESP32-S3 BME680 Sensor - Bosch Implementation

Full port of the RPi `bme680-service` to ESP32-S3, matching functionality exactly.

## Status

**✅ COMPLETE**: Full port of Bosch BME680 library and all RPi functionality.

## Features (Matching RPi Version)

- ✅ **Base Readings**: Temperature, Humidity, Pressure, Gas Resistance
- ✅ **Heatsoak Detection**: Temperature smoothing, rate calculations, ready detection
- ✅ **IAQ Monitoring**: Baseline calibration, air quality scoring, safe-to-open detection
- ✅ **MQTT Integration**: Same topics and JSON format as RPi version
- ✅ **Bosch BME680 Library**: Fully ported from Python to Arduino C++

## MQTT Topics (Matching RPi)

- `sensors/bme680/raw` - Base readings with heatsoak calculations
- `homeassistant/sensor/bme680/state` - IAQ readings
- `homeassistant/sensor/bme680_chamber/state` - Heatsoak only
- `sensors/bme680/status` - Status updates

## Library

The `BME680_Bosch` library has been fully ported from the Python implementation and is located at:
- `_libs/esp32-s3/BME680_Bosch/`

This is a complete port of the official Bosch BME680 Python library, including:
- All calibration calculations
- All sensor reading functions
- Baseline calibration for IAQ
- Exact same algorithms as the Python version

## Implementation Details

### Heatsoak Detection
- Temperature smoothing: 4-second window (configurable)
- Rate smoothing: 30-second window (configurable)
- Least squares linear regression for rate calculation
- Matches `monitor-heatsoak.py` exactly

### IAQ Calculations
- Baseline calibration: 90-second burn-in (configurable)
- Gas and humidity baseline averaging
- IAQ score calculation: 25% humidity, 75% gas weighting
- Safe-to-open threshold: 80.0 (configurable)
- Matches `monitor-iaq.py` exactly

### Base Readings
- All sensor data with heatsoak calculations
- Matches `base-readings.py` exactly

## Usage

The implementation functions identically to the RPi version:

1. **Base Readings**: Automatically publishes to `sensors/bme680/raw` every 30 seconds with all sensor data and heatsoak calculations
2. **IAQ Readings**: Publishes to `homeassistant/sensor/bme680/state` when baseline is calibrated
3. **Heatsoak Readings**: Publishes to `homeassistant/sensor/bme680_chamber/state` with temperature and rate data

To calibrate baseline for IAQ (one-time setup):
- Call `iaq.calibrate_baseline(bme680_ptr, BASELINE_BURN_IN)` in setup() or via MQTT command

## Configuration

All configuration matches RPi defaults:
- Read interval: 1 second
- Publish interval: 30 seconds
- Temp smooth: 4 seconds
- Rate smooth: 30 seconds
- Soak temp: 40°C
- Max rate: 0.1°C/min
- IAQ threshold: 80.0

## Building

This project uses a Python-based build system (`make` script) that provides interactive sketch selection and flexible build options.

### Quick Start

```bash
# Interactive sketch selection (recommended)
./make build

# Or specify sketch directly
SKETCH=bme680 ./make build
SKETCH=bme680-simplified ./make build
```

### Available Commands

- `./make build` - Compile the sketch (interactive sketch selection)
- `./make compile` - Compile with verbose output
- `./make progress` - Compile with progress bar
- `./make upload` - Upload to ESP32-S3 (no compile)
- `./make monitor` - Open serial monitor
- `./make clean` - Clean build artifacts
- `./make all` - Compile and upload in one step
- `./make help` - Show help message
- `./make sketch-status` - Show current sketch status

### Sketch Selection

The build system supports two sketches:

1. **`bme680.ino`** - Full version with heatsoak and IAQ calculations on ESP32
2. **`bme680-simplified.ino`** - Simplified version that publishes raw sensor data only (calculations done on RPi)

By default, running `./make build` will show an interactive menu to select which sketch to compile. You can also specify the sketch directly using the `SKETCH` environment variable.

### Legacy Makefile

A traditional `Makefile` is also available if you prefer standard make syntax:

```bash
make build
make SKETCH=bme680-simplified build
```

