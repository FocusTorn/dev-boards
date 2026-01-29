#!/usr/bin/env python3
"""
Bamboo Labs A1 Heat Soak Monitor
Monitors SHT21 temperature via MQTT and resumes printer when heatsoak threshold is met.

This script:
- Subscribes to SHT21 MQTT temperature readings
- Applies BME680-style smoothing and rate calculations
- Detects heatsoak readiness based on temperature thresholds
- Sends resume command to Bamboo Labs A1 printer via MQTT

Usage:
    # Use defaults (configured at top of script)
    python monitor_heatsoak.py
    
    # Override specific settings
    python monitor_heatsoak.py --printer-ip 192.168.1.163 --mqtt-host localhost
"""

# IMPORTS ------------------->> 

import sys
import time
import json
import argparse
import re
from pathlib import Path
from collections import deque
from typing import Optional, Any

try:
    import paho.mqtt.client as mqtt
    MQTT_AVAILABLE = True
except ImportError:
    MQTT_AVAILABLE = False
    print("⚠️  paho-mqtt not installed. Install with: uv pip install paho-mqtt")
    sys.exit(1)

import ssl
import threading
import os

try:
    import win32gui
    import win32con
    import win32process
    import ctypes
    from ctypes import wintypes
    WIN32_AVAILABLE = True

    # Define FLASHWINFO structure for FlashWindowEx
    class FLASHWINFO(ctypes.Structure):
        _fields_ = [
            ('cbSize', wintypes.UINT),
            ('hwnd', wintypes.HWND),
            ('dwFlags', wintypes.DWORD),
            ('uCount', wintypes.UINT),
            ('dwTimeout', wintypes.DWORD)
        ]
    FLASHW_STOP = 0
except ImportError:
    WIN32_AVAILABLE = False

#---------------------------------------------------------<<
# MQTT HELPERS -------------->> 


def get_secrets_file() -> Path:
    """Get secrets file path (same as mqtt-helper.py)."""
    return Path.home() / ".secrets"

def get_mqtt_password_from_secrets() -> Optional[str]:
    """Get MQTT password from secrets file or environment (same as mqtt-helper.py)."""
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
    Get MQTT credentials using universal auth (same as mqtt-helper.py).
    Returns (username, password) tuple.
    
    Priority:
    1. Secrets file (~/.secrets with MQTT_PASSWORD=...)
    2. Environment variables (MQTT_USERNAME, MQTT_PASSWORD)
    3. Default: mqtt/mqtt
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

#-----------------------------------------------------------------------------<<


# Heatsoak detection settings
DEFAULT_RATE_START_TYPE = "absolute"  # "absolute" or "offset"
DEFAULT_RATE_THRESHOLD_TEMP = 55.0  # °C - minimum temp for rate-based heatsoak detection
DEFAULT_MIN_TEMP_FOR_PLATEAU = 26.0  # °C - minimum temp for plateau check to be valid
DEFAULT_RATE_CHANGE_PLATEAU = 0.1  # °C/min
DEFAULT_TARGET_TEMP = None  # °C (None to disable)

# Smoothing settings
DEFAULT_TEMP_SMOOTH = 4.0  # number of entries (~25 seconds at 5-sec intervals)
DEFAULT_RATE_SMOOTH = 10.0  # number of entries (~65 seconds at 5-sec intervals)

# Temperature maintenance settings
DEFAULT_MAINTAIN_TEMP = None  # °C - Temperature to maintain during pause (None to disable)
DEFAULT_TEMP_COMMAND_INTERVAL = 30.0  # seconds - Interval between temperature maintenance commands

# Cooling settings
DEFAULT_COOLING_THRESHOLD = 35.0  # °C - Bed temperature threshold for "Completed" stage

# Printer MQTT settings
DEFAULT_PRINTER_IP = "192.168.1.163"
DEFAULT_PRINTER_ACCESS_CODE = "27004398"
DEFAULT_PRINTER_DEVICE_ID = "03919D532705945"
DEFAULT_PRINTER_CERT_FILE = "printer_cert.pem"  # Set to None to use insecure mode

# Sensor MQTT settings
DEFAULT_MQTT_HOST = "localhost"
DEFAULT_MQTT_PORT = 1883
DEFAULT_MQTT_TOPIC = "sensors/sht21/readings"

# Get credentials from secrets file (same as mqtt-helper.py)
_default_username, _default_password = get_mqtt_credentials()
DEFAULT_MQTT_USERNAME = _default_username
DEFAULT_MQTT_PASSWORD = _default_password



# ============================================================================



