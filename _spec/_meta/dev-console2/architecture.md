# Architecture: dev-console2

## Overview
`dev-console2` is a Rust-based TUI application designed for managing development boards (ESP32/Arduino). It follows the **Elm Architecture** (Model-Update-View) to maintain a clean separation of concerns.

## Structural Patterns
- **Module System**: The app is organized into sub-modules under `src/app/` (e.g., `mod.rs`, `view.rs`, `system.rs`, `ansi.rs`).
- **Elm Architecture**:
    - **Model**: `App` struct (State).
    - **Update**: `App::update` and semantic `Action` dispatching.
    - **View**: `App::view` and modular `render_*` methods.
- **Widget-Driven UI**: UI components are encapsulated as "Widgets" in `src/widgets/`, making them reusable across different layouts.

## Configuration
- **Application Config (`build-config.yaml`)**: Defines the layout, tab-bars, and global/tab-specific keybindings.
- **Hardware Profiles (`config.yaml`)**: Manages device connections, sketch paths, and MQTT settings.
