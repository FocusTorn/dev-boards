# Troubleshooting: Sensors Showing "Unknown"

## Quick Fixes

### 1. Restart Home Assistant
After updating the sensor configs, **restart Home Assistant**:
```bash
sudo systemctl restart home-assistant
# Or however you restart your HA instance
```

### 2. Verify MQTT Integration is Connected
1. Go to **Settings** → **Devices & Services** → **MQTT**
2. Ensure it shows **Connected** (green status)
3. If not connected, check MQTT broker settings

### 3. Check MQTT Broker is Running
```bash
systemctl status mosquitto
```

### 4. Force Sensor Update
After restart, sensors should receive messages automatically. If they don't:
- Check Home Assistant logs: **Settings** → **System** → **Logs**
- Look for MQTT-related errors
- Verify sensors are subscribed to correct topics

### 5. Verify Sensor Configuration
The sensors are configured to listen to:
- **Topic**: `sensors/bme680/raw`
- **Expected JSON structure**:
  ```json
  {
    "temperature": 26.23,
    "humidity": 50.0,
    "pressure": 982.73,
    "gas_resistance": 173576.0,
    "heat_stable": true,
    "timestamp": 1762466376.3157194
  }
  ```

### 6. Check Entity States
After restart, check entity states:
1. Go to **Settings** → **Devices & Services** → **Entities**
2. Search for "BME680"
3. Click on a sensor entity
4. Check "Last Updated" timestamp
5. If it's old or "unknown", the sensor isn't receiving MQTT messages

### 7. Manual MQTT Test
Test if messages are being published:
```bash
# Listen for messages (should see one every 30 seconds)
mosquitto_sub -h localhost -t "sensors/bme680/raw" -v
```

If you see messages but HA sensors are still "unknown", the issue is likely:
- Home Assistant MQTT integration not connected
- Wrong MQTT broker host/port in HA config
- Firewall blocking MQTT traffic

