# BME680 Service Package

Installable systemd service package for BME680 sensor monitoring with automatic sensor detection and service management.

## Features

- ✅ **Automatic sensor detection** - Checks I2C bus for BME680 before installation
- ✅ **Systemd service management** - Professional service installation/uninstallation
- ✅ **Auto-start on boot** - Services start automatically after system reboot
- ✅ **Auto-restart on failure** - Services automatically restart if they crash
- ✅ **No code modifications** - Works with existing snake-pit monitor scripts
- ✅ **Multiple monitors** - Supports both IAQ and temperature monitoring

## Package Structure

```
bme680-service/
├── data/                         # Shared package files
│   ├── monitor/                  # BME680 library code (shared)
│   ├── bme680-cli                # CLI tool
│   └── requirements.txt          # Python dependencies
├── mqtt/                         # MQTT integration components
│   ├── data/                     # MQTT scripts
│   │   ├── base-readings.py      # Base readings publisher (MQTT)
│   │   ├── monitor-iaq.py        # IAQ monitor (MQTT)
│   │   └── monitor-heatsoak.py  # Heat soak monitor (MQTT)
│   ├── services/                 # MQTT systemd services
│   │   ├── bme680-base-mqtt.service
│   │   ├── bme680-iaq-mqtt.service
│   │   └── bme680-heatsoak-mqtt.service
│   └── ha/                       # Home Assistant MQTT configs
│       ├── sensors-bme680-mqtt.yaml
│       └── sensors-bme680-heatsoak-mqtt.yaml
├── hacs/                         # HACS custom component (future)
│   ├── data/                     # HACS scripts (placeholder)
│   ├── services/                 # HACS services (placeholder)
│   └── ha/                       # HACS component (placeholder)
├── detectors/
│   └── detect-bme680.sh         # Sensor detection script
├── install.sh                    # Installation script (root)
├── uninstall.sh                  # Uninstallation script (root)
└── README.md                     # This file
```

## Installation Locations

When installed, the package creates a **self-contained installation**:

```
~/.local/share/bme680-service/    # Package installation
├── monitor/                      # BME680 library (shared)
├── mqtt/                         # MQTT scripts
│   ├── base-readings.py          # Base readings publisher (MQTT)
│   ├── monitor-iaq.py            # IAQ monitor (MQTT)
│   └── monitor-heatsoak.py      # Heat soak monitor (MQTT)
├── .venv/                        # Virtual environment with dependencies
└── config/                       # Configuration files (if needed)

/etc/systemd/system/               # Systemd service files
├── bme680-base-mqtt.service      # Base readings service (MQTT)
├── bme680-iaq-mqtt.service       # IAQ monitor service (MQTT)
└── bme680-heatsoak-mqtt.service  # Heat soak service (MQTT)
```

**Key Features:**
- ✅ **Self-contained** - No dependencies on development paths
- ✅ **Standard locations** - Uses `~/.local/share/` and `~/.local/bin/`
- ✅ **Clean uninstall** - Removes all installed files
- ✅ **Isolated dependencies** - Own virtual environment

## Prerequisites

1. **Python 3** - Python 3.x must be installed (usually pre-installed on Raspberry Pi)

2. **I2C enabled** - Enable I2C on Raspberry Pi:
   ```bash
   sudo raspi-config  # Interface Options → I2C → Enable
   ```

3. **BME680 sensor** - Hardware must be connected and working

4. **MQTT broker** - Recommended (mosquitto or other MQTT service)

**Note:** This package is **self-contained** and does NOT require snake-pit or any other development environment. All dependencies are installed automatically.

## Installation

### Option 1: Bootstrap Script (Recommended)

```bash
/home/pi/_playground/_scripts/bootstraps/bootstrap-bme680.sh
```

### Option 2: Direct Installation

```bash
cd /home/pi/_playground/_dev/packages/bme680-service
./install.sh
```

**Note:** The installer will automatically prompt for sudo privileges when needed. No need to prefix with `sudo`.

The installer will:
1. **Detect BME680 sensor** - Checks both I2C addresses (0x76, 0x77) and verifies chip ID
2. **Install package files** - Copies scripts and library to `~/.local/share/bme680-service/`
3. **Create virtual environment** - Sets up isolated Python environment using `uv` (fast, reliable)
4. **Install dependencies** - Installs Python packages (smbus2, paho-mqtt, colored) using `uv pip install`
5. **Install systemd services** - Copies service files to `/etc/systemd/system/`
6. **Enable services** - Enables services to start on boot
7. **Start services** - Starts the monitoring services immediately

