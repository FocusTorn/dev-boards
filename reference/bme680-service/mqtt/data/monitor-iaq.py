#!/usr/bin/env python3
"""
BME680 IAQ Monitor for 3D Printer Enclosure
Continuous monitoring service that publishes to MQTT for Home Assistant integration

Features:
- One-time baseline calibration on startup
- Continuous IAQ monitoring
- Heat stable detection (only publishes valid readings)
- MQTT integration for Home Assistant
- "Safe to open enclosure" threshold detection

Usage:
    # Calibrate baseline (do this once, when enclosure is at normal conditions)
    uv run python monitor-iaq.py --calibrate 90
    
    # Start monitoring service
    uv run python monitor-iaq.py --monitor --mqtt
    
    # Or as systemd service (continuous)
    monitor-iaq.py --calibrate 90 --monitor --mqtt --daemon
"""

import sys
import time
import json
import argparse
from pathlib import Path
from dataclasses import dataclass, asdict

# Add monitor to path
sys.path.insert(0, str(Path(__file__).parent.parent))
from monitor import BME680, I2C_ADDR_SECONDARY

try:
    import paho.mqtt.client as mqtt
    MQTT_AVAILABLE = True
except ImportError:
    MQTT_AVAILABLE = False
    print("⚠️  paho-mqtt not installed. MQTT features disabled.")
    print("   Install with: uv pip install paho-mqtt")


@dataclass
class IAQState:
    """Current IAQ monitoring state."""
    baseline_gas: float = None
    baseline_hum: float = None
    current_gas: float = None
    current_hum: float = None
    temperature: float = None
    pressure: float = None
    air_quality_score: float = None
    heat_stable: bool = False
    is_safe: bool = None


class IAQMonitor:
    """BME680 IAQ monitor with enclosure safety checking."""
    
    def __init__(self, i2c_address=0x77, heater_temp=320, heater_duration=150):
        """Initialize BME680 sensor and configure heater."""
        self.sensor = BME680(i2c_address)
        self.sensor.set_gas_heater_temperature(heater_temp)
        self.sensor.set_gas_heater_duration(heater_duration)
        self.sensor.select_gas_heater_profile(0)
        
        self.state = IAQState()
        self.baseline_established = False
        
    def calibrate_baseline(self, burn_in_time=90, verbose=True):
        """Establish baseline in clean air (when not printing)."""
        if verbose:
            print(f"\n📊 Calibrating baseline for {burn_in_time}s...")
            print("   Room should be in normal state (no printing)")
        
        # Call internal set_baselines method
        self.sensor.set_baselines(burn_in_time=burn_in_time, verbose=verbose)
        
        if self.sensor.baseline_status == 1:
            self.state.baseline_gas = self.sensor.get_gas_baseline()
            self.state.baseline_hum = self.sensor.get_hum_baseline()
            self.baseline_established = True
            if verbose:
                print(f"\n✅ Baseline established:")
                print(f"   Gas: {self.state.baseline_gas:.0f} Ohms")
                print(f"   Humidity: {self.state.baseline_hum:.1f}%")
            return True
        else:
            if verbose:
                print("\n❌ Baseline calibration failed")
            return False
    
    def read_current(self) -> bool:
        """Read current sensor values (returns True if successful and stable)."""
        if not self.sensor.get_sensor_data():
            return False
            
        if not self.sensor.data.heat_stable:
            return False
            
        self.state.current_gas = self.sensor.data.gas_resistance
        self.state.current_hum = self.sensor.data.humidity
        self.state.temperature = self.sensor.data.temperature
        self.state.pressure = self.sensor.data.pressure
        self.state.heat_stable = True
        return True
    
    def calculate_iaq_score(self) -> float:
        """Calculate air quality score (higher = better air quality)."""
        if not self.baseline_established:
            return None
            
        if self.state.current_gas is None:
            return None
            
        # From Bosch IAQ calculation
        gas_baseline = self.state.baseline_gas
        hum_baseline = self.state.baseline_hum
        gas = self.state.current_gas
        hum = self.state.current_hum
        hum_weighting = 0.25  # 25% humidity, 75% gas
        
        # Calculate offsets
        gas_offset = gas_baseline - gas
        hum_offset = hum - hum_baseline
        
        # Calculate humidity score
        if hum_offset > 0:
            hum_score = (100 - hum_baseline - hum_offset) / (100 - hum_baseline) * (hum_weighting * 100)
        else:
            hum_score = (hum_baseline + hum_offset) / hum_baseline * (hum_weighting * 100)
        
        # Calculate gas score
        if gas_offset > 0:
            gas_score = (gas / gas_baseline) * (100 - (hum_weighting * 100))
        else:
            gas_score = 100 - (hum_weighting * 100)
        
        air_quality_score = hum_score + gas_score
        self.state.air_quality_score = air_quality_score
        return air_quality_score
    
    def check_safe_to_open(self, threshold=80.0) -> bool:
        """Check if safe to open enclosure based on IAQ threshold."""
        if not self.baseline_established:
            return None
            
        score = self.calculate_iaq_score()
        if score is None:
            return None
            
        is_safe = score >= threshold
        self.state.is_safe = is_safe
        return is_safe
    
    def get_json_status(self) -> dict:
        """Get current status as dictionary."""
        return {
            'baseline_established': self.baseline_established,
            'heat_stable': self.state.heat_stable,
            'current': {
                'gas': self.state.current_gas,
                'humidity': self.state.current_hum,
                'temperature': self.state.temperature,
                'pressure': self.state.pressure,
            },
            'baseline': {
                'gas': self.state.baseline_gas,
                'humidity': self.state.baseline_hum,
            } if self.baseline_established else None,
            'air_quality_score': self.state.air_quality_score,
            'safe_to_open': self.state.is_safe,
        }


