#!/usr/bin/env python3
"""
Raspberry Pi LED Controller for ESP32-S3
Direct HTTP REST API communication - NO POLLING REQUIRED

This script sends HTTP POST requests directly to the ESP32-S3
to control the SK6812 LED strip. The ESP32-S3 runs an HTTP server
that responds immediately to commands.

Usage:
    python3 rpi_led_controller.py --ip 192.168.1.100 --color 255 0 0 0
    python3 rpi_led_controller.py --ip 192.168.1.100 --pattern rainbow --speed 50
    python3 rpi_led_controller.py --ip 192.168.1.100 --brightness 128
    python3 rpi_led_controller.py --ip 192.168.1.100 --clear
"""

import argparse
import json
import requests
import sys
from typing import Optional, Tuple


class ESP32LEDController:
    """Controller for ESP32-S3 LED strip via HTTP REST API"""
    
    def __init__(self, ip_address: str, port: int = 80):
        """
        Initialize LED controller
        
        Args:
            ip_address: IP address of ESP32-S3
            port: HTTP port (default 80)
        """
        self.base_url = f"http://{ip_address}:{port}"
        self.timeout = 5  # seconds
        
    def set_color(self, r: int, g: int, b: int, w: int = 0) -> bool:
        """
        Set all LEDs to a specific color
        
        Args:
            r: Red value (0-255)
            g: Green value (0-255)
            b: Blue value (0-255)
            w: White value (0-255), default 0
            
        Returns:
            True if successful, False otherwise
        """
        url = f"{self.base_url}/api/led/color"
        payload = {
            "r": max(0, min(255, r)),
            "g": max(0, min(255, g)),
            "b": max(0, min(255, b)),
            "w": max(0, min(255, w))
        }
        
        try:
            response = requests.post(url, json=payload, timeout=self.timeout)
            response.raise_for_status()
            print(f"✓ Color set: RGBW({r}, {g}, {b}, {w})")
            return True
        except requests.exceptions.RequestException as e:
            print(f"✗ Error setting color: {e}", file=sys.stderr)
            return False
    
    def set_brightness(self, value: int) -> bool:
        """
        Set LED strip brightness
        
        Args:
            value: Brightness value (0-255)
            
        Returns:
            True if successful, False otherwise
        """
        url = f"{self.base_url}/api/led/brightness"
        payload = {"value": max(0, min(255, value))}
        
        try:
            response = requests.post(url, json=payload, timeout=self.timeout)
            response.raise_for_status()
            print(f"✓ Brightness set: {value}")
            return True
        except requests.exceptions.RequestException as e:
            print(f"✗ Error setting brightness: {e}", file=sys.stderr)
            return False
    
    def clear(self) -> bool:
        """
        Clear all LEDs (turn off)
        
        Returns:
            True if successful, False otherwise
        """
        url = f"{self.base_url}/api/led/clear"
        
        try:
            response = requests.post(url, timeout=self.timeout)
            response.raise_for_status()
            print("✓ LEDs cleared")
            return True
        except requests.exceptions.RequestException as e:
            print(f"✗ Error clearing LEDs: {e}", file=sys.stderr)
            return False
    
    def start_pattern(self, name: str, speed: int = 50) -> bool:
        """
        Start a LED pattern
        
        Args:
            name: Pattern name (rainbow, chase, fade, wave, sparkle)
            speed: Pattern speed (1-100, default 50)
            
        Returns:
            True if successful, False otherwise
        """
        url = f"{self.base_url}/api/led/pattern"
        payload = {
            "name": name,
            "speed": max(1, min(100, speed))
        }
        
        try:
            response = requests.post(url, json=payload, timeout=self.timeout)
            response.raise_for_status()
            print(f"✓ Pattern started: {name} (speed: {speed})")
            return True
        except requests.exceptions.RequestException as e:
            print(f"✗ Error starting pattern: {e}", file=sys.stderr)
            return False
    
    def stop_pattern(self) -> bool:
        """
        Stop current pattern
        
        Returns:
            True if successful, False otherwise
        """
        url = f"{self.base_url}/api/led/stop"
        
        try:
            response = requests.post(url, timeout=self.timeout)
            response.raise_for_status()
            print("✓ Pattern stopped")
            return True
        except requests.exceptions.RequestException as e:
            print(f"✗ Error stopping pattern: {e}", file=sys.stderr)
            return False
    
    def get_status(self) -> Optional[dict]:
        """
        Get ESP32-S3 status
        
        Returns:
            Status dictionary or None if error
        """
        url = f"{self.base_url}/api/status"
        
        try:
            response = requests.get(url, timeout=self.timeout)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            print(f"✗ Error getting status: {e}", file=sys.stderr)
            return None


def parse_color(color_str: str) -> Tuple[int, int, int, int]:
    """
    Parse color string (e.g., "255,0,0" or "255,0,0,0")
    
    Returns:
        Tuple of (r, g, b, w)
    """
    parts = [int(x.strip()) for x in color_str.split(",")]
    if len(parts) == 3:
        return (parts[0], parts[1], parts[2], 0)
    elif len(parts) == 4:
        return tuple(parts)
    else:
        raise ValueError("Color must be 3 or 4 comma-separated values")


def main():
    parser = argparse.ArgumentParser(
        description="Control ESP32-S3 LED strip via HTTP REST API",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Set LED to red
  %(prog)s --ip 192.168.1.100 --color 255 0 0
  
  # Set LED to white with white channel
  %(prog)s --ip 192.168.1.100 --color 0 0 0 255
  
  # Set brightness
  %(prog)s --ip 192.168.1.100 --brightness 128
  
  # Start rainbow pattern
  %(prog)s --ip 192.168.1.100 --pattern rainbow --speed 50
  
  # Clear LEDs
  %(prog)s --ip 192.168.1.100 --clear
  
  # Get status
  %(prog)s --ip 192.168.1.100 --status
        """
    )
    
    parser.add_argument(
        "--ip",
        required=True,
        help="IP address of ESP32-S3"
    )
    
    parser.add_argument(
        "--port",
        type=int,
        default=80,
        help="HTTP port (default: 80)"
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
    
    action_group.add_argument(
        "--status",
        action="store_true",
        help="Get device status"
    )
    
    parser.add_argument(
        "--speed",
        type=int,
        default=50,
        help="Pattern speed (1-100, default: 50)"
    )
    
    args = parser.parse_args()
    
    # Create controller
    controller = ESP32LEDController(args.ip, args.port)
    
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
    
    elif args.status:
        status = controller.get_status()
        if status:
            print(json.dumps(status, indent=2))
            success = True
    
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()

