# Specification: Dev-Console V3 (Modular TUI System)

## 1. Problem Statement
The legacy `dev-console-v2` architecture proved too monolithic for rapid feature expansion and maintenance. Business logic, TUI rendering, and state management were tightly coupled, leading to oversized "God Objects." `dev-console3` aims to solve this by being built from the ground up with a decoupled, component-first architecture.

## 2. Goals & Success Criteria

### 2.1 Goals
- **G-001**: Implement a **Component-Based Architecture** where logic, state, and view are encapsulated.
- **G-002**: Utilize a **Focus-Stack Router** for deterministic input delegation.
- **G-003**: Use **Macro-Driven Registry** to eliminate boilerplate for component registration.
- **G-004**: Implement the **"Unified Expansion"** dropdown pattern for all selection fields.

### 2.2 Success Criteria
- **SC-001**: 100% decoupling of "Smart" components (no manual view logic in `main`).
- **SC-002**: Passing a `max_shown` parameter to dropdowns correctly governs scrollbar and expansion direction.
- **SC-003**: Background dimming is consistently handled by a global `Dimmer` layer.
- **SC-004**: Project compiles with zero circular dependencies between widgets and app state.

## 3. User Scenarios

### Scenario 1: Using the Unified Dropdown
**Given** the user is in the Profile Settings
**When** they select the "Port" field
**Then** the dropdown consumes the field area, expands (up or down depending on space), and borders turn Cyan
**And** the rest of the UI is dimmed to focus the interaction.

### Scenario 2: Adding a new system tool
**Given** a developer wants to add an "MQTT Monitor"
**When** they implement the `Component` trait in a new directory
**Then** the `tui_component!` macro automatically registers it for use in the app layout.

## 4. Functional Requirements

### 4.1 Architecture (FR-01x)
- **FR-011**: System SHALL use a `Component` trait defining `on_tick`, `handle_key`, `handle_mouse`, and `view`.
- **FR-012**: `ComponentManager` SHALL maintain a focus stack (`Vec<ComponentId>`).
- **FR-013**: System SHALL distinguish between `elements/` (dumb) and `components/` (smart).

### 4.2 Unified Overlay Dropdown (FR-02x)
- **FR-021**: Dropdowns SHALL use the "Unified Expansion" pattern (Cyan border, anchor consumption).
- **FR-022**: Dropdowns SHALL calculate expansion direction (Up/Down) based on terminal height and `max_shown`.
- **FR-023**: Dropdowns SHALL render a scrollbar if items > `max_shown`.

### 4.3 Configuration (FR-03x)
- **FR-031**: Each Smart Component SHALL load its own `config.yaml` from its directory.
- **FR-032**: Global application state (Profiles/Hardware) SHALL be modularized and accessible via shared references or messages.

## 5. Non-Functional Requirements
- **NFR-001**: **Performance**: TUI MUST maintain 60fps even with multiple active components.
- **NFR-002**: **Maintainability**: No single file SHALL exceed 400 lines of code.
- **NFR-003**: **Robustness**: Borrow checker safety MUST be prioritized over complex shared state.

## 6. Out of Scope
- Direct refactoring of V2 code (migration of logic only).
- Non-TUI interfaces.