def main():
    parser = argparse.ArgumentParser(
        description='BME680 IAQ Monitor for 3D printer enclosures',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Calibrate baseline (do once)
  monitor-iaq.py --calibrate 90
  
  # Monitor with console output
  monitor-iaq.py --monitor --interval 10
  
  # Monitor with MQTT for Home Assistant
  monitor-iaq.py --monitor --mqtt --interval 30
  
  # Systemd service (background)
  monitor-iaq.py --monitor --mqtt --daemon
        """
    )
    
    # Calibration
    parser.add_argument('--calibrate', type=int, default=0, metavar='SECONDS',
                       help='Calibrate baseline (0 = skip). Do this once when enclosure is normal.')
    
    # Monitoring
    parser.add_argument('--monitor', action='store_true',
                       help='Continuous monitoring mode')
    parser.add_argument('--interval', type=int, default=30,
                       help='Monitoring interval in seconds (default: 30)')
    
    # Safety threshold
    parser.add_argument('--threshold', type=float, default=80.0,
                       help='IAQ threshold for "safe to open" (default: 80.0)')
    
    # MQTT
    parser.add_argument('--mqtt', action='store_true',
                       help='Enable MQTT publishing')
    parser.add_argument('--mqtt-host', default='localhost',
                       help='MQTT broker host (default: localhost)')
    parser.add_argument('--mqtt-port', type=int, default=1883,
                       help='MQTT broker port (default: 1883)')
    parser.add_argument('--mqtt-topic', default='homeassistant/sensor/bme680/state',
                       help='MQTT topic (default: homeassistant/sensor/bme680/state)')
    parser.add_argument('--mqtt-client-id', default='bme680-iaq-monitor',
                       help='MQTT client ID (default: bme680-iaq-monitor)')
    
    # Output
    parser.add_argument('--json', action='store_true',
                       help='Output JSON format')
    parser.add_argument('--quiet', action='store_true',
                       help='Suppress console output')
    parser.add_argument('--daemon', action='store_true',
                       help='Daemon mode (background service)')
    
    args = parser.parse_args()
    
    # Daemon mode
    if args.daemon:
        import daemon
        with daemon.DaemonContext():
            args.quiet = True
            _run_monitor(args)
    else:
        _run_monitor(args)


def _run_monitor(args):
    """Main monitor loop."""
    try:
        monitor = IAQMonitor()
        
        # Calibrate if requested
        if args.calibrate:
            if not monitor.calibrate_baseline(burn_in_time=args.calibrate, verbose=not args.quiet):
                sys.exit(1)
        
        if not args.monitor:
            # Single read
            if monitor.read_current():
                status = monitor.get_json_status()
                if args.json:
                    print(json.dumps(status, indent=2))
                else:
                    print_readout(monitor)
            sys.exit(0)
        
        # Monitor loop
        if not args.quiet:
            print(f"\n🔍 Starting IAQ monitoring service...")
            print(f"   Interval: {args.interval}s, Threshold: {args.threshold}")
            print("   Press Ctrl+C to stop\n")
        
        # MQTT setup
        mqtt_client = None
        if args.mqtt:
            if not MQTT_AVAILABLE:
                print("❌ MQTT not available. Install with: uv pip install paho-mqtt")
                sys.exit(1)
            
            mqtt_client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2, args.mqtt_client_id)
            try:
                mqtt_client.connect(args.mqtt_host, args.mqtt_port)
                mqtt_client.loop_start()
                if not args.quiet:
                    print(f"✅ Connected to MQTT: {args.mqtt_host}:{args.mqtt_port}")
            except Exception as e:
                print(f"❌ MQTT connection failed: {e}")
                sys.exit(1)
        
        # Main monitoring loop
        last_safe_state = None
        
        try:
            while True:
                if monitor.read_current():
                    score = monitor.calculate_iaq_score()
                    is_safe = monitor.check_safe_to_open(args.threshold)
                    
                    # Display
                    if args.json:
                        print(json.dumps(monitor.get_json_status()))
                    elif not args.quiet:
                        status_icon = "🟢" if is_safe else "🔴" if is_safe is False else "⚪"
                        print(f"{status_icon} Gas: {monitor.state.current_gas:.0f}Ω | "
                              f"Hum: {monitor.state.current_hum:.1f}% | "
                              f"Temp: {monitor.state.temperature:.1f}°C | "
                              f"IAQ: {score:.1f} | Safe: {is_safe}")
                    
                    # MQTT publish
                    if mqtt_client:
                        status = monitor.get_json_status()
                        mqtt_client.publish(
                            args.mqtt_topic,
                            json.dumps(status),
                            qos=1,
                            retain=True
                        )
                    
                    # Alert on state change
                    if is_safe != last_safe_state and last_safe_state is not None:
                        if is_safe:
                            if not args.quiet:
                                print(f"\n✅ ENCLOSURE SAFE TO OPEN (IAQ: {score:.1f})\n")
                        else:
                            if not args.quiet:
                                print(f"\n⚠️  ENCLOSURE NOT SAFE (IAQ: {score:.1f})\n")
                    
                    last_safe_state = is_safe
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


def print_readout(monitor):
    """Print formatted readout."""
    print("\n📊 Current IAQ Status:")
    print(f"Gas: {monitor.state.current_gas:.0f}Ω")
    print(f"Humidity: {monitor.state.current_hum:.1f}%")
    print(f"Temperature: {monitor.state.temperature:.1f}°C")
    print(f"Pressure: {monitor.state.pressure:.1f} hPa")
    
    if monitor.baseline_established:
        print(f"\nBaseline: Gas={monitor.state.baseline_gas:.0f}Ω, Hum={monitor.state.baseline_hum:.1f}%")
        if monitor.state.air_quality_score:
            print(f"IAQ Score: {monitor.state.air_quality_score:.1f}")
            print(f"Safe to open: {monitor.state.is_safe}")


if __name__ == '__main__':
    main()

