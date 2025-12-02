#!/usr/bin/env python3
"""
BME680 Unified Readings Publisher
Publishes all sensor data including calculated values to a single MQTT topic

Features:
- Reads temperature, humidity, pressure, and gas resistance
- Calculates heatsoak metrics (smoothed temp, rate of change, etc.)
- Publishes all data to single MQTT topic
- Simple JSON format for easy consumption

Usage:
    # Run with MQTT publishing
    python base-readings.py --mqtt-host localhost --topic sensors/bme680/raw
    
    # Run with custom interval
    python base-readings.py --mqtt-host localhost --topic sensors/bme680/raw --interval 10
"""

import sys
import time
import json
import argparse
from pathlib import Path
from collections import deque
from typing import Optional

# Add monitor to path (parent directory)
sys.path.insert(0, str(Path(__file__).parent.parent))
from monitor import BME680, I2C_ADDR_SECONDARY

try:
    import paho.mqtt.client as mqtt
    MQTT_AVAILABLE = True
except ImportError:
    MQTT_AVAILABLE = False
    print("⚠️  paho-mqtt not installed. MQTT features disabled.")
    print("   Install with: uv pip install paho-mqtt")

try:
    import yaml
    YAML_AVAILABLE = True
except ImportError:
    YAML_AVAILABLE = False
    print("⚠️  PyYAML not installed. Config file reading disabled.")
    print("   Install with: uv pip install pyyaml")


