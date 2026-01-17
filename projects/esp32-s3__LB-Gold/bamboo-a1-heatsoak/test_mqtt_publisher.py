#!/usr/bin/env python3
"""
Test MQTT Publisher for SHT21 Sensor Simulation
Publishes fake temperature data to MQTT broker for testing heatsoak monitor.

This script simulates an SHT21 sensor publishing temperature readings,
allowing you to test the heatsoak monitor without the actual ESP32-S3.

Usage:
    # Simulate gradual temperature rise (for heatsoak testing)
    python test_mqtt_publisher.py --mqtt-host localhost --simulate-rise
    
    # Publish single reading
    python test_mqtt_publisher.py --mqtt-host localhost --temp 25.5 --humidity 50.0
    
    # Simulate heatsoak scenario (temp rises then plateaus)
    python test_mqtt_publisher.py --mqtt-host localhost --simulate-heatsoak
"""

import sys
import time
import json
import argparse
import math

try:
    import paho.mqtt.client as mqtt
    MQTT_AVAILABLE = True
except ImportError:
    MQTT_AVAILABLE = False
    print("⚠️  paho-mqtt not installed. Install with: uv pip install paho-mqtt")
    sys.exit(1)


class SHT21Simulator:
    """Simulates SHT21 sensor publishing temperature data."""
    
    def __init__(self, mqtt_host="localhost", mqtt_port=1883, topic="sensors/sht21/readings",
                 mqtt_username="mqtt", mqtt_password="mqtt"):
        """Initialize MQTT client."""
        self.mqtt_client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2, "sht21-simulator")
        self.mqtt_client.username_pw_set(mqtt_username, mqtt_password)
        self.mqtt_host = mqtt_host
        self.mqtt_port = mqtt_port
        self.topic = topic
        self.connected = False
        
        self.mqtt_client.on_connect = self._on_connect
    
    def _on_connect(self, client, userdata, flags, rc, properties=None):
        """MQTT connection callback."""
        if rc == 0:
            self.connected = True
            print(f"✅ Connected to MQTT broker at {self.mqtt_host}:{self.mqtt_port}")
        else:
            print(f"❌ Failed to connect to MQTT broker, return code {rc}")
    
    def connect(self):
        """Connect to MQTT broker."""
        try:
            self.mqtt_client.connect(self.mqtt_host, self.mqtt_port)
            self.mqtt_client.loop_start()
            time.sleep(0.5)  # Wait for connection
            return self.connected
        except Exception as e:
            print(f"❌ Connection error: {e}")
            return False
    
    def publish(self, temperature: float, humidity: float = 50.0):
        """Publish temperature and humidity reading."""
        if not self.connected:
            print("⚠️  Not connected to MQTT broker")
            return False
        
        payload = {
            "temperature": round(temperature, 2),
            "humidity": round(humidity, 2),
            "timestamp": int(time.time())
        }
        
        try:
            result = self.mqtt_client.publish(
                self.topic,
                json.dumps(payload),
                qos=1
            )
            
            if result.rc == mqtt.MQTT_ERR_SUCCESS:
                print(f"📤 Published: {payload['temperature']}°C, {payload['humidity']}%")
                return True
            else:
                print(f"❌ Publish failed: {result.rc}")
                return False
        except Exception as e:
            print(f"❌ Publish error: {e}")
            return False
    
    def simulate_rise(self, start_temp=20.0, end_temp=50.0, duration=300, interval=1.0):
        """
        Simulate gradual temperature rise.
        
        Args:
            start_temp: Starting temperature (°C)
            end_temp: Ending temperature (°C)
            duration: Duration in seconds
            interval: Publish interval in seconds
        """
        print(f"\n🌡️  Simulating temperature rise: {start_temp}°C → {end_temp}°C")
        print(f"   Duration: {duration}s, Interval: {interval}s")
        print(f"   Publishing to: {self.topic}\n")
        
        steps = int(duration / interval)
        temp_step = (end_temp - start_temp) / steps
        
        for i in range(steps + 1):
            current_temp = start_temp + (temp_step * i)
            # Add small random variation
            import random
            current_temp += random.uniform(-0.2, 0.2)
            
            self.publish(current_temp)
            time.sleep(interval)
    
    def simulate_heatsoak(self, start_temp=20.0, target_temp=45.0, ramp_time=180, 
                          plateau_time=120, interval=1.0):
        """
        Simulate heatsoak scenario: temperature rises, then plateaus.
        
        Args:
            start_temp: Starting temperature (°C)
            target_temp: Target/plateau temperature (°C)
            ramp_time: Time to reach target (seconds)
            plateau_time: Time to hold at target (seconds)
            interval: Publish interval in seconds
        """
        print(f"\n🔥 Simulating heatsoak scenario:")
        print(f"   Start: {start_temp}°C → Target: {target_temp}°C ({ramp_time}s)")
        print(f"   Plateau: {target_temp}°C for {plateau_time}s")
        print(f"   Publishing to: {self.topic}\n")
        
        # Ramp phase
        ramp_steps = int(ramp_time / interval)
        temp_step = (target_temp - start_temp) / ramp_steps
        
        for i in range(ramp_steps + 1):
            current_temp = start_temp + (temp_step * i)
            # Add small random variation
            import random
            current_temp += random.uniform(-0.1, 0.1)
            
            self.publish(current_temp)
            time.sleep(interval)
        
        # Plateau phase (with very small variations to simulate rate calculation)
        plateau_steps = int(plateau_time / interval)
        for i in range(plateau_steps):
            # Very small variations around target (simulates rate < 0.1°C/min)
            import random
            variation = random.uniform(-0.05, 0.05)
            current_temp = target_temp + variation
            
            self.publish(current_temp)
            time.sleep(interval)
        
        print(f"\n✅ Heatsoak simulation complete")
    
    def simulate_sine_wave(self, base_temp=25.0, amplitude=5.0, period=60.0, 
                          duration=300, interval=1.0):
        """
        Simulate sine wave temperature pattern.
        
        Args:
            base_temp: Base temperature (°C)
            amplitude: Temperature variation amplitude (°C)
            period: Wave period in seconds
            duration: Total duration in seconds
            interval: Publish interval in seconds
        """
        print(f"\n🌊 Simulating sine wave temperature pattern:")
        print(f"   Base: {base_temp}°C, Amplitude: ±{amplitude}°C")
        print(f"   Period: {period}s, Duration: {duration}s")
        print(f"   Publishing to: {self.topic}\n")
        
        steps = int(duration / interval)
        
        for i in range(steps + 1):
            t = i * interval
            # Sine wave: base + amplitude * sin(2π * t / period)
            current_temp = base_temp + amplitude * math.sin(2 * math.pi * t / period)
            
            self.publish(current_temp)
            time.sleep(interval)
    
    def disconnect(self):
        """Disconnect from MQTT broker."""
        self.mqtt_client.loop_stop()
        self.mqtt_client.disconnect()
        print("\n✅ Disconnected from MQTT broker")


