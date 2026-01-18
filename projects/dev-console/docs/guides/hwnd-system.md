# HWND System Guide

The HWND (Window Handle) system is a core component of the dev-console that manages UI element positioning, layout, and interaction. This guide explains how to work with HWNDs to update element positions and manage the interface.

## Overview

The HWND system provides a Windows-inspired approach to managing TUI elements, where each UI component has a unique handle identifier that can be used to reference, position, and manipulate it.

## Key Concepts

### HWND Identifiers

HWNDs are string constants that uniquely identify UI elements:

```rust
// Main content areas
pub const HWND_MAIN_CONTENT_BOX: &str = "hwndMainContentBox";
pub const HWND_MAIN_CONTENT_TAB_BAR: &str = "hwndMainContentTabBar";

// Settings fields
pub const HWND_SETTINGS_FIELD_SKETCH_DIR: &str = "hwndSettingsFieldSketchDir";
pub const HWND_SETTINGS_FIELD_SKETCH_NAME: &str = "hwndSettingsFieldSketchName";
pub const HWND_SETTINGS_FIELD_ENV: &str = "hwndSettingsFieldEnv";
// ... more field constants

// Profile components
pub const HWND_PROFILE_BOX: &str = "hwndProfileBox";
pub const HWND_PROFILE_LIST: &str = "hwndProfileList";
pub const HWND_PROFILE_SELECTOR: &str = "hwndProfileSelector";
```

### RectRegistry

The `RectRegistry` is the central registry that manages all HWND elements and their positions:

```rust
use tui_components::RectRegistry;

// Create registry
let mut registry = RectRegistry::new();

// Access elements by HWND
if let Some(box_manager) = get_box_by_name(&registry, HWND_MAIN_CONTENT_BOX) {
    // Work with the element
}
```

## Updating Element Positions

### 1. Configuration-Based Positioning

Element positions are primarily defined in `config.yaml`:

```yaml
tab_bars:
    main_content_tab_bar:
        hwnd: "hwndMainContentTabBar"
        anchor: "hwndMainContentBox"
        alignment:
            vertical: "top"
            horizontal: "center"
            offset_x: 0
            offset_y: 0
```

### 2. Programmatic Position Updates

To update element positions programmatically:

```rust
// Get the current metrics for an element
if let Some(box_manager) = get_box_by_name(&registry, hwnd) {
    if let Some(metrics) = box_manager.metrics(&registry) {
        let rect: Rect = metrics.into();
        // Work with the rectangle position
    }
}
```

### 3. Layout Manager Integration

The `LayoutManager` provides cached layout calculations:

```rust
use crate::layout_manager::LayoutManager;

let mut layout_manager = LayoutManager::new();

// Get content area with caching
if let Some(content_area) = layout_manager.get_content_area(content_rect) {
    // Use the calculated area for positioning
}
```

## Working with Different Element Types

### Tab Bars

Tab bars are positioned relative to their anchor:

```rust
// Create tab bar with HWND
let tab_bar = TabBarManager::create(
    &mut registry, 
    HWND_MAIN_CONTENT_TAB_BAR, 
    tab_bar_config
);

// Handle tab clicks
handle_tab_click(
    &mouse_event,
    &current_tab_bar,
    &mut registry,
    &main_content_tab_bar,
    tab_style,
);
```

### Settings Fields

Settings fields are positioned within their sections:

```rust
// Handle field clicks by HWND
if let Some(new_state) = handle_settings_field_click(
    &mouse_event,
    &app_state.settings,
    &app_state.settings_fields,
    &registry,
    &main_content_tab_bar,
) {
    app_state.field_editor_state = new_state;
}
```

### Profile Selector

The profile selector has special positioning logic:

```rust
// Check if profile selector was clicked
if let Some(box_manager) = get_box_by_name(&registry, HWND_PROFILE_SELECTOR) {
    if let Some(metrics) = box_manager.metrics(&registry) {
        let rect: Rect = metrics.into();
        if mouse_event.column >= rect.x && mouse_event.column < rect.x + rect.width &&
           mouse_event.row >= rect.y && mouse_event.row < rect.y + rect.height {
            // Handle profile selector click
        }
    }
}
```

## Adding New Elements

