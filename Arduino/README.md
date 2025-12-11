# Arduino CLI Environment

This directory contains the Arduino CLI setup, structured similarly to the Raspberry Pi environment.

## Directory Structure

- `arduino-cli` - Arduino CLI executable (download separately)
- `home/` - Arduino home directory (user-specific data)
- `libraries/` - Custom Arduino libraries
- `sketchbook/` - Arduino sketches/projects
- `_utilities_/` - Utility scripts and tools

## Setup Instructions

### 1. Download Arduino CLI

Download the latest Arduino CLI for Windows from:
https://arduino.github.io/arduino-cli/latest/installation/

Place the `arduino-cli.exe` file in this directory.

### 2. Initialize Arduino CLI

Run the following command to initialize Arduino CLI with this directory as the user directory:

```powershell
.\arduino-cli.exe config init --overwrite
.\arduino-cli.exe config set directories.user "D:\_dev\projects\dev-boards\Arduino\home"
.\arduino-cli.exe config set directories.data "D:\_dev\projects\dev-boards\Arduino"
```

### 3. Install ESP32-S3 Board Support

```powershell
.\arduino-cli.exe core update-index
.\arduino-cli.exe core install esp32:esp32
```

### 4. Verify Installation

```powershell
.\arduino-cli.exe board listall | Select-String "esp32"
```

## Usage

### Create a new sketch

```powershell
.\arduino-cli.exe sketch new sketchbook\MyProject
```

### Compile a sketch

```powershell
.\arduino-cli.exe compile --fqbn esp32:esp32:esp32s3 sketchbook\MyProject
```

### Upload a sketch

```powershell
.\arduino-cli.exe upload -p COM3 --fqbn esp32:esp32:esp32s3 sketchbook\MyProject
```

## Configuration

The Arduino CLI configuration file is typically located at:
`%APPDATA%\Arduino15\arduino-cli.yaml`

You can also create a local `arduino-cli.yaml` in this directory for project-specific settings.

