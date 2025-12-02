"""The BME680 Monitor integration."""
from __future__ import annotations

import json
import subprocess
import logging
import yaml
from pathlib import Path
from typing import Any

from homeassistant.core import HomeAssistant
from homeassistant.const import Platform
from homeassistant.helpers import discovery

from .const import (
    DOMAIN,
    DEFAULT_ENABLE_I2C,
    DEFAULT_ENABLE_MQTT,
    DEFAULT_HEATSOAK_RATE_CHANGE_PLATEAU,
    DEFAULT_BUS,
    DEFAULT_ADDR,
    DEFAULT_INTERVAL,
    DEFAULT_MQTT_TOPIC_BASE,
)

_LOGGER = logging.getLogger(__name__)

PLATFORMS: list[Platform] = [Platform.SENSOR]

# YAML config file location
# Use /config path (mapped from /home/pi/homeassistant/) so it's accessible from container
CONFIG_FILE = Path("/config/bme680-monitor/config.yaml")
# Also check host path for backwards compatibility
CONFIG_FILE_HOST = Path("/home/pi/.config/bme680-monitor/config.yaml")


def _load_yaml_config() -> dict[str, Any]:
    """Load configuration from YAML file."""
    # Try container path first, then host path
    config_path = CONFIG_FILE if CONFIG_FILE.exists() else CONFIG_FILE_HOST
    
    if not config_path.exists():
        _LOGGER.warning(
            f"Config file not found: {CONFIG_FILE} or {CONFIG_FILE_HOST}. Using defaults. "
            f"Create the file to configure BME680 Monitor."
        )
        return {}
    
    try:
        with open(config_path, "r") as f:
            config = yaml.safe_load(f) or {}
        _LOGGER.info(f"Loaded config from {config_path}")
        return config
    except Exception as e:
        _LOGGER.error(f"Failed to load config file {config_path}: {e}. Using defaults.")
        return {}


def _get_config_value(config: dict[str, Any], *keys: str, default: Any = None) -> Any:
    """Get nested config value with fallback to default."""
    value = config
    for key in keys:
        if isinstance(value, dict):
            value = value.get(key)
        else:
            return default
        if value is None:
            return default
    return value if value is not None else default


def _write_heatsoak_config_and_restart_service(heatsoak_max_rate: float, enable_mqtt: bool) -> None:
    """Write heatsoak config file and restart systemd service if MQTT is enabled."""
    if not enable_mqtt:
        return
    
    try:
        # Write config file
        config_file = Path("/home/pi/.local/share/bme680-service/heatsoak-config.json")
        config_file.parent.mkdir(parents=True, exist_ok=True)
        with open(config_file, "w") as f:
            json.dump({"rate_change_plateau": heatsoak_max_rate}, f)
        
        _LOGGER.info(f"Wrote heatsoak config: rate_change_plateau={heatsoak_max_rate}")
        
        # Try to restart the systemd service
        # Use a script in the HA config directory (mounted from host)
        restart_script = Path("/config/scripts/restart-heatsoak-service.sh")
        
        service_restarted = False
        if restart_script.exists():
            try:
                result = subprocess.run(
                    [str(restart_script)],
                    capture_output=True,
                    text=True,
                    timeout=5,
                    check=False
                )
                if result.returncode == 0:
                    _LOGGER.info("Successfully restarted bme680-heatsoak-mqtt.service")
                    service_restarted = True
                else:
                    _LOGGER.debug(f"Restart script output: {result.stdout} {result.stderr}")
            except (subprocess.TimeoutExpired, PermissionError) as e:
                _LOGGER.debug(f"Could not execute restart script: {e}")
        
        if not service_restarted:
            _LOGGER.info(
                "Heatsoak config file updated successfully. "
                "The bme680-heatsoak-mqtt service will use the new threshold on its next restart. "
                "To apply immediately, restart the service: sudo systemctl restart bme680-heatsoak-mqtt.service"
            )
    except Exception as e:
        _LOGGER.error(f"Failed to write heatsoak config file: {e}")


