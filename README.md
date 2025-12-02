# Dev Boards Development Environment

This workspace contains development environments for ESP32-S3 and Arduino development.

## Directory Structure

```
dev-boards/
├── Arduino/          # Arduino CLI environment
│   ├── home/         # Arduino user directory
│   ├── libraries/    # Custom Arduino libraries
│   ├── sketchbook/   # Arduino sketches/projects
│   └── _utilities_/  # Utility scripts
├── esp-idf/          # ESP-IDF framework
└── README.md         # This file
```

## Quick Start

### Arduino CLI Setup

1. Navigate to the `Arduino` directory
2. Run the setup script:
   ```powershell
   cd Arduino
   .\setup-arduino-cli.ps1
   ```

See `Arduino/README.md` for detailed instructions.

### ESP-IDF Setup

1. Navigate to the `esp-idf` directory
2. Run the setup script:
   ```powershell
   cd esp-idf
   .\setup-esp-idf.ps1
   ```

See `esp-idf/README.md` for detailed instructions.

## Environment Setup

### Arduino CLI

The Arduino CLI environment is structured to match the Raspberry Pi setup:
- User data in `Arduino/home/`
- Custom libraries in `Arduino/libraries/`
- Sketches in `Arduino/sketchbook/`

### ESP-IDF

The ESP-IDF environment is set up fresh for ESP32-S3 development:
- Framework in `esp-idf/`
- Projects should be created in a separate directory (e.g., `esp32-projects/`)

## Daily Usage

### Arduino CLI

```powershell
cd Arduino
.\arduino-cli.exe compile --fqbn esp32:esp32:esp32s3 sketchbook\MyProject
.\arduino-cli.exe upload -p COM3 --fqbn esp32:esp32:esp32s3 sketchbook\MyProject
```

### ESP-IDF

```powershell
cd esp-idf
.\export.bat
cd ..\esp32-projects\my_project
idf.py build
idf.py -p COM3 flash monitor
```

## Notes

- Both environments are configured for ESP32-S3 development
- Arduino CLI structure mirrors the Raspberry Pi setup
- ESP-IDF is a fresh installation
- Remember to run `esp-idf\export.bat` in each new terminal session for ESP-IDF

