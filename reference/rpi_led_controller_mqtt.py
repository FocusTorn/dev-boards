#!/usr/bin/env python3
"""
Raspberry Pi LED Controller for ESP32-S3 via MQTT
Uses MQTT to send commands to ESP32-S3 LED controller

Usage:
    python3 rpi_led_controller_mqtt.py --color 255 0 0 0
    python3 rpi_led_controller_mqtt.py --pattern rainbow --speed 50
    python3 rpi_led_controller_mqtt.py --brightness 128
    python3 rpi_led_controller_mqtt.py --clear
"""

import argparse
import json
import paho.mqtt.client as mqtt
import sys
import time


class ESP32LEDControllerMQTT:
    """Controller for ESP32-S3 LED strip via MQTT"""
    
    def __init__(self, broker="localhost", port=1883, username=None, password=None, 
                 command_topic="sensors/esp32-s3-led/command"):
        """
        Initialize MQTT LED controller
        
        Args:
            broker: MQTT broker address (default: localhost)
            port: MQTT port (default: 1883)
            username: MQTT username (optional)
            password: MQTT password (optional)
            command_topic: MQTT topic for commands
        """
        self.broker = broker
        self.port = port
        self.command_topic = command_topic
        self.client = mqtt.Client()
        
        if username and password:
            self.client.username_pw_set(username, password)
        
        # Connect to broker
        try:
            self.client.connect(broker, port, 60)
            self.client.loop_start()
            time.sleep(0.1)  # Give connection time to establish
        except Exception as e:
            print(f"✗ Error connecting to MQTT broker: {e}", file=sys.stderr)
            sys.exit(1)
    
    def _publish(self, payload):
        """Internal method to publish MQTT message"""
        try:
            result = self.client.publish(self.command_topic, json.dumps(payload))
            result.wait_for_publish(timeout=2)
            return result.rc == mqtt.MQTT_ERR_SUCCESS
        except Exception as e:
            print(f"✗ Error publishing message: {e}", file=sys.stderr)
            return False
        finally:
            self.client.loop_stop()
            self.client.disconnect()
    
    def set_color(self, r, g, b, w=0):
        """Set all LEDs to a specific color"""
        payload = {
            "action": "set_color",
            "r": max(0, min(255, r)),
            "g": max(0, min(255, g)),
            "b": max(0, min(255, b)),
            "w": max(0, min(255, w))
        }
        
        if self._publish(payload):
            print(f"✓ Color set: RGBW({r}, {g}, {b}, {w})")
            return True
        return False
    
    def set_brightness(self, value):
        """Set LED strip brightness"""
        payload = {
            "action": "set_brightness",
            "value": max(0, min(255, value))
        }
        
        if self._publish(payload):
            print(f"✓ Brightness set: {value}")
            return True
        return False
    
    def clear(self):
        """Clear all LEDs (turn off)"""
        payload = {"action": "clear"}
        
        if self._publish(payload):
            print("✓ LEDs cleared")
            return True
        return False
    
    def start_pattern(self, name, speed=50):
        """Start a LED pattern"""
        payload = {
            "action": "pattern",
            "name": name,
            "speed": max(1, min(100, speed))
        }
        
        if self._publish(payload):
            print(f"✓ Pattern started: {name} (speed: {speed})")
            return True
        return False
    
    def stop_pattern(self):
        """Stop current pattern"""
        payload = {"action": "stop"}
        
        if self._publish(payload):
            print("✓ Pattern stopped")
            return True
        return False


def main():
    parser = argparse.ArgumentParser(
        description="Control ESP32-S3 LED strip via MQTT",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Set LED to red
  %(prog)s --color 255 0 0
  
  # Set LED to white with white channel
  %(prog)s --color 0 0 0 255
  
  # Set brightness
  %(prog)s --brightness 128
  
  # Start rainbow pattern
  %(prog)s --pattern rainbow --speed 50
  
  # Clear LEDs
  %(prog)s --clear
        """
    )
    
    parser.add_argument(
        "--broker",
        default="localhost",
        help="MQTT broker address (default: localhost)"
    )
    
    parser.add_argument(
        "--port",
        type=int,
        default=1883,
        help="MQTT port (default: 1883)"
    )
    
    parser.add_argument(
        "--username",
        help="MQTT username (optional)"
    )
    
    parser.add_argument(
        "--password",
        help="MQTT password (optional)"
    )
    
    parser.add_argument(
        "--topic",
        default="sensors/esp32-s3-led/command",
        help="MQTT command topic (default: sensors/esp32-s3-led/command)"
    )
    
    # Action arguments (mutually exclusive)
    action_group = parser.add_mutually_exclusive_group(required=True)
    
    action_group.add_argument(
        "--color",
        nargs="+",
        metavar=("R", "G", "B", "[W]"),
        help="Set color: R G B [W] (values 0-255)"
    )
    
    action_group.add_argument(
        "--brightness",
        type=int,
        metavar="VALUE",
        help="Set brightness (0-255)"
    )
    
    action_group.add_argument(
        "--pattern",
        choices=["rainbow", "chase", "fade", "wave", "sparkle"],
        help="Start a pattern"
    )
    
    action_group.add_argument(
        "--clear",
        action="store_true",
        help="Clear all LEDs"
    )
    
    action_group.add_argument(
        "--stop",
        action="store_true",
        help="Stop current pattern"
    )
    
    parser.add_argument(
        "--speed",
        type=int,
        default=50,
        help="Pattern speed (1-100, default: 50)"
    )
    
    args = parser.parse_args()
    
    # Create controller
    controller = ESP32LEDControllerMQTT(
        broker=args.broker,
        port=args.port,
        username=args.username,
        password=args.password,
        command_topic=args.topic
    )
    
    # Execute action
    success = False
    
    if args.color:
        try:
            if len(args.color) == 3:
                r, g, b = map(int, args.color)
                success = controller.set_color(r, g, b, 0)
            elif len(args.color) == 4:
                r, g, b, w = map(int, args.color)
                success = controller.set_color(r, g, b, w)
            else:
                print("✗ Color requires 3 or 4 values (R G B [W])", file=sys.stderr)
                sys.exit(1)
        except ValueError as e:
            print(f"✗ Invalid color values: {e}", file=sys.stderr)
            sys.exit(1)
    
    elif args.brightness is not None:
        success = controller.set_brightness(args.brightness)
    
    elif args.pattern:
        success = controller.start_pattern(args.pattern, args.speed)
    
    elif args.clear:
        success = controller.clear()
    
    elif args.stop:
        success = controller.stop_pattern()
    
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()

