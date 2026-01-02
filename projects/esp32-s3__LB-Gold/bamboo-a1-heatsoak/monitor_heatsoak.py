#!/usr/bin/env python3
"""
Bamboo Labs A1 Heat Soak Monitor
Monitors SHT21 temperature via MQTT and resumes printer when heatsoak threshold is met.

This script:
- Subscribes to SHT21 MQTT temperature readings
- Applies BME680-style smoothing and rate calculations
- Detects heatsoak readiness based on temperature thresholds
- Sends resume command to Bamboo Labs A1 printer via HTTP API

Usage:
    python monitor_heatsoak.py --printer-ip 192.168.1.100 --mqtt-host localhost
"""

import sys
import time
import json
import argparse
from pathlib import Path
from collections import deque
from typing import Optional

try:
    import paho.mqtt.client as mqtt
    MQTT_AVAILABLE = True
except ImportError:
    MQTT_AVAILABLE = False
    print("⚠️  paho-mqtt not installed. Install with: uv pip install paho-mqtt")
    sys.exit(1)

try:
    import requests
    REQUESTS_AVAILABLE = True
except ImportError:
    REQUESTS_AVAILABLE = False
    print("⚠️  requests not installed. Install with: uv pip install requests")
    sys.exit(1)


class TemperatureMonitor:
    """Temperature monitor with heatsoak detection using BME680-style calculations."""
    
    def __init__(self, temp_smooth_time=4.0, rate_smooth_time=30.0):
        """Initialize temperature smoothing windows."""
        self.temp_smooth_time = int(temp_smooth_time)
        self.rate_smooth_time = int(rate_smooth_time)
        
        # Temperature history buffers
        self.temp_history = deque(maxlen=self.temp_smooth_time + 1)
        self.smoothed_temp_history = deque(maxlen=self.rate_smooth_time + 1)
        
        # Current readings
        self.current_temp = None
        self.smoothed_temp = None
        self.rate_per_minute = None
        
        # Heatsoak tracking
        self.soak_started = False
        self.max_rate_since_soak_start = None
        self.initial_soak_temp = None
        
        # Timestamps for rate calculation
        self.temp_timestamps = deque(maxlen=self.rate_smooth_time + 1)
    
    def add_reading(self, temperature: float, timestamp: Optional[float] = None):
        """Add a new temperature reading."""
        if timestamp is None:
            timestamp = time.time()
        
        self.current_temp = temperature
        
        # Add to raw temp history
        self.temp_history.append(temperature)
        
        # Calculate smoothed temp (simple average)
        if len(self.temp_history) > self.temp_smooth_time:
            # Remove oldest to maintain window
            if len(self.temp_history) > self.temp_smooth_time + 1:
                self.temp_history.popleft()
            
            # Calculate average
            self.smoothed_temp = sum(self.temp_history) / len(self.temp_history)
            
            # Add to smoothed history for rate calculation
            self.smoothed_temp_history.append(self.smoothed_temp)
            self.temp_timestamps.append(timestamp)
            
            # Remove oldest if window full
            if len(self.smoothed_temp_history) > self.rate_smooth_time + 1:
                self.smoothed_temp_history.popleft()
                self.temp_timestamps.popleft()
            
            # Calculate rate using least squares linear regression
            if len(self.smoothed_temp_history) > self.rate_smooth_time:
                self.rate_per_minute = self._calculate_rate()
    
    def _calculate_rate(self) -> float:
        """Calculate rate of temperature change using least squares linear regression."""
        if len(self.smoothed_temp_history) <= 1:
            return 0.0
        
        # Use timestamps for accurate rate calculation
        times = [(ts - self.temp_timestamps[0]) for ts in self.temp_timestamps]
        temps = list(self.smoothed_temp_history)
        
        count = len(times)
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
    
    def check_heat_soak_ready(self, rate_start_type="absolute", rate_start_temp=40.0, 
                              rate_change_plateau=0.1, target_temp=None) -> tuple[bool, dict]:
        """Check if heatsoak conditions are met (BME680-style logic)."""
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
        if ready:
            if not self.soak_started:
                # Starting a new soak cycle - reset tracking and store initial temp (for offset mode)
                self.soak_started = True
                self.initial_soak_temp = self.smoothed_temp
                self.max_rate_since_soak_start = abs(self.rate_per_minute) if self.rate_per_minute is not None else None
        else:
            # Not ready - stop soaking
            if self.soak_started:
                self.soak_started = False
                self.initial_soak_temp = None
                self.max_rate_since_soak_start = None
        
        # Update max_rate_since_soak_start
        if self.soak_started and self.rate_per_minute is not None:
            current_abs_rate = abs(self.rate_per_minute)
            if self.max_rate_since_soak_start is None or current_abs_rate > self.max_rate_since_soak_start:
                self.max_rate_since_soak_start = current_abs_rate
        
        return ready, {
            'ready': ready,
            'temp_ok': temp_ok,
            'rate_ok': rate_ok,
            'soak_started': self.soak_started,
            'max_rate_since_soak_start': round(self.max_rate_since_soak_start, 3) if self.max_rate_since_soak_start is not None else None,
            'smoothed_temp': round(self.smoothed_temp, 2),
            'rate_per_minute': self.rate_per_minute,
            'current_temp': round(self.current_temp, 2) if self.current_temp is not None else None
        }


