---
globs: **/*.rs

---
# TUI HWND System Rules

## HWND System Protocol

All UI elements in TUI applications must be registered with the HWND (handle) system using RectRegistry.

**AI Agent Directive**: Follow HWND system rules exactly for all TUI UI element registration and positioning.

**MANDATORY EXECUTION PROTOCOL**:

1. **NO DEVIATION**: All UI elements must be registered with HWND system
2. **NO SKIPPING**: No UI element may be rendered without HWND registration
3. **NO SELECTIVE COMPLIANCE**: All rules apply to all UI elements
4. **FAILURE TO COMPLY**: Violating these rules constitutes a critical protocol violation

## Registration Requirements

### **1. :: All UI Elements Must Use HWND**

**CRITICAL ENFORCEMENT**: All UI elements in TUI applications MUST be registered with the HWND (handle) system using RectRegistry. Every interactive or positionable UI element (fields, boxes, panels, lists, etc.) MUST:

1. **Have a HWND constant defined** in `constants.rs` with naming pattern `HWND_[DESCRIPTION]`
2. **Be registered using `registry.register(Some(handle_name), rect)`** during rendering
3. **Use registered positions** for rendering and interaction tracking
4. **Retrieve positions using `get_box_by_name(registry, handle_name)`** when needed for interaction

**✅ CORRECT - Register UI elements with HWND**:

```rust
// In constants.rs
pub const HWND_SETTINGS_FIELD_SKETCH_DIR: &str = "hwndSettingsFieldSketchDir";
pub const HWND_SETTINGS_FIELD_SKETCH_NAME: &str = "hwndSettingsFieldSketchName";
pub const HWND_PROFILE_BOX: &str = "hwndProfileBox";

// In render function
pub fn render_settings(
    f: &mut Frame,
    area: Rect,
    registry: &mut RectRegistry,
    // ... other params
) {
    // Calculate field area
    let field_area = Rect { /* ... */ };
    
    // Register with HWND
    if let Some(handle) = registry.get_handle(HWND_SETTINGS_FIELD_SKETCH_DIR) {
        registry.update(handle, field_area);
    } else {
        registry.register(Some(HWND_SETTINGS_FIELD_SKETCH_DIR), field_area);
    }
    
    // Render using registered area
    render_field(f, field_area, /* ... */);
}
```

**❌ INCORRECT - Direct rendering without HWND registration**:

```rust
// Wrong: Rendering directly without registration
pub fn render_settings(f: &mut Frame, area: Rect, /* ... */) {
    let field_area = Rect { /* ... */ };
    render_field(f, field_area, /* ... */);  // No HWND registration!
}
```

### **2. :: HWND Constant Naming Convention**

**✅ CORRECT - Use consistent naming pattern**:

```rust
// Format: HWND_[COMPONENT]_[DESCRIPTION]
pub const HWND_SETTINGS_FIELD_SKETCH_DIR: &str = "hwndSettingsFieldSketchDir";
pub const HWND_SETTINGS_FIELD_SKETCH_NAME: &str = "hwndSettingsFieldSketchName";
pub const HWND_SETTINGS_SECTION_DEVICE: &str = "hwndSettingsSectionDevice";
pub const HWND_SETTINGS_SECTION_CONNECTION: &str = "hwndSettingsSectionConnection";
pub const HWND_PROFILE_BOX: &str = "hwndProfileBox";
pub const HWND_PROFILE_LIST: &str = "hwndProfileList";
```

**❌ INCORRECT - Inconsistent or missing constants**:

```rust
// Wrong: No constants defined
// Wrong: Inconsistent naming
pub const FIELD_1: &str = "field1";  // Should be HWND_ prefix
```

### **3. :: Registration Pattern**

**✅ CORRECT - Register or update pattern**:

```rust
// Standard pattern: Check if exists, update if present, register if new
if let Some(handle) = registry.get_handle(hwnd_name) {
    registry.update(handle, rect);
} else {
    registry.register(Some(hwnd_name), rect);
}
```

**✅ CORRECT - Register all elements in render function**:

```rust
pub fn render_settings(
    f: &mut Frame,
    area: Rect,
    registry: &mut RectRegistry,
    // ... other params
) {
    // Register profile box
    let profile_area = calculate_profile_area(area);
    register_or_update(registry, HWND_PROFILE_BOX, profile_area);
    
    // Register each field
    for (index, field_area) in calculate_field_areas(area).iter().enumerate() {
        let hwnd = get_field_hwnd(index);
        register_or_update(registry, hwnd, *field_area);
    }
    
    // Then render using registered areas
    // ...
}
```

### **4. :: Interaction Tracking**

**✅ CORRECT - Use registered positions for interaction**:

```rust
// In event handler
pub fn handle_settings_click(
    mouse_event: &MouseEvent,
    registry: &RectRegistry,
) -> Option<usize> {
    // Get field positions from registry
    for (index, hwnd) in FIELD_HWNDS.iter().enumerate() {
        if let Some(box_manager) = get_box_by_name(registry, hwnd) {
            if let Some(rect) = box_manager.metrics(registry) {
                if is_point_in_rect(mouse_event, rect) {
                    return Some(index);
                }
            }
        }
    }
    None
}
```


## Common Mistakes

- ❌ **Direct Rendering** - Don't render UI elements without HWND registration
- ❌ **Missing Constants** - Don't create UI elements without HWND constants
- ❌ **Inconsistent Naming** - Don't use inconsistent HWND naming patterns
- ❌ **Skipping Registration** - Don't skip registration for "simple" elements
- ❌ **Not Updating Registry** - Don't forget to update registry when positions change

## Checklist

- [ ] **All Elements Registered**: Every UI element has HWND registration
- [ ] **Constants Defined**: All HWND constants defined in constants.rs
- [ ] **Consistent Naming**: All HWND names follow naming convention
- [ ] **Registration Pattern**: All elements use register/update pattern
- [ ] **Interaction Tracking**: Mouse/keyboard interactions use registered positions
