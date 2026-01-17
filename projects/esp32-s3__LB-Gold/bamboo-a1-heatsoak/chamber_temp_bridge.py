#!/usr/bin/env python3
"""
Chamber Temperature Bridge for OrcaSlicer
Reads SHT21 temperature from MQTT and makes it available for OrcaSlicer.

This script:
- Subscribes to SHT21 MQTT temperature readings
- Writes current chamber temperature to a file
- Can be used by OrcaSlicer post-processing scripts or G-code macros
- Optionally sends temperature updates via HTTP API if printer supports it

Usage:
    # Run as background service
    python chamber_temp_bridge.py
    
    # With custom settings
    python chamber_temp_bridge.py --output-file chamber_temp.txt --mqtt-host localhost
"""

import sys
import time
import json
import argparse
from pathlib import Path
from typing import Optional

try:
    import paho.mqtt.client as mqtt
    MQTT_AVAILABLE = True
except ImportError:
    MQTT_AVAILABLE = False
    print("⚠️  paho-mqtt not installed. Install with: uv pip install paho-mqtt")
    sys.exit(1)

import os
import ssl

# ============================================================================
# MQTT AUTHENTICATION HELPERS
# ============================================================================

def get_secrets_file() -> Path:
    """Get secrets file path (same as monitor_heatsoak.py)."""
    return Path.home() / ".secrets"

def get_mqtt_password_from_secrets() -> Optional[str]:
    """Get MQTT password from secrets file or environment (same as monitor_heatsoak.py)."""
    secrets_file = get_secrets_file()
    
    if secrets_file.exists():
        try:
            with open(secrets_file, 'r') as f:
                for line in f:
                    line = line.strip()
                    if line.startswith('MQTT_PASSWORD='):
                        password = line.split('=', 1)[1].strip().strip('"').strip("'")
                        return password if password else None
        except IOError:
            pass
    
    # Check environment variable
    password = os.environ.get('MQTT_PASSWORD')
    return password if password else None

def get_mqtt_username() -> str:
    """Get MQTT username from environment or default to 'mqtt'."""
    return os.environ.get('MQTT_USERNAME', 'mqtt')

def get_mqtt_credentials() -> tuple[str, str]:
    """
    Get MQTT credentials using universal auth (same as monitor_heatsoak.py).
    Returns (username, password) tuple.
    """
    username = get_mqtt_username()
    password = get_mqtt_password_from_secrets()
    
    # If password not found in secrets or environment, use defaults
    if not password:
        # Try common defaults
        if username == "mqtt":
            password = "mqtt"  # Common default: username same as password
        else:
            password = "mqtt"  # Fallback default
    
    return (username, password)

# ============================================================================
# CONFIGURATION DEFAULTS
# ============================================================================

DEFAULT_MQTT_HOST = "localhost"
DEFAULT_MQTT_PORT = 1883
DEFAULT_MQTT_TOPIC = "sensors/sht21/readings"
_default_username, _default_password = get_mqtt_credentials()
DEFAULT_MQTT_USERNAME = _default_username
DEFAULT_MQTT_PASSWORD = _default_password

# Output file for chamber temperature (can be read by OrcaSlicer post-processing)
DEFAULT_OUTPUT_FILE = "chamber_temp.txt"

# ============================================================================
# CHAMBER TEMPERATURE BRIDGE
# ============================================================================

