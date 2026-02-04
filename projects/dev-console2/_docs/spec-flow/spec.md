# Specification: Dev-Console V2 System Restructure & Widget Modularization

## 1. Problem Statement
The current `dev-console-v2` architecture is monolithic, leading to several maintainability issues:
- **High Coupling**: Core logic in `App` is tightly coupled with widget rendering and state management.
- **"God Objects"**: Files like `src/app/mod.rs` (1200+ lines), `src/app/executors.rs` (600+ lines), and `src/config.rs` are oversized and handle too many responsibilities.
- **Boilerplate Overload**: Adding a new widget requires manual "handshakes" across multiple files (View, Update, Action enums).
- **Inconsistent Patterns**: No clear distinction between pure UI elements and complex stateful components.

## 2. Goals & Success Criteria

### 2.1 Goals
- **G-001**: Decouple UI elements from stateful business logic.
- **G-002**: Encapsulate component-specific state, view, and configuration.
- **G-003**: Standardize input routing and focus management.
- **G-004**: Reduce boilerplate for new component registration via macros.

### 2.2 Success Criteria
- **SC-001**: No single source file exceeds 400 lines (excluding tests).
- **SC-002**: Adding a new "Smart" component requires zero changes to `app/view.rs`.
- **SC-003**: 100% logic coverage for new `InteractiveWidget` implementations.
- **SC-004**: All widgets migrated to `ratatui::prelude`.

## 3. User Scenarios

### Scenario 1: Developer adds a new Smart Component
**Given** a developer wants to add a "System Monitor" widget
**When** they create a new directory `src/widgets/components/system_monitor/`
**And** they implement the `Component` trait in `mod.rs`
**And** they add a `config.yaml` to the same directory
**Then** the macro-generated registry automatically includes the component
**And** the component is available for layout without modifying the main `App` view logic.

### Scenario 2: System routes input to a focused component
**Given** the "File Browser" component currently has focus
**When** the user presses `Enter`
**Then** the `ComponentManager` delegates the key event directly to the "File Browser"
**And** other components do not receive or process the event.

### Scenario 3: Component loads local configuration
**Given** a "Toast" component with a local `config.yaml`
**When** the application starts
**Then** the "Toast" component loads its specific duration and position settings from its own directory
**And** it does not rely on a global monolithic configuration file.

## 4. Functional Requirements

### 4.1 Widget Tiering (FR-00x)
- **FR-001**: System SHALL distinguish between **Elements** (stateless, pure view) and **Components** (stateful, encapsulated logic).
- **FR-002**: Elements SHALL reside in `src/widgets/elements/`.
- **FR-003**: Components SHALL reside in `src/widgets/components/<name>/`.

### 4.2 Encapsulation (FR-01x)
- **FR-011**: Each Smart Component SHALL own its `mod.rs` (Logic/Trait impl), `state.rs` (Internal State), and `view.rs` (Rendering logic).
- **FR-012**: Each Smart Component SHALL maintain its own `config.yaml` for component-specific defaults.

### 4.3 Component Registry & Macros (FR-02x)
- **FR-021**: System SHALL use a `tui_component!` or similar macro to generate a static Enum-based registry.
- **FR-022**: The Registry SHALL support static dispatch for performance and type safety.
- **FR-023**: The Macro SHALL automatically generate boilerplate for the `Widget` and `InteractiveWidget` traits.

### 4.4 Focus & Input Routing (FR-03x)
- **FR-031**: `ComponentManager` SHALL maintain a focus stack.
- **FR-032**: Key and Mouse events SHALL be delegated to the top-most focused component.
- **FR-033**: Components SHALL return an `Outcome` or `Action` to the `App` to signal state changes or required side effects.

### 4.5 Decomposition (FR-04x)
- **FR-041**: `src/app/mod.rs` SHALL be split into `actions.rs`, `router.rs`, and `state.rs`.
- **FR-042**: `src/app/executors.rs` SHALL be split into domain-specific modules (`arduino.rs`, `serial.rs`, `system.rs`).
- **FR-043**: `src/config.rs` SHALL be split into `profiles.rs`, `hardware.rs`, and `ui.rs`.

## 5. Non-Functional Requirements

- **NFR-001**: **Maintainability**: No file SHALL exceed 400 lines.
- **NFR-002**: **Performance**: The abstraction layer for component routing SHALL have negligible impact on TUI frame rates (aiming for 60fps target).
- **NFR-003**: **Consistency**: All widgets MUST use `ratatui::prelude`.
- **NFR-004**: **Portability**: Components SHOULD be easily movable between projects with minimal external dependencies.

## 6. Out of Scope
- Migrating the legacy `dev-console` (V1) code.
- Implementing a full-blown GUI (remains a TUI application).
- Rewriting the `arduino-cli` wrapper logic (only reorganizing it).

## 7. Risks & Assumptions

### 7.1 Risks
- **Macro Complexity**: Writing robust macros for trait implementation can be complex and hard to debug.
- **Circular Dependencies**: Splitting "God Objects" may reveal hidden circular dependencies between state and view.

### 7.2 Assumptions
- `cargo build` and `cargo test` will be the primary verification tools.
- `serde-saphyr` remains the YAML engine of choice.

---

## 8. Assumption Inventory

| # | Assumption | Source | Challenge | Status |
|---|------------|--------|-----------|--------|
| 1 | Encapsulation improves portability | Discovery Record | Does local config complicate global overrides? | Validated |
| 2 | Macro-based Registry is better than manual | Discovery Record | Is the debugging trade-off worth the boilerplate reduction? | Validated |
| 3 | File size limit improves readability | Plan | Does it force unnatural fragmentation? | To be Monitored |
