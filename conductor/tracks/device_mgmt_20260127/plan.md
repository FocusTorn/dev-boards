# Implementation Plan: Device Management & Profile UI (device_mgmt_20260127)

## Phase 1: Port Detection Infrastructure (TDD)
- [ ] Task: Research crate for rich serial port metadata (e.g., `serialport`)
- [ ] Task: Write Tests: Port discovery service with mocked serial hardware
- [ ] Task: Implement: `PortScanner` service to retrieve VID, PID, Manufacturer, and Serial Number
- [ ] Task: Conductor - User Manual Verification 'Phase 1' (Protocol in workflow.md)

## Phase 2: Sidebar-Based Settings Framework
- [ ] Task: Write Tests: Navigation state transitions in the new Profiles tab
- [ ] Task: Implement: Reusable `SettingsLayout` component (Sidebar + Content area)
- [ ] Task: Implement: Sidebar categories (Device, MQTT, Paths) with "Device" active
- [ ] Task: Conductor - User Manual Verification 'Phase 2' (Protocol in workflow.md)

## Phase 3: "Quick Pick" & Modal Dimming
- [ ] Task: Implement: `dim_and_mute_background` function using specified Color::Indexed range
- [ ] Task: Write Tests: Search/Filter logic for the Quick Pick menu
- [ ] Task: Implement: Quick Pick modal overlay with searchable port list
- [ ] Task: Implement: In-place dropdown fallback for port selection
- [ ] Task: Conductor - User Manual Verification 'Phase 3' (Protocol in workflow.md)

## Phase 4: Profile CRUD & Persistence
- [ ] Task: Write Tests: Profile cloning and deletion logic
- [ ] Task: Implement: Profile Manager service for CRUD operations on `config.yaml`
- [ ] Task: Implement: UI for New/Clone/Delete actions and "Save Changes" footer
- [ ] Task: Implement: Form fields for Baud Rate, Display Name, and Description
- [ ] Task: Conductor - User Manual Verification 'Phase 4' (Protocol in workflow.md)

## Phase 5: Final Integration & Polish
- [ ] Task: Integrate "Device" selection into the main Profiles rectangle
- [ ] Task: Final workspace-wide build and regression testing
- [ ] Task: Conductor - User Manual Verification 'Phase 5' (Protocol in workflow.md)
