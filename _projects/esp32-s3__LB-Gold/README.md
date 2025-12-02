# ESP32-S3 (LB-Gold) Projects

Projects for the ESP32-S3 Lonely Binary Gold Edition board.

## Projects

- **sht21-solo** - SHT21/HTU21 temperature and humidity sensor with MQTT
- **sk6822-solo** - SK6812 RGBW LED strip controller with MQTT

## Quick Start

### Using Makefiles

Each project has a Makefile for easy building:

```powershell
# Navigate to project directory
cd sht21-solo
# or
cd sk6822-solo

# Compile
make compile

# Upload
make upload

# Compile and upload in one step
make all

# Open serial monitor
make monitor

# Clean build artifacts
make clean

# Show help
make help
```

### Available Make Targets

- `make` or `make help` - Show available targets
- `make compile` - Compile the sketch
- `make upload` - Upload to ESP32-S3 (compiles first)
- `make monitor` - Open serial monitor
- `make all` - Compile and upload
- `make clean` - Clean build artifacts
- `make verify` - Verify sketch syntax
- `make board-info` - Show board information
- `make list-boards` - List connected boards

### Configuration

Edit the Makefile to change:
- `PORT` - Serial port (default: COM9)
- `BAUDRATE` - Serial monitor baud rate (default: 115200)
- `FQBN` - Board FQBN (default: esp32:esp32:esp32s3)

Example:
```makefile
PORT = COM10        # Change port
BAUDRATE = 9600     # Change baud rate
```

## Requirements

- Arduino CLI installed and in PATH
- ESP32 board support installed
- Required libraries installed (see individual project READMEs)

## Library Path

The `MQTT_RPi_Client` library is located at:
`D:\_dev\_Projects\dev-boards\_libs\esp32-s3\MQTT_RPi_Client\`

The Makefile automatically includes this path when compiling.

