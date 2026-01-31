# Product Definition: Dev Boards Workspace

## Vision
A unified, high-performance development workspace for the ESP32-S3 ecosystem, designed for a single "jack of all trades" developer managing firmware, systems, and UI components. The project aims to consolidate the development lifecycle into a streamlined, cross-platform environment capable of running on Windows 11 and Debian/Linux.

## Core Goals
- **Consolidation:** Centralize multiple ESP32-S3 and Arduino projects into a single managed workspace.
- **Enhanced DX:** Transition from legacy build systems to a modern, Rust-based Terminal User Interface (dev-console-v2).
- **Cross-Platform Portability:** Maintain a consistent development experience between Windows 11 and Debian/Linux environments.
- **Lifecycle Management:** Provide integrated tools for compiling, flashing, monitoring, and profile management.

## Key Features
- **Primary Interface:** dev-console-v2 serves as the centralized management hub (replacing the original dev-console).
- **Profile-Based Configuration:** Current management of connections, boards, and projects via config.yaml.
- **Integrated Serial Monitoring:** High-performance logging and interaction built directly into the Rust console.
- **Future-Proofing:** 
    - Eventual migration to tabbed project interfaces.
    - Rich port auto-detection (VID/PID/Manufacturer) integrated into the Profile management system.

## Target User
- A single developer handling the full stack: firmware (C++/Arduino/ESP-IDF), systems (environment setup/automation), and TUI (Rust).

## Architecture
- **Hardware Layer:** ESP32-S3 boards running Arduino or ESP-IDF firmware.
- **Management Layer:** Rust-based TUIs (dev-console-v2) providing the control plane.
- **Configuration Layer:** YAML-based profiles for project and environment settings.
