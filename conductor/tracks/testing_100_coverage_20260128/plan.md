# Implementation Plan: `testing_100_coverage_20260128`

## Phase 1: Foundation & Setup
- [ ] Task: Environment Preparation
    - [ ] Install and configure `cargo-tarpaulin` or `llvm-cov` for coverage reporting.
    - [ ] Establish a baseline coverage report for `dev-console-v2`.
- [ ] Task: Testing Infrastructure
    - [ ] Define traits for hardware/command interactions in `src/commands/` to enable mocking.
    - [ ] Set up `mockall` or equivalent boilerplate for core dependencies.
- [ ] Task: Obtain user's approval to continue to the next phase

## Phase 2: Core Logic & Configuration
- [ ] Task: `config.rs` Coverage
    - [ ] Write tests for YAML parsing of `config.yaml` and `widget-config.yaml`.
    - [ ] Implement validation logic tests (e.g., missing fields, invalid types).
    - [ ] Ensure 100% coverage for all config-related structs and enums.
- [ ] Task: `app.rs` State Transitions
    - [ ] Write unit tests for the `update` function using a wide range of `Message` variants.
    - [ ] Verify state transitions for tab switching, profile selection, and error handling.
- [ ] Task: Obtain user's approval to continue to the next phase

## Phase 3: UI & Widget Components
- [ ] Task: Widget Rendering Tests
    - [ ] Set up `ratatui::backend::TestBackend` for UI verification.
    - [ ] Implement rendering tests for `Toast`, `OutputBox`, `ProgressBar`, and `StatusBox`.
    - [ ] Verify interactive widget logic (e.g., scrolling, selection) via state-driven tests.
- [ ] Task: Main View Composition
    - [ ] Write integration tests for the top-level `view` function in `app.rs`.
    - [ ] Verify layout constraints and area allocations.
- [ ] Task: Obtain user's approval to continue to the next phase

## Phase 4: Commands & Integration
- [ ] Task: Command Execution Mocking
    - [ ] Implement mocks for `arduino-cli` and `idf.py` interactions.
    - [ ] Write tests for asynchronous command handling and output streaming.
- [ ] Task: Serial/Hardware Simulation
    - [ ] Create stub implementations for serial port monitoring.
    - [ ] Verify that the TUI responds correctly to simulated hardware events (e.g., connection lost).
- [ ] Task: Obtain user's approval to continue to the next phase

## Phase 5: Final Audit & Hardening
- [ ] Task: 100% Coverage Verification
    - [ ] Run final coverage reports and identify any remaining "dark" spots.
    - [ ] Implement targeted tests to close any gaps.
- [ ] Task: Documentation & Cleanup
    - [ ] Document the testing strategy and how to run coverage reports in `README.md`.
    - [ ] Clean up any temporary test artifacts or debug logs.
- [ ] Task: Obtain user's approval - 'Final Audit & Hardening' (Protocol in workflow.md)

