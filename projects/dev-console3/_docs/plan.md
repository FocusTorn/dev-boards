# Implementation Plan: Dev-Console V3 (Greenfield)

## 1. Research & Architecture Decisions

1. **Foundational Trait**: Define a `Component` trait that encapsulates the Elm Architecture (Init, Update, View) at the widget level.
2. **Static Dispatch Registry**: Use `macro_rules!` to generate a `ComponentRegistry` enum. This avoids `Box<dyn Component>` and provides better performance and borrow-checker clarity.
3. **Focus Routing**: The `ComponentManager` will act as the "Traffic Controller," using a focus stack to decide which component receives raw `crossterm` events.

## 2. Phases

### Phase 1: Project Scaffolding & Core Traits
- **Task 1.1**: Initialize `cargo` project and add dependencies (`ratatui`, `crossterm`, `serde`, `serde-saphyr`, `strum`).
- **Task 1.2**: Define `src/widgets/traits.rs` with `Component` and `WidgetOutcome`.
- **Task 1.3**: Implement the `tui_component!` macro in `src/widgets/macros.rs`.

### Phase 2: The Component Manager & Terminal Loop
- **Task 2.1**: Implement `src/widgets/manager.rs` (Focus stack, event delegation).
- **Task 2.2**: Implement the main terminal event loop in `src/main.rs` and `src/terminal.rs`.
- **Task 2.3**: Create a "Mock" component to test the focus routing and macro registration.

### Phase 3: The Unified Overlay Dropdown
- **Task 3.1**: Implement `src/widgets/elements/dimmer.rs`.
- **Task 3.2**: Implement `src/widgets/components/dropdown/` using the "Unified Expansion" visual spec.
- **Task 3.3**: Add directional logic (Up/Down) based on `max_shown` and terminal height.

### Phase 4: Logic Migration (Selective)
- **Task 4.1**: Migrate `ProfileConfig` and `HardwareConfig` structures from V2 to `src/config/`.
- **Task 4.2**: Implement the "Dashboard" as a Smart Component.
- **Task 4.3**: Implement the "Serial Monitor" using the new decoupled executor pattern.

## 3. Risks & Constraints
- **Risk**: Borrow checker issues when components need to access global `App` state.
  - **Mitigation**: Use a message-passing system or `WidgetOutcome` to request global state changes.
- **Constraint**: Windows-specific serial port handling and pathing must be preserved from V2.
