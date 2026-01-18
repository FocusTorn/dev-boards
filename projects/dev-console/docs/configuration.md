# Configuration Reference

This document provides a comprehensive reference for the dev-console configuration system, including all available options, their meanings, and examples.

## Configuration Files

The application uses two main configuration files:

1. **`config.yaml`** - Main application configuration (in project root)
2. **`settings.yaml`** - User settings (in user config directory)

## Main Configuration (config.yaml)

### Application Settings

```yaml
application:
    title: "ESP32-S3 Dev Console"
    min_width: 80
    min_height: 21
    bindings:
        - key: "[q]"
          description: "Quit"
    status_bar:
        default_text: "Status: Ready | [q] Quit"
        modal_text: "Status: Modal Mode"
```

#### Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | String | "ESP32-S3 Dev Console" | Application title displayed in the UI |
| `min_width` | Integer | 80 | Minimum terminal width required |
| `min_height` | Integer | 21 | Minimum terminal height required |
| `bindings` | Array | See below | Global keyboard bindings |
| `status_bar` | Object | See below | Status bar configuration |

#### Global Bindings

Global keyboard bindings available throughout the application:

```yaml
bindings:
    - key: "[q]"
      description: "Quit application"
    - key: "[F1]"
      description: "Show help"
    - key: "[Ctrl+s]"
      description: "Save settings"
```

**Binding Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key` | String | Yes | Key combination (format: `[Ctrl+Key]`, `[F1]`, etc.) |
| `description` | String | Yes | Human-readable description |

#### Status Bar Configuration

```yaml
status_bar:
    default_text: "Status: Ready | [q] Quit"
    modal_text: "Status: Modal Mode"
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `default_text` | String | "Status: Ready | [q] Quit" | Status text in normal mode |
| `modal_text` | String | "Status: Modal Mode" | Status text in modal states |

### Tab Bars Configuration

Tab bars define the navigation structure of the application:

```yaml
tab_bars:
    main_content_tab_bar:
        hwnd: "hwndMainContentTabBar"
        anchor: "hwndMainContentBox"
        alignment:
            vertical: "top"
            horizontal: "center"
            offset_x: 0
            offset_y: 0
        style: "tabbed"
        color: "cyan"
        min_tab_width: 8
        tab_tooltips: true
        navigation:
            left: ["Left"]
            right: ["Right"]
        tabs:
            - id: "dashboard"
              name: "Dashboard"
              default: "active"
              content:
                  type: "text"
                  value: "ESP32-S3 Development Settings"
            - id: "settings"
              name: "Settings"
              content:
                  type: "text"
                  value: "ESP32-S3 Development Settings"
```

#### Tab Bar Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hwnd` | String | Yes | Window handle identifier |
| `anchor` | String | Yes | Anchor element for positioning |
| `alignment` | Object | Yes | Positioning configuration |
| `style` | String | Yes | Tab bar style (`"tabbed"`, `"plain"`) |
| `color` | String | Yes | Color theme (`"cyan"`, `"blue"`, etc.) |
| `min_tab_width` | Integer | Yes | Minimum width of each tab |
| `tab_tooltips` | Boolean | Yes | Enable tab tooltips |
| `navigation` | Object | Yes | Navigation key bindings |
| `tabs` | Array | Yes | Tab definitions |

#### Alignment Configuration

```yaml
alignment:
    vertical: "top"      # "top", "center", "bottom"
    horizontal: "center" # "left", "center", "right"
    offset_x: 0          # Horizontal offset
    offset_y: 0          # Vertical offset
```

| Field | Type | Values | Description |
|-------|------|--------|-------------|
| `vertical` | String | "top", "center", "bottom" | Vertical alignment |
| `horizontal` | String | "left", "center", "right" | Horizontal alignment |
| `offset_x` | Integer | Any | Horizontal offset from alignment |
| `offset_y` | Integer | Any | Vertical offset from alignment |

#### Navigation Configuration

```yaml
navigation:
    left: ["Left", "h"]     # Keys to navigate left
    right: ["Right", "l"]   # Keys to navigate right
    up: ["Up", "k"]         # Keys to navigate up (if applicable)
    down: ["Down", "j"]     # Keys to navigate down (if applicable)
```

#### Tab Configuration