**Note:** This package uses `uv` for Python package management (installed globally via `bootstrap-snake-pit.sh`). If `uv` is not available, it falls back to traditional `pip`.

## CLI Tool

The package includes a unified CLI tool (`bme680-cli`) for easy interaction:

```bash
# View all current sensor readings
bme680-cli read

# View specific sensor values
bme680-cli read --temperature
bme680-cli read --humidity
bme680-cli read --pressure
bme680-cli read --gas
bme680-cli read --iaq

# View as JSON
bme680-cli read --json
bme680-cli read --humidity --json

# Check service status
bme680-cli status

# View service logs
bme680-cli logs bme680-readings

# Follow logs live
bme680-cli logs bme680-readings --follow

# Calibrate baseline (do this once)
bme680-cli calibrate

# Monitor continuously (console output)
bme680-cli monitor --iaq --interval 10
```

**Note:** The CLI tool is automatically installed to `~/.local/bin/bme680-cli` during package installation.

## Usage

### Check Service Status

```bash
# Base Readings service (MQTT)
systemctl status bme680-base-mqtt

# IAQ Monitor service (MQTT)
systemctl status bme680-iaq-mqtt

# Heat Soak Detection service (MQTT)
systemctl status bme680-heatsoak-mqtt
```

### View Logs

```bash
# Follow base readings logs (MQTT)
journalctl -u bme680-base-mqtt -f

# Follow IAQ monitor logs (MQTT)
journalctl -u bme680-iaq-mqtt -f

# Follow heat soak logs (MQTT)
journalctl -u bme680-heatsoak-mqtt -f

# View recent logs (last 50 lines)
journalctl -u bme680-base-mqtt -n 50
```

### Manual Service Control

```bash
# Base readings service (MQTT)
sudo systemctl start bme680-base-mqtt
sudo systemctl stop bme680-base-mqtt
sudo systemctl restart bme680-base-mqtt
sudo systemctl enable bme680-base-mqtt
sudo systemctl disable bme680-base-mqtt

# IAQ monitor service (MQTT)
sudo systemctl start bme680-iaq-mqtt
sudo systemctl stop bme680-iaq-mqtt
sudo systemctl restart bme680-iaq-mqtt
sudo systemctl enable bme680-iaq-mqtt
sudo systemctl disable bme680-iaq-mqtt

# Heat soak service (MQTT)
sudo systemctl start bme680-heatsoak-mqtt
sudo systemctl stop bme680-heatsoak-mqtt
sudo systemctl restart bme680-heatsoak-mqtt
sudo systemctl enable bme680-heatsoak-mqtt
sudo systemctl disable bme680-heatsoak-mqtt
```

## Uninstallation

```bash
cd /home/pi/_playground/_dev/packages/bme680-service
./uninstall.sh
```

**Note:** The uninstaller will automatically prompt for sudo privileges when needed.

The uninstaller will:
1. Stop and disable systemd services
2. Remove service files from `/etc/systemd/system/`
3. Optionally remove package files from `~/.local/share/bme680-service/`
4. Optionally remove wrapper scripts from `~/.local/bin/`
5. Reload systemd daemon

**Manual removal** (if needed):
```bash
# Stop and disable services (MQTT)
sudo systemctl stop bme680-base-mqtt bme680-iaq-mqtt bme680-heatsoak-mqtt
sudo systemctl disable bme680-base-mqtt bme680-iaq-mqtt bme680-heatsoak-mqtt
sudo rm /etc/systemd/system/bme680-*-mqtt.service
sudo systemctl daemon-reload

# Remove package files
rm -rf ~/.local/share/bme680-service
```

## Service Configuration

### Base Readings Service (MQTT) ⭐ Recommended for Start

- **Service name**: `bme680-base-mqtt.service`
- **Script**: `mqtt/base-readings.py`
- **MQTT topic**: `sensors/bme680/raw`
- **Readings**: ✅ Temperature, Humidity, Pressure, Gas Resistance
- **Purpose**: Simple base sensor readings published to MQTT
- **Features**: 
  - Raw sensor data (no calculations)
  - Simple JSON format
  - Heat stable detection
  - 30-second interval
- **Auto-restart**: Yes (5 second delay)
- **User**: `pi`

**Note**: This is the simplest service - just publishes basic sensor readings. Perfect for getting started with MQTT integration.

**MQTT Message Format**:
```json
{
    "temperature": 24.75,
    "humidity": 52.07,
    "pressure": 983.91,
    "gas_resistance": 8506.0,
    "heat_stable": true,
    "timestamp": 1762459600.717239
}
```

### IAQ Monitor Service (MQTT)