class TemperatureMonitor:
    """Temperature monitor with heatsoak detection using BME680-style calculations."""
    
    def __init__(self, temp_smooth_time=4.0, rate_smooth_time=30.0):
        """
        Initialize temperature smoothing windows.
        
        Args:
            temp_smooth_time: Number of entries for temp smoothing (converted to int, buffer size = value + 1)
            rate_smooth_time: Number of entries for rate smoothing (converted to int, buffer size = value + 1)
        """
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
    
    def get_smoothing_progress(self) -> dict:
        """Get progress indicators for smoothing calculations."""
        # Temp smoothing needs temp_smooth_time + 1 readings
        temp_needed = self.temp_smooth_time + 1
        temp_current = len(self.temp_history)
        temp_ready = temp_current >= temp_needed
        
        # Rate smoothing needs rate_smooth_time + 1 smoothed readings
        # But smoothed readings only start after temp smoothing is ready
        rate_needed = self.rate_smooth_time + 1
        rate_current = len(self.smoothed_temp_history)
        rate_ready = rate_current >= rate_needed
        
        return {
            'temp_current': temp_current,
            'temp_needed': temp_needed,
            'temp_ready': temp_ready,
            'rate_current': rate_current,
            'rate_needed': rate_needed,
            'rate_ready': rate_ready,
        }
    
    def check_heat_soak_ready(self, rate_start_type="absolute", rate_threshold_temp=40.0,
                              min_temp_for_plateau=30.0, rate_change_plateau=0.1, target_temp=None) -> tuple[bool, dict]:
        """Check if heatsoak conditions are met (BME680-style logic)."""
        # Return early if we don't have enough data, but still include available values
        if self.smoothed_temp is None or self.rate_per_minute is None:
            if self.soak_started:
                self.soak_started = False
                self.max_rate_since_soak_start = None
            return False, {
                'ready': False,
                'reason': 'Insufficient data',
                'soak_started': False,
                'max_rate_since_soak_start': None,
                'smoothed_temp': self.smoothed_temp,  # Include even if None
                'rate_per_minute': self.rate_per_minute,  # Include even if None
                'current_temp': self.current_temp,  # Include current temp
            }
        
        # Calculate readiness conditions
        # temp_ok: true when smoothed_temp > target_temp (if target_temp set) OR smoothed_temp > min_temp_for_plateau
        if target_temp is not None:
            temp_ok = self.smoothed_temp > target_temp
        else:
            temp_ok = self.smoothed_temp > min_temp_for_plateau
        
        # Calculate rate threshold based on rate_start_type
        if rate_start_type == "offset":
            # Offset mode: rate_threshold_temp is added to initial temp when soak_started becomes true
            if self.initial_soak_temp is None:
                # Haven't started soaking yet, use absolute comparison
                rate_threshold = rate_threshold_temp
            else:
                # Use offset from initial soak temperature
                rate_threshold = self.initial_soak_temp + rate_threshold_temp
        else:
            # Absolute mode: rate_threshold_temp is used as-is
            rate_threshold = rate_threshold_temp
        
        # rate_ok: true when smoothed_temp > rate_threshold OR (rate plateaus AND temp > min_temp_for_plateau)
        # Either temperature reaches threshold OR rate plateaus (but plateau only counts if temp > min_temp_for_plateau)
        rate_ok = (self.smoothed_temp > rate_threshold or 
                   (abs(self.rate_per_minute) < rate_change_plateau and self.smoothed_temp > min_temp_for_plateau))
        
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
    """Bamboo Labs A1 printer controller via MQTT."""
    
    def __init__(self, ip_address: str, access_code: str, device_id: str, 
                 cert_file: Optional[str] = None, port: int = 8883, verbose: bool = False):
        """
        Initialize printer controller.
        
        Args:
            ip_address: Printer IP address
            access_code: Printer LAN access code (used as MQTT password)
            device_id: Printer device ID (e.g., "03919D532705945")
            cert_file: Path to printer certificate file (printer_cert.pem)
            port: MQTT port (default 8883 for TLS)
            verbose: If True, show verbose output
        """
        self.ip_address = ip_address
        self.access_code = access_code
        self.device_id = device_id
        self.cert_file = cert_file
        self.port = port
        self.verbose = verbose
        self.command_topic = f"device/{device_id}/request"
        self.status_topic = f"device/{device_id}/report"
        self.connected = False
        self.message_sent = False
        self.printer_status = None
        self.status_message_received = False
    
    def _on_connect(self, client, userdata, flags, rc, properties=None):
        """MQTT connection callback (VERSION2 compatible)."""
        if rc == 0:
            self.connected = True
        else:
            self.connected = False
    
    def _on_publish(self, client, userdata, mid, reason_code=None, properties=None):
        """MQTT publish callback (VERSION2 compatible)."""
        self.message_sent = True
    
    def _on_message(self, client, userdata, message):
        """MQTT message callback for status updates."""
        try:
            # Only process messages from report topic (or wildcard device topics)
            topic = message.topic
            if not (topic.endswith("/report") or topic.startswith(f"device/{self.device_id}/")):
                return
            
            payload = json.loads(message.payload.decode())
            
            # Verify this looks like a status message (has print key with gcode_state nested)
            # Store any message from report topic as it's likely a status update
            if "print" in payload:
                print_obj = payload.get("print", {})
                if isinstance(print_obj, dict) and "gcode_state" in print_obj:
                    self.printer_status = payload
                    self.status_message_received = True
                    # Debug: log that we received a status message
                    print(f"   ✓ Status message received from: {topic}")
            elif isinstance(payload, dict):
                # Store anyway if it's a dict - might have different structure
                self.printer_status = payload
                self.status_message_received = True
                print(f"   ✓ Message received from: {topic} (checking structure...)")
        except (json.JSONDecodeError, UnicodeDecodeError) as e:
            # Ignore parse errors, keep existing status
            pass
    
    def is_paused(self, timeout: float = 3.0) -> Optional[bool]:
        """
        Check if printer is currently paused by subscribing to status topic.
        
        Args:
            timeout: Maximum time to wait for status message (seconds)
            
        Returns:
            True if paused, False if running/other state, None if status unknown
        """
        # Use MQTT v3.1.1 (not v5) as required by Bambu Lab printers
        client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2, "bamboo-a1-status-check")
        client.username_pw_set("bblp", self.access_code)
        
        # Setup TLS with certificate (use insecure mode like mosquitto_pub --insecure)
        if self.cert_file and Path(self.cert_file).exists():
            client.tls_set(
                ca_certs=str(Path(self.cert_file).resolve()),
                cert_reqs=ssl.CERT_NONE
            )
            client.tls_insecure_set(True)
        else:
            client.tls_set(cert_reqs=ssl.CERT_NONE)
            client.tls_insecure_set(True)
        
        try:
            self.connected = False
            self.status_message_received = False
            self.printer_status = None
            
            client.on_connect = self._on_connect
            client.on_message = self._on_message
            
            client.connect(self.ip_address, self.port, 60)
            client.loop_start()
            
            # Wait for connection
            connection_timeout = 5
            start_time = time.time()
            while not self.connected and (time.time() - start_time) < connection_timeout:
                time.sleep(0.1)
            
            if not self.connected:
                print("⚠️  Failed to connect to printer for status check")
                client.loop_stop()
                client.disconnect()
                return None
            
            # Subscribe to status topic
            client.subscribe(self.status_topic)
            print(f"   Subscribed to: {self.status_topic}")
            time.sleep(0.5)  # Give subscription time to register
            
            # Also try subscribing to wildcard to catch all device messages (for debugging)
            # Some printers might publish to different topics
            client.subscribe(f"device/{self.device_id}/#")
            time.sleep(0.2)
            
            # Wait for status message
            start_time = time.time()
            while not self.status_message_received and (time.time() - start_time) < timeout:
                time.sleep(0.1)
            
            # Clean up
            client.loop_stop()
            time.sleep(0.2)
            client.disconnect()
            
            if self.printer_status is None:
                print(f"   ⚠️  No status message received within {timeout}s timeout")
                print(f"   Status topic: {self.status_topic}")
                print(f"   The printer might not be publishing status automatically")
                return None
            
            # Debug: Show received status (first time only, can be removed later)
            print(f"   📨 Status received: {json.dumps(self.printer_status)[:200]}...")
            
            # Parse status - check for print state
            # Bambu Lab status can be in different formats, check common fields
            print_status = self.printer_status.get("print", {})
            if not print_status:
                # Try direct access (some formats have gcode_state at root)
                print_status = self.printer_status
            
            gcode_state = print_status.get("gcode_state")
            
            # Common states: "PAUSE", "RUNNING", "FINISH", "IDLE", "PREPARE", etc.
            if gcode_state == "PAUSE":
                return True
            elif gcode_state in ["RUNNING", "FINISH", "IDLE", "PREPARE"]:
                return False
            elif gcode_state is not None:
                # If we got a status but don't recognize the state, log it and return None
                print(f"⚠️  Unknown printer state: {gcode_state}")
                return None
            else:
                # No gcode_state found in status
                return None
                
        except Exception as e:
            print(f"⚠️  Error checking printer status: {e}")
            try:
                client.loop_stop()
                client.disconnect()
            except:
                pass
            return None
    
    def resume(self, check_status: bool = True) -> bool:
        """
        Resume paused print via MQTT.
        
        Args:
            check_status: If True, check if printer is paused before sending resume command
        
        Returns:
            True if successful, False otherwise
        """
        # Use MQTT v3.1.1 (not v5) as required by Bambu Lab printers
        # Updated to VERSION2 to fix deprecation warning
        client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2, "bamboo-a1-resume")
        client.username_pw_set("bblp", self.access_code)
        
        # Setup TLS with certificate (use insecure mode like mosquitto_pub --insecure)
        # This matches the working command: mosquitto_pub --cafile printer_cert.pem --insecure
        if self.cert_file and Path(self.cert_file).exists():
            # Use CA file but disable verification (equivalent to --insecure flag)
            client.tls_set(
                ca_certs=str(Path(self.cert_file).resolve()),
                cert_reqs=ssl.CERT_NONE
            )
            client.tls_insecure_set(True)  # Disable certificate and hostname verification
        else:
            # Fallback: use insecure mode if cert file not found
            client.tls_set(cert_reqs=ssl.CERT_NONE)
            client.tls_insecure_set(True)  # Disable certificate and hostname verification
        
        try:
            self.connected = False
            self.message_sent = False
            self.printer_status = None
            self.status_message_received = False
            
            client.on_connect = self._on_connect
            client.on_publish = self._on_publish
            
            # Set up status message handler if checking status
            if check_status:
                client.on_message = self._on_message
            
            if self.verbose:
                print(f"🔌 Connecting to printer at {self.ip_address}:{self.port}...")
            
            client.connect(self.ip_address, self.port, 60)
            client.loop_start()
            
            # Wait for connection
            timeout = 5
            start_time = time.time()
            while not self.connected and (time.time() - start_time) < timeout:
                time.sleep(0.1)
            
            if not self.connected:
                print(f"❌ Failed to connect to printer MQTT broker at {self.ip_address}")
                client.loop_stop()
                client.disconnect()
                return False
            
            time.sleep(0.3)  # Give connection time to stabilize
            
            # Check printer status if requested
            if check_status:
                print("🔍 Checking printer status...")
                # Subscribe to report topic - printer publishes status automatically
                # No push command needed - just subscribe and wait for automatic updates
                if self.verbose:
                    print(f"   📡 Subscribing to: {self.status_topic}")
                client.subscribe(self.status_topic)
                # Also subscribe to wildcard to catch all device messages
                client.subscribe(f"device/{self.device_id}/#")
                time.sleep(0.5)  # Give subscription time to register
                
                # Wait for status message - printer publishes status updates automatically
                # The printer typically publishes status periodically or on state changes
                print(f"   ⏳ Waiting for status update from printer...")
                status_timeout = 4.0  # Give enough time for printer to publish status
                start_time = time.time()
                while not self.status_message_received and (time.time() - start_time) < status_timeout:
                    time.sleep(0.1)
                
                if self.status_message_received and self.printer_status:
                    # Parse status - check nested structure: data.print.gcode_state
                    print_status = self.printer_status.get("print", {})
                    
                    # Handle different response formats
                    if not print_status and isinstance(self.printer_status, dict):
                        # Try direct access or alternative structure
                        if "print" in self.printer_status:
                            print_status = self.printer_status["print"]
                        else:
                            # Maybe gcode_state is at root level
                            print_status = self.printer_status
                    
                    # Extract gcode_state (can be "PAUSED", "PAUSE", "RUNNING", etc.)
                    gcode_state = None
                    if isinstance(print_status, dict):
                        gcode_state = print_status.get("gcode_state")
                    
                    # Normalize state to uppercase for comparison
                    if gcode_state:
                        gcode_state = str(gcode_state).upper()
                    
                    # Debug: show received status (truncated)
                    if self.verbose:
                        status_preview = json.dumps(self.printer_status)[:200]
                        print(f"   📨 Status received: {status_preview}...")
                    
                    # Check if printer is paused - Bambu Lab uses "PAUSED" (all caps)
                    if gcode_state in ["PAUSED", "PAUSE"]:
                        print(f"✅ Printer is paused (state: {gcode_state}), proceeding with resume...")
                    elif gcode_state in ["RUNNING", "FINISH", "IDLE", "PREPARE"]:
                        print(f"ℹ️  Printer is not paused (current state: {gcode_state})")
                        print("   Skipping resume command.")
                        client.loop_stop()
                        client.disconnect()
                        return False
                    elif gcode_state is not None:
                        print(f"⚠️  Unknown printer state: {gcode_state}, proceeding anyway...")
                    else:
                        print("⚠️  Could not find gcode_state in status response")
                        if self.verbose:
                            print(f"   Available keys in print object: {list(print_status.keys()) if isinstance(print_status, dict) else 'N/A'}")
                        print("   Proceeding with resume command anyway...")
                else:
                    print("ℹ️  No status update received within timeout")
                    print(f"   Printer may not be publishing status automatically, or status update is delayed")
                    print(f"   Proceeding with resume command anyway (printer will ignore if not paused)...")
            
            # Send resume command using the working format
            payload = {
                "print": {
                    "command": "resume",
                    "sequence_id": "0"
                }
            }
            
            if self.verbose:
                print(f"📤 Sending resume command to {self.command_topic}...")
            
            try:
                result = client.publish(self.command_topic, json.dumps(payload), qos=0)
                if result.rc != mqtt.MQTT_ERR_SUCCESS:
                    print(f"❌ Failed to publish resume command, MQTT error code: {result.rc}")
                    client.loop_stop()
                    client.disconnect()
                    return False
            except Exception as publish_error:
                print(f"❌ Exception during publish: {publish_error}")
                client.loop_stop()
                client.disconnect()
                return False
            
            # Wait for publish confirmation
            publish_timeout = 2
            start_time = time.time()
            while not self.message_sent and (time.time() - start_time) < publish_timeout:
                time.sleep(0.1)
            
            if self.message_sent:
                if self.verbose:
                    print("✅ Resume command sent successfully")
                time.sleep(0.5)  # Give printer time to process
                # Disconnect
                client.loop_stop()
                time.sleep(0.2)
                client.disconnect()
                return True
            else:
                print("⚠️  Command queued but publish confirmation timeout")
                # Disconnect
                client.loop_stop()
                time.sleep(0.2)
                client.disconnect()
                return False
                
        except Exception as e:
            print(f"❌ Critical error in resume method: {e}")
            import traceback
            if self.verbose:
                traceback.print_exc()
            try:
                client.loop_stop()
                client.disconnect()
            except:
                pass
            return False
    
    def send_gcode(self, gcode_command: str, check_status: bool = False) -> bool:
        """
        Send G-code command to printer via MQTT.
        
        Args:
            gcode_command: G-code command to send (e.g., "M104 S180")
            check_status: If True, verify printer is paused before sending
        
        Returns:
            True if successful, False otherwise
        """
        # Use MQTT v3.1.1 (not v5) as required by Bambu Lab printers
        client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2, "bamboo-a1-gcode")
        client.username_pw_set("bblp", self.access_code)
        
        # Setup TLS with certificate
        if self.cert_file and Path(self.cert_file).exists():
            client.tls_set(
                ca_certs=str(Path(self.cert_file).resolve()),
                cert_reqs=ssl.CERT_NONE
            )
            client.tls_insecure_set(True)
        else:
            client.tls_set(cert_reqs=ssl.CERT_NONE)
            client.tls_insecure_set(True)
        
        try:
            self.connected = False
            self.message_sent = False
            
            client.on_connect = self._on_connect
            client.on_publish = self._on_publish
            
            client.connect(self.ip_address, self.port, 60)
            client.loop_start()
            
            # Wait for connection
            timeout = 5
            start_time = time.time()
            while not self.connected and (time.time() - start_time) < timeout:
                time.sleep(0.1)
            
            if not self.connected:
                if self.verbose:
                    print(f"❌ Failed to connect to printer for G-code command")
                client.loop_stop()
                client.disconnect()
                return False
            
            # Send G-code command
            payload = {
                "print": {
                    "command": "gcode_line",
                    "param": gcode_command,
                    "sequence_id": "0"
                }
            }
            
            if self.verbose:
                print(f"📤 Sending G-code command: {gcode_command}")
            result = client.publish(self.command_topic, json.dumps(payload), qos=0)
            
            if result.rc == mqtt.MQTT_ERR_SUCCESS:
                # Wait for publish confirmation
                publish_timeout = 2
                start_time = time.time()
                while not self.message_sent and (time.time() - start_time) < publish_timeout:
                    time.sleep(0.1)
                
                if self.message_sent:
                    if self.verbose:
                        print(f"✅ G-code command sent successfully")
                    time.sleep(0.2)  # Give printer time to process
                    client.loop_stop()
                    time.sleep(0.2)
                    client.disconnect()
                    return True
                else:
                    if self.verbose:
                        print("⚠️  Command queued but publish confirmation timeout")
                    client.loop_stop()
                    client.disconnect()
                    return False
            else:
                if self.verbose:
                    print(f"❌ Failed to publish G-code command, return code: {result.rc}")
                client.loop_stop()
                client.disconnect()
                return False
                
        except Exception as e:
            if self.verbose:
                print(f"❌ Error sending G-code command: {e}")
            try:
                client.loop_stop()
                client.disconnect()
            except:
                pass
            return False