class BambooLabPrinter:
    """Bamboo Labs A1 printer controller via HTTP API."""
    
    def __init__(self, ip_address: str, port: int = 80):
        """
        Initialize printer controller.
        
        Args:
            ip_address: Printer IP address
            port: HTTP port (default 80)
        """
        self.base_url = f"http://{ip_address}:{port}"
        self.timeout = 5  # seconds
    
    def resume(self) -> bool:
        """
        Resume paused print.
        
        Returns:
            True if successful, False otherwise
        """
        try:
            # Bamboo Labs printers typically use HTTP API
            # Try common endpoints - adjust based on your printer's API
            endpoints = [
                "/api/v1/printer/print/resume",  # Common API endpoint
                "/api/printer/print/resume",     # Alternative
                "/command",                       # Generic command endpoint
            ]
            
            for endpoint in endpoints:
                try:
                    response = requests.post(
                        f"{self.base_url}{endpoint}",
                        json={"command": "M24"},  # M24 is resume command
                        timeout=self.timeout
                    )
                    if response.status_code == 200:
                        print(f"✅ Resume command sent successfully via {endpoint}")
                        return True
                except requests.exceptions.RequestException:
                    continue
            
            # If HTTP API doesn't work, try sending raw G-code
            # This may require printer-specific configuration
            print("⚠️  HTTP API endpoints failed, trying alternative methods...")
            print("   Note: You may need to configure printer API access")
            return False
            
        except Exception as e:
            print(f"❌ Failed to send resume command: {e}")
            return False
    
    def send_gcode(self, gcode: str) -> bool:
        """
        Send raw G-code command to printer.
        
        Args:
            gcode: G-code command (e.g., "M24")
            
        Returns:
            True if successful, False otherwise
        """
        try:
            endpoints = [
                "/api/v1/printer/command",
                "/api/printer/command",
                "/command",
            ]
            
            for endpoint in endpoints:
                try:
                    response = requests.post(
                        f"{self.base_url}{endpoint}",
                        json={"command": gcode},
                        timeout=self.timeout
                    )
                    if response.status_code == 200:
                        print(f"✅ G-code '{gcode}' sent successfully via {endpoint}")
                        return True
                except requests.exceptions.RequestException:
                    continue
            
            return False
            
        except Exception as e:
            print(f"❌ Failed to send G-code: {e}")
            return False


class HeatSoakMonitor:
    """Main heatsoak monitoring class."""
    
    def __init__(self, printer_ip: str, mqtt_host: str = "localhost", 
                 mqtt_port: int = 1883, mqtt_topic: str = "sensors/sht21/readings",
                 rate_start_type: str = "absolute", rate_start_temp: float = 40.0,
                 rate_change_plateau: float = 0.1, target_temp: Optional[float] = None,
                 temp_smooth: float = 4.0, rate_smooth: float = 30.0):
        """Initialize heatsoak monitor."""
        self.printer = BambooLabPrinter(printer_ip)
        self.monitor = TemperatureMonitor(temp_smooth, rate_smooth)
        
        self.rate_start_type = rate_start_type
        self.rate_start_temp = rate_start_temp
        self.rate_change_plateau = rate_change_plateau
        self.target_temp = target_temp
        
        # MQTT setup
        self.mqtt_client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2, "bamboo-a1-heatsoak")
        self.mqtt_client.on_connect = self._on_mqtt_connect
        self.mqtt_client.on_message = self._on_mqtt_message
        self.mqtt_topic = mqtt_topic
        self.mqtt_host = mqtt_host
        self.mqtt_port = mqtt_port
        
        self.resumed = False
    
    def _on_mqtt_connect(self, client, userdata, flags, rc, properties=None):
        """MQTT connection callback."""
        if rc == 0:
            print(f"✅ Connected to MQTT broker at {self.mqtt_host}:{self.mqtt_port}")
            client.subscribe(self.mqtt_topic)
            print(f"✅ Subscribed to topic: {self.mqtt_topic}")
        else:
            print(f"❌ Failed to connect to MQTT broker, return code {rc}")
    
    def _on_mqtt_message(self, client, userdata, message):
        """MQTT message callback."""
        try:
            payload = json.loads(message.payload.decode())
            temperature = payload.get('temperature')
            
            if temperature is None:
                print("⚠️  No temperature in MQTT message")
                return
            
            # Add reading to monitor
            self.monitor.add_reading(temperature)
            
            # Check heatsoak status
            ready, info = self.monitor.check_heat_soak_ready(
                self.rate_start_type,
                self.rate_start_temp,
                self.rate_change_plateau,
                self.target_temp
            )
            
            # Display status
            icon = "✅" if ready else "⏳"
            rate_str = f"{info.get('rate_per_minute', 0):.3f}" if info.get('rate_per_minute') is not None else "---"
            smoothed_str = f"{info.get('smoothed_temp', 0):.2f}" if info.get('smoothed_temp') is not None else "---"
            current_str = f"{info.get('current_temp', 0):.2f}" if info.get('current_temp') is not None else "---"
            
            print(f"{icon} Temp: {current_str}°C | Smoothed: {smoothed_str}°C | "
                  f"Rate: {rate_str}°C/min | Ready: {ready}")
            
            # Resume printer if ready and not already resumed
            if ready and not self.resumed:
                print("\n🔥 HEATSOAK READY! Resuming printer...")
                if self.printer.resume():
                    self.resumed = True
                    print("✅ Printer resume command sent successfully")
                else:
                    # Try sending M24 G-code directly
                    print("   Trying alternative method: sending M24 G-code...")
                    if self.printer.send_gcode("M24"):
                        self.resumed = True
                        print("✅ Printer resume command sent successfully")
                    else:
                        print("❌ Failed to resume printer. Please resume manually.")
                print()
                
        except json.JSONDecodeError as e:
            print(f"⚠️  Failed to parse MQTT message: {e}")
        except Exception as e:
            print(f"❌ Error processing MQTT message: {e}")
    
    def start(self):
        """Start monitoring."""
        print(f"\n🌡️  Starting heatsoak monitoring...")
        print(f"   MQTT: {self.mqtt_host}:{self.mqtt_port}")
        print(f"   Topic: {self.mqtt_topic}")
        print(f"   Printer: {self.printer.base_url}")
        print(f"   Heatsoak: rate_start_type={self.rate_start_type}, "
              f"rate_start_temp={self.rate_start_temp}°C, "
              f"plateau={self.rate_change_plateau}°C/min", end="")
        if self.target_temp is not None:
            print(f", target={self.target_temp}°C")
        else:
            print()
        print("   Press Ctrl+C to stop\n")
        
        try:
            self.mqtt_client.connect(self.mqtt_host, self.mqtt_port)
            self.mqtt_client.loop_forever()
        except KeyboardInterrupt:
            print("\n\n✅ Monitoring stopped")
            self.mqtt_client.disconnect()


