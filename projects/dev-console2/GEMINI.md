# Project Overview

`dev-console-v2` is a Terminal User Interface (TUI) application built in Rust, designed for managing development boards (ESP32/Arduino). It utilizes the **Elm Architecture** pattern for state management and the `ratatui` library for rendering.

**Key Technologies:**
*   **Language:** Rust (Edition 2021)
*   **TUI Framework:** `ratatui`
*   **Terminal Backend:** `crossterm`
*   **Error Handling:** `color-eyre`
*   **Configuration:** `serde`, `serde-saphyr` (YAML)

# Architecture

The application follows the Elm Architecture, enforcing a clean separation of concerns:

1.  **Model (`App` struct):** Holds the entire application state (tabs, commands, widgets, active profile).
2.  **Message (`Message` enum):** Represents all events and actions (User input, background tasks, timer ticks).
3.  **Update (`update` function):** A pure function that transitions the state based on the received message.
4.  **View (`view` function):** Renders the UI based on the current state.

**Directory Structure:**
*   `src/main.rs`: Entry point. Initializes the terminal, creates the `App`, and runs the main event loop.
*   `src/app.rs`: Contains the `App` struct, `Message` enum, `update` logic, and the main `view` composition.
*   `src/terminal.rs`: Handles raw terminal mode setup and teardown.
*   `src/config.rs`: Structs and logic for loading `config.yaml` and `widget-config.yaml`.
*   `src/widgets/`: Modular UI components (Toast, OutputBox, ProgressBar, etc.).
*   `src/commands/`: Logic for executing external build commands (Arduino CLI, etc.).

# Building and Running

**Build:**
```bash
cargo build
```

**Run:**
```bash
cargo run
```

**Run Tests:**
```bash
cargo test
```

# Development Conventions

## Verification Rule: `cargo build` > `cargo check`
**CRITICAL:** When verifying code changes, **always use `cargo build`** instead of `cargo check`.
*   **Reason:** `cargo check` has been observed to miss certain compilation errors (e.g., specific string formatting issues or macro expansions) that `cargo build` correctly identifies. Relying solely on `check` may lead to broken builds.

## Configuration
*   **Main Config:** `config.yaml` (Project root) - Defines profiles, boards, and connections.
*   **Widget Config:** `src/widgets/widget-config.yaml` - Defines UI specific settings (e.g., Toast duration/position).
*   **Loading:** Configuration loading uses `serde-saphyr`. Ensure enums in Rust match YAML strings (use `#[serde(rename_all = "snake_case")]` if YAML keys are snake_case).
*   **Best Practice:** NEVER use large coordinate offsets in YAML configurations to 'fix' alignment issues; instead, verify that the correct `Rect` is being passed to the widget in the view logic. Offsets are brittle and break across different terminal dimensions.

## TUI Layout & Area Delegation
The main layout is defined in `src/app.rs` and consists of:
1.  **Title Bar** (Top)
2.  **Main Content** (Middle, Tabbed interface)
3.  **Bindings** (Bottom, Context-sensitive help)
4.  **Status Bar** (Bottom, Global status)

**Area Delegation:** When rendering interactive sub-widgets (like `ButtonBar`) within a composite layout, ALWAYS pass the specific sub-rect intended for that widget (e.g., `input_area`) rather than the parent container's area. This ensures alignment calculations and mouse-hit detection are accurate relative to the visual component.

## Error Handling
*   **Initialization:** Errors during startup (e.g., config loading) should be propagated up to `main` and printed to `stderr` *before* initializing the TUI to ensure visibility.
*   **Runtime:** Runtime errors are captured and displayed via the `StatusBox` or `Toast` widgets.

## Sub-Agents
*   Use `codebase_investigator` for deep dives into specific module interactions or when refactoring the event loop.
