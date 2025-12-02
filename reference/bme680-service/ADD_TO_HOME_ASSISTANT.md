# How to Add BME680 MQTT Sensors to Home Assistant

## ✅ Already Done

The sensor configuration files have been copied to your Home Assistant packages directory:
- `/home/pi/homeassistant/packages/bme680-mqtt.yaml` - Base sensors (temperature, humidity, pressure, gas, IAQ)
- `/home/pi/homeassistant/packages/bme680-heatsoak-mqtt.yaml` - Heat soak sensors

## 📋 Prerequisites

1. **MQTT Integration Configured**: Home Assistant must have the MQTT integration configured
   - Go to Settings → Devices & Services → MQTT
   - Ensure it's configured and connected to your MQTT broker (usually `localhost:1883`)

2. **Packages Include**: Your `configuration.yaml` should include packages (already verified ✅)

## 🔄 Steps to Add Sensors

### Step 1: Verify MQTT Integration

1. Open Home Assistant
2. Go to **Settings** → **Devices & Services**
3. Find **MQTT** integration
4. Verify it shows as **Connected**

### Step 2: Restart Home Assistant

After copying the sensor configs, restart Home Assistant to load them:

```bash
# If using systemd
sudo systemctl restart home-assistant

# Or if running in a container/venv, restart appropriately
```

### Step 3: Verify Sensors Appear

After restart, the sensors should automatically appear:

1. Go to **Settings** → **Devices & Services** → **Entities**
2. Search for "BME680" - you should see:
   - **BME680 Temperature (MQTT)**
   - **BME680 Humidity (MQTT)**
   - **BME680 Pressure (MQTT)**
   - **BME680 Gas Resistance (MQTT)**
   - **BME680 IAQ Score (MQTT)**
   - **BME680 Heat Stable (MQTT)** (binary sensor)
   - **BME680 Safe to Open (MQTT)** (binary sensor)
   - **BME680 Chamber Temperature (MQTT)**
   - **BME680 Chamber Smoothed Temperature (MQTT)**
   - **BME680 Chamber Rate of Change (MQTT)**
   - **BME680 Heat Soak Ready (MQTT)** (binary sensor)

## 🔍 Troubleshooting

### Sensors Not Appearing?

1. **Check MQTT Messages Are Being Published:**
   ```bash
   # Check base readings
   mosquitto_sub -h localhost -t "sensors/bme680/raw" -C 1
   
   # Check heat soak readings
   mosquitto_sub -h localhost -t "homeassistant/sensor/bme680_chamber/state" -C 1
   ```

2. **Check Home Assistant Logs:**
   - Go to **Settings** → **System** → **Logs**
   - Look for MQTT-related errors

3. **Verify Configuration:**
   ```bash
   # Check if configs are valid YAML
   python3 -c "import yaml; yaml.safe_load(open('/home/pi/homeassistant/packages/bme680-mqtt.yaml'))"
   ```

4. **Check MQTT Broker Connection:**
   - Ensure Home Assistant can reach the MQTT broker
   - If broker is on different host, update `mqtt:` section in `configuration.yaml`

## 📊 MQTT Topics Being Published

Your services are publishing to these topics:

- **`sensors/bme680/raw`** - Base readings (temperature, humidity, pressure, gas)
- **`sensors/bme680/iaq`** - IAQ score and safety flags
- **`homeassistant/sensor/bme680_chamber/state`** - Heat soak data (temperature, rate, ready status)

## 🎯 Next Steps

Once sensors appear:
1. Add them to a dashboard
2. Create automations based on sensor values
3. Set up alerts for temperature/humidity thresholds
4. Use heat soak ready sensor in your print workflow

## 📝 Manual Addition (Alternative)

If you prefer to add sensors manually through the UI:

1. Go to **Settings** → **Devices & Services** → **Add Integration**
2. Search for **MQTT**
3. Select **Configure a device manually**
4. Enter sensor details from the YAML files

However, using the package files is recommended as it's easier to manage and update.

