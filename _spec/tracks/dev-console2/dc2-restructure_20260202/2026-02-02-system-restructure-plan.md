# Restructure Plan: Dev-Console V2 Architecture & Widget Reorganization

## 1. Overview
This plan outlines the phased transformation of `dev-console-v2` from a monolithic TUI structure into a modular, decoupled architecture. The primary goals are to reduce boilerplate, improve separation of concerns, and establish a scalable pattern for "Smart" components versus "Dumb" elements.

## 2. Phase 1: Widget Tiering & Directory Cleanup
Reorganize `src/widgets/` to distinguish between pure UI elements and stateful components.

### 2.1 Directory Structure
- **`src/widgets/elements/`**: Stateless "Dumb" widgets (Pure View).
    - `progress_bar.rs`, `status_box.rs`, `dimmer.rs`.
- **`src/widgets/components/`**: Stateful "Smart" widgets (Encapsulated logic).
    - Each gets its own directory: `toast/`, `file_browser/`, `command_list/`.
    - Contents: `mod.rs` (Logic), `README.md` (Docs), `config.yaml` (Defaults).

### 2.2 Modernization
- Adopt `use ratatui::prelude::*;` across all widget files.
- Standardize `InteractiveWidget` trait outcomes to reduce `App` logic overhead.

## 3. Phase 2: Breaking Down "God Objects"
Decompose the three primary monolithic files that currently handle too many responsibilities.








### 3.1 `src/app/mod.rs` (State & Routing)
- **Problem**: 1200+ lines handling input, state, and widget routing.
- **Solution**:
    - Move `Action` definitions to a dedicated `src/app/actions.rs`.
    - Split `update()` logic into functional modules (e.g., `src/app/router.rs` for focus/input delegation).
    - Move `App` state initialization to `src/app/state.rs`.

### 3.2 `src/app/executors.rs` (Side Effects)
- **Problem**: 600+ lines of command execution logic tightly coupled to `App`.
- **Solution**:
    - Break into domain-specific executors: `arduino.rs`, `serial.rs`, `system.rs`.
    - Use a `CommandExecutor` trait to standardize how background tasks report back to the UI.

### 3.3 `src/config.rs` (Configuration)
- **Problem**: Single file managing serialization for profiles, boards, and UI settings.
- **Solution**:
    - Split into `config/profiles.rs`, `config/hardware.rs`, and `config/ui.rs`.
    - Centralize YAML loading/saving in a small `config/mod.rs` wrapper.

## 4. Phase 3: Decoupling & DRY (Macros/Traits)
Eliminate repeated boilerplate and the "manual handshake" required to add new widgets.

### 4.1 The Component Registry
- Implement a `ComponentManager` in `App` that maintains a collection of `InteractiveWidget` objects.
- Use a focus-based system to automatically route input to the active component without manual `match` statements in `mod.rs`.

### 4.2 TUI Boilerplate Macros
- Draft a `tui_component!` or `impl_interactive!` macro to automatically generate:
    - Standard `Widget` trait implementations.
    - Default `handle_mouse` logic (coordinate translation).
    - Configuration loading logic.

## 5. Success Criteria
- No single file exceeds 400 lines (excluding tests).
- Adding a new "Smart" widget requires ZERO changes to `app/view.rs`.
- `cargo build` and `cargo test` pass with 100% logic coverage for new components.
- All widgets use `ratatui::prelude`.
