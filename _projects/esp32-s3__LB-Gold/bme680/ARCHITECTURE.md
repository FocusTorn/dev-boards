# BME680 Sensor Architecture - Recommended Approach

## Architecture Overview

```
┌─────────────────┐         ┌──────────────────┐         ┌─────────────────┐
│   ESP32-S3      │         │   Raspberry Pi   │         │  Home Assistant │
│                 │         │                  │         │                 │
│  ┌───────────┐  │         │  ┌────────────┐  │         │  ┌───────────┐  │
│  │  BME680   │  │         │  │  MQTT      │  │         │  │  Custom   │  │
│  │  Sensor   │  │         │  │  Broker    │  │         │  │  Card     │  │
│  └─────┬─────┘  │         │  └─────┬──────┘  │         │  └─────┬─────┘  │
│        │        │         │        │         │         │        │        │
│  ┌─────▼─────┐  │         │  ┌─────▼──────┐  │         │  ┌─────▼─────┐  │
│  │  Bosch    │  │         │  │  Python    │  │         │  │  Display  │  │
│  │  Reading  │  │         │  │  Service   │  │         │  │  Results  │  │
│  │  (Raw)    │  │         │  │            │  │         │  └───────────┘  │
│  └─────┬─────┘  │         │  │  - Heatsoak│  │         │                 │
│        │        │         │  │  - IAQ     │  │         │                 │
│  ┌─────▼─────┐  │         │  │  - Baselines│ │         │                 │
│  │  MQTT     │──┼─────────┼─▶│            │  │         │                 │
│  │  Publish  │  │         │  └─────┬──────┘  │         │                 │
│  │  Raw Data │  │         │        │         │         │                 │
│  └──────────┘  │         │  ┌─────▼──────┐  │         │                 │
│                 │         │  │  MQTT      │──┼─────────┼─▶│                 │
│                 │         │  │  Publish   │  │         │                 │
│                 │         │  │  Calculated│  │         │                 │
│                 │         │  └────────────┘  │         │                 │
└─────────────────┘         └──────────────────┘         └─────────────────┘
```

## Data Flow

### 1. ESP32-S3 (Edge Device)
**Responsibility**: Raw sensor data collection only

**Publishes to MQTT**:
- Topic: `sensors/bme680/raw`
- Payload:
```json
{
  "temperature": 25.3,
  "humidity": 45.2,
  "pressure": 1013.25,
  "gas_resistance": 50000,
  "heat_stable": true,
  "timestamp": 1234567890
}
```

**What ESP32 does**:
- ✅ Read BME680 using Bosch library (accurate calibration)
- ✅ Apply sensor calibration (temperature, humidity, pressure)
- ✅ Check heat stability
- ✅ Publish raw readings to MQTT
- ❌ NO heatsoak calculations
- ❌ NO IAQ calculations
- ❌ NO baseline management

### 2. Raspberry Pi (Processing Hub)
**Responsibility**: All calculations and derived metrics

**Subscribes to**: `sensors/bme680/raw`
**Publishes to**:
- `sensors/bme680/heatsoak` - Heatsoak calculations
- `sensors/bme680/iaq` - IAQ calculations
- `homeassistant/sensor/bme680/state` - Combined state for HA

**What RPi does**:
- ✅ Subscribe to raw sensor data
- ✅ Calculate heatsoak metrics (smoothed temp, rate of change)
- ✅ Manage baseline calibration for IAQ
- ✅ Calculate IAQ scores
- ✅ Publish calculated values to MQTT
- ✅ Handle historical data (if needed)

### 3. Home Assistant
**Responsibility**: Display and automation

**Subscribes to**:
- `sensors/bme680/raw` - Raw readings
- `sensors/bme680/heatsoak` - Heatsoak status
- `sensors/bme680/iaq` - IAQ scores

**Custom Card**: Displays all metrics from MQTT topics

## Benefits of This Architecture

### ✅ Maintainability
- Calculation logic in one place (Python on RPi)
- Easy to update algorithms without reflashing ESP32
- Version control for calculation code

### ✅ Flexibility
- Change calculation parameters via config files
- A/B test different algorithms
- Add new calculations without touching ESP32

### ✅ Separation of Concerns
- ESP32: Hardware interface and data collection
- RPi: Data processing and analysis
- HA: Visualization and automation

### ✅ Scalability
- Add more ESP32 sensors easily (just change MQTT topic)
- Centralized calculation logic handles all sensors
- Easy to add new calculation types

### ✅ Testing
- Test calculations independently from hardware
- Mock sensor data for algorithm development
- Unit test calculation logic

### ✅ Power Efficiency
- ESP32 can sleep between readings
- Minimal processing on battery-powered device
- Calculations on always-powered RPi

## Implementation Steps

### Step 1: Simplify ESP32 Code
Remove all calculation classes:
- Remove `HeatsoakMonitor` class
- Remove `IAQMonitor` class
- Keep only raw sensor reading and MQTT publishing

### Step 2: Create RPi Processing Service
Use existing reference implementation:
- `reference/bme680-service/mqtt/data/base-readings.py` - Heatsoak calculations
- `reference/bme680-service/mqtt/data/monitor-iaq.py` - IAQ calculations

### Step 3: Update MQTT Topics
ESP32 publishes to: `sensors/bme680/raw`
RPi subscribes to: `sensors/bme680/raw`
RPi publishes to: `sensors/bme680/heatsoak`, `sensors/bme680/iaq`

### Step 4: Update Home Assistant
Custom card subscribes to all topics:
- Raw data: `sensors/bme680/raw`
- Heatsoak: `sensors/bme680/heatsoak`
- IAQ: `sensors/bme680/iaq`

## MQTT Topic Structure

```
sensors/bme680/raw              # ESP32 → RPi (raw sensor data)
sensors/bme680/heatsoak         # RPi → HA (heatsoak calculations)
sensors/bme680/iaq              # RPi → HA (IAQ calculations)
sensors/bme680/status            # ESP32 status (online/offline)
```

## Code Changes Required

### ESP32 Code (Simplified)
- Remove `HeatsoakMonitor` class (~230 lines)
- Remove `IAQMonitor` class (~80 lines)
- Simplify `publishBaseReadings()` to only publish raw data
- Remove `publishIAQReadings()` and `publishHeatsoakReadings()`

**Estimated reduction**: ~400 lines of code removed

### RPi Service (Already Exists)
- Use `base-readings.py` for heatsoak calculations
- Use `monitor-iaq.py` for IAQ calculations
- Both subscribe to `sensors/bme680/raw`
- Both publish calculated values

## Best Practices Followed

1. **Edge Computing**: Raw data collection at edge
2. **Centralized Processing**: Calculations in one place
3. **MQTT Pub/Sub**: Decoupled architecture
4. **Separation of Concerns**: Each component has single responsibility
5. **Maintainability**: Logic changes don't require firmware updates
6. **Scalability**: Easy to add more sensors

## When to Use Edge Calculations

Edge calculations (on ESP32) are appropriate when:
- ❌ Network connectivity is unreliable
- ❌ Real-time response is critical (< 100ms)
- ❌ Battery-powered device needs to minimize transmissions
- ❌ Calculations are simple and unlikely to change

For this use case (3D printer enclosure monitoring):
- ✅ Network is reliable (local MQTT)
- ✅ Latency requirements are not critical (30s intervals)
- ✅ ESP32 is powered (not battery)
- ✅ Calculations are complex and may need updates

**Conclusion**: Centralized processing on RPi is the better choice.