class UnifiedBME680Monitor:
    """BME680 monitor that includes base readings and heatsoak calculations."""
    
    def __init__(self, i2c_address=0x77, heater_temp=320, heater_duration=150,
                 temp_smooth_time=4.0, rate_smooth_time=30.0):
        """Initialize BME680 sensor and smoothing windows."""
        self.sensor = BME680(i2c_address)
        self.sensor.set_gas_heater_temperature(heater_temp)
        self.sensor.set_gas_heater_duration(heater_duration)
        self.sensor.select_gas_heater_profile(0)
        
        # Heatsoak smoothing configuration
        self.temp_smooth_time = int(temp_smooth_time)
        self.rate_smooth_time = int(rate_smooth_time)
        
        # Temperature history buffers for heatsoak calculations
        self.temp_history = deque(maxlen=self.temp_smooth_time + 1)
        self.smoothed_temp_history = deque(maxlen=self.rate_smooth_time + 1)
        
        # Current readings
        self.current_temp = None
        self.smoothed_temp = None
        self.rate_per_minute = None
        
        # Heatsoak tracking
        self.soak_started = False
        self.max_rate_since_soak_start = None
        self.initial_soak_temp = None  # Temperature when soak_started became true (for offset mode)
        
    def read(self) -> bool:
        """Read current sensor values (returns True if successful and stable)."""
        if not self.sensor.get_sensor_data():
            return False
            
        if not self.sensor.data.heat_stable:
            return False
            
        self.current_temp = self.sensor.data.temperature
        return True
    
    def update_smoothing(self):
        """Update smoothed temperature and rate of change."""
        if self.current_temp is None:
            return
        
        # Add to raw temp history
        self.temp_history.append(self.current_temp)
        
        # Calculate smoothed temp (simple average)
        if len(self.temp_history) > self.temp_smooth_time:
            if len(self.temp_history) > self.temp_smooth_time + 1:
                self.temp_history.popleft()
            
            # Calculate average
            self.smoothed_temp = sum(self.temp_history) / len(self.temp_history)
            
            # Add to smoothed history for rate calculation
            self.smoothed_temp_history.append(self.smoothed_temp)
            
            # Remove oldest if window full
            if len(self.smoothed_temp_history) > self.rate_smooth_time + 1:
                self.smoothed_temp_history.popleft()
            
            # Calculate rate using least squares linear regression
            if len(self.smoothed_temp_history) > self.rate_smooth_time:
                self.rate_per_minute = self._calculate_rate()
    
    def _calculate_rate(self) -> float:
        """Calculate rate of temperature change using least squares linear regression."""
        if len(self.smoothed_temp_history) <= 1:
            return 0.0
        
        count = len(self.smoothed_temp_history)
        times = list(range(count))
        temps = list(self.smoothed_temp_history)
        
        x_sum = sum(times)
        y_sum = sum(temps)
        xx_sum = sum(x * x for x in times)
        xy_sum = sum(x * y for x, y in zip(times, temps))
        
        denominator = float(count * xx_sum - x_sum * x_sum)
        if abs(denominator) < 1e-10:
            return 0.0
        
        slope_per_second = (count * xy_sum - x_sum * y_sum) / denominator
        slope_per_minute = slope_per_second * 60.0
        return round(slope_per_minute, 3)
    
    def check_heat_soak_ready(self, rate_start_type="absolute", rate_start_temp=40.0, rate_change_plateau=0.1, target_temp=None) -> tuple[bool, dict]:
        """Check if heat soak conditions are met."""
        if self.smoothed_temp is None or self.rate_per_minute is None:
            if self.soak_started:
                self.soak_started = False
                self.max_rate_since_soak_start = None
            return False, {
                'ready': False,
                'reason': 'Insufficient data',
                'soak_started': False,
                'max_rate_since_soak_start': None
            }
        
        # Calculate readiness conditions
        # temp_ok: true when smoothed_temp > target_temp
        temp_ok = target_temp is not None and self.smoothed_temp > target_temp
        
        # Calculate rate threshold based on rate_start_type
        if rate_start_type == "offset":
            # Offset mode: rate_start_temp is added to initial temp when soak_started becomes true
            if self.initial_soak_temp is None:
                # Haven't started soaking yet, use absolute comparison
                rate_threshold = rate_start_temp
            else:
                # Use offset from initial soak temperature
                rate_threshold = self.initial_soak_temp + rate_start_temp
        else:
            # Absolute mode: rate_start_temp is used as-is
            rate_threshold = rate_start_temp
        
        # rate_ok: true when smoothed_temp > rate_threshold AND smoothed_change_rate < rate_change_plateau
        rate_ok = (self.smoothed_temp > rate_threshold and 
                   abs(self.rate_per_minute) < rate_change_plateau)
        
        # ready: true when temp_ok == true OR rate_ok == true
        ready = temp_ok or rate_ok
        
        # Track soak status transitions
        # soak_started: flag that indicates if heat soaking is happening
        # Resets max_rate_since_soak_start when transitioning from false to true
        if ready:
            if not self.soak_started:
                # Starting a new soak cycle - reset tracking and store initial temp (for offset mode)
                self.soak_started = True
                self.initial_soak_temp = self.smoothed_temp
                self.max_rate_since_soak_start = abs(self.rate_per_minute) if self.rate_per_minute is not None else None
            # If already soaking, continue tracking (don't reset)
        else:
            # Not ready - stop soaking
            if self.soak_started:
                self.soak_started = False
                self.initial_soak_temp = None
                self.max_rate_since_soak_start = None
        
        # Update max_rate_since_soak_start
        # The largest jump in rate change since heat soaking started
        # Is reset for each heatsoak
        # Used for dashboard gauge start value
        if self.soak_started and self.rate_per_minute is not None:
            current_abs_rate = abs(self.rate_per_minute)
            if self.max_rate_since_soak_start is None or current_abs_rate > self.max_rate_since_soak_start:
                self.max_rate_since_soak_start = current_abs_rate
        
        return ready, {
            'ready': ready,
            'temp_ok': temp_ok,
            'rate_ok': rate_ok,
            'soak_started': self.soak_started,
            'max_rate_since_soak_start': round(self.max_rate_since_soak_start, 3) if self.max_rate_since_soak_start is not None else None
        }
    
    def get_readings(self, temp_offset: float = 0.0, rate_start_type: str = "absolute", 
                     rate_start_temp: float = 40.0, rate_change_plateau: float = 0.1, 
                     target_temp: Optional[float] = None) -> dict:
        """Get all sensor readings including calculated values.
        
        Args:
            temp_offset: Temperature offset in Celsius to apply
            rate_start_type: "offset" (adds to initial soak temp) or "absolute" (uses as-is)
            rate_start_temp: Temperature to start checking rate (prevents false positives during ramp-up)
            rate_change_plateau: Maximum rate of change threshold (°C/min) - indicates diminishing returns
            target_temp: Target temperature (if reached, automatically ready)
        """
        if not self.read():
            return None
        
        # Update smoothing calculations
        self.update_smoothing()
        
        # Check heatsoak status
        ready, heatsoak_info = self.check_heat_soak_ready(rate_start_type, rate_start_temp, rate_change_plateau, target_temp)
        
        raw_temp = self.sensor.data.temperature
        adjusted_temp = raw_temp + temp_offset
            
        # Build comprehensive readings dictionary
        readings = {
            # Base sensor readings
            'temperature': round(adjusted_temp, 2),
            'humidity': round(self.sensor.data.humidity, 2),
            'pressure': round(self.sensor.data.pressure, 2),
            'gas_resistance': round(self.sensor.data.gas_resistance, 0),
            'heat_stable': self.sensor.data.heat_stable,
            'timestamp': time.time(),
            
            # Heatsoak calculations
            # smoothed_temp: Average of raw temperature readings over temp_smoothing_buffer
            'smoothed_temp': round(self.smoothed_temp, 2) if self.smoothed_temp is not None else None,
            # smoothed_change_rate: Rate of temperature change per minute calculated from smoothed_temp history
            'smoothed_change_rate': self.rate_per_minute,
            # temp_smoothing_buffer: Count of temperature readings being used to calculate smoothed_temp
            'temp_smoothing_buffer': len(self.temp_history),
            # rate_smoothing_buffer: Count of smoothed_temp values being used to calculate smoothed_change_rate
            'rate_smoothing_buffer': len(self.smoothed_temp_history),
        }
        
        # Add heatsoak status
        readings.update(heatsoak_info)
        readings['rate_start_type'] = rate_start_type
        readings['rate_start_temp'] = rate_start_temp
        readings['rate_change_plateau'] = rate_change_plateau
        if target_temp is not None:
            readings['target_temp'] = target_temp
        if self.initial_soak_temp is not None:
            readings['initial_soak_temp'] = round(self.initial_soak_temp, 2)
        
        return readings