def main():
    parser = argparse.ArgumentParser(
        description='SHT21 MQTT Publisher Simulator',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Simulate gradual temperature rise
  python test_mqtt_publisher.py --simulate-rise
  
  # Simulate heatsoak scenario (ramp then plateau)
  python test_mqtt_publisher.py --simulate-heatsoak
  
  # Publish single reading
  python test_mqtt_publisher.py --temp 25.5 --humidity 50.0
  
  # Custom MQTT broker
  python test_mqtt_publisher.py --mqtt-host 192.168.1.50 --simulate-rise
        """
    )
    
    # MQTT settings
    parser.add_argument('--mqtt-host', default='localhost',
                       help='MQTT broker host (default: localhost)')
    parser.add_argument('--mqtt-port', type=int, default=1883,
                       help='MQTT broker port (default: 1883)')
    parser.add_argument('--topic', default='sensors/sht21/readings',
                       help='MQTT topic (default: sensors/sht21/readings)')
    parser.add_argument('--mqtt-username', default='mqtt',
                       help='MQTT username (default: mqtt)')
    parser.add_argument('--mqtt-password', default='mqtt',
                       help='MQTT password (default: mqtt)')
    
    # Single reading
    parser.add_argument('--temp', type=float, default=None,
                       help='Single temperature reading (°C)')
    parser.add_argument('--humidity', type=float, default=50.0,
                       help='Humidity value (default: 50.0%%)')
    
    # Simulation modes
    parser.add_argument('--simulate-rise', action='store_true',
                       help='Simulate gradual temperature rise')
    parser.add_argument('--simulate-heatsoak', action='store_true',
                       help='Simulate heatsoak scenario (ramp then plateau)')
    parser.add_argument('--simulate-sine', action='store_true',
                       help='Simulate sine wave temperature pattern')
    
    # Simulation parameters
    parser.add_argument('--start-temp', type=float, default=20.0,
                       help='Starting temperature for simulations (°C)')
    parser.add_argument('--end-temp', type=float, default=50.0,
                       help='Ending temperature for rise simulation (°C)')
    parser.add_argument('--target-temp', type=float, default=45.0,
                       help='Target temperature for heatsoak simulation (°C)')
    parser.add_argument('--duration', type=float, default=300.0,
                       help='Simulation duration in seconds (default: 300)')
    parser.add_argument('--interval', type=float, default=1.0,
                       help='Publish interval in seconds (default: 1.0)')
    parser.add_argument('--ramp-time', type=float, default=180.0,
                       help='Ramp time for heatsoak simulation (seconds)')
    parser.add_argument('--plateau-time', type=float, default=120.0,
                       help='Plateau time for heatsoak simulation (seconds)')
    
    args = parser.parse_args()
    
    if not MQTT_AVAILABLE:
        print("❌ MQTT not available. Install with: uv pip install paho-mqtt")
        sys.exit(1)
    
    simulator = SHT21Simulator(
        mqtt_host=args.mqtt_host,
        mqtt_port=args.mqtt_port,
        topic=args.topic,
        mqtt_username=args.mqtt_username,
        mqtt_password=args.mqtt_password
    )
    
    if not simulator.connect():
        print("❌ Failed to connect to MQTT broker")
        sys.exit(1)
    
    try:
        if args.simulate_rise:
            simulator.simulate_rise(
                start_temp=args.start_temp,
                end_temp=args.end_temp,
                duration=args.duration,
                interval=args.interval
            )
        elif args.simulate_heatsoak:
            simulator.simulate_heatsoak(
                start_temp=args.start_temp,
                target_temp=args.target_temp,
                ramp_time=args.ramp_time,
                plateau_time=args.plateau_time,
                interval=args.interval
            )
        elif args.simulate_sine:
            simulator.simulate_sine_wave(
                base_temp=args.start_temp,
                amplitude=5.0,
                period=60.0,
                duration=args.duration,
                interval=args.interval
            )
        elif args.temp is not None:
            simulator.publish(args.temp, args.humidity)
            time.sleep(0.5)  # Wait for publish to complete
        else:
            print("❌ No action specified. Use --simulate-rise, --simulate-heatsoak, --simulate-sine, or --temp")
            parser.print_help()
            sys.exit(1)
    
    except KeyboardInterrupt:
        print("\n\n⚠️  Interrupted by user")
    
    finally:
        simulator.disconnect()


if __name__ == '__main__':
    main()