class ChamberTempBridge:
    """Bridge between MQTT temperature readings and OrcaSlicer."""
    
    def __init__(self, mqtt_host: str = "localhost", mqtt_port: int = 1883,
                 mqtt_topic: str = "sensors/sht21/readings",
                 mqtt_username: str = "mqtt", mqtt_password: str = "mqtt",
                 output_file: str = "chamber_temp.txt"):
        """Initialize chamber temperature bridge."""
        self.mqtt_client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2, "chamber-temp-bridge")
        self.mqtt_client.username_pw_set(mqtt_username, mqtt_password)
        self.mqtt_client.on_connect = self._on_mqtt_connect
        self.mqtt_client.on_message = self._on_mqtt_message
        self.mqtt_topic = mqtt_topic
        self.mqtt_host = mqtt_host
        self.mqtt_port = mqtt_port
        
        self.output_file = Path(output_file)
        self.current_temp = None
        self.last_update = None
        self.message_count = 0
        self._connected = False
    
    def _on_mqtt_connect(self, client, userdata, flags, rc, properties=None):
        """MQTT connection callback."""
        if rc == 0:
            self._connected = True
            print(f"✅ Connected to MQTT broker at {self.mqtt_host}:{self.mqtt_port}")
            client.subscribe(self.mqtt_topic)
            print(f"✅ Subscribed to topic: {self.mqtt_topic}")
            print(f"✅ Writing chamber temperature to: {self.output_file}")
            print("")
        else:
            self._connected = False
            error_messages = {
                1: "Connection refused - incorrect protocol version",
                2: "Connection refused - invalid client identifier",
                3: "Connection refused - server unavailable",
                4: "Connection refused - bad username or password",
                5: "Connection refused - not authorised"
            }
            error_msg = error_messages.get(rc, f"Unknown error (code {rc})")
            print(f"❌ Failed to connect to MQTT broker: {error_msg}")
            if rc == 4 or rc == 5:
                print(f"   Check username/password credentials")
                print(f"   Try: --mqtt-username mqtt --mqtt-password mqtt")
    
    def _on_mqtt_message(self, client, userdata, message):
        """MQTT message callback."""
        try:
            self.message_count += 1
            self.last_update = time.time()
            
            payload = json.loads(message.payload.decode())
            temperature = payload.get('temperature')
            
            if temperature is None:
                print("⚠️  No temperature in MQTT message")
                return
            
            # Update current temperature
            self.current_temp = float(temperature)
            
            # Write to output file (format: simple text file with just the temperature)
            try:
                with open(self.output_file, 'w') as f:
                    # Write temperature as integer (most G-code systems expect integers)
                    f.write(str(int(round(self.current_temp))))
                
                # Also write a JSON file with more details (optional, for debugging)
                json_file = self.output_file.with_suffix('.json')
                with open(json_file, 'w') as f:
                    json.dump({
                        'temperature': round(self.current_temp, 2),
                        'timestamp': self.last_update,
                        'message_count': self.message_count
                    }, f, indent=2)
                
                # Print update (throttled to avoid spam)
                if self.message_count % 10 == 1:  # Print every 10th message
                    print(f"🌡️  Chamber temp: {self.current_temp:.1f}°C → {self.output_file}")
                
            except IOError as e:
                print(f"❌ Failed to write temperature file: {e}")
                
        except json.JSONDecodeError as e:
            print(f"⚠️  Failed to parse MQTT message: {e}")
        except Exception as e:
            print(f"❌ Error processing MQTT message: {e}")
    
    def start(self):
        """Start the bridge service."""
        print(f"\n🌡️  Starting Chamber Temperature Bridge...")
        print(f"   MQTT: {self.mqtt_host}:{self.mqtt_port}")
        print(f"   Topic: {self.mqtt_topic}")
        print(f"   Output: {self.output_file}")
        print("   Press Ctrl+C to stop\n")
        
        try:
            print(f"🔌 Connecting to MQTT broker...")
            print("")
            
            self.mqtt_client.connect(self.mqtt_host, self.mqtt_port, 60)
            self.mqtt_client.loop_start()
            
            # Wait for connection
            connection_timeout = 5
            start_time = time.time()
            while not self._connected and (time.time() - start_time) < connection_timeout:
                time.sleep(0.1)
            
            if not self._connected:
                print(f"❌ Failed to connect to MQTT broker within {connection_timeout} seconds")
                print(f"   Check credentials and broker accessibility")
                self.mqtt_client.loop_stop()
                self.mqtt_client.disconnect()
                return
            
            # Wait a bit and check if we're receiving messages
            time.sleep(10)  # Wait 10 seconds
            if self.message_count == 0:
                print("\n⚠️  WARNING: No messages received after 10 seconds!")
                print(f"   Expected topic: {self.mqtt_topic}")
                print(f"   Check:")
                print(f"   1. ESP32 is powered on and connected to WiFi")
                print(f"   2. ESP32 is publishing to topic: {self.mqtt_topic}")
                print(f"   3. MQTT broker is accessible at {self.mqtt_host}:{self.mqtt_port}")
                print(f"\n   Continuing to wait for messages...\n")
            
            # Keep running
            while True:
                time.sleep(1)
                
                # Periodic status update (every 60 seconds)
                if self.last_update and (time.time() - self.last_update) > 300:
                    print(f"⚠️  No temperature updates for {int(time.time() - self.last_update)} seconds")
                    self.last_update = time.time()  # Reset to avoid spam
                    
        except KeyboardInterrupt:
            print("\n\n✅ Bridge stopped")
            print(f"   Total messages received: {self.message_count}")
            if self.current_temp is not None:
                print(f"   Last temperature: {self.current_temp:.1f}°C")
            self.mqtt_client.loop_stop()
            self.mqtt_client.disconnect()
    
    def get_current_temp(self) -> Optional[float]:
        """Get current chamber temperature (for external scripts)."""
        return self.current_temp