def load_config(config_path: str = None) -> dict:
    """Load configuration from YAML file.
    
    Args:
        config_path: Path to config file. If None, uses default location.
        
    Returns:
        Dictionary with config values, or empty dict if file not found.
    """
    if not YAML_AVAILABLE:
        return {}
    
    if config_path is None:
        config_path = Path.home() / ".config" / "bme680-monitor" / "config.yaml"
    else:
        config_path = Path(config_path)
    
    if not config_path.exists():
        return {}
    
    try:
        with open(config_path, 'r') as f:
            config = yaml.safe_load(f) or {}
        return config
    except Exception as e:
        print(f"⚠️  Warning: Failed to load config from {config_path}: {e}", file=sys.stderr)
        return {}


def get_config_value(config: dict, *keys, default=None):
    """Safely get nested config value.
    
    Args:
        config: Config dictionary
        *keys: Nested keys to traverse (e.g., 'mqtt', 'read_interval')
        default: Default value if key not found
        
    Returns:
        Config value or default
    """
    value = config
    for key in keys:
        if isinstance(value, dict):
            value = value.get(key)
        else:
            return default
        if value is None:
            return default
    return value if value is not None else default


def main():
    # Load config file first
    config = load_config()
    
    parser = argparse.ArgumentParser(
        description='BME680 Unified Readings Publisher',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Publish to MQTT (all data including heatsoak)
  base-readings.py --mqtt-host localhost --topic sensors/bme680/raw
  
  # Custom interval and heatsoak settings
  base-readings.py --mqtt-host localhost --topic sensors/bme680/raw --interval 10 --soak-temp 40 --rate-change-plateau 0.1
  
  # Single reading (no MQTT)
  base-readings.py --once
  
  # Use custom config file
  base-readings.py --config /path/to/config.yaml
        """
    )
    
    # Config file override
    parser.add_argument('--config', type=str, default=None,
                       help='Path to config file (default: ~/.config/bme680-monitor/config.yaml)')
    
    # Reading mode
    parser.add_argument('--once', action='store_true',
                       help='Read once and exit (no MQTT)')
    
    # Calibration
    parser.add_argument('--temp-offset', type=float, default=None,
                       help='Temperature offset in Celsius to apply. Use negative values to subtract heat from Pi.')
    
    # Monitoring
    parser.add_argument('--read-interval', type=int, default=None,
                       help='Sensor reading interval in seconds')
    parser.add_argument('--publish-interval', type=int, default=None,
                       help='MQTT publish interval in seconds')
    
    # Heatsoak settings
    parser.add_argument('--rate-start-type', type=str, default=None, choices=['offset', 'absolute'],
                       help='Rate start type: "offset" (adds to initial soak temp) or "absolute" (uses as-is)')
    parser.add_argument('--rate-start-temp', type=float, default=None,
                       help='Temperature to start checking rate - prevents false positives during ramp-up (°C)')
    parser.add_argument('--rate-change-plateau', type=float, default=None,
                       help='Maximum rate of change threshold (°C/min) - indicates diminishing returns')
    parser.add_argument('--target-temp', type=float, default=None,
                       help='Target temperature - if reached, automatically ready')
    parser.add_argument('--temp-smooth', type=float, default=None,
                       help='Temperature smoothing window, seconds')
    parser.add_argument('--rate-smooth', type=float, default=None,
                       help='Rate smoothing window, seconds')
    
    # MQTT
    parser.add_argument('--mqtt-host', default=None,
                       help='MQTT broker host')
    parser.add_argument('--mqtt-port', type=int, default=None,
                       help='MQTT broker port')
    parser.add_argument('--topic', default=None,
                       help='MQTT topic for all readings')
    parser.add_argument('--mqtt-client-id', default='bme680-unified-readings',
                       help='MQTT client ID (default: bme680-unified-readings)')
    parser.add_argument('--retain', type=str, default=None, choices=['true', 'false', 'True', 'False'],
                       help='Retain MQTT messages. Set to true if you want HA to get last value on startup')
    
    # Output
    parser.add_argument('--json', action='store_true',
                       help='Output JSON format')
    parser.add_argument('--quiet', action='store_true',
                       help='Suppress console output')
    
    args = parser.parse_args()
    
    # Reload config if custom path provided
    if args.config:
        config = load_config(args.config)
    
    # Apply config values as defaults (command-line args override config)
    temp_offset = args.temp_offset if args.temp_offset is not None else get_config_value(config, 'mqtt', 'temp_offset', default=0.0)
    read_interval = args.read_interval if args.read_interval is not None else get_config_value(config, 'mqtt', 'read_interval', default=1)
    publish_interval = args.publish_interval if args.publish_interval is not None else get_config_value(config, 'mqtt', 'publish_interval', default=30)
    temp_smooth = args.temp_smooth if args.temp_smooth is not None else get_config_value(config, 'mqtt', 'temp_smooth', default=4.0)
    rate_smooth = args.rate_smooth if args.rate_smooth is not None else get_config_value(config, 'mqtt', 'heatsoak', 'rate_smooth_time', default=30.0)
    rate_start_type = args.rate_start_type if args.rate_start_type is not None else get_config_value(config, 'mqtt', 'heatsoak', 'rate_start_type', default='absolute')
    rate_start_temp = args.rate_start_temp if args.rate_start_temp is not None else get_config_value(config, 'mqtt', 'heatsoak', 'rate_start_temp', default=40.0)
    rate_change_plateau = args.rate_change_plateau if args.rate_change_plateau is not None else get_config_value(config, 'mqtt', 'heatsoak', 'rate_change_plateau', default=0.1)
    target_temp = args.target_temp if args.target_temp is not None else get_config_value(config, 'mqtt', 'heatsoak', 'target_temp', default=None)
    mqtt_host = args.mqtt_host if args.mqtt_host is not None else 'localhost'
    mqtt_port = args.mqtt_port if args.mqtt_port is not None else 1883
    topic = args.topic if args.topic is not None else get_config_value(config, 'mqtt', 'topic_base', default='sensors/bme680/raw')
    retain = args.retain if args.retain is not None else str(get_config_value(config, 'mqtt', 'retain', default=False)).lower()
    
    try:
        monitor = UnifiedBME680Monitor(
            temp_smooth_time=temp_smooth,
            rate_smooth_time=rate_smooth
        )
        
        # Single read mode
        if args.once:
            readings = monitor.get_readings(
                temp_offset=temp_offset,
                rate_start_type=rate_start_type,
                rate_start_temp=rate_start_temp,
                rate_change_plateau=rate_change_plateau,
                target_temp=target_temp
            )
            if readings:
                if args.json:
                    print(json.dumps(readings, indent=2))
                else:
                    print(f"Temperature: {readings['temperature']}°C")
                    print(f"Humidity: {readings['humidity']}%")
                    print(f"Pressure: {readings['pressure']} hPa")
                    print(f"Gas Resistance: {readings['gas_resistance']}Ω")
                    if readings.get('smoothed_temp'):
                        print(f"Smoothed Temp: {readings['smoothed_temp']}°C")
                    if readings.get('smoothed_change_rate') is not None:
                        print(f"Rate: {readings['smoothed_change_rate']}°C/min")
            else:
                print("❌ Failed to read sensor (not heat stable)")
                sys.exit(1)
            sys.exit(0)
        
        # MQTT mode (required for continuous monitoring)
        if not MQTT_AVAILABLE:
            print("❌ MQTT not available. Install with: uv pip install paho-mqtt")
            sys.exit(1)
        
        if not args.quiet:
            print(f"\n📊 Starting BME680 unified readings publisher...")
            print(f"   MQTT: {mqtt_host}:{mqtt_port}")
            print(f"   Topic: {topic}")
            print(f"   Read interval: {read_interval}s")
            print(f"   Publish interval: {publish_interval}s")
            print(f"   Heatsoak: rate_start_type={rate_start_type}, rate_start_temp={rate_start_temp}°C, plateau={rate_change_plateau}°C/min", end="")
            if target_temp is not None:
                print(f", target={target_temp}°C")
            else:
                print()
            print("   Press Ctrl+C to stop\n")
        
        # Setup MQTT client
        mqtt_client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2, args.mqtt_client_id)
        try:
            mqtt_client.connect(mqtt_host, mqtt_port)
            mqtt_client.loop_start()
            if not args.quiet:
                print(f"✅ Connected to MQTT broker")
        except Exception as e:
            print(f"❌ MQTT connection failed: {e}")
            sys.exit(1)
        
        # Main monitoring loop
        last_publish_time = 0
        try:
            while True:
                current_time = time.time()
                
                # Always read sensor (for smoothing calculations)
                readings = monitor.get_readings(
                    temp_offset=temp_offset,
                    rate_start_type=rate_start_type,
                    rate_start_temp=rate_start_temp,
                    rate_change_plateau=rate_change_plateau,
                    target_temp=target_temp
                )
                
                if readings:
                    # Display (only if json mode or not quiet)
                    if args.json:
                        print(json.dumps(readings))
                        sys.stdout.flush()
                    elif not args.quiet:
                        ready_icon = "✅" if readings.get('ready') else "⏳"
                        rate_str = f"{readings.get('smoothed_change_rate', 0):.3f}" if readings.get('smoothed_change_rate') is not None else "---"
                        smoothed_str = f"{readings.get('smoothed_temp', 0):.2f}" if readings.get('smoothed_temp') is not None else "---"
                        print(f"{ready_icon} Temp: {readings['temperature']}°C (smoothed: {smoothed_str}°C) | "
                              f"Rate: {rate_str}°C/min")
                        sys.stdout.flush()
                    
                    # Publish to MQTT only at specified interval
                    should_publish = (current_time - last_publish_time) >= publish_interval
                    if should_publish:
                        try:
                            # Convert retain string to boolean
                            retain_flag = str(retain).lower() == 'true'
                            result = mqtt_client.publish(
                                topic,
                                json.dumps(readings),
                                qos=1,
                                retain=retain_flag  # Retain flag from config/command line
                            )
                            last_publish_time = current_time
                            if not args.quiet:
                                print(f"   ✅ Published to {topic}")
                                sys.stdout.flush()
                        except Exception as e:
                            print(f"   ❌ MQTT publish failed: {e}", file=sys.stderr)
                            sys.stderr.flush()
                else:
                    if not args.quiet and not args.json:
                        print("⏳ Waiting for heat stable reading...")
                
                time.sleep(read_interval)
                
        except KeyboardInterrupt:
            if not args.quiet:
                print("\n\n✅ Monitoring stopped")
        
        # Cleanup
        if mqtt_client:
            mqtt_client.loop_stop()
            mqtt_client.disconnect()
        
    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == '__main__':
    main()

