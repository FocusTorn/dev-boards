# Implementation Plan: Device Management & Profile UI (device_mgmt_20260127)

## Phase 1: Port Detection Infrastructure (TDD)
- [x] Task: Research crate for rich serial port metadata (e.g., `serialport`)
- [x] Task: Write Tests: Port discovery service with mocked serial hardware
- [x] Task: Implement: `PortScanner` service to retrieve VID, PID, Manufacturer, and Serial Number

## Phase 2: Sidebar-Based Settings Framework
- [ ] Task: Write Tests: Navigation state transitions in the new Profiles tab
- [ ] Task: Implement: Reusable `SettingsLayout` component (Sidebar + Content area)
- [ ] Task: Implement: Sidebar categories (Device, MQTT, Paths) with "Device" active

## Phase 3: "Quick Pick" & Modal Dimming
- [ ] Task: Implement: `dim_and_mute_background` function using specified Color::Indexed range
- [ ] Task: Write Tests: Search/Filter logic for the Quick Pick menu
- [ ] Task: Implement: Quick Pick modal overlay with searchable port list
- [ ] Task: Implement: In-place dropdown fallback for port selection

## Phase 4: Profile CRUD & Persistence
- [ ] Task: Write Tests: Profile cloning and deletion logic
- [ ] Task: Implement: Profile Manager service for CRUD operations on `config.yaml`
- [ ] Task: Implement: UI for New/Clone/Delete actions and "Save Changes" footer
- [ ] Task: Implement: Form fields for Baud Rate, Display Name, and Description

## Phase 5: Final Integration & Polish
- [ ] Task: Integrate "Device" selection into the main Profiles rectangle
- [ ] Task: Final workspace-wide build and regression testing

