# Specification: dc2-restructure_20260202

## Problem Statement
The current `dev-console-v2` architecture is monolithic, with "God Objects" like `app/mod.rs` (1200+ lines) and `app/executors.rs` (600+ lines) handling too many responsibilities. Widgets are flatly organized, mixing stateless UI elements with stateful components, leading to high boilerplate and low portability.

## Goals & Success Criteria
- **G1 (Modularity)**: Reorganize `src/widgets/` into `elements/` (stateless) and `components/` (stateful).
- **G2 (Encapsulation)**: Smart components are fully self-contained with their own `state.rs`, `view.rs`, and `config.yaml`.
- **G3 (Decoupling)**: Implement a `ComponentManager` and focus-routing system to eliminate manual `match` statements in `App`.
- **G4 (Automation)**: Use macros to generate the Component Registry and dispatch logic.
- **G5 (Maintainability)**: No single file (excluding tests) exceeds 400 lines.

## User Scenarios

### Scenario 1: Adding a New Smart Widget
**Given** a developer wants to add a new "LogViewer" component
**When** they create the directory `src/widgets/components/log_viewer/` and implement the `Component` trait
**And** they add `LogViewer => log_viewer::LogViewer` to the `register_components!` macro
**Then** the component is automatically integrated into the App's focus and input routing without manual updates to `app/mod.rs`.

### Scenario 2: Focus Delegation
**Given** the App is running and a `FileBrowser` modal is opened
**When** the user presses keyboard keys
**Then** input is automatically routed to the `FileBrowser` because it is at the top of the `ComponentManager` focus stack.

### Scenario 3: Component Portability
**Given** the `Toast` component is fully encapsulated
**When** a developer copies the `toast/` directory to another project with the same `Component` trait
**Then** the component works immediately with its own local `config.yaml` and state logic.

## Functional Requirements

### Phase 1: Tiered Widgets & Encapsulation
- **FR-001**: Move stateless widgets (`ProgressBar`, `StatusBox`, etc.) to `src/widgets/elements/`.
- **FR-002**: Move stateful components (`Toast`, `FileBrowser`, etc.) to `src/widgets/components/`.
- **FR-003**: Each smart component must have a local `config.yaml` for its settings.
- **FR-004**: Each smart component must implement an evolved `Component` trait supporting `on_tick`, `handle_key`, `handle_mouse`, and `render`.

### Phase 2: Registry & Routing
- **FR-005**: Create `register_components!` macro to generate the `Component` enum and dispatch logic.
- **FR-006**: Implement `ComponentManager` to manage the collection of active components and a focus stack.
- **FR-007**: Delegate input and view rendering from `App` to `ComponentManager`.

### Phase 3: App Decomposition
- **FR-008**: Split `app/mod.rs` into `actions.rs`, `state.rs`, and `router.rs`.
- **FR-009**: Break `executors.rs` into domain-specific modules (`arduino.rs`, `serial.rs`, etc.).
- **FR-010**: Standardize `ComponentAction` to communicate back to the global `AppAction` loop.

## Non-Functional Requirements
- **NFR-001 (Performance)**: Use static dispatch (Enums) via macros for component routing to maintain high TUI responsiveness.
- **NFR-002 (Consistency)**: All widgets must use `ratatui::prelude::*`.
- **NFR-003 (Testability)**: All new components must have unit tests covering state transitions.

## Out of Scope
- Rewriting the core serial communication protocol.
- Migrating the entire build system to another language.
- Redesigning the visual theme of the console (beyond organization).

## Open Questions
- [NEEDS CLARIFICATION] Should the `AppAction` be defined in a shared location to avoid circular dependencies between `app/` and `widgets/`?