class HeatSoakMonitor:
    """Main heatsoak monitoring class."""
    
    def __init__(self, printer_ip: str, printer_access_code: str, printer_device_id: str,
                 printer_cert_file: Optional[str] = None,
                 mqtt_host: str = "localhost", 
                 mqtt_port: int = 1883, mqtt_topic: str = "sensors/sht21/readings",
                 mqtt_username: str = "mqtt", mqtt_password: str = "mqtt",
                 rate_start_type: str = "absolute", rate_threshold_temp: float = 40.0,
                 min_temp_for_plateau: float = 30.0, rate_change_plateau: float = 0.1, 
                 target_temp: Optional[float] = None,
                 temp_smooth: float = 4.0, rate_smooth: float = 30.0,
                 force_ready: bool = False, debug_resume: bool = False, 
                 click_orca: bool = False, skip_status_check: bool = False,
                 force_instructions: Optional[list[tuple[str, float | str, int]]] = None,
                 maintain_temp: Optional[float] = DEFAULT_MAINTAIN_TEMP,
                 temp_command_interval: float = DEFAULT_TEMP_COMMAND_INTERVAL,
                 cooling_threshold: float = DEFAULT_COOLING_THRESHOLD,
                 verbose: bool = False):
        """Initialize heatsoak monitor."""
        self.printer = BambooLabPrinter(printer_ip, printer_access_code, printer_device_id, printer_cert_file, verbose=verbose)
        self.monitor = TemperatureMonitor(temp_smooth, rate_smooth)
        
        self.rate_start_type = rate_start_type
        self.rate_threshold_temp = rate_threshold_temp
        self.min_temp_for_plateau = min_temp_for_plateau
        self.rate_change_plateau = rate_change_plateau
        self.target_temp = target_temp
        self.force_ready = force_ready
        self.debug_resume = debug_resume
        self.click_orca = click_orca
        self.skip_status_check = skip_status_check
        self.force_instructions = force_instructions or []
        self.verbose = verbose
        self.start_time = time.time()
        
        self.popup_handled = False # Track if Orca popup has been dealt with
        
        # MQTT setup for sensor readings
        self.mqtt_client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2, "bamboo-a1-heatsoak")
        self.mqtt_client.username_pw_set(mqtt_username, mqtt_password)
        self.mqtt_client.on_connect = self._on_mqtt_connect
        self.mqtt_client.on_message = self._on_mqtt_message
        self.mqtt_topic = mqtt_topic
        self.mqtt_host = mqtt_host
        self.mqtt_port = mqtt_port
        
        # MQTT setup for printer status (separate connection to printer's broker)
        self.printer_mqtt_client = None
        self.printer_mqtt_connected = False
        
        self.resumed = False
        self.resume_confirmed = False  # Track when printer has confirmed resumed (status changed to RUNNING)
        self.resume_sent_time = None  # Track when resume was sent (for timeout check)
        self.resume_timeout = 30  # Timeout in seconds for waiting for resume confirmation
        self.message_count = 0
        self.last_message_time = None
        self.last_printer_status_time = None  # Track last time we received printer status update
        self.response_timeout = 15  # Timeout in seconds for checking if sensor/printer are responding
        self._connected = False
        self.printer_status = None  # Store current printer status from MQTT report topic
        self.printer_gcode_state = None  # Extracted gcode_state from printer status
        self.printer_bed_temper = None  # Bed temperature from printer status
        self.printer_nozzle_temper = None  # Nozzle temperature from printer status
        self._connections_ready = False  # Track when both connections are established
        self.ready_achieved = False  # Track when ready state has been achieved (maintains ready even if rate increases)
        self.rate_exceeded_plateau = False  # Track if rate has ever exceeded plateau threshold
        self.rate_crossed_below_plateau = False  # Track if rate has crossed below plateau after exceeding it (stays green)
        
        # Nozzle temperature maintenance during pause
        # If maintain_temp is None, temperature maintenance is disabled
        self.maintain_temp_enabled = maintain_temp is not None
        self.maintain_temp_value = maintain_temp if maintain_temp is not None else 180.0  # Temperature to maintain (°C)
        self.temp_command_interval = temp_command_interval  # Interval between temperature commands (seconds)
        self.last_temp_command_time = None  # Track when last temperature command was sent
        
        # Stage tracking for display
        self.current_stage = "soaking"  # Current stage: "soaking", "waiting for pause", "ready", "resumed", "printing", "cooling", "completed"
        self.cooling_threshold = cooling_threshold  # Bed temperature threshold for "Completed" stage
        self.has_seen_pause = False  # Track if we've seen PAUSE status (to distinguish RUNNING before/after pause)
        
        # Log file path
        self.log_file_path = Path(__file__).parent / "_heatsoaklog"
    
    def _write_to_terminal_and_log(self, message: str):
        """
        Write message to both terminal and log file.
        Log file gets plain text (ANSI codes stripped), terminal gets colored version.
        """
        # Write to terminal (with ANSI codes)
        print(message)
        
        # Strip ANSI codes for log file
        ansi_escape = re.compile(r'\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])')
        plain_message = ansi_escape.sub('', message)
        
        # Write to log file (append mode)
        try:
            with open(self.log_file_path, 'a', encoding='utf-8') as f:
                f.write(plain_message + '\n')
        except IOError as e:
            # Silently fail if log file can't be written (don't interrupt monitoring)
            if self.verbose:
                print(f"⚠️  Failed to write to log file: {e}")
    
    def _on_mqtt_connect(self, client, userdata, flags, rc, properties=None):
        """MQTT connection callback for sensor readings."""
        if rc == 0:
            self._connected = True
            client.subscribe(self.mqtt_topic)
            # Always show connection messages (dimmed)
            DIM = "\x1B[38;5;240m"
            RESET = "\x1B[0m"
            print(f"{DIM}   Connected to MQTT broker: {self.mqtt_host}:{self.mqtt_port}{RESET}")
            print(f"{DIM}   Subscribed to topic: {self.mqtt_topic}{RESET}")
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
    
    def _on_printer_mqtt_connect(self, client, userdata, flags, rc, properties=None):
        """MQTT connection callback for printer status."""
        if rc == 0:
            self.printer_mqtt_connected = True
            printer_status_topic = f"device/{self.printer.device_id}/report"
            client.subscribe(printer_status_topic)
            # Always show connection messages (dimmed)
            DIM = "\x1B[38;5;240m"
            RESET = "\x1B[0m"
            print(f"{DIM}   Connected to printer MQTT broker: {self.printer.ip_address}:{self.printer.port}{RESET}")
            print(f"{DIM}   Subscribed to topic: {printer_status_topic}{RESET}")
            print("")  # Blank line after all connection messages
    
        else:
            self.printer_mqtt_connected = False
            print(f"⚠️  Failed to connect to printer MQTT broker for status (rc: {rc})")
    
    def _on_printer_mqtt_message(self, client, userdata, message):
        """MQTT message callback for printer status updates."""
        try:
            payload = json.loads(message.payload.decode())
            # Extract gcode_state from print.gcode_state (matching the PowerShell command)
            print_status = payload.get("print", {})
            if isinstance(print_status, dict):
                # Extract bed and nozzle temperatures (always extract, not just when gcode_state exists)
                bed_temper = print_status.get("bed_temper")
                if bed_temper is not None:
                    self.printer_bed_temper = bed_temper
                
                nozzle_temper = print_status.get("nozzle_temper")
                if nozzle_temper is not None:
                    self.printer_nozzle_temper = nozzle_temper
                
                # Extract and update gcode_state if present
                gcode_state = print_status.get("gcode_state")
                if gcode_state:
                    previous_state = self.printer_gcode_state
                    self.printer_status = payload
                    self.printer_gcode_state = str(gcode_state).upper()
                    self.last_printer_status_time = time.time()  # Track when we received status update
                    
                    # Don't print confirmation here - let it happen in sensor message handler after display line
                    # Just update the flag so the display line will show RUNNING status
                    if (self.resumed and not self.resume_confirmed and 
                        self.printer_gcode_state == "RUNNING"):
                        # Flag will be set in sensor message handler after display line is shown
                        pass
                    
                    # Debug: uncomment to see when status updates are received
                    # print(f"   📨 Printer status updated: {self.printer_gcode_state}")
        except (json.JSONDecodeError, UnicodeDecodeError, KeyError) as e:
            # Silently ignore parse errors
            pass
    
    def _on_mqtt_message(self, client, userdata, message):
        """MQTT message callback for sensor readings."""
        try:
            raw_payload = message.payload.decode()
            payload = json.loads(raw_payload)
            
            # Handle sensor temperature messages (from sensors/sht21/readings topic)
            self.message_count += 1
            self.last_message_time = time.time()
            
            # Suppress display until both connections are ready
            if not self._connections_ready:
                return
            
            temperature = payload.get('temperature')
            
            if temperature is None:
                print("⚠️  No temperature in MQTT message")
                print(f"   Available keys: {list(payload.keys())}")
                return
            
            # Add reading to monitor first
            self.monitor.add_reading(temperature)
            
            # Apply forced values if reading count matches
            # count means "starting from reading (count+1)" - so count=0 means reading 1, count=3 means reading 4
            # But user wants count=3 to mean "starting from reading 3", so use >=
            # count=0 means apply starting from reading 1 (message_count >= 0, which is always true after first reading)
            # count=3 means apply starting from reading 3 (message_count >= 3)
            # Find the most recent matching force instruction for each type (highest count that matches)
            forced_temp = None
            forced_rate = None
            forced_status = None
            forced_temp_count = -1
            forced_rate_count = -1
            forced_status_count = -1
            
            for force_type, force_value, force_count in self.force_instructions:
                # Use >= so count=3 means apply starting from reading 3
                if self.message_count >= force_count:
                    if force_type == 'temp':
                        # Use the most recent matching instruction (highest count)
                        if force_count >= forced_temp_count:
                            forced_temp = float(force_value)
                            forced_temp_count = force_count
                            self.monitor.current_temp = forced_temp
                            self.monitor.smoothed_temp = forced_temp
                    elif force_type == 'rate':
                        # Use the most recent matching instruction (highest count)
                        if force_count >= forced_rate_count:
                            forced_rate = float(force_value)
                            forced_rate_count = force_count
                            self.monitor.rate_per_minute = forced_rate
                    elif force_type == 'status':
                        # Use the most recent matching instruction (highest count)
                        if force_count >= forced_status_count:
                            forced_status = str(force_value).upper()
                            forced_status_count = force_count
                            self.printer_gcode_state = forced_status
            
            # Display current temperature (use forced value if set, otherwise use actual reading)
            if forced_temp is not None:
                current_temp_str = f"{forced_temp:.2f}"
            else:
                current_temp_str = f"{temperature:.2f}"
            
            # Get smoothing progress
            progress = self.monitor.get_smoothing_progress()
            
            # Check heatsoak status (skip if resume already confirmed)
            if self.resume_confirmed:
                # Don't check heatsoak conditions after resume is confirmed
                ready = False
                info = {
                    'ready': False,
                    'temp_ok': False,
                    'rate_ok': False,
                    'soak_started': False,
                    'max_rate_since_soak_start': None,
                    'smoothed_temp': self.monitor.smoothed_temp,
                    'rate_per_minute': self.monitor.rate_per_minute,
                    'current_temp': self.monitor.current_temp,
                }
            else:
                # Check heatsoak status
                ready, info = self.monitor.check_heat_soak_ready(
                    self.rate_start_type,
                    self.rate_threshold_temp,
                    self.min_temp_for_plateau,
                    self.rate_change_plateau,
                    self.target_temp
                )
            
            # Override info dictionary with forced values (display uses info dict, not monitor directly)
            if forced_temp is not None:
                info['smoothed_temp'] = round(forced_temp, 2)
                info['current_temp'] = round(forced_temp, 2)
            if forced_rate is not None:
                info['rate_per_minute'] = forced_rate
            
            # Get current status (forced or actual) - needed for stage tracking
            current_status = forced_status if forced_status is not None else self.printer_gcode_state
            status_is_paused = current_status and str(current_status).upper() in ["PAUSED", "PAUSE"]
            
            # Override ready state if force_ready flag is set (for testing)
            # Skip all status checking if resume is already confirmed
            if self.resume_confirmed:
                ready = False
                info['ready'] = False
                info['chamber_ready'] = False  # Set chamber_ready for display
                self.ready_achieved = False  # Reset ready_achieved when resume is confirmed
            elif self.debug_resume and not self.resumed and (time.time() - self.start_time) > 3.0:
                ready = True
                info['ready'] = True
                info['chamber_ready'] = True
                info['reason'] = 'DEBUG_RESUME (3s timeout)'
                # Force status to PAUSED to trigger resume logic
                status_is_paused = True
                status_value = "PAUSED"
                self.printer_gcode_state = "PAUSED"
            elif self.force_ready:
                ready = True
                info['ready'] = True
                info['reason'] = 'FORCED (--ready flag)'
                # Calculate chamber_ready for display
                current_temp = info.get('smoothed_temp')
                current_rate = info.get('rate_per_minute')
                chamber_ready = False
                if current_temp is not None:
                    temp_condition = (current_temp > self.rate_threshold_temp or
                                    (current_temp > self.min_temp_for_plateau and 
                                     current_rate is not None and current_rate < self.rate_change_plateau))
                    if temp_condition:
                        chamber_ready = True
                info['chamber_ready'] = chamber_ready
            else:
                # Ready state should only be triggered if status is paused AND temperature/rate conditions are met
                # Always check temperature/rate conditions to determine chamber readiness
                # Get current temperature and rate (may be forced values)
                current_temp = info.get('smoothed_temp')
                current_rate = info.get('rate_per_minute')
                
                # Chamber ready if: temp > rate_threshold_temp OR (temp > min_temp_for_plateau AND rate < rate_change_plateau)
                # Rate is ready if: rate < plateau (between 0 and plateau, or negative/cooling)
                chamber_ready = False
                if current_temp is not None:
                    temp_condition = (current_temp > self.rate_threshold_temp or
                                    (current_temp > self.min_temp_for_plateau and 
                                     current_rate is not None and current_rate < self.rate_change_plateau))
                    
                    # Once ready is achieved, maintain it as long as temp > min_temp_for_plateau
                    # This persists even if rate goes back up above the plateau threshold
                    if temp_condition:
                        self.ready_achieved = True
                        chamber_ready = True
                    elif self.ready_achieved:
                        # Maintain ready state if temperature is still above minimum (temp > min_temp_for_plateau)
                        # Ready persists regardless of current rate once it has been achieved
                        if current_temp > self.min_temp_for_plateau:
                            chamber_ready = True
                        else:
                            self.ready_achieved = False  # Reset if temp drops below minimum
                
                # Ready state requires BOTH chamber ready AND printer paused (for resume logic)
                # But chamber status display shows chamber_ready regardless of printer status
                if status_is_paused and chamber_ready:
                    ready = True
                else:
                    ready = False
                    # Only reset ready_achieved when status changes from paused to not paused
                    if not status_is_paused:
                        # Don't reset ready_achieved here - keep it so chamber status can show ready
                        pass
                
                info['ready'] = ready
                info['chamber_ready'] = chamber_ready  # Store for display
            
            # Track if we've seen PAUSE status (to distinguish RUNNING before/after pause)
            if status_is_paused:
                self.has_seen_pause = True
            
            # Update stage based on current state (always runs)
            current_status_upper = str(current_status).upper() if current_status else ""
            
            if current_status_upper == "FINISH":
                # Check if bed has cooled below threshold
                bed_temp = self.printer_bed_temper
                if bed_temp is not None and bed_temp <= self.cooling_threshold:
                    self.current_stage = "completed"
                else:
                    self.current_stage = "cooling"
            elif self.resume_confirmed and current_status_upper == "RUNNING":
                self.current_stage = "printing"
            elif self.resume_confirmed:
                self.current_stage = "complete"
            elif self.resumed:
                self.current_stage = "resumed"
            elif ready:
                self.current_stage = "ready"
            else:
                # Get chamber_ready from info dict
                chamber_ready = info.get('chamber_ready', False)
                if chamber_ready and not status_is_paused:
                    self.current_stage = "waiting for pause"
                else:
                    self.current_stage = "soaking"
            
            # Display status - always show current temp, show smoothed/rate when available
            icon = "✅" if ready else "⏳"
            
            # Format smoothed temperature display
            # Show forced value if set, otherwise use progress-based display
            if forced_temp is not None:
                smoothed_display = f"{forced_temp:.2f}°"
            elif progress['temp_ready'] and info.get('smoothed_temp') is not None:
                smoothed_display = f"{info.get('smoothed_temp', 0):.2f}°"
            else:
                smoothed_display = f"[{progress['temp_current']}/{progress['temp_needed']}]"
            
            # Format rate display
            # Show forced value if set, otherwise use progress-based display
            # Remove ± symbol, wrap negative values in parentheses
            # Handle -0.0 edge case by normalizing to 0.0
            if forced_rate is not None:
                # Normalize -0.0 to 0.0
                normalized_rate = 0.0 if forced_rate == 0.0 else forced_rate
                if normalized_rate < 0:
                    rate_display = f"({abs(normalized_rate):.2f})"
                else:
                    rate_display = f"{normalized_rate:.2f}"
            elif progress['rate_ready'] and info.get('rate_per_minute') is not None:
                rate_value = info.get('rate_per_minute', 0)
                # Normalize -0.0 to 0.0
                normalized_rate = 0.0 if rate_value == 0.0 else rate_value
                if normalized_rate < 0:
                    rate_display = f"({abs(normalized_rate):.2f})"
                else:
                    rate_display = f"{normalized_rate:.2f}"
            elif progress['temp_ready']:
                # Rate smoothing is collecting data
                rate_display = f"[{progress['rate_current']}/{progress['rate_needed']}]"
            else:
                # Rate smoothing hasn't started yet
                rate_display = "---"
            
            # ANSI color codes
            GREEN = "\033[32m"
            RED = "\033[31m"
            GOLD = "\033[38;5;179m"  # Gold color (same as outerm highlight)
            YELLOW = "\033[33m"  # Yellow color for prepping state
            RESET = "\033[0m"
            
            # Get current temp and rate (forced or actual) for color coding
            current_smoothed_temp = forced_temp if forced_temp is not None else info.get('smoothed_temp')
            current_rate = forced_rate if forced_rate is not None else info.get('rate_per_minute')
            
            # Determine smoothed temp color based on conditions
            smoothed_temp_color = RED  # Default to red
            if current_smoothed_temp is not None:
                # Gold when min_temp_for_plateau is passed
                if current_smoothed_temp > self.min_temp_for_plateau:
                    smoothed_temp_color = GOLD
                    # Green when either:
                    # 1. rate_threshold_temp is passed, OR
                    # 2. rate goes below plateau (rate < plateau, includes negatives) AND min_temp_for_plateau is passed
                    if (current_smoothed_temp > self.rate_threshold_temp or
                        (current_rate is not None and current_rate < self.rate_change_plateau and 
                         current_smoothed_temp > self.min_temp_for_plateau)):
                        smoothed_temp_color = GREEN
            
            # Determine rate color based on state machine:
            # GOLD: initially (before rate has ever exceeded plateau)
            # RED: after rate exceeds plateau (rate >= plateau), until it goes below
            # GREEN: once rate goes below plateau (rate < plateau) after being above it, stays green FOREVER
            # Note: rate < plateau includes negative rates (cooling) and small positive rates
            if self.rate_crossed_below_plateau:
                # Once rate has crossed below plateau after exceeding it, always stay green
                rate_color = GREEN
            elif current_rate is not None:
                if current_rate >= self.rate_change_plateau:
                    # Rate is at or above plateau threshold (positive rate >= plateau)
                    self.rate_exceeded_plateau = True
                    rate_color = RED
                else:
                    # Rate is below plateau threshold (rate < plateau, includes negatives)
                    if self.rate_exceeded_plateau:
                        # Rate has crossed below plateau after being above it - stay green forever
                        self.rate_crossed_below_plateau = True
                        rate_color = GREEN
                    else:
                        # Rate hasn't exceeded plateau yet - stay gold
                        rate_color = GOLD
            else:
                # No rate data yet - default to gold
                rate_color = GOLD
            
            # Get status value and determine color
            if forced_status is not None:
                status_value = forced_status
            elif self.printer_gcode_state:
                status_value = self.printer_gcode_state
            else:
                status_value = "unknown"
            
            # Color code based on status value:
            # - PAUSED/PAUSE: green (desired state for heatsoak)
            # - RUNNING before PAUSE: yellow (prepping)
            # - RUNNING after PAUSE: green (printing)
            # - FINISHED: yellow (cooling)
            # - Other: red
            status_value_upper = str(status_value).upper()
            if status_value_upper in ["PAUSED", "PAUSE"]:
                status_color = GREEN
            elif status_value_upper == "RUNNING":
                if self.has_seen_pause:
                    # RUNNING after pause (printing) - green
                    status_color = GREEN
                else:
                    # RUNNING before pause (prepping) - yellow
                    status_color = YELLOW
            elif status_value_upper == "FINISHED":
                status_color = YELLOW
            else:
                status_color = RED
            
            # Format bed and nozzle temperatures for display (match smoothed format: Label(°C): value°)
            bed_display = f"{self.printer_bed_temper:.1f}°" if self.printer_bed_temper is not None else "---"
            nozzle_display = f"{self.printer_nozzle_temper:.1f}°" if self.printer_nozzle_temper is not None else "---"
            chamber_display = f"{current_temp_str}°"
            
            # Apply color to smoothed temp value (like the flag was before)
            if forced_temp is not None:
                smoothed_colored = f"{smoothed_temp_color}{smoothed_display}{RESET}"
            elif progress['temp_ready'] and info.get('smoothed_temp') is not None:
                smoothed_colored = f"{smoothed_temp_color}{smoothed_display}{RESET}"
            else:
                smoothed_colored = smoothed_display
            
            # Apply color to rate value (like the flag was before)
            if forced_rate is not None:
                rate_colored = f"{rate_color}{rate_display}{RESET}"
            elif progress['rate_ready'] and info.get('rate_per_minute') is not None:
                rate_colored = f"{rate_color}{rate_display}{RESET}"
            else:
                rate_colored = rate_display
            
            # Apply color to status value (like the flag was before)
            status_colored = f"{status_color}{status_value}{RESET}"
            
            # Display status summary line - hide chamber status after resume_confirmed
            chamber_status_part = ""
            if not self.resume_confirmed:
                chamber_ready = info.get('chamber_ready', False)
                chamber_status = "Ready" if chamber_ready else "Not Ready"
                chamber_status_color = GREEN if chamber_ready else RED
                chamber_status_colored = f"{chamber_status_color}{chamber_status}{RESET}"
                chamber_status_part = f"Chamber: {chamber_status_colored} "
            
            # Format stage for display
            stage_display = f"Stage: {self.current_stage}"
            
            # Write display line to both terminal and log file
            display_line = (f"{icon} B: {bed_display} | N: {nozzle_display} | C: {chamber_display} | "
                          f"S: {smoothed_colored} | R: {rate_colored} | "
                          f"{chamber_status_part}Printer: {status_colored} | {stage_display}")
            self._write_to_terminal_and_log(display_line)
            
            # Maintain nozzle temperature at 180°C during pause (M400 U1 reduces it to 90°C)
            # Send M104 command periodically while printer is paused
            # CRITICAL: Stop maintaining temp when ready to resume or after resume is sent
            # M104 commands can interfere with resume and prevent the print from actually resuming
            current_status = forced_status if forced_status is not None else self.printer_gcode_state
            is_paused = current_status and str(current_status).upper() in ["PAUSED", "PAUSE"]
            
            # Only maintain temp if:
            # 1. Printer is paused
            # 2. Temperature maintenance is enabled
            # 3. Resume has NOT been sent (resumed = False)
            # 4. Resume has NOT been confirmed (resume_confirmed = False)
            # 5. NOT ready to resume (ready = False) - stop maintaining before resume
            if (is_paused and self.maintain_temp_enabled and 
                not self.resumed and not self.resume_confirmed and not ready):
                # Check if it's time to send temperature command
                current_time = time.time()
                if (self.last_temp_command_time is None or 
                    current_time - self.last_temp_command_time >= self.temp_command_interval):
                    if self.printer.send_gcode(f"M104 S{int(self.maintain_temp_value)}", check_status=False):
                        self.last_temp_command_time = current_time
                        if self.verbose:
                            print(f"🌡️  Maintaining nozzle at {int(self.maintain_temp_value)}°C during pause")
            elif self.resumed or self.resume_confirmed:
                # Ensure temperature maintenance is completely stopped after resume
                self.last_temp_command_time = None
            
            # Resume printer if ready and not already resumed
            # Skip status check in resume() since we already verified status is PAUSED before marking ready
            if ready and not self.resumed and not self.resume_confirmed:
                # CRITICAL: Stop temperature maintenance BEFORE sending resume
                # M104 commands sent too close to resume can interfere and prevent resume from working
                self.last_temp_command_time = None
                self.maintain_temp_enabled = False  # Temporarily disable to prevent any M104 commands
                
                print("\n🔥 HEATSOAK READY! Resuming printer...")
                try:
                    resume_success = self.printer.resume(check_status=False)
                    if resume_success:
                        self.resumed = True
                        self.resume_sent_time = time.time()  # Track when resume was sent
                        self.current_stage = "resumed"  # Update stage to resumed
                        # Temperature maintenance already stopped above
                        print("✅ Printer resume command sent successfully")
                        print("   Waiting for printer status to change to RUNNING...")
                        print()
                    else:
                        print("❌ Failed to resume printer (resume() returned False). Please resume manually.")
                        print()
                        # Re-enable temperature maintenance if resume failed
                        self.maintain_temp_enabled = True
                except Exception as resume_err:
                    print(f"❌ Exception during printer.resume(): {resume_err}")
                    import traceback
                    traceback.print_exc()
                    print("   Please resume manually.")
                    print()
                    # Re-enable temperature maintenance if resume failed
                    self.maintain_temp_enabled = True
            
            # Check if status is RUNNING after resume was sent (also check in sensor message handler)
            # Get current status (forced or actual)
            current_status = forced_status if forced_status is not None else self.printer_gcode_state
            if (self.resumed and not self.resume_confirmed and 
                current_status and str(current_status).upper() == "RUNNING"):
                self.resume_confirmed = True
                self.resume_sent_time = None  # Clear resume time tracking
                self.current_stage = "complete"  # Update stage to complete
                print(f"\n✅ Printer confirmed resumed (status: {current_status})\n")
                
                # Handle Orca Slicer popup if not already done
                if not self.popup_handled:
                    # Small delay to ensure Orca has updated its internal state
                    time.sleep(1.0)
                    
                    if WIN32_AVAILABLE:
                        try:
                            # 1. Find main Orca Slicer window (to stop flashing later)
                            main_orca_hwnd = win32gui.FindWindow("wxWindowNR", "OrcaSlicer")
                            if not main_orca_hwnd:
                                found_main = []
                                def find_main_orca(h, _):
                                    if "OrcaSlicer" in win32gui.GetWindowText(h) and win32gui.GetClassName(h) == "wxWindowNR":
                                        found_main.append(h)
                                    return True
                                win32gui.EnumWindows(find_main_orca, None)
                                if found_main:
                                    main_orca_hwnd = found_main[0]

                            # 2. Handle the popup directly (background closure)
                            if self._handle_orca_popup():
                                # 3. Stop any taskbar flashing on Orca
                                if main_orca_hwnd:
                                    try:
                                        # Use FlashWindowEx with FLASHW_STOP (v2 winner)
                                        info = FLASHWINFO(
                                            cbSize=ctypes.sizeof(FLASHWINFO),
                                            hwnd=main_orca_hwnd,
                                            dwFlags=FLASHW_STOP,
                                            uCount=0,
                                            dwTimeout=0
                                        )
                                        ctypes.windll.user32.FlashWindowEx(ctypes.byref(info))
                                        print("   ✨ Cleared Orca Slicer taskbar alert (verified method).")
                                    except Exception as flash_err:
                                        if self.verbose:
                                            print(f"   ⚠️  Could not clear flash: {flash_err}")
                                
                        except Exception as e:
                            print(f"   ⚠️  Error during Orca handling: {e}")
                                
                        except Exception as e:
                            print(f"   ⚠️  Error during Orca window sequence: {e}")
            
            # Check for resume timeout (non-blocking, continues processing messages)
            if (self.resumed and not self.resume_confirmed and self.resume_sent_time is not None and
                time.time() - self.resume_sent_time > self.resume_timeout):
                print(f"\n⚠️  Timeout waiting for printer to resume (status did not change to RUNNING within {self.resume_timeout}s)")
                print("   Status checking will continue...\n")
                self.resume_sent_time = None  # Clear to prevent repeated timeout messages
                
        except json.JSONDecodeError as e:
            print(f"⚠️  Failed to parse MQTT message: {e}")
        except Exception as e:
            print(f"❌ Error processing MQTT message: {e}")
    
    def _handle_orca_popup(self):
        """Find and handle the Orca Slicer error popup."""
        if not WIN32_AVAILABLE:
            if self.verbose:
                print("⚠️  win32gui not available, skipping Orca popup handling")
            return

        REQUIRED_TEXT = "Printing paused due to the pause command added to the printing file."
        BUTTON_TEXT = "Resume Printing"
        WINDOW_TITLE = "Error"
        WINDOW_CLASS = "wxWindowNR"

        def get_text(h):
            return win32gui.GetWindowText(h)
        
        def get_cls(h):
            return win32gui.GetClassName(h)

        found_hwnd = []
        def enum_cb(h, _):
            if win32gui.IsWindowVisible(h) and get_text(h) == WINDOW_TITLE and get_cls(h) == WINDOW_CLASS:
                found_hwnd.append(h)
            return True
        
        win32gui.EnumWindows(enum_cb, None)
        
        if not found_hwnd:
            if self.verbose:
                print("🔍 Orca Slicer error popup not found.")
            return False

        hwnd = found_hwnd[0]
        
        # Verify text in children
        children = []
        def child_cb(h, _):
            children.append((h, get_text(h), get_cls(h)))
            return True
        win32gui.EnumChildWindows(hwnd, child_cb, None)
        
        combined_text = " ".join([t for _, t, _ in children if t] + [get_text(hwnd)])
        if REQUIRED_TEXT not in re.sub(r'\s+', ' ', combined_text):
            if self.verbose:
                print("🔍 Orca popup found but text does not match.")
            return False

        print(f"\n📦 Handling Orca Slicer popup (HWND: {hwnd})...")
        
        try:
            if self.click_orca:
                button_hwnd = None
                for c_h, c_t, c_c in children:
                    if c_t == BUTTON_TEXT or BUTTON_TEXT.lower() in c_t.lower():
                        button_hwnd = c_h
                        break
                
                if button_hwnd:
                    print(f"   🖱️  Clicking '{BUTTON_TEXT}' button...")
                    # BM_CLICK = 0x00F5
                    win32gui.PostMessage(button_hwnd, 0x00F5, 0, 0)
                    self.popup_handled = True
                    return True
                else:
                    print(f"   ⚠️  Button '{BUTTON_TEXT}' not found, falling back to close.")
                    win32gui.PostMessage(hwnd, win32con.WM_CLOSE, 0, 0)
                    self.popup_handled = True
                    return True
            else:
                print("   ❌ Closing popup window...")
                win32gui.PostMessage(hwnd, win32con.WM_CLOSE, 0, 0)
                self.popup_handled = True
                return True
        except Exception as e:
            print(f"   ❌ Error handling popup: {e}")
            return False

    def start(self):
        """Start monitoring."""
        # Clear log file when starting a new heatsoak session
        try:
            with open(self.log_file_path, 'w', encoding='utf-8') as f:
                f.write('')  # Clear the file
        except IOError as e:
            if self.verbose:
                print(f"⚠️  Failed to clear log file: {e}")
        
        # Always show starting message
        print(f"\n🌡️  Starting heatsoak monitoring...")
        
        # Detailed config only in verbose mode
        if self.verbose:
            print(f"   Sensor MQTT: {self.mqtt_host}:{self.mqtt_port}")
            print(f"   Sensor Topic: {self.mqtt_topic}")
            print(f"   Printer: {self.printer.ip_address}:{self.printer.port} (MQTT)")
            print(f"   Printer Device ID: {self.printer.device_id}")
            print(f"   Printer Command Topic: {self.printer.command_topic}")
            print(f"   Heatsoak: rate_start_type={self.rate_start_type}, "
                  f"rate_threshold_temp={self.rate_threshold_temp}°C, "
                  f"min_temp_for_plateau={self.min_temp_for_plateau}°C, "
                  f"plateau={self.rate_change_plateau}°C/min", end="")
            if self.target_temp is not None:
                print(f", target={self.target_temp}°C")
            else:
                print()
            if self.force_ready:
                print(f"   ⚠️  TEST MODE: --ready flag enabled (will force heatsoak ready on first message)")
            print("   Press Ctrl+C to stop\n")
        
        try:
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
            
            # Connect to printer MQTT broker for status updates
            self.printer_mqtt_client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2, "bamboo-a1-status")
            self.printer_mqtt_client.username_pw_set("bblp", self.printer.access_code)
            
            # Setup TLS with certificate (use insecure mode)
            if self.printer.cert_file and Path(self.printer.cert_file).exists():
                self.printer_mqtt_client.tls_set(
                    ca_certs=str(Path(self.printer.cert_file).resolve()),
                    cert_reqs=ssl.CERT_NONE
                )
                self.printer_mqtt_client.tls_insecure_set(True)
            else:
                self.printer_mqtt_client.tls_set(cert_reqs=ssl.CERT_NONE)
                self.printer_mqtt_client.tls_insecure_set(True)
            
            self.printer_mqtt_client.on_connect = self._on_printer_mqtt_connect
            self.printer_mqtt_client.on_message = self._on_printer_mqtt_message
            
            try:
                self.printer_mqtt_client.connect(self.printer.ip_address, self.printer.port, 60)
                self.printer_mqtt_client.loop_start()
                
                # Wait for printer connection
                printer_connection_timeout = 5
                start_time = time.time()
                while not self.printer_mqtt_connected and (time.time() - start_time) < printer_connection_timeout:
                    time.sleep(0.1)
                
                if not self.printer_mqtt_connected:
                    print(f"⚠️  Could not connect to printer MQTT broker for status updates")
                    print(f"   Status will show as 'unknown' until connection is established")
                    print("")  # Blank line after all connection messages
            except Exception as e:
                print(f"⚠️  Error connecting to printer MQTT broker: {e}")
                print(f"   Status will show as 'unknown' until connection is established")
                print("")  # Blank line after all connection messages
            
            # Enable display now that all connection messages have been printed
            self._connections_ready = True
            
            # Wait a bit and check if we're receiving messages
            time.sleep(10)  # Wait 10 seconds
            if self.message_count == 0:
                print("\n⚠️  WARNING: No messages received after 10 seconds!")
                print(f"   Expected topic: {self.mqtt_topic}")
                print(f"   Check:")
                print(f"   1. ESP32 is powered on and connected to WiFi")
                print(f"   2. ESP32 is publishing to topic: {self.mqtt_topic}")
                print(f"   3. MQTT broker is accessible at {self.mqtt_host}:{self.mqtt_port}")
                print(f"   4. ESP32 MQTT server matches monitor MQTT host")
                print(f"      (ESP32 default: 192.168.1.10, Monitor: {self.mqtt_host})")
                print(f"   5. Check ESP32 serial output for MQTT connection status")
                print(f"\n   Continuing to wait for messages...\n")
            
            # Keep running
            while True:
                time.sleep(1)
        except KeyboardInterrupt:
            print("\n\n✅ Monitoring stopped")
            print(f"   Total messages received: {self.message_count}")
            self.mqtt_client.loop_stop()
            self.mqtt_client.disconnect()
            if self.printer_mqtt_client:
                self.printer_mqtt_client.loop_stop()
                self.printer_mqtt_client.disconnect()


