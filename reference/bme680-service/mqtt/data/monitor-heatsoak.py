#!/usr/bin/env python3
"""
BME680 Temperature Monitor for 3D Printer Heat Soak
Replaces Klipper HEAT_SOAK macro for HA HACS integration

Features:
- Reads BME680 temperature
- Calculates smoothed temperature and rate of change (slope)
- Least squares linear regression for rate calculation
- Publishes to MQTT for Home Assistant
- Replaces Klipper macro logic: SOAK_TEMP, RATE, smoothing

Usage:
    # Monitor chamber temp (publishes to MQTT)
    uv run python monitor-heatsoak.py --monitor --mqtt
    
    # Custom thresholds
    monitor-heatsoak.py --monitor --soak-temp 40 --max-rate 0.1
"""

import sys
import time
import json
import argparse
from pathlib import Path
from collections import deque
from typing import Optional

# Add monitor to path
sys.path.insert(0, str(Path(__file__).parent.parent))
from monitor import BME680, I2C_ADDR_SECONDARY

try:
    import paho.mqtt.client as mqtt
    MQTT_AVAILABLE = True
except ImportError:
    MQTT_AVAILABLE = False


class TemperatureMonitor:
    """BME680 temperature monitor with heat soak detection."""
    
    def __init__(self, i2c_address=0x77, heater_temp=320, heater_duration=150,
                 temp_smooth_time=4.0, rate_smooth_time=30.0):
        """Initialize BME680 sensor and smoothing windows."""
        self.sensor = BME680(i2c_address)
        self.sensor.set_gas_heater_temperature(heater_temp)
        self.sensor.set_gas_heater_duration(heater_duration)
        self.sensor.select_gas_heater_profile(0)
        
        self.temp_smooth_time = int(temp_smooth_time)
        self.rate_smooth_time = int(rate_smooth_time)
        
        # Temperature history buffers
        self.temp_history = deque(maxlen=self.temp_smooth_time + 1)
        self.smoothed_temp_history = deque(maxlen=self.rate_smooth_time + 1)
        
        self.current_temp = None
        self.smoothed_temp = None
        self.rate_per_minute = None
        self.heat_stable = False
        
        # Heat soak tracking
        self.soak_started = False  # True when heat soak becomes ready
        self.max_rate_since_soak_start = None  # Maximum rate seen since soak started
        
    def read(self) -> bool:
        """Read current temperature (returns True if successful and stable)."""
        if not self.sensor.get_sensor_data():
            return False
            
        if not self.sensor.data.heat_stable:
            return False
            
        self.current_temp = self.sensor.data.temperature
        self.heat_stable = True
        return True
    
    def update_smoothing(self):
        """Update smoothed temperature and rate of change."""
        if self.current_temp is None:
            return
        
        # Add to raw temp history
        self.temp_history.append(self.current_temp)
        
        # Calculate smoothed temp (simple average)
        if len(self.temp_history) > self.temp_smooth_time:
            # Remove oldest to maintain window
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
        """
        Calculate rate of temperature change using least squares linear regression.
        Returns rate in degrees C per minute.
        """
        if len(self.smoothed_temp_history) <= 1:
            return 0.0
        
        # Time points (x)
        count = len(self.smoothed_temp_history)
        times = list(range(count))
        
        # Temperature values (y)
        temps = list(self.smoothed_temp_history)
        
        # Sums
        x_sum = sum(times)
        y_sum = sum(temps)
        xx_sum = sum(x * x for x in times)
        xy_sum = sum(x * y for x, y in zip(times, temps))
        
        # Slope calculation (per second)
        denominator = float(count * xx_sum - x_sum * x_sum)
        if abs(denominator) < 1e-10:
            return 0.0
        
        slope_per_second = (count * xy_sum - x_sum * y_sum) / denominator
        
        # Convert to per minute and round
        slope_per_minute = slope_per_second * 60.0
        return round(slope_per_minute, 3)
    
    def check_heat_soak_ready(self, soak_temp=40.0, max_rate=0.1, target_temp=None) -> tuple[bool, dict]:
        """
        Check if heat soak conditions are met.
        
        Args:
            soak_temp: Minimum temperature to start checking rate (default: 40°C)
            max_rate: Maximum rate of change allowed (°C/min, default: 0.1)
            target_temp: Target temperature - if reached, automatically ready (default: None)
        
        Returns:
            (ready, info) where ready is bool and info contains details
        """
        if self.smoothed_temp is None or self.rate_per_minute is None:
            # Reset soak tracking if we don't have data
            if self.soak_started:
                self.soak_started = False
                self.max_rate_since_soak_start = None
            return False, {
                'current_temp': self.current_temp,
                'smoothed_temp': self.smoothed_temp,
                'rate': self.rate_per_minute,
                'ready': False,
                'reason': 'Insufficient data',
                'soak_started': False,
                'max_rate_since_soak_start': None
            }
        
        # Check if target temp reached (automatically ready)
        if target_temp is not None and self.smoothed_temp >= target_temp:
            # Mark soak as started if not already
            if not self.soak_started:
                self.soak_started = True
                self.max_rate_since_soak_start = abs(self.rate_per_minute) if self.rate_per_minute is not None else None
            
            # Update max rate since soak started
            if self.soak_started and self.rate_per_minute is not None:
                current_abs_rate = abs(self.rate_per_minute)
                if self.max_rate_since_soak_start is None or current_abs_rate > self.max_rate_since_soak_start:
                    self.max_rate_since_soak_start = current_abs_rate
            
            return True, {
                'current_temp': round(self.current_temp, 2),
                'smoothed_temp': round(self.smoothed_temp, 2),
                'rate': self.rate_per_minute,
                'ready': True,
                'reason': 'Target temperature reached',
                'target_temp': target_temp,
                'temp_ok': True,
                'rate_ok': True,
                'soak_temp': soak_temp,
                'max_rate': max_rate,
                'soak_started': self.soak_started,
                'max_rate_since_soak_start': round(self.max_rate_since_soak_start, 3) if self.max_rate_since_soak_start is not None else None
            }
        
        # Check rate-based conditions
        temp_ok = self.smoothed_temp >= soak_temp
        rate_ok = abs(self.rate_per_minute) <= max_rate
        
        ready = temp_ok and rate_ok
        
        # Track soak status transitions
        if ready and not self.soak_started:
            # Heat soak just started - initialize tracking
            self.soak_started = True
            self.max_rate_since_soak_start = abs(self.rate_per_minute) if self.rate_per_minute is not None else None
        elif not ready and self.soak_started:
            # Heat soak ended/reset - clear tracking
            self.soak_started = False
            self.max_rate_since_soak_start = None
        
        # Update max rate since soak started (if soak is active)
        if self.soak_started and self.rate_per_minute is not None:
            current_abs_rate = abs(self.rate_per_minute)
            if self.max_rate_since_soak_start is None or current_abs_rate > self.max_rate_since_soak_start:
                self.max_rate_since_soak_start = current_abs_rate
        
        return ready, {
            'current_temp': round(self.current_temp, 2),
            'smoothed_temp': round(self.smoothed_temp, 2),
            'rate': self.rate_per_minute,
            'ready': ready,
            'temp_ok': temp_ok,
            'rate_ok': rate_ok,
            'soak_temp': soak_temp,
            'max_rate': max_rate,
            'target_temp': target_temp,
            'soak_started': self.soak_started,
            'max_rate_since_soak_start': round(self.max_rate_since_soak_start, 3) if self.max_rate_since_soak_start is not None else None
        }
    
    def get_json_status(self) -> dict:
        """Get current status as dictionary for MQTT."""
        return {
            'temperature': self.current_temp,
            'smoothed_temp': self.smoothed_temp,
            'rate_per_minute': self.rate_per_minute,
            'heat_stable': self.heat_stable,
            'readings': len(self.temp_history),
            'smoothed_readings': len(self.smoothed_temp_history),
            'soak_started': self.soak_started,
            'max_rate_since_soak_start': round(self.max_rate_since_soak_start, 3) if self.max_rate_since_soak_start is not None else None
        }


