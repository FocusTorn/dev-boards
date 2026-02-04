
# 1. :: Mandatory Rules

**CRITICAL:** This workspace is governed by a set of strict, non-negotiable integrity rules. You MUST load and adhere to these rules before executing any tool calls or proposing code changes.

*   **[Workspace Rules Orchestrator](.gemini/rules/_readme.md)**: **MANDATORY READING.** This is the central source of truth for all active standards (Code Integrity, Formatting, Stability, and Shell Usage).
*   **Compliance Requirement**: Every response must be verified against the standards defined in the Orchestrator. Failure to comply with section numbering, title formatting, or code-elision rules is considered a system-level failure.








# Project Overview

This workspace serves as a comprehensive development environment primarily focused on ESP32-S3 and Arduino development. It integrates Rust applications for development console functionalities and Python tooling for build system automation and environment management. The project is structured to support mixed-technology development, featuring shared resources and modular components.

**Key Technologies:**

*   **Embedded:** Arduino (for ESP32-S3), ESP-IDF (for ESP32-S3)
*   **Rust:** `dev-console`, `dev-console-v2` (Text User Interface applications)
*   **Python:** `uv` (package and tool manager), `outerm`, `pyprompt` (shared libraries)
*   **Scripting:** PowerShell for Windows environment setup

**Architecture:**

The project employs a modular architecture with distinct directories for different components:
*   `Arduino/`: Contains the Arduino CLI environment, libraries, and sketches.
*   `esp-idf/`: Dedicated to the ESP-IDF framework.
*   `projects/`: Houses various sub-projects, including Rust applications like `dev-console` and `dev-console-v2`.
*   `lib/`: Contains shared Python libraries.
*   `_shared-resources/`: Centralized location for shared code and assets across different language ecosystems (e.g., `shared-rust`, `shared-python`).

# Building and Running

## 3D Printing (Bambu A1)
The project includes a comprehensive tuning roadmap for high-performance materials (like ASA) in the `_docs/A1 Tuning` directory.

### Core Principles for A1 Tuning:
*   **Showcase Priority:** Prioritize motion stability (Acceleration < 3000 mm/s²) **before** material calibration to ensure tests reflect final print conditions.
*   **Sequential Calibration:** Follow the roadmap: Volumetric Flow -> Motion -> Flow Rate -> Pressure Advance -> Feature Tuning.
*   **Enclosure Awareness:** When printing in an enclosure (~43°C), prioritize cooling for overhangs and monitor base electronics for thermal stress.
*   **Software Preference:** Use **Orca Slicer** for advanced calibration tools (Flow, PA, Max Flow tests).
*   **Material Specifics:** Overture ASA requires ~260°C nozzle and ~100°C bed for optimal layer bonding.

## Arduino Projects
...

### Setup
To set up the Arduino CLI environment, navigate to the `Arduino` directory and run the PowerShell setup script:
```powershell
cd Arduino
.\setup-arduino-cli.ps1
```
This script will ensure `arduino-cli.exe` is present, configure its settings, and install the necessary ESP32 core.

### Daily Usage
For compiling and uploading Arduino sketches:
```powershell
cd Arduino
.\arduino-cli.exe compile --fqbn esp32:esp32:esp32s3 sketchbook\MyProject
.\arduino-cli.exe upload -p COM3 --fqbn esp32:esp32:esp32s3 sketchbook\MyProject
```
*(Replace `MyProject` with your sketch name and `COM3` with your ESP32's serial port.)*

## ESP-IDF Projects

The ESP-IDF environment is configured for ESP32-S3 development.

### Setup
Refer to `esp-idf/README.md` for detailed setup instructions. Typically, this involves running a setup script within the `esp-idf` directory.

### Daily Usage
Before working with ESP-IDF projects in a new terminal session, you must run the export script:
```powershell
cd esp-idf
.\export.bat
```
Then, navigate to your project directory and use `idf.py` for building and flashing:
```powershell
cd ..\esp32-projects\my_project
idf.py build
idf.py -p COM3 flash monitor
```
*(Replace `esp32-projects\my_project` with your project path and `COM3` with your ESP32's serial port.)*

## Rust Applications (`dev-console`, `dev-console-v2`)

The Rust applications are located under the `projects/` directory.

### Building
To build a specific Rust application, navigate to its directory and use Cargo:
```bash
cd projects/dev-console
cargo build
# or for a release build
cargo build --release
```
Similarly for `dev-console-v2`:
```bash
cd projects/dev-console2
cargo build
```

### Running
After building, you can run the applications from their `target/debug` or `target/release` directories, or by using `cargo run`:
```bash
cd projects/dev-console
cargo run
```

## UV Tool Management

The project utilizes `uv` for managing Python packages and development tools. The `scripts/ensure-uv-tools.py` (though currently commented out) indicates a convention for ensuring `uv` tools specified in `pyproject.toml` are installed.

# Development Conventions

*   **Python Environment Management:** `uv` is the preferred tool for Python package and tool manager.
*   **Modular Codebase:** The project is organized into distinct modules and sub-projects, with `_shared-resources` facilitating code reuse across different parts of the workspace.
*   **Windows Scripting:** PowerShell scripts (`.ps1`) are used for automating environment setup and tasks on Windows.
*   **Type Checking:** The presence of `pyrightconfig.json` and commented sections in `pyproject.toml` suggest the use of `basedpyright` for static type checking in Python code.
*   **Code Formatting/Linting:** Although not explicitly detailed here, `editorconfig` suggests adherence to consistent code styling across different editors.

