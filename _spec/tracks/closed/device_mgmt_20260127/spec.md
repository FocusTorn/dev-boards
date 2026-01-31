# Specification: Device Management & Selection Refactor (device_mgmt_20260127)

## Overview
This track refactors the current `CommandListWidget` into a reusable `SelectionListWidget` and implements a sidebar-based framework for the Profiles/Settings tab. It establishes a "dispatch-on-highlight" pattern for navigation while maintaining "dispatch-on-select" for command execution.

## Functional Requirements

### 1. Reusable SelectionListWidget
- **Titleless Rendering:** The widget will no longer render its own `Block` or Title. It will render a list of items within a provided `Rect`.
- **Theming:** Support for standard and highlight styles, initially handled via widget-level overrides with future path to theme-config.
- **Interactions:**
    - Mouse: Hover highlights, Click selects.
    - Keyboard: Up/Down cycles highlights.

### 2. Sidebar-Based Settings Framework
- **Category Navigation:** Implement a sidebar on the Profiles tab using the `SelectionListWidget`.
- **Categories:** "Device" (Active), "MQTT" (Placeholder), "Paths" (Placeholder).
- **Sub-Layout Rendering:** Selecting a category in the sidebar updates the main content area (initially focusing on "Device" settings).

### 3. Dispatcher Enhancements
- **Dispatch Modes:**
    - `OnSelect` (Dashboard): Action triggers only on Enter or Click.
    - `OnHighlight` (Settings): Action triggers as the user cycles through categories with Up/Down.
- **Keybindings Interweaving:**
    - `PgUp/PgDown`: Main Tab navigation.
    - `Up/Down`: Sidebar/Command navigation.
    - `Tab`: Transition focus from Sidebar to Content Area.
    - `Enter`: Activate/Execute.

### 4. Device Management Implementation
- **Port Detection:** Service to retrieve rich metadata (VID, PID, Manufacturer).
- **Profile CRUD:** Basic logic for Create, Clone, and Delete (UI implemented as fields in the content area).

## Acceptance Criteria
- [ ] `CommandListWidget` is replaced by `SelectionListWidget` in the Dashboard.
- [ ] Profiles tab features a sidebar that switches content areas on highlight.
- [ ] Port metadata is correctly detected and displayed.
- [ ] Tab/Arrow keys allow seamless transition between sidebar and fields.
- [ ] 80% test coverage for the new selection logic.

## Out of Scope
- Full implementation of MQTT or Paths categories.
- Move to centralized theme configuration (future track).