### 1. Define HWND Constant

Add a new constant in `src/constants.rs`:

```rust
/// New element HWND constant
pub const HWND_NEW_ELEMENT: &str = "hwndNewElement";
```

### 2. Update Configuration

Add the element to `config.yaml`:

```yaml
# Add to appropriate section
new_element:
    hwnd: "hwndNewElement"
    anchor: "hwndMainContentBox"
    alignment:
        vertical: "top"
        horizontal: "left"
        offset_x: 10
        offset_y: 5
```

### 3. Register Element

Register the element in the UI coordinator:

```rust
// In render function
let new_element_handle = registry.create_box(
    HWND_NEW_ELEMENT,
    // ... positioning parameters
);
```

### 4. Handle Events

Add event handling for the new element:

```rust
// Handle mouse clicks
if let Some(box_manager) = get_box_by_name(&registry, HWND_NEW_ELEMENT) {
    if let Some(metrics) = box_manager.metrics(&registry) {
        let rect: Rect = metrics.into();
        // Check if click is within bounds and handle
    }
}
```

## Positioning Best Practices

### 1. Use Anchors

Always position elements relative to anchors rather than absolute positions:

```yaml
good:
    anchor: "hwndMainContentBox"
    alignment:
        vertical: "top"
        horizontal: "center"

bad:
    x: 50
    y: 10  # Absolute positioning
```

### 2. Leverage Layout Manager

Use the `LayoutManager` for consistent layout calculations:

```rust
// Good: Use layout manager
let content_area = layout_manager.get_content_area(main_rect);

// Avoid: Manual calculations
let content_area = Rect {
    x: main_rect.x + main_rect.width / 4,
    y: main_rect.y + main_rect.height / 4,
    width: main_rect.width / 2,
    height: main_rect.height / 2,
};
```

### 3. Cache Calculations

Leverage the built-in caching system:

```rust
impl LayoutManager {
    pub fn get_content_area(&mut self, content_rect: Rect) -> Option<Rect> {
        // Uses cache for performance
        self.cache.get_content_area()
            .filter(|cached| {
                cached.width == content_rect.width && cached.height == content_rect.height
            })
            .or_else(|| {
                // Calculate and cache if needed
                calculate_centered_content_area(content_rect).map(|area| {
                    self.cache.set_content_area(area);
                    area
                })
            })
    }
}
```

## Debugging Position Issues

### 1. Enable Debug Output

Add debug logging to track positions:

```rust
// Debug: Log element positions
if let Some(box_manager) = get_box_by_name(&registry, hwnd) {
    if let Some(metrics) = box_manager.metrics(&registry) {
        let rect: Rect = metrics.into();
        eprintln!("[DEBUG] {} position: x={}, y={}, w={}, h={}", 
                 hwnd, rect.x, rect.y, rect.width, rect.height);
    }
}
```

### 2. Visual Debugging

Use visual indicators to see element boundaries:

```rust
// Add border for debugging
let block = Block::default()
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::Red));
f.render_widget(block, rect);
```

### 3. Registry Inspection

Inspect the registry state:

```rust
// List all registered elements
eprintln!("[DEBUG] Registered HWNDs:");
for (hwnd, _) in registry.boxes() {
    eprintln!("  - {}", hwnd);
}
```

## Advanced Topics

### Dynamic Position Updates

For elements that need to change position based on state:

```rust
// Update position based on content
let new_y = if has_content { base_y + content_height } else { base_y };

// Update the element's position
if let Some(box_manager) = get_box_by_name(&mut registry, hwnd) {
    box_manager.set_position(new_x, new_y);
}
```

### Responsive Layout

Implement responsive positioning based on terminal size:

```rust
// Adjust layout based on terminal size
let is_compact = area.width < 100;
let field_width = if is_compact { area.width - 10 } else { 60 };

// Position elements accordingly
```

## Summary

The HWND system provides a robust foundation for managing TUI element positioning. By following these guidelines and best practices, you can effectively update element positions and create responsive, maintainable UI layouts.

Key takeaways:
- Use descriptive HWND constants
- Leverage the RectRegistry for element management
- Position relative to anchors, not absolute coordinates
- Use the LayoutManager for consistent calculations
- Implement proper event handling for interactive elements