```yaml
tabs:
    - id: "dashboard"
      name: "Dashboard"
      default: "active"     # "active" or omitted
      content:
          type: "text"
          value: "ESP32-S3 Development Settings"
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | Yes | Unique tab identifier |
| `name` | String | Yes | Display name of the tab |
| `default` | String | No | Mark as default tab (`"active"`) |
| `content` | Object | Yes | Tab content configuration |

### Tab Content Configuration

Tab-specific key bindings and content:

```yaml
tab_content:
    - tab_id: "dashboard"
      bindings:
        - key: "[🡘 🡙]"
          description: "Navigate commands"
        - key: "[Enter]"
          description: "Execute command"
        - key: "[Esc]"
          description: "Cancel command"
    
    - tab_id: "settings"
      bindings:
        - key: "[🡙]"
          description: "Navigate profiles"
        - key: "[Tab]"
          description: "Next field"
        - key: "[Shift+Tab]"
          description: "Previous field"
        - key: "[Enter/Click]"
          description: "Confirm/Edit field"
        - key: "[s]"
          description: "Save profile"
        - key: "[l]"
          description: "Load profile"
        - key: "[r]"
          description: "Refresh profiles"
```

#### Tab Content Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tab_id` | String | Yes | Corresponds to tab ID from tab bar |
| `bindings` | Array | Yes | Tab-specific key bindings |

## User Settings (settings.yaml)

User settings are stored in `~/.config/dev-console/settings.yaml` (on Windows: `%APPDATA%\dev-console\settings.yaml`).

### Settings Structure

```yaml
# Project Configuration
sketch_directory: "/path/to/your/sketch"
sketch_name: "my-esp32-project"
env: "arduino"

# Hardware Configuration
board_model: "esp32-s3"
fqbn: "esp32:esp32:esp32s3"
port: "COM9"              # Windows: "COM9", Linux: "/dev/ttyUSB0", macOS: "/dev/cu.usbserial-XXXX"
baudrate: 115200

# General Settings
create_log: false

# MQTT Configuration (Optional)
mqtt_host: "localhost"
mqtt_port: 1883
mqtt_username: "mqtt"
mqtt_password: "mqtt"
mqtt_topic_command: "controller/esp32-s3-led/command"
mqtt_topic_state: "controller/esp32-s3-led/state"
mqtt_topic_status: "controller/esp32-s3-led/status"
```

### Settings Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sketch_directory` | String | "" | Path to Arduino sketch directory |
| `sketch_name` | String | "" | Name of the sketch/project |
| `env` | String | "arduino" | Development environment (`"arduino"`, `"platformio"`) |
| `board_model` | String | "esp32-s3" | Board model identifier |
| `fqbn` | String | "esp32:esp32:esp32s3" | Fully Qualified Board Name |
| `port` | String | "COM9" | Serial port for communication |
| `baudrate` | Integer | 115200 | Serial communication baud rate |
| `create_log` | Boolean | false | Enable logging during operations |
| `mqtt_host` | String (Optional) | None | MQTT broker hostname |
| `mqtt_port` | Integer (Optional) | None | MQTT broker port |
| `mqtt_username` | String (Optional) | None | MQTT authentication username |
| `mqtt_password` | String (Optional) | None | MQTT authentication password |
| `mqtt_topic_command` | String (Optional) | None | MQTT command topic |
| `mqtt_topic_state` | String (Optional) | None | MQTT state topic |
| `mqtt_topic_status` | String (Optional) | None | MQTT status topic |

## Profile Configuration

Profiles are stored as separate YAML files in `~/.config/dev-console/profiles/`:

### Profile File Structure

```yaml
# ~/.config/dev-console/profiles/my-profile.yaml
sketch_directory: "/path/to/specific/project"
sketch_name: "specific-project"
env: "platformio"
board_model: "esp32-s3"
fqbn: "esp32:esp32:esp32s3"
port: "/dev/ttyUSB0"
baudrate: 115200
create_log: true
mqtt_host: "broker.example.com"
mqtt_port: 1883
mqtt_username: "user"
mqtt_password: "pass"
mqtt_topic_command: "device/command"
mqtt_topic_state: "device/state"
mqtt_topic_status: "device/status"
```

## Configuration Validation

The application validates configuration files on startup:

### Validation Rules

1. **Required Fields**: All required fields must be present
2. **Data Types**: Fields must match expected types
3. **Value Ranges**: Numeric values must be within valid ranges
4. **Path Validation**: File paths must be valid (when applicable)
5. **Port Validation**: Serial ports must be valid format

### Error Handling

Configuration errors are handled gracefully:

```yaml
# Example of validation error handling
# If config.yaml is invalid, the application will:
# 1. Log the error
# 2. Use default values where possible
# 3. Continue startup with warnings
```

## Environment Variables

The application supports environment variable overrides:

### Supported Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DEV_CONSOLE_CONFIG` | Path to custom config.yaml | `/path/to/custom/config.yaml` |
| `DEV_CONSOLE_SETTINGS` | Path to custom settings.yaml | `/path/to/custom/settings.yaml` |
| `DEV_CONSOLE_LOG_LEVEL` | Logging level | `debug`, `info`, `warn`, `error` |
| `DEV_CONSOLE_PROFILE_DIR` | Custom profile directory | `/path/to/profiles` |

