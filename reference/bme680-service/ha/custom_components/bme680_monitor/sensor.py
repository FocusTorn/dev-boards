"""Sensor platform for BME680 Monitor integration."""
from __future__ import annotations

from typing import Any

from datetime import timedelta
from homeassistant.components.sensor import SensorEntity
from homeassistant.const import PERCENTAGE, UnitOfPressure, UnitOfTemperature
from homeassistant.helpers.update_coordinator import DataUpdateCoordinator, UpdateFailed
from homeassistant.core import HomeAssistant

from .const import (
    DOMAIN,
    DEFAULT_BUS,
    DEFAULT_ADDR,
    DEFAULT_INTERVAL,
    DEFAULT_ENABLE_I2C,
)


def _open_sensor(bus: int, addr: int):
    # Use vendored monitor driver
    from .monitor import BME680
    sensor = BME680(addr)
    sensor.set_gas_heater_temperature(320)
    sensor.set_gas_heater_duration(150)
    sensor.select_gas_heater_profile(0)
    return sensor


class BME680Coordinator(DataUpdateCoordinator):
    def __init__(self, hass: HomeAssistant, bus: int, addr: int, interval: int) -> None:
        super().__init__(hass, name="bme680_monitor", update_interval=timedelta(seconds=interval))
        self._bus = bus
        self._addr = addr
        self._sensor = None

    async def _async_update_data(self) -> dict[str, Any]:
        try:
            if self._sensor is None:
                self._sensor = await self.hass.async_add_executor_job(_open_sensor, self._bus, self._addr)
            got = await self.hass.async_add_executor_job(self._sensor.get_sensor_data)
            data = {}
            if got:
                d = self._sensor.data
                data = {
                    "temperature": d.temperature,
                    "humidity": d.humidity,
                    "pressure": d.pressure,
                    "gas": d.gas_resistance,
                    "heat_stable": bool(getattr(d, "heat_stable", False)),
                }
            return data
        except Exception as err:  # noqa: BLE001
            raise UpdateFailed(str(err))


async def async_setup_platform(
    hass: HomeAssistant, config: dict[str, Any], async_add_entities, discovery_info=None
) -> None:
    """Set up BME680 Monitor sensor platform."""
    # Get configuration from domain data
    domain_config = hass.data.get(DOMAIN, {}).get("config", {})
    
    # Check if I2C is enabled
    enable_i2c = domain_config.get("enable_i2c", DEFAULT_ENABLE_I2C)
    
    if not enable_i2c:
        # I2C is disabled, don't create any entities
        return
    
    bus = domain_config.get("bus", DEFAULT_BUS)
    addr = domain_config.get("address", DEFAULT_ADDR)
    interval = domain_config.get("scan_interval", DEFAULT_INTERVAL)

    coordinator = BME680Coordinator(hass, bus, addr, interval)
    await coordinator.async_refresh()

    entities: list[SensorEntity] = [
        TemperatureEntity(coordinator),
        HumidityEntity(coordinator),
        PressureEntity(coordinator),
        GasEntity(coordinator),
    ]
    async_add_entities(entities)


class BaseBMEEntity(SensorEntity):
    _attr_has_entity_name = True
    
    def __init__(self, coordinator: BME680Coordinator) -> None:
        self.coordinator = coordinator
        self._attr_available = True

    @property
    def available(self) -> bool:
        """Return if entity is available."""
        return self.coordinator.last_update_success

    async def async_added_to_hass(self) -> None:
        """When entity is added to hass."""
        await super().async_added_to_hass()
        self.async_on_remove(
            self.coordinator.async_add_listener(self._handle_coordinator_update)
        )

    def _handle_coordinator_update(self) -> None:
        """Handle updated data from the coordinator."""
        self.async_write_ha_state()

    @property
    def device_info(self):
        return {
            "identifiers": {(DOMAIN, "bme680_monitor")},
            "name": "BME680 Monitor",
            "manufacturer": "Bosch",
            "model": "BME680",
        }


class TemperatureEntity(BaseBMEEntity):
    _attr_name = "BME680 Temperature"
    _attr_native_unit_of_measurement = UnitOfTemperature.CELSIUS

    def __init__(self, coordinator: BME680Coordinator) -> None:
        super().__init__(coordinator)
        self._attr_unique_id = "bme680_monitor_temperature"

    @property
    def native_value(self):
        return self.coordinator.data.get("temperature")


class HumidityEntity(BaseBMEEntity):
    _attr_name = "BME680 Humidity"
    _attr_native_unit_of_measurement = PERCENTAGE

    def __init__(self, coordinator: BME680Coordinator) -> None:
        super().__init__(coordinator)
        self._attr_unique_id = "bme680_monitor_humidity"

    @property
    def native_value(self):
        return self.coordinator.data.get("humidity")


class PressureEntity(BaseBMEEntity):
    _attr_name = "BME680 Pressure"
    _attr_native_unit_of_measurement = UnitOfPressure.HPA

    def __init__(self, coordinator: BME680Coordinator) -> None:
        super().__init__(coordinator)
        self._attr_unique_id = "bme680_monitor_pressure"

    @property
    def native_value(self):
        return self.coordinator.data.get("pressure")


class GasEntity(BaseBMEEntity):
    _attr_name = "BME680 Gas Resistance"

    def __init__(self, coordinator: BME680Coordinator) -> None:
        super().__init__(coordinator)
        self._attr_unique_id = "bme680_monitor_gas"

    @property
    def native_value(self):
        return self.coordinator.data.get("gas")