def main():
    parser = argparse.ArgumentParser(
        description='Chamber Temperature Bridge for OrcaSlicer',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Basic usage (uses defaults)
  python chamber_temp_bridge.py
  
  # Custom output file location
  python chamber_temp_bridge.py --output-file /path/to/chamber_temp.txt
  
  # Custom MQTT settings
  python chamber_temp_bridge.py --mqtt-host 192.168.1.50 --mqtt-port 1883
  
  # Run as background service (Windows PowerShell)
  Start-Process python -ArgumentList "chamber_temp_bridge.py" -WindowStyle Hidden
        """
    )
    
    # MQTT settings
    parser.add_argument('--mqtt-host', type=str, default=DEFAULT_MQTT_HOST,
                       help=f'MQTT broker host (default: {DEFAULT_MQTT_HOST})')
    parser.add_argument('--mqtt-port', type=int, default=DEFAULT_MQTT_PORT,
                       help=f'MQTT broker port (default: {DEFAULT_MQTT_PORT})')
    parser.add_argument('--mqtt-topic', type=str, default=DEFAULT_MQTT_TOPIC,
                       help=f'MQTT topic for SHT21 readings (default: {DEFAULT_MQTT_TOPIC})')
    parser.add_argument('--mqtt-username', type=str, default=DEFAULT_MQTT_USERNAME,
                       help=f'MQTT username (default: {DEFAULT_MQTT_USERNAME})')
    parser.add_argument('--mqtt-password', type=str, default=DEFAULT_MQTT_PASSWORD,
                       help=f'MQTT password (default: {DEFAULT_MQTT_PASSWORD})')
    
    # Output settings
    parser.add_argument('--output-file', type=str, default=DEFAULT_OUTPUT_FILE,
                       help=f'Output file for chamber temperature (default: {DEFAULT_OUTPUT_FILE})')
    
    args = parser.parse_args()
    
    if not MQTT_AVAILABLE:
        print("❌ MQTT not available. Install with: uv pip install paho-mqtt")
        sys.exit(1)
    
    bridge = ChamberTempBridge(
        mqtt_host=args.mqtt_host,
        mqtt_port=args.mqtt_port,
        mqtt_topic=args.mqtt_topic,
        mqtt_username=args.mqtt_username,
        mqtt_password=args.mqtt_password,
        output_file=args.output_file
    )
    
    bridge.start()


if __name__ == '__main__':
    main()