### Usage

```bash
# Use custom configuration
export DEV_CONSOLE_CONFIG="/path/to/my-config.yaml"
cargo run

# Set log level
export DEV_CONSOLE_LOG_LEVEL="debug"
cargo run

# Custom profile directory
export DEV_CONSOLE_PROFILE_DIR="/shared/profiles"
cargo run
```

## Configuration Examples

### Basic Development Setup

```yaml
# config.yaml - Basic setup
application:
    title: "My ESP32 Project"
    min_width: 100
    min_height: 25
    bindings:
        - key: "[q]"
          description: "Quit"
        - key: "[Ctrl+r]"
          description: "Refresh"

tab_bars:
    main_content_tab_bar:
        hwnd: "hwndMainContentTabBar"
        anchor: "hwndMainContentBox"
        alignment:
            vertical: "top"
            horizontal: "center"
            offset_x: 0
            offset_y: 0
        style: "tabbed"
        color: "blue"
        min_tab_width: 10
        tab_tooltips: true
        navigation:
            left: ["Left"]
            right: ["Right"]
        tabs:
            - id: "dashboard"
              name: "Dashboard"
              default: "active"
            - id: "settings"
              name: "Settings"

tab_content:
    - tab_id: "dashboard"
      bindings:
        - key: "[Enter]"
          description: "Execute"
    - tab_id: "settings"
      bindings:
        - key: "[Tab]"
          description: "Next field"
```

### Advanced MQTT Setup

```yaml
# settings.yaml - Advanced MQTT configuration
sketch_directory: "/home/user/esp32-projects/sensor-node"
sketch_name: "sensor-node"
env: "arduino"
board_model: "esp32-s3"
fqbn: "esp32:esp32:esp32s3"
port: "/dev/ttyUSB0"
baudrate: 115200
create_log: true

# MQTT Configuration
mqtt_host: "mqtt.broker.com"
mqtt_port: 8883
mqtt_username: "sensor_user"
mqtt_password: "secure_password"
mqtt_topic_command: "sensors/node-001/command"
mqtt_topic_state: "sensors/node-001/state"
mqtt_topic_status: "sensors/node-001/status"
```

### Multi-Environment Profiles

```yaml
# profiles/development.yaml
sketch_directory: "/home/user/projects/esp32-dev"
sketch_name: "esp32-dev"
env: "platformio"
board_model: "esp32-s3"
fqbn: "esp32:esp32:esp32s3"
port: "/dev/ttyUSB0"
baudrate: 115200
create_log: true

# profiles/production.yaml
sketch_directory: "/home/user/projects/esp32-prod"
sketch_name: "esp32-prod"
env: "arduino"
board_model: "esp32-s3"
fqbn: "esp32:esp32:esp32s3"
port: "/dev/ttyUSB1"
baudrate: 9600
create_log: false
```

## Configuration Migration

### Version Compatibility

The configuration system supports backward compatibility:

```yaml
# Migration handling
# - Missing fields are filled with defaults
# - Deprecated fields are ignored with warnings
# - New fields are optional unless required
```

### Upgrade Process

When upgrading versions:

1. **Backup existing configuration**
2. **Review changelog for breaking changes**
3. **Update configuration as needed**
4. **Test with new version**

## Troubleshooting

### Common Configuration Issues

#### 1. Invalid YAML Syntax

```yaml
# Invalid - missing colon after key
application
    title: "My App"

# Valid
application:
    title: "My App"
```

#### 2. Missing Required Fields

```yaml
# Error: Missing required 'hwnd' field
tab_bars:
    main_content_tab_bar:
        anchor: "hwndMainContentBox"
        # Missing: hwnd: "hwndMainContentTabBar"
```

#### 3. Invalid Data Types

```yaml
# Error: min_width should be integer
application:
    min_width: "80"  # Should be 80 without quotes
```

#### 4. Path Issues

```yaml
# Windows paths need proper escaping or quotes
sketch_directory: "C:\\Users\\User\\Documents\\Arduino"
# or
sketch_directory: 'C:\Users\User\Documents\Arduino'
```

### Debug Configuration

Enable debug logging to troubleshoot configuration issues:

```bash
# Set debug log level
export DEV_CONSOLE_LOG_LEVEL="debug"
cargo run

# Check configuration loading
# Debug output will show:
# - Configuration file path
# - Parsed configuration values
# - Validation errors
# - Default values applied
```

### Configuration Validation Tool

Use the built-in validation:

```bash
# Validate configuration
cargo run -- --validate-config

# Validate specific file
cargo run -- --validate-config --config /path/to/config.yaml
```

This configuration reference provides comprehensive information for customizing and troubleshooting the dev-console application configuration.
