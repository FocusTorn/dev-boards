# Implementation Plan: Device Management & Selection Refactor (device_mgmt_20260127)

## Phase 1: Widget Refactor (SelectionList)
- [x] Task: **Write Tests**: Verify `SelectionListWidget` rendering and interaction (Click/Hover) without internal borders.
- [x] Task: **Verify Failure (Red)**: Confirm tests fail or do not compile before implementation.
- [x] Task: **Implement**: Extract logic from `CommandListWidget` into `SelectionListWidget` in `src/widgets/`.
- [x] Task: **Verify Success (Green)**: Confirm `SelectionListWidget` tests pass.
- [x] Task: **Write Tests**: Verify Dashboard regression (ensure titled blocks correctly wrap the new list).
- [x] Task: **Verify Failure (Red)**: Confirm Dashboard tests fail with the new widget before layout update.
- [x] Task: **Implement**: Update Dashboard `view` to wrap `SelectionListWidget` in a titled `Block`.
- [x] Task: **Verify Success (Green)**: Confirm all Dashboard tests pass.

## Phase 2: Dispatcher & State Logic
- [x] Task: **Write Tests**: Validate `dispatch_mode` transitions (Select vs Highlight) in `App` state.
- [x] Task: **Verify Failure (Red)**: Confirm tests fail.
- [x] Task: **Implement**: Add `dispatch_mode` support to `App` and `SelectionListWidget`.
- [x] Task: **Verify Success (Green)**: Confirm tests pass.
- [x] Task: **Write Tests**: Verify `Up/Down` navigation correctly triggers `OnHighlight` actions.
- [x] Task: **Verify Failure (Red)**: Confirm navigation tests fail.
- [x] Task: **Implement**: Update `dispatch_key` to support contextual `Up/Down` and `OnHighlight` dispatch.
- [x] Task: **Verify Success (Green)**: Confirm navigation tests pass.
- [x] Task: **Write Tests**: Verify `Tab` key correctly toggles focus state between sidebar and mock content areas.
- [x] Task: **Verify Failure (Red)**: Confirm focus tests fail.
- [x] Task: **Implement**: Add `Tab` key handling to `dispatch_key`.
- [x] Task: **Verify Success (Green)**: Confirm focus tests pass.

## Phase 3: Settings Framework & Sidebar
- [x] Task: **Write Tests**: Verify `SettingsLayout` component correctly partitions areas for Sidebar and Content.
- [x] Task: **Verify Failure (Red)**: Confirm layout tests fail.
- [x] Task: **Implement**: Create the `SettingsLayout` structure.
- [x] Task: **Verify Success (Green)**: Confirm layout tests pass.
- [x] Task: **Write Tests**: Verify category switching logic (Device -> MQTT -> Paths) updates the active sub-view.
- [x] Task: **Verify Failure (Red)**: Confirm switching tests fail.
- [x] Task: **Implement**: Implement the Settings Sidebar and "OnHighlight" content switching.
- [x] Task: **Verify Success (Green)**: Confirm switching tests pass.

## Phase 4: Device Implementation (TDD)
- [x] Task: **Write Tests**: Mock `serialport` to verify `PortScanner` correctly parses VID/PID/Manufacturer.
- [x] Task: **Verify Failure (Red)**: Confirm scanner tests fail.
- [x] Task: **Implement**: Implement `PortScanner` service.
- [x] Task: **Verify Success (Green)**: Confirm scanner tests pass.
- [x] Task: **Write Tests**: Verify Profile CRUD operations (Load/Save/Delete) against a mock `config.yaml`.
- [x] Task: **Verify Failure (Red)**: Confirm CRUD tests fail.
- [x] Task: **Implement**: Implement `ProfileManager` and "Device" configuration view.
- [x] Task: **Verify Success (Green)**: Confirm CRUD tests pass.

## Phase 5: Final Verification & Polish
- [x] Task: **Write Tests**: 100% Entry-point test coverage for new Profile management actions.
- [x] Task: **Verify Success (Green)**: Confirm all entry-point tests pass.
- [x] Task: **Verify**: Final workspace build and regression check.
- [x] Task: **Documentation**: Update `_meta` with implementation notes and usage examples for `SelectionListWidget`.