async def async_setup(hass: HomeAssistant, config: dict[str, Any]) -> bool:
    """Set up BME680 Monitor from YAML configuration."""
    hass.data.setdefault(DOMAIN, {})
    
    # Load configuration from YAML file
    yaml_config = await hass.async_add_executor_job(_load_yaml_config)
    
    # Extract configuration values
    enable_i2c = _get_config_value(yaml_config, "i2c", "enabled", default=DEFAULT_ENABLE_I2C)
    enable_mqtt = _get_config_value(yaml_config, "mqtt", "enabled", default=DEFAULT_ENABLE_MQTT)
    bus = _get_config_value(yaml_config, "i2c", "bus", default=DEFAULT_BUS)
    addr = _get_config_value(yaml_config, "i2c", "address", default=DEFAULT_ADDR)
    interval = _get_config_value(yaml_config, "i2c", "scan_interval", default=DEFAULT_INTERVAL)
    mqtt_topic_base = _get_config_value(yaml_config, "mqtt", "topic_base", default=DEFAULT_MQTT_TOPIC_BASE)
    heatsoak_max_rate = _get_config_value(yaml_config, "mqtt", "heatsoak", "rate_change_plateau", default=DEFAULT_HEATSOAK_RATE_CHANGE_PLATEAU)
    
    # Store config for sensor platform to use
    hass.data[DOMAIN]["config"] = {
        "enable_i2c": enable_i2c,
        "enable_mqtt": enable_mqtt,
        "bus": bus,
        "address": addr,
        "scan_interval": interval,
        "mqtt_topic_base": mqtt_topic_base,
        "heatsoak_max_rate": heatsoak_max_rate,
    }
    
    # Write heatsoak config and restart service (runs in executor to avoid blocking)
    await hass.async_add_executor_job(
        _write_heatsoak_config_and_restart_service,
        heatsoak_max_rate,
        enable_mqtt
    )
    
    # Clean up any orphaned I2C entities if I2C is disabled
    if not enable_i2c:
        try:
            from homeassistant.helpers import entity_registry as er
            registry = er.async_get(hass)
            
            # Find and remove any I2C entities
            entities_to_remove = []
            for entity_entry in list(registry.entities.values()):
                if (entity_entry.platform == DOMAIN and
                    not '_mqtt' in entity_entry.entity_id):
                    entities_to_remove.append(entity_entry.entity_id)
            
            for entity_id in entities_to_remove:
                registry.async_remove(entity_id)
        except Exception:
            # If entity registry access fails, continue anyway
            pass
    
    # Set up I2C sensors if enabled
    if enable_i2c:
        # Load sensor platform
        await discovery.async_load_platform(
            hass,
            Platform.SENSOR,
            DOMAIN,
            {},
            config,
        )
    
    # Manage MQTT package based on enable_mqtt setting
    # All MQTT sensors are now in a single package file (sensors-bme680-mqtt.yaml)
    # The heatsoak package is deprecated - all data comes from sensors/bme680/raw
    mqtt_package_path = Path("/config/packages/bme680_mqtt.yaml")
    mqtt_package_disabled = Path("/config/packages/bme680_mqtt.yaml.disabled")
    mqtt_heatsoak_package_path = Path("/config/packages/bme680_heatsoak_mqtt.yaml")
    mqtt_heatsoak_package_disabled = Path("/config/packages/bme680_heatsoak_mqtt.yaml.disabled")
    
    if enable_mqtt:
        # Enable main MQTT package
        if mqtt_package_disabled.exists():
            mqtt_package_disabled.rename(mqtt_package_path)
            _LOGGER.info("Enabled bme680_mqtt.yaml package")
        
        # Disable old heatsoak package (deprecated - all data now in main package)
        if mqtt_heatsoak_package_path.exists():
            mqtt_heatsoak_package_path.rename(mqtt_heatsoak_package_disabled)
            _LOGGER.info("Disabled deprecated bme680_heatsoak_mqtt.yaml package (consolidated into main package)")
    else:
        # Disable MQTT packages
        if mqtt_package_path.exists():
            mqtt_package_path.rename(mqtt_package_disabled)
            _LOGGER.info("Disabled bme680_mqtt.yaml package")
        if mqtt_heatsoak_package_path.exists():
            mqtt_heatsoak_package_path.rename(mqtt_heatsoak_package_disabled)
            _LOGGER.info("Disabled bme680_heatsoak_mqtt.yaml package")
        
        # Clean up MQTT entities from entity registry
        try:
            from homeassistant.helpers import entity_registry as er
            registry = er.async_get(hass)
            
            # Find and remove any MQTT entities
            entities_to_remove = []
            for entity_entry in list(registry.entities.values()):
                if ('_mqtt' in entity_entry.entity_id and
                    'bme680' in entity_entry.entity_id.lower() and
                    entity_entry.platform == 'mqtt'):
                    entities_to_remove.append(entity_entry.entity_id)
            
            for entity_id in entities_to_remove:
                registry.async_remove(entity_id)
        except Exception:
            # If entity registry access fails, continue anyway
            pass
    
    _LOGGER.info(
        f"BME680 Monitor setup complete: I2C={enable_i2c}, MQTT={enable_mqtt}, "
        f"Bus={bus}, Address=0x{addr:02x}, Interval={interval}s"
    )
    
    return True
