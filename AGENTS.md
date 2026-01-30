# Dev Boards Development Environment - Agent Context

## Project Overview

This workspace serves as a comprehensive development environment primarily focused on ESP32-S3 and Arduino development. It integrates Rust applications for development console functionalities and Python tooling for build system automation and environment management. The project is structured to support mixed-technology development, featuring shared resources and modular components.

### Key Technologies:
- **Embedded:** Arduino (for ESP32-S3), ESP-IDF (for ESP32-S3)
- **Rust:** `dev-console`, `dev-console-v2` (Text User Interface applications)
- **Python:** `uv` (package and tool manager), `outerm`, `pyprompt` (shared libraries)
- **Scripting:** PowerShell for Windows environment setup
- **3D Printing:** Bambu A1 printer tuning and calibration

### Architecture:
The project employs a modular architecture with distinct directories for different components:
- `Arduino/`: Contains the Arduino CLI environment, libraries, and sketches.
- `esp-idf/`: Dedicated to the ESP-IDF framework.
- `projects/`: Houses various sub-projects, including Rust applications like `dev-console` and `dev-console-v2`.
- `lib/`: Contains shared Python libraries.
- `_shared-resources/`: Centralized location for shared code and assets across different language ecosystems (e.g., `shared-rust`, `shared-python`).
- `_spec/`: Contains project specifications, tracks, and plans.
- `scripts/`: Utility scripts for various development tasks.

## Building and Running

### 3D Printing (Bambu A1)
The project includes a comprehensive tuning roadmap for high-performance materials (like ASA) in the `_docs/A1 Tuning` directory.

#### Core Principles for A1 Tuning:
- **Showcase Priority:** Prioritize motion stability (Acceleration < 3000 mm/s²) **before** material calibration to ensure tests reflect final print conditions.
- **Sequential Calibration:** Follow the roadmap: Volumetric Flow -> Motion -> Flow Rate -> Pressure Advance -> Feature Tuning.
- **Enclosure Awareness:** When printing in an enclosure (~43°C), prioritize cooling for overhangs and monitor base electronics for thermal stress.
- **Software Preference:** Use **Orca Slicer** for advanced calibration tools (Flow, PA, Max Flow tests).
- **Material Specifics:** Overture ASA requires ~260°C nozzle and ~100°C bed for optimal layer bonding.

### Arduino Projects

#### Setup
To set up the Arduino CLI environment, navigate to the `Arduino` directory and run the PowerShell setup script:
```powershell
cd Arduino
.\setup-arduino-cli.ps1
```
This script will ensure `arduino-cli.exe` is present, configure its settings, and install the necessary ESP32 core.

