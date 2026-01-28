# Technology Stack: Dev Boards Workspace

## 1. Programming Languages
- **Rust:** Primary language for the management TUI (dev-console-v2).
- **C++:** Primary language for firmware development (Arduino sketches and ESP-IDF components).
- **Python:** Used for utility scripts and environment automation.

## 2. Embedded Ecosystem
- **Hardware:** ESP32-S3, Arduino Nano.
- **Frameworks:**
    - **ESP-IDF:** Native development for ESP32-S3.
    - **Arduino CLI:** Cross-platform tool for compiling and uploading sketches to both ESP32 and Nano.

## 3. TUI & CLI Development
- **Ratatui:** Rust library used for building the terminal user interface in dev-console-v2.
- **Crossterm:** Terminal backend for cross-platform support.

## 4. Tooling & Environment
- **uv:** Python package and tool manager for managing project scripts and dependencies.
- **Git:** Version control for the workspace and sub-projects.
- **YAML (Serde):** Configuration management for project profiles.

## 5. Deployment & Build
- **Platform Support:** Windows 11 (primary current), Debian/Linux (target).
