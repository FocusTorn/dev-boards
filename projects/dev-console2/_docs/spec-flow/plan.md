# Implementation Plan: Dev-Console V2 System Restructure

This plan details the phased transformation of the `dev-console-v2` codebase from its current monolithic state into a modular, decoupled architecture.

## 1. Research Decisions

1. **State Management**: Migrate from monolithic `App` state to encapsulated `Component` state.
   - Source: `projects/dev-console2/src/app/mod.rs` (current implementation)
   - Rationale: Reducing `App` size and improving component portability.
2. **Event Routing**: Focus-stack based delegation in `ComponentManager`.
   - Source: Existing modal logic in `src/app/mod.rs:565` (to be generalized).
   - Rationale: Eliminates manual `match` chains in the main loop.
3. **Macro Registry**: Use a hybrid `macro_rules!` for static dispatch enum generation.
   - Rationale: Provides performance of an enum with the convenience of dynamic discovery.

## 2. Phases

### Phase 1: Widget Tiering & Pre-Cleanup
**Goal**: Reorganize `src/widgets/` and standardize imports.
- **Task 1.1**: Create `src/widgets/elements/` and `src/widgets/components/`.
- **Task 1.2**: Move "dumb" widgets (`ProgressBar`, `StatusBox`, `Dimmer`) to `elements/`.
- **Task 1.3**: Migrate all widget files to use `ratatui::prelude::*`.
- **Task 1.4**: Standardize `InteractiveWidget` trait and outcomes across existing components.

### Phase 2: Breaking Down God Objects
**Goal**: Decompose `app/mod.rs`, `executors.rs`, and `config.rs`.
- **Task 2.1**: Split `src/app/mod.rs` into `src/app/state.rs` (App struct), `src/app/actions.rs` (Action enum), and `src/app/router.rs` (Event dispatching).
- **Task 2.2**: Decompose `src/app/executors.rs` into `src/app/executors/mod.rs`, `arduino.rs`, `serial.rs`, and `system.rs`.
- **Task 2.3**: Refactor `src/config.rs` into a directory `src/config/` containing `profiles.rs`, `hardware.rs`, and `ui.rs`.

### Phase 3: The Component Manager & Registry
**Goal**: Implement the focus-based routing system and registration macro.
- **Task 3.1**: Implement `src/widgets/manager.rs` with `ComponentManager` and focus stack logic.
- **Task 3.2**: Create the `tui_component!` macro to generate the `ComponentRegistry` enum.
- **Task 3.3**: Refactor `App::dispatch_key` and `App::dispatch_mouse` to delegate to `ComponentManager`.

### Phase 4: Encapsulation & Final Migration
**Goal**: Ensure all components are fully self-contained.
- **Task 4.1**: Move component-specific state and view logic into `src/widgets/components/<name>/{state.rs, view.rs}`.
- **Task 4.2**: Implement local `config.yaml` loading for "Smart" components.
- **Task 4.3**: Final verification: `cargo build` and logic coverage tests.

## 3. Risks & Constraints

- **Risk: Breaking Change**: Massive refactoring of `App` will break many tests; requires incremental commits.
- **Constraint: Windows Pathing**: Ensure YAML loading/saving handles Windows paths correctly (as noted in `GEMINI.md`).
- **Performance**: Monitor TUI latency during the transition from direct matching to focus-stack delegation.

## 4. Codebase Soul Summary

**Dominant Patterns**: Command-based UI, Trait-based widgets.
**Philosophy**: Encapsulation of logic within widgets where possible, using a central "App" for global state.
**Conventions**: Snake_case YAML, enum-based actions.

**Reuse Opportunities**:
- Generalize the existing `modal` handling pattern into the new `ComponentManager`.
- Reuse `WidgetOutcome` for all interaction feedback.