#### Daily Usage
For compiling and uploading Arduino sketches:
```powershell
cd Arduino
.\arduino-cli.exe compile --fqbn esp32:esp32:esp32s3 sketchbook\MyProject
.\arduino-cli.exe upload -p COM3 --fqbn esp32:esp32:esp32s3 sketchbook\MyProject
```
*(Replace `MyProject` with your sketch name and `COM3` with your ESP32's serial port.)*

### ESP-IDF Projects

The ESP-IDF environment is configured for ESP32-S3 development.

#### Setup
Refer to `esp-idf/README.md` for detailed setup instructions. Typically, this involves running a setup script within the `esp-idf` directory.

#### Daily Usage
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

### Rust Applications (`dev-console`, `dev-console-v2`)

The Rust applications are located under the `projects/` directory.

#### Building
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

#### Running
After building, you can run the applications from their `target/debug` or `target/release` directories, or by using `cargo run`:
```bash
cd projects/dev-console
cargo run
```

### UV Tool Management

The project utilizes `uv` for managing Python packages and development tools. The `scripts/ensure-uv-tools.py` (though currently commented out) indicates a convention for ensuring `uv` tools specified in `pyproject.toml` are installed.

#### Setup
From the workspace root (`D:\_dev\projects\dev-boards`):
```powershell
# Create workspace-level virtual environment
uv venv

# Activate the workspace venv
.\.venv\Scripts\Activate.ps1

# Install workspace dependencies
uv sync
```

## Key Components

### Rust TUI Applications

#### dev-console (projects/dev-console)
- A powerful Terminal User Interface (TUI) application for managing ESP32-S3 development workflows
- Features: Dashboard interface, settings management, real-time monitoring, profile system, modern TUI with mouse support
- Architecture: Modular design with main loop, UI components, settings management, command execution framework
- Configuration: Uses YAML configuration files, with settings stored in user's config directory

#### dev-console-v2 (projects/dev-console2)
- A terminal user interface (TUI) application built with Rust using the Elm architecture pattern and Ratatui library
- Features: Elm architecture (Model, Update, View), responsive layout, component-based UI, event handling
- Architecture: Clean separation of Model, Update, and View components
- Dependencies: `ratatui`, `crossterm`, `color-eyre`

### Python Development Tools

#### Workspace Structure
- **Workspace-level venv**: Shared dependencies (like `prompt-toolkit`) used across multiple projects
- **Project-level venvs**: Individual projects can extend the workspace dependencies with their own specific needs
- **Shared Libraries**: `outerm` and `pyprompt` libraries in `_shared-resources/shared-python/`

### Embedded Development Environment

#### Arduino Environment (Arduino/)
- Contains Arduino CLI setup for cross-platform compilation and uploading
- Structured similarly to Raspberry Pi environment
- Supports ESP32-S3 development with proper board support

#### ESP-IDF Environment (esp-idf/)
- Native development environment for ESP32-S3
- Full-featured framework with build system, tools, and libraries

### Device Management & Profiles
Based on the implementation plan in `_spec/tracks/device_mgmt_20260127/plan.md`:
- Port detection infrastructure using serial port metadata
- Sidebar-based settings framework
- Quick Pick & Modal dimming features
- Profile CRUD & Persistence operations
- Configuration stored in YAML format

## Development Conventions

- **Python Environment Management:** `uv` is the preferred tool for Python package and tool manager.
- **Modular Codebase:** The project is organized into distinct modules and sub-projects, with `_shared-resources` facilitating code reuse across different parts of the workspace.
- **Windows Scripting:** PowerShell scripts (`.ps1`) are used for automating environment setup and tasks on Windows.
- **Type Checking:** The presence of `pyrightconfig.json` and commented sections in `pyproject.toml` suggest the use of `basedpyright` for static type checking in Python code.
- **Code Formatting/Linting:** The `editorconfig` file suggests adherence to consistent code styling across different editors.
- **Version Control:** Git is used for version control with a typical workflow.
- **Testing:** The project supports TDD (Test Driven Development) as evidenced by test plans in the specification files.

## Workspace Configuration

### Python Dependencies
The root `pyproject.toml` specifies workspace-level dependencies:
- `outerm`: Located at `_shared-resources/shared-python/outerm`
- `pyprompt`: Located at `_shared-resources/shared-python/pyprompt`

### Shared Resources
- `_shared-resources/` contains shared bootstraps, cursor configurations, Python libraries, and Rust components
- This enables code reuse across different parts of the workspace

## Skills and Specializations

The workspace includes specialized skills for different development tasks:
- `3d-tuning-router.skill`: 3D printing workflow management
- `bambu-tuning-pro.skill`: Bambu A1 printer tuning and calibration
- These skills enhance the development experience for specific domains

## Current Development Tracks

Based on the `_spec/tracks/` directory, the project has several active development tracks:
- `device_mgmt_20260127`: Device management and profile UI
- `hardening_coverage_20260128`: Code hardening and test coverage
- `testing_100_coverage_20260128`: Achieving 100% test coverage

This comprehensive development environment provides a complete solution for ESP32-S3 development, from embedded programming to device management, with a focus on creating robust, maintainable tooling.