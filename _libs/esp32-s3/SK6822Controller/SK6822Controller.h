/*
 * SK6822Controller - Library for controlling SK6812/SK6822 RGBW LED strips
 * 
 * This library provides a simple interface for controlling RGBW LED strips
 * using the Adafruit NeoPixel library. It supports color control, brightness,
 * and various animated patterns.
 * 
 * Features:
 * - RGBW color control
 * - Brightness control (0-255)
 * - Multiple animated patterns (rainbow, chase, fade, wave, sparkle)
 * - State tracking
 * 
 * Usage:
 *   #include <SK6822Controller.h>
 *   
 *   SK6822Controller leds(6, 31);  // pin, count
 *   
 *   void setup() {
 *     leds.begin();
 *     leds.setColor(255, 0, 0, 0);  // Red
 *   }
 *   
 *   void loop() {
 *     leds.update();  // Call in loop if patterns are running
 *   }
 */

#ifndef SK6822_CONTROLLER_H
#define SK6822_CONTROLLER_H

#include <Adafruit_NeoPixel.h>

// LED State structure
struct LEDState {
  uint8_t r = 0;
  uint8_t g = 0;
  uint8_t b = 0;
  uint8_t w = 0;
  uint8_t brightness = 50;
  String pattern = "";
  bool pattern_active = false;
};

class SK6822Controller {
public:
  // Constructor
  SK6822Controller(uint8_t pin, uint16_t count, uint8_t brightness = 50);
  
  // Initialization
  void begin();
  
  // Color control
  void setColor(uint8_t r, uint8_t g, uint8_t b, uint8_t w = 0);
  void setPixel(int index, uint8_t r, uint8_t g, uint8_t b, uint8_t w = 0);
  void clear();
  void show();
  
  // Brightness control
  void setBrightness(uint8_t brightness);
  void setBrightnessPercent(uint8_t percent);  // 0-100
  uint8_t getBrightness() const;
  
  // Pattern control
  void startPattern(const String& pattern_name, int speed = 50);
  void stopPattern();
  bool isPatternRunning() const;
  String getCurrentPattern() const;
  
  // Update (call in loop() for patterns)
  void update();
  
  // State management
  LEDState getState() const;
  void setState(const LEDState& state);
  
  // Status LED helper
  void showStatus(uint8_t r, uint8_t g, uint8_t b, uint8_t w = 0, unsigned long duration = 500);
  
  // Get underlying strip (for advanced usage)
  Adafruit_NeoPixel& getStrip() { return _strip; }

private:
  Adafruit_NeoPixel _strip;
  uint8_t _pin;
  uint16_t _count;
  uint8_t _brightness;
  
  // Pattern state
  bool _pattern_running;
  String _current_pattern;
  unsigned long _pattern_start_time;
  int _pattern_speed;
  
  // LED state
  LEDState _state;
  
  // Pattern functions
  void updatePattern();
  void rainbowPattern(unsigned long elapsed);
  void chasePattern(unsigned long elapsed);
  void fadePattern(unsigned long elapsed);
  void wavePattern(unsigned long elapsed);
  void sparklePattern(unsigned long elapsed);
  
  // Helper function for rainbow
  uint32_t Wheel(byte WheelPos);
};

#endif

