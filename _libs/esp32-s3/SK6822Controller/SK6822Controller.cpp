/*
 * SK6822Controller Implementation
 */

#include "SK6822Controller.h"

SK6822Controller::SK6822Controller(uint8_t pin, uint16_t count, uint8_t brightness) :
  _strip(count, pin, NEO_GRBW + NEO_KHZ800),
  _pin(pin),
  _count(count),
  _brightness(brightness),
  _pattern_running(false),
  _current_pattern(""),
  _pattern_start_time(0),
  _pattern_speed(50)
{
  _state.brightness = brightness;
}

void SK6822Controller::begin() {
  _strip.begin();
  _strip.setBrightness(_brightness);
  _strip.clear();
  _strip.show();
}

void SK6822Controller::setColor(uint8_t r, uint8_t g, uint8_t b, uint8_t w) {
  for (int i = 0; i < _count; i++) {
    _strip.setPixelColor(i, _strip.Color(r, g, b, w));
  }
  _strip.show();
  
  // Update state
  _state.r = r;
  _state.g = g;
  _state.b = b;
  _state.w = w;
  _state.pattern = "";
  _state.pattern_active = false;
}

void SK6822Controller::setPixel(int index, uint8_t r, uint8_t g, uint8_t b, uint8_t w) {
  if (index >= 0 && index < _count) {
    _strip.setPixelColor(index, _strip.Color(r, g, b, w));
  }
}

void SK6822Controller::clear() {
  _strip.clear();
  _strip.show();
  
  // Update state
  _state.r = 0;
  _state.g = 0;
  _state.b = 0;
  _state.w = 0;
  _state.pattern = "";
  _state.pattern_active = false;
}

void SK6822Controller::show() {
  _strip.show();
}

void SK6822Controller::setBrightness(uint8_t brightness) {
  if (brightness > 255) brightness = 255;
  _brightness = brightness;
  _state.brightness = brightness;
  _strip.setBrightness(brightness);
  _strip.show();
}

void SK6822Controller::setBrightnessPercent(uint8_t percent) {
  if (percent > 100) percent = 100;
  uint8_t brightness = (percent * 255) / 100;
  setBrightness(brightness);
}

uint8_t SK6822Controller::getBrightness() const {
  return _brightness;
}

void SK6822Controller::startPattern(const String& pattern_name, int speed) {
  _pattern_running = true;
  _current_pattern = pattern_name;
  _pattern_start_time = millis();
  _pattern_speed = speed;
  
  _state.pattern = pattern_name;
  _state.pattern_active = true;
}

void SK6822Controller::stopPattern() {
  _pattern_running = false;
  _current_pattern = "";
  
  _state.pattern = "";
  _state.pattern_active = false;
}

bool SK6822Controller::isPatternRunning() const {
  return _pattern_running;
}

String SK6822Controller::getCurrentPattern() const {
  return _current_pattern;
}

void SK6822Controller::update() {
  if (_pattern_running) {
    updatePattern();
  }
}

LEDState SK6822Controller::getState() const {
  return _state;
}

void SK6822Controller::setState(const LEDState& state) {
  _state = state;
  _brightness = state.brightness;
  _strip.setBrightness(_brightness);
  
  if (state.pattern_active) {
    _pattern_running = true;
    _current_pattern = state.pattern;
  } else {
    _pattern_running = false;
    _current_pattern = "";
  }
}

void SK6822Controller::showStatus(uint8_t r, uint8_t g, uint8_t b, uint8_t w, unsigned long duration) {
  setColor(r, g, b, w);
  delay(duration);
  clear();
}

void SK6822Controller::updatePattern() {
  if (!_pattern_running) return;
  
  unsigned long elapsed = millis() - _pattern_start_time;
  
  if (_current_pattern == "rainbow") {
    rainbowPattern(elapsed);
  } else if (_current_pattern == "chase") {
    chasePattern(elapsed);
  } else if (_current_pattern == "fade") {
    fadePattern(elapsed);
  } else if (_current_pattern == "wave") {
    wavePattern(elapsed);
  } else if (_current_pattern == "sparkle") {
    sparklePattern(elapsed);
  }
}

void SK6822Controller::rainbowPattern(unsigned long elapsed) {
  for (int i = 0; i < _count; i++) {
    int hue = (elapsed / 10 + i * 256 / _count) % 256;
    uint32_t color = Wheel(hue);
    _strip.setPixelColor(i, color);
  }
  _strip.show();
}

void SK6822Controller::chasePattern(unsigned long elapsed) {
  _strip.clear();
  int pos = (elapsed / 50) % (_count * 2);
  if (pos < _count) {
    setPixel(pos, 255, 0, 0, 0); // Red
  } else {
    setPixel(_count * 2 - pos - 1, 0, 0, 255, 0); // Blue
  }
  _strip.show();
}

void SK6822Controller::fadePattern(unsigned long elapsed) {
  int brightness = (sin(elapsed / 50.0) + 1.0) * 127.5;
  _strip.setBrightness(brightness);
  setColor(255, 255, 255, 0); // White
  _strip.show();
  _strip.setBrightness(_brightness); // Restore brightness
}

void SK6822Controller::wavePattern(unsigned long elapsed) {
  for (int i = 0; i < _count; i++) {
    float wave = (sin((elapsed / 20.0) + (i * 0.5)) + 1.0) / 2.0;
    uint8_t r = wave * 255;
    uint8_t g = wave * 128;
    uint8_t b = wave * 64;
    setPixel(i, r, g, b, 0);
  }
  _strip.show();
}

void SK6822Controller::sparklePattern(unsigned long elapsed) {
  static unsigned long last_sparkle = 0;
  if (elapsed - last_sparkle > 100) {
    _strip.clear();
    for (int i = 0; i < 5; i++) {
      int pos = random(_count);
      setPixel(pos, 255, 255, 255, 0);
    }
    _strip.show();
    last_sparkle = elapsed;
  }
}

// Helper function for rainbow effect
uint32_t SK6822Controller::Wheel(byte WheelPos) {
  WheelPos = 255 - WheelPos;
  if (WheelPos < 85) {
    return _strip.Color(255 - WheelPos * 3, 0, WheelPos * 3, 0);
  }
  if (WheelPos < 170) {
    WheelPos -= 85;
    return _strip.Color(0, WheelPos * 3, 255 - WheelPos * 3, 0);
  }
  WheelPos -= 170;
  return _strip.Color(WheelPos * 3, 255 - WheelPos * 3, 0, 0);
}