def parse_force_argument(force_str: str) -> tuple[str, float | str, int]:
    """
    Parse force argument in format: type=value,count
    
    Examples:
        temp=33,0 -> ('temp', 33.0, 0)
        rate=0.07,3 -> ('rate', 0.07, 3)
        status="PAUSED",3 -> ('status', 'PAUSED', 3)
    
    Returns:
        tuple: (type, value, count)
    """
    # Match pattern: type=value,count
    # Handle both quoted and unquoted string values
    pattern = r'^(\w+)=(.+),(\d+)$'
    match = re.match(pattern, force_str)
    
    if not match:
        raise ValueError(f"Invalid force argument format: {force_str}. Expected format: type=value,count")
    
    arg_type = match.group(1)
    value_str = match.group(2)
    count = int(match.group(3))
    
    # Parse value based on type
    if arg_type == 'temp':
        value = float(value_str)
    elif arg_type == 'rate':
        value = float(value_str)
    elif arg_type == 'status':
        # Remove quotes if present
        value = value_str.strip('"\'')
    else:
        raise ValueError(f"Unknown force type: {arg_type}. Supported types: temp, rate, status")
    
    return (arg_type, value, count)


def main():
    parser = argparse.ArgumentParser(
        description='Bamboo Labs A1 Heat Soak Monitor',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Basic usage (uses defaults from script)
  python monitor_heatsoak.py
  
  # Override specific settings
  python monitor_heatsoak.py --printer-ip 192.168.1.100 --mqtt-host 192.168.1.50
  
  # Custom thresholds
  python monitor_heatsoak.py --rate-start-temp 45 --rate-change-plateau 0.15
  
  # With target temperature
  python monitor_heatsoak.py --target-temp 50.0
  
  # Use insecure TLS (no certificate file)
  python monitor_heatsoak.py --printer-cert-file ""
  
  # Test mode: force heatsoak ready immediately (bypasses temperature checks)
  python monitor_heatsoak.py --ready
  
  # Skip status check (send resume even if printer is not paused)
  python monitor_heatsoak.py --skip-status-check
  
  # Force values after specific reading counts (for testing)
  python monitor_heatsoak.py --force temp=33,0 --force rate=0.07,3 --force status="PAUSED",3
        """
    )
    
    # Printer settings
    parser.add_argument('--printer-ip', type=str, default=DEFAULT_PRINTER_IP,
                       help=f'Bamboo Labs A1 printer IP address (default: {DEFAULT_PRINTER_IP})')
    parser.add_argument('--printer-access-code', type=str, default=DEFAULT_PRINTER_ACCESS_CODE,
                       help=f'Printer LAN access code (used as MQTT password) (default: {DEFAULT_PRINTER_ACCESS_CODE})')
    parser.add_argument('--printer-device-id', type=str, default=DEFAULT_PRINTER_DEVICE_ID,
                       help=f'Printer device ID (default: {DEFAULT_PRINTER_DEVICE_ID})')
    parser.add_argument('--printer-cert-file', type=str, default=DEFAULT_PRINTER_CERT_FILE,
                       help=f'Path to printer certificate file (default: {DEFAULT_PRINTER_CERT_FILE}). Set to empty string to use insecure TLS mode.')
    
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
    
    # Heatsoak settings
    parser.add_argument('--rate-start-type', type=str, default=DEFAULT_RATE_START_TYPE,
                       choices=['offset', 'absolute'],
                       help=f'Rate start type: "offset" (adds to initial soak temp) or "absolute" (uses as-is) (default: {DEFAULT_RATE_START_TYPE})')
    parser.add_argument('--rate-threshold-temp', type=float, default=DEFAULT_RATE_THRESHOLD_TEMP,
                       help=f'Minimum temperature for rate-based heatsoak detection - prevents false positives during ramp-up (°C) (default: {DEFAULT_RATE_THRESHOLD_TEMP})')
    parser.add_argument('--min-temp-for-plateau', type=float, default=DEFAULT_MIN_TEMP_FOR_PLATEAU,
                       help=f'Minimum temperature for plateau check to be valid (°C) (default: {DEFAULT_MIN_TEMP_FOR_PLATEAU})')
    parser.add_argument('--rate-change-plateau', type=float, default=DEFAULT_RATE_CHANGE_PLATEAU,
                       help=f'Maximum rate of change threshold (°C/min) - indicates diminishing returns (default: {DEFAULT_RATE_CHANGE_PLATEAU})')
    parser.add_argument('--target-temp', type=float, default=DEFAULT_TARGET_TEMP,
                       help=f'Target temperature - if reached, automatically ready (°C) (default: {DEFAULT_TARGET_TEMP or "disabled"})')
    
    # Smoothing settings
    parser.add_argument('--temp-smooth', type=float, default=DEFAULT_TEMP_SMOOTH,
                       help=f'Temperature smoothing entry count (default: {DEFAULT_TEMP_SMOOTH}). With 5-sec intervals, this becomes ~(value+1)*5 seconds of smoothing.')
    parser.add_argument('--rate-smooth', type=float, default=DEFAULT_RATE_SMOOTH,
                       help=f'Rate smoothing entry count (default: {DEFAULT_RATE_SMOOTH}). With 5-sec intervals, this becomes ~(value+1)*5 seconds of smoothing.')
    
    # Temperature maintenance settings
    def parse_maintain_temp(value):
        """Parse maintain_temp argument - allow None to disable."""
        if value is None or value.lower() in ['none', 'null', '']:
            return None
        return float(value)
    
    parser.add_argument('--maintain-temp', type=parse_maintain_temp, default=DEFAULT_MAINTAIN_TEMP,
                       help=f'Temperature to maintain during pause (°C). Set to None to disable (default: {DEFAULT_MAINTAIN_TEMP})')
    parser.add_argument('--temp-command-interval', type=float, default=DEFAULT_TEMP_COMMAND_INTERVAL,
                       help=f'Interval between temperature maintenance commands (seconds) (default: {DEFAULT_TEMP_COMMAND_INTERVAL})')
    
    # Cooling settings
    parser.add_argument('--cooling-threshold', type=float, default=DEFAULT_COOLING_THRESHOLD,
                       help=f'Bed temperature threshold for "Completed" stage (°C) (default: {DEFAULT_COOLING_THRESHOLD})')
    
    # Testing flags
    parser.add_argument('--ready', action='store_true',
                       help='Force heatsoak ready state immediately (for testing resume functionality)')
    parser.add_argument('--debug-resume', action='store_true',
                       help='Debug mode: force everything to ready after 3 seconds and resume')
    parser.add_argument('--click-orca', action='store_true',
                       help='Click the "Resume Printing" button in Orca Slicer popup instead of just closing it')
    parser.add_argument('--skip-status-check', action='store_true',
                       help='Skip checking if printer is paused before sending resume command')
    parser.add_argument('--verbose', action='store_true',
                       help='Show verbose output (connection messages, configuration details, resume command details)')
    
    # Force arguments: accept multiple --force flags in format type=value,count
    parser.add_argument('--force', action='append', dest='force_args', metavar='TYPE=VALUE,COUNT',
                       help='Force values after reading count. Format: type=value,count. Can be used multiple times. Examples: --force temp=33,0 --force rate=0.07,3 --force status="PAUSED",3')
    
    args, unknown_args = parser.parse_known_args()
    
    # Ignore PowerShell-specific arguments silently (e.g., -encodedCommand, -inputFormat, -outputFormat)
    # These are added by PowerShell when executing scripts in certain ways
    if unknown_args:
        # Filter out known PowerShell-specific arguments and their values
        powershell_args = {'-encodedCommand', '-inputFormat', '-outputFormat'}
        filtered_unknown = []
        skip_next = False
        for arg in unknown_args:
            if skip_next:
                skip_next = False
                continue
            if arg in powershell_args:
                skip_next = True  # Skip the next argument (the value)
                continue
            filtered_unknown.append(arg)
        
        if filtered_unknown:
            parser.error(f"unrecognized arguments: {' '.join(filtered_unknown)}")
    
    # Parse force arguments
    force_instructions = []
    if args.force_args:
        for force_str in args.force_args:
            try:
                force_type, force_value, force_count = parse_force_argument(force_str)
                force_instructions.append((force_type, force_value, force_count))
            except ValueError as e:
                print(f"❌ Error parsing force argument '{force_str}': {e}", file=sys.stderr)
                sys.exit(1)
    
    if not MQTT_AVAILABLE:
        print("❌ MQTT not available. Install with: uv pip install paho-mqtt")
        sys.exit(1)
    
    # Handle empty string for cert file (use insecure mode)
    cert_file = args.printer_cert_file if args.printer_cert_file else None
    
    monitor = HeatSoakMonitor(
        printer_ip=args.printer_ip,
        printer_access_code=args.printer_access_code,
        printer_device_id=args.printer_device_id,
        printer_cert_file=cert_file,
        mqtt_host=args.mqtt_host,
        mqtt_port=args.mqtt_port,
        mqtt_topic=args.mqtt_topic,
        mqtt_username=args.mqtt_username,
        mqtt_password=args.mqtt_password,
        rate_start_type=args.rate_start_type,
        rate_threshold_temp=args.rate_threshold_temp,
        min_temp_for_plateau=args.min_temp_for_plateau,
        rate_change_plateau=args.rate_change_plateau,
        target_temp=args.target_temp,
        temp_smooth=args.temp_smooth,
        rate_smooth=args.rate_smooth,
        force_ready=args.ready,
        debug_resume=args.debug_resume,
        click_orca=args.click_orca,
        skip_status_check=args.skip_status_check,
        force_instructions=force_instructions,
        maintain_temp=args.maintain_temp,
        temp_command_interval=args.temp_command_interval,
        cooling_threshold=args.cooling_threshold,
        verbose=args.verbose
    )
    
    monitor.start()


if __name__ == '__main__':
    main()