def main():
    parser = argparse.ArgumentParser(
        description='BME680 Temperature Monitor for Heat Soak',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Basic monitoring
  monitor-heatsoak.py --monitor
  
  # With MQTT for HA
  monitor-heatsoak.py --monitor --mqtt --interval 5
  
  # Custom thresholds
  monitor-heatsoak.py --monitor --soak-temp 45 --max-rate 0.15
        """
    )
    
    # Monitoring
    parser.add_argument('--monitor', action='store_true',
                       help='Continuous monitoring mode')
    parser.add_argument('--interval', type=int, default=1,
                       help='Reading interval in seconds (default: 1)')
    
    # Heat soak thresholds
    parser.add_argument('--soak-temp', type=float, default=40.0,
                       help='Minimum temperature to start checking rate (default: 40.0)')
    parser.add_argument('--target-temp', type=float, default=None,
                       help='Target temperature - if reached, automatically ready (default: None)')
    parser.add_argument('--max-rate', type=float, default=0.1,
                       help='Maximum rate change allowed, °C/min (default: 0.1)')
    
    # Smoothing
    parser.add_argument('--temp-smooth', type=float, default=4.0,
                       help='Temperature smoothing window, seconds (default: 4.0)')
    parser.add_argument('--rate-smooth', type=float, default=30.0,
                       help='Rate smoothing window, seconds (default: 30.0)')
    
    # MQTT
    parser.add_argument('--mqtt', action='store_true',
                       help='Enable MQTT publishing')
    parser.add_argument('--mqtt-host', default='localhost',
                       help='MQTT broker host (default: localhost)')
    parser.add_argument('--mqtt-port', type=int, default=1883,
                       help='MQTT broker port (default: 1883)')
    parser.add_argument('--mqtt-topic', default='homeassistant/sensor/bme680_chamber/state',
                       help='MQTT topic (default: homeassistant/sensor/bme680_chamber/state)')
    parser.add_argument('--mqtt-client-id', default='bme680-temp-monitor',
                       help='MQTT client ID (default: bme680-temp-monitor)')
    parser.add_argument('--mqtt-interval', type=int, default=None,
                       help='MQTT publish interval in seconds (default: same as --interval). Use to publish less frequently than readings.')
    
    # Output
    parser.add_argument('--json', action='store_true',
                       help='Output JSON format')
    parser.add_argument('--quiet', action='store_true',
                       help='Suppress console output')
    
    args = parser.parse_args()
    
    try:
        monitor = TemperatureMonitor(
            temp_smooth_time=args.temp_smooth,
            rate_smooth_time=args.rate_smooth
        )
        
        if not args.monitor:
            # Single read
            if monitor.read():
                monitor.update_smoothing()
                ready, info = monitor.check_heat_soak_ready(args.soak_temp, args.max_rate, args.target_temp)
                
                if args.json:
                    print(json.dumps(monitor.get_json_status()))
                else:
                    print(f"Temperature: {monitor.current_temp:.2f}°C")
                    if monitor.smoothed_temp:
                        print(f"Smoothed: {monitor.smoothed_temp:.2f}°C")
                    if monitor.rate_per_minute is not None:
                        print(f"Rate: {monitor.rate_per_minute:.3f}°C/min")
                    print(f"Heat soak ready: {ready}")
            sys.exit(0)
        
        # Monitor loop
        if not args.quiet:
            print(f"\n🌡️  Starting temperature monitoring...")
            print(f"   Interval: {args.interval}s")
            if args.target_temp:
                print(f"   Heat soak: Target {args.target_temp}°C OR (>{args.soak_temp}°C at <{args.max_rate}°C/min)")
            else:
                print(f"   Heat soak threshold: >{args.soak_temp}°C at <{args.max_rate}°C/min")
            print("   Press Ctrl+C to stop\n")
        
        # MQTT setup
        mqtt_client = None
        if args.mqtt:
            if not MQTT_AVAILABLE:
                print("❌ MQTT not available. Install: uv pip install paho-mqtt")
                sys.exit(1)
            
            mqtt_client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2, args.mqtt_client_id)
            try:
                mqtt_client.connect(args.mqtt_host, args.mqtt_port)
                mqtt_client.loop_start()
                if not args.quiet:
                    print(f"✅ MQTT: {args.mqtt_host}:{args.mqtt_port}\n")
            except Exception as e:
                print(f"❌ MQTT failed: {e}")
                sys.exit(1)
        
        last_ready = None
        last_mqtt_publish = 0
        mqtt_publish_interval = args.mqtt_interval if args.mqtt_interval is not None else args.interval
        
        try:
            while True:
                if monitor.read():
                    monitor.update_smoothing()
                    ready, info = monitor.check_heat_soak_ready(args.soak_temp, args.max_rate, args.target_temp)
                    
                    current_time = time.time()
                    should_publish_mqtt = (current_time - last_mqtt_publish) >= mqtt_publish_interval
                    
                    # Display
                    if args.json:
                        status = monitor.get_json_status()
                        status.update(info)
                        print(json.dumps(status))
                    elif not args.quiet:
                        icon = "✅" if ready else "⏳"
                        rate_str = f"{monitor.rate_per_minute:.3f}" if monitor.rate_per_minute else "---"
                        smoothed_str = f"{monitor.smoothed_temp:.2f}" if monitor.smoothed_temp is not None else "---"
                        print(f"{icon} {monitor.current_temp:.1f}°C | "
                              f"Smoothed: {smoothed_str}°C | "
                              f"Rate: {rate_str}°C/min | Ready: {ready}")
                        sys.stdout.flush()
                    
                    # MQTT publish (only at specified interval)
                    if mqtt_client and should_publish_mqtt:
                        try:
                            status = monitor.get_json_status()
                            status.update(info)
                            mqtt_client.publish(
                                args.mqtt_topic,
                                json.dumps(status),
                                qos=1,
                                retain=True
                            )
                            last_mqtt_publish = current_time
                            if not args.quiet:
                                print(f"   ✅ Published to {args.mqtt_topic}")
                                sys.stdout.flush()
                        except Exception as e:
                            print(f"   ❌ MQTT publish failed: {e}", file=sys.stderr)
                            sys.stderr.flush()
                    
                    # Alert on state change
                    if ready != last_ready and last_ready is not None:
                        if ready:
                            print(f"\n✅ HEAT SOAK READY! ({monitor.smoothed_temp:.1f}°C @ {monitor.rate_per_minute:.3f}°C/min)\n")
                        else:
                            print(f"\n⚠️  Heat soak waiting...\n")
                    
                    last_ready = ready
                else:
                    if not args.quiet and not args.json:
                        print("⏳ Waiting for heat stable...")
                
                time.sleep(args.interval)
                
        except KeyboardInterrupt:
            if not args.quiet:
                print("\n\n✅ Monitoring stopped")
        
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

