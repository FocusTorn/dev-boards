"""Constants for the BME680 Monitor integration."""
from __future__ import annotations

DOMAIN = "bme680_monitor"

# I2C Settings
DEFAULT_BUS = 1
DEFAULT_ADDR = 0x77
DEFAULT_INTERVAL = 5

# MQTT Settings
DEFAULT_MQTT_TOPIC_BASE = "sensors/bme680/raw"

# Enable/Disable Settings
DEFAULT_ENABLE_I2C = True
DEFAULT_ENABLE_MQTT = True

# Heatsoak MQTT Settings
DEFAULT_HEATSOAK_RATE_CHANGE_PLATEAU = 0.1  # Maximum rate of change threshold (°C/min)

