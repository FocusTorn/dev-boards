# Implementation Plan: `testing_100_coverage_20260128`

## Phase 1: Foundation & Setup
- [x] Task: Environment Preparation
    - [x] Install and configure `cargo-tarpaulin` or `llvm-cov` for coverage reporting.
    - [x] Establish a baseline coverage report for `dev-console-v2`.
- [x] Task: Testing Infrastructure
    - [x] Define traits for hardware/command interactions in `src/commands/` to enable mocking.
    - [x] Set up `mockall` or equivalent boilerplate for core dependencies.
- [ ] Task: Conductor - User Manual Verification 'Foundation & Setup' (Protocol in workflow.md)

## Phase 2: Core Logic & Configuration
- [x] Task: `config.rs` Coverage
    - [x] Write tests for YAML parsing of `config.yaml` and `widget-config.yaml`.
    - [x] Implement validation logic tests (e.g., missing fields, invalid types).
    - [x] Ensure 100% coverage for all config-related structs and enums.
- [x] Task: `app.rs` State Transitions
    - [x] Write unit tests for the `update` function using a wide range of `Message` variants.
    - [x] Verify state transitions for tab switching, profile selection, and error handling.
- [ ] Task: Conductor - User Manual Verification 'Core Logic & Configuration' (Protocol in workflow.md)

## Phase 3: UI & Widget Components
- [ ] Task: Widget Rendering Tests
    - [ ] Set up `ratatui::backend::TestBackend` for UI verification.
    - [ ] Implement rendering tests for `Toast`, `OutputBox`, `ProgressBar`, and `StatusBox`.
    - [ ] Verify interactive widget logic (e.g., scrolling, selection) via state-driven tests.
- [ ] Task: Main View Composition
    - [ ] Write integration tests for the top-level `view` function in `app.rs`.
    - [ ] Verify layout constraints and area allocations.
- [ ] Task: Conductor - User Manual Verification 'UI & Widget Components' (Protocol in workflow.md)

## Phase 4: Commands & Integration
- [ ] Task: Command Execution Mocking
    - [ ] Implement mocks for `arduino-cli` and `idf.py` interactions.
    - [ ] Write tests for asynchronous command handling and output streaming.
- [ ] Task: Serial/Hardware Simulation
    - [ ] Create stub implementations for serial port monitoring.
    - [ ] Verify that the TUI responds correctly to simulated hardware events (e.g., connection lost).
- [ ] Task: Conductor - User Manual Verification 'Commands & Integration' (Protocol in workflow.md)

## Phase 5: Final Audit & Hardening
- [ ] Task: 100% Coverage Verification
    - [ ] Run final coverage reports and identify any remaining "dark" spots.
    - [ ] Implement targeted tests to close any gaps.
- [ ] Task: Documentation & Cleanup
    - [ ] Document the testing strategy and how to run coverage reports in `README.md`.
    - [ ] Clean up any temporary test artifacts or debug logs.
- [ ] Task: Conductor - User Manual Verification 'Final Audit & Hardening' (Protocol in workflow.md)