def main():
    parser = argparse.ArgumentParser(
        description='Bamboo Labs A1 Heat Soak Monitor',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Basic usage
  python monitor_heatsoak.py --printer-ip 192.168.1.100
  
  # Custom thresholds
  python monitor_heatsoak.py --printer-ip 192.168.1.100 --rate-start-temp 45 --rate-change-plateau 0.15
  
  # With target temperature
  python monitor_heatsoak.py --printer-ip 192.168.1.100 --target-temp 50.0
        """
    )
    
    # Printer settings
    parser.add_argument('--printer-ip', required=True,
                       help='Bamboo Labs A1 printer IP address')
    parser.add_argument('--printer-port', type=int, default=80,
                       help='Printer HTTP port (default: 80)')
    
    # MQTT settings
    parser.add_argument('--mqtt-host', default='localhost',
                       help='MQTT broker host (default: localhost)')
    parser.add_argument('--mqtt-port', type=int, default=1883,
                       help='MQTT broker port (default: 1883)')
    parser.add_argument('--mqtt-topic', default='sensors/sht21/readings',
                       help='MQTT topic for SHT21 readings (default: sensors/sht21/readings)')
    
    # Heatsoak settings
    parser.add_argument('--rate-start-type', type=str, default='absolute', 
                       choices=['offset', 'absolute'],
                       help='Rate start type: "offset" (adds to initial soak temp) or "absolute" (uses as-is)')
    parser.add_argument('--rate-start-temp', type=float, default=40.0,
                       help='Temperature to start checking rate - prevents false positives during ramp-up (°C)')
    parser.add_argument('--rate-change-plateau', type=float, default=0.1,
                       help='Maximum rate of change threshold (°C/min) - indicates diminishing returns')
    parser.add_argument('--target-temp', type=float, default=None,
                       help='Target temperature - if reached, automatically ready (°C)')
    
    # Smoothing settings
    parser.add_argument('--temp-smooth', type=float, default=4.0,
                       help='Temperature smoothing window, seconds (default: 4.0)')
    parser.add_argument('--rate-smooth', type=float, default=30.0,
                       help='Rate smoothing window, seconds (default: 30.0)')
    
    args = parser.parse_args()
    
    if not MQTT_AVAILABLE:
        print("❌ MQTT not available. Install with: uv pip install paho-mqtt")
        sys.exit(1)
    
    if not REQUESTS_AVAILABLE:
        print("❌ Requests not available. Install with: uv pip install requests")
        sys.exit(1)
    
    monitor = HeatSoakMonitor(
        printer_ip=args.printer_ip,
        mqtt_host=args.mqtt_host,
        mqtt_port=args.mqtt_port,
        mqtt_topic=args.mqtt_topic,
        rate_start_type=args.rate_start_type,
        rate_start_temp=args.rate_start_temp,
        rate_change_plateau=args.rate_change_plateau,
        target_temp=args.target_temp,
        temp_smooth=args.temp_smooth,
        rate_smooth=args.rate_smooth
    )
    
    monitor.start()


if __name__ == '__main__':
    main()
