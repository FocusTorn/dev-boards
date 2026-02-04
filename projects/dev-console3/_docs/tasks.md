# Tasks: Dev-Console V3 Implementation

## Phase 1: Foundations

- [ ] T001 Initialize Cargo project and add dependencies
  - deps: `ratatui`, `crossterm`, `serde`, `serde-saphyr`, `color-eyre`, `strum`, `strum_macros`.
- [ ] T002 Define `Component` trait and `WidgetOutcome`
  - file: `src/widgets/traits.rs`
- [ ] T003 Implement `tui_component!` registration macro
  - file: `src/widgets/macros.rs`
- [ ] T004 Setup `src/terminal.rs` for raw mode management

## Phase 2: Orchestration

- [ ] T010 Implement `ComponentManager` focus stack logic
  - file: `src/widgets/manager.rs`
- [ ] T011 Create `App` structure as the top-level container
  - file: `src/app/mod.rs`
- [ ] T012 Implement main event loop with event-to-component delegation
- [ ] T013 [RED] Write test for focus-stack routing (ensure events reach the top component)

## Phase 3: Unified Overlay Dropdown (High Priority)

- [ ] T020 Implement `src/widgets/elements/dimmer.rs` for background de-emphasis
- [ ] T021 Implement `OverlayDropdown` state and view logic
  - Requirement: Cyan border, anchor consumption.
- [ ] T022 Implement directional logic (Open Up/Down)
  - Formula: `anchor_y + max_shown + 2 < terminal_height`
- [ ] T023 Add scrollbar implementation to `OverlayDropdown`
- [ ] T024 [RED] Write tests for dropdown direction calculation

## Phase 4: Component Migration

- [ ] T030 Migrate `config.yaml` parsing logic from V2
- [ ] T031 Implement `Dashboard` as a Smart Component
- [ ] T032 Implement `ToastManager` as a global overlay component
- [ ] T033 Implement `FileBrowser` modal using the focus stack

## Phase 5: Polish & Verification

- [ ] T040 Finalize theme engine and ANSI color support
- [ ] T041 Verify SC-001: No single file > 400 lines
- [ ] T042 Run `cargo build` and `cargo test` for full logic coverage
