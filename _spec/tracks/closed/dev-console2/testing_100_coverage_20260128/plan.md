# Implementation Plan: `testing_100_coverage_20260128`

## Phase 1: Foundation & Setup

- [x] Task: Environment Preparation
    - [x] Install and configure `cargo-tarpaulin` or `llvm-cov` for coverage reporting.
    - [x] Establish a baseline coverage report for `dev-console-v2`.
- [x] Task: Testing Infrastructure
    - [x] Define traits for hardware/command interactions in `src/commands/` to enable mocking.
    - [x] Set up `mockall` or equivalent boilerplate for core dependencies.

## Phase 2: Core Logic & Configuration

- [x] Task: `config.rs` Coverage
    - [x] Write tests for YAML parsing of `config.yaml` and `widget-config.yaml`.
    - [x] Implement validation logic tests (e.g., missing fields, invalid types).
    - [x] Ensure 100% coverage for all config-related structs and enums.
- [x] Task: `app.rs` State Transitions
    - [x] Write unit tests for the `update` function using a wide range of `Message` variants.
    - [x] Verify state transitions for tab switching, profile selection, and error handling.

## Phase 3: UI & Widget Components

- [x] Task: Widget Rendering Tests
    - [x] Set up `ratatui::backend::TestBackend` for UI verification.
    - [x] Implement rendering tests for `Toast`, `OutputBox`, `ProgressBar`, and `StatusBox`.
    - [x] Verify interactive widget logic (e.g., scrolling, selection) via state-driven tests.
- [x] Task: Main View Composition
    - [x] Write integration tests for the top-level `view` function in `app.rs`.
    - [x] Verify layout constraints and area allocations.
- [x] Task: _spec - User Manual Verification 'UI & Widget Components' (Protocol in workflow.md)



## Phase 4: Commands & Integration

- [x] Task: Command Execution Mocking
    - [x] Implement mocks for `arduino-cli` and `idf.py` interactions.
    - [x] Write tests for asynchronous command handling and output streaming.
- [x] Task: Serial/Hardware Simulation
    - [x] Create stub implementations for serial port monitoring.
    - [x] Verify that the TUI responds correctly to simulated hardware events (e.g., connection lost).
- [x] Task: _spec - User Manual Verification 'Commands & Integration' (Protocol in workflow.md)

## Phase 5: Final Audit & Hardening

- [ ] Task: 100% Coverage Verification
    - [ ] Run final coverage reports and identify any remaining "dark" spots.
    - [ ] Implement targeted tests to close any gaps.
- [ ] Task: Documentation & Cleanup
    - [ ] Document the testing strategy and how to run coverage reports in `README.md`.
    - [ ] Clean up any temporary test artifacts or debug logs.


# Implementation Plan: `hardening_coverage_20260128`

## Phase 1: Command Logic Hardening

- [ ] Task: Fix `Generating` Stage Detection
    - [ ] Analyze `detect_stage_change` logic for regex/substring mismatches.
    - [ ] Update `tests.rs` with correct expectations.
    - [ ] Verify transition from `Linking` to `Generating`.
- [ ] Task: 100% Coverage for `compile_parser.rs`
    - [ ] Add tests for all error detection branches.
    - [ ] Add tests for every stage transition variant.
- [ ] Task: 100% Coverage for `compile_state.rs`
    - [ ] Add tests for `update_stage_progress` boundary conditions.
    - [ ] Add tests for `check_for_missing_markers` warning logic.
- [ ] Task: 100% Coverage for `history.rs`
    - [ ] Add tests for invalid JSON loading recovery.
    - [ ] Add tests for normalization edge cases (zero weights).

## Phase 2: Widget & UI Hardening

- [ ] Task: 100% Coverage for `smooth_scrollbar.rs`
    - [ ] Implement rendering tests for all scroll states.
    - [ ] Verify interaction logic for mouse clicks and dragging.
- [ ] Task: 100% Coverage for `output_box.rs`
    - [ ] Add tests for max line limits and auto-scroll toggling.
    - [ ] Verify input handling when active vs inactive.
- [ ] Task: 100% Coverage for `tab_bar.rs`
    - [ ] Refactor and simplify existing tests.
    - [ ] Add coverage for all color parsing variants.

## Phase 3: Final Audit & Documentation

- [ ] Task: Global Coverage Verification
    - [ ] Run final `llvm-cov` report and fix any remaining 1-2% gaps.
- [ ] Task: Documentation Update
    - [ ] Document testing strategy and GitBash requirement in `CONTRIBUTING.md` or `README.md`.
    - [ ] Clean up `NOTES.md` and archive the track.
- [ ] Task: _spec - Final Verification (Protocol in workflow.md)