- **Service name**: `bme680-iaq-mqtt.service`
- **Script**: `mqtt/monitor-iaq.py`
- **MQTT topic**: `homeassistant/sensor/bme680/state`
- **Readings**: ✅ Temperature, Humidity, Pressure, Gas Resistance, IAQ Score
- **Purpose**: All sensor readings and enclosure air quality monitoring
- **Features**: Baseline calibration, IAQ scoring, safety thresholds
- **Auto-restart**: Yes (10 second delay)
- **User**: `pi`

**Note**: This service provides advanced IAQ calculations. Requires baseline calibration.

### Heat Soak Detection Service (MQTT)

- **Service name**: `bme680-heatsoak-mqtt.service`
- **Script**: `mqtt/monitor-heatsoak.py`
- **MQTT topic**: `homeassistant/sensor/bme680_chamber/state`
- **Readings**: Temperature only (with heat soak detection)
- **Purpose**: 3D printing heat soak detection (pre-heating check)
- **Features**: Temperature smoothing, rate of change calculation, heat soak ready detection
- **Auto-restart**: Yes (10 second delay)
- **User**: `pi`

**Note**: This is separate because it has specialized heat soak logic for 3D printing. Only install if you need heat soak detection.

## Sensor Detection

The installer automatically detects BME680 sensors with **smart verification**:

1. **Checks BOTH addresses** - Always checks both 0x76 and 0x77
2. **Verifies chip ID** - Confirms chip ID is 0x61 (BME680 specific), not just any sensor
3. **Safe detection** - Won't confuse other I2C devices with BME680

**I2C Addresses:**
- **0x76** (primary address, SDO → GND)
- **0x77** (secondary address, SDO → 3.3V)

**Chip ID Verification:**
- Reads register `0xD0` (chip ID register)
- Must return `0x61` to confirm it's a BME680
- Prevents false positives from other sensors/devices

If no sensor is detected, the installer will show detailed diagnostics about what devices (if any) are present at each address, and you can still proceed with installation for testing purposes.

## Troubleshooting

### Service Won't Start

1. **Check sensor is detected:**
   ```bash
   i2cdetect -y 1
   # Should show 0x76 or 0x77
   ```

2. **Check logs:**
   ```bash
   # Base readings service (MQTT)
   journalctl -u bme680-base-mqtt -n 50 --no-pager
   
   # IAQ monitor service (MQTT)
   journalctl -u bme680-iaq-mqtt -n 50 --no-pager
   
   # Heat soak service (MQTT)
   journalctl -u bme680-heatsoak-mqtt -n 50 --no-pager
   ```

3. **Check installation:**
   ```bash
   # Verify package is installed
   test -d ~/.local/share/bme680-service && echo "OK" || echo "Not installed"
   
   # Verify virtual environment exists
   test -f ~/.local/share/bme680-service/.venv/bin/python && echo "OK" || echo "Missing"
   
   # Test Python environment
   ~/.local/share/bme680-service/.venv/bin/python --version
   ```

### Sensor Not Detected

1. **Enable I2C:**
   ```bash
   sudo raspi-config  # Interface Options → I2C
   ```

2. **Check wiring** - See `/home/pi/_playground/snake-pit/projects/bme680-monitor/WIRING.md`

3. **Verify sensor power** - Check 3.3V connection

### MQTT Not Publishing

1. **Check MQTT broker is running:**
   ```bash
   systemctl status mosquitto
   # or
   systemctl status mqtt
   ```

2. **Test MQTT connection:**
   ```bash
   # Monitor base readings topic
   mosquitto_sub -h localhost -t "sensors/bme680/raw" -v
   
   # Monitor IAQ readings topic
   mosquitto_sub -h localhost -t "homeassistant/sensor/bme680/state" -v
   
   # Monitor heat soak topic
   mosquitto_sub -h localhost -t "homeassistant/sensor/bme680_chamber/state" -v
   ```

3. **Check service logs for MQTT errors:**
   ```bash
   journalctl -u bme680-base-mqtt -n 50 --no-pager
   ```

## Integration with Home Assistant

Once services are running and publishing to MQTT, configure Home Assistant sensors:

```yaml
# Already configured in:
# /home/pi/_playground/home-assistant/sensors-bme680-sensor.yaml
# /home/pi/_playground/home-assistant/sensors-bme680-binary.yaml
```

Restart Home Assistant to load the new sensors:
```bash
ha restart
```

## Notes

- Services run as user `pi` (not root) for security
- Services automatically restart on failure (10 second delay)
- Logs are available via `journalctl`
- Services start after network and MQTT are available
- Works with both venv and uv run environments

