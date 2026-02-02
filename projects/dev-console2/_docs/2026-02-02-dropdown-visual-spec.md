# Visual Specification: Unified Overlay Dropdown

## 1. Concept: "Unified Expansion"
The `OverlayDropdown` is designed to feel like an extension of the input field it anchors to. Instead of appearing as a separate floating menu, it "consumes" the input field's area and grows from its borders.

### Key Visual Rules:
- **Border**: Switches to **Cyan** when open.
- **Top Entry**: Always contains the current value of the field, positioned exactly where the input text was.
- **Unity**: The border encapsulates both the original field area and the list items as a single cohesive unit.
- **Background**: All other UI elements are dimmed via the `Dimmer` widget.



It should allow to be passed a max_shown which would govern the decision wether to expand up or down, as well as how large to renmder the dropdown display and scrollbar
 (Anchor Y + max_shown + 2 < Terminal Height).

---

## 2. Standard State (Before)
The field is rendered as a standard `ratatui` `Block`. The background is fully visible and high-contrast.

```text
 ┌───────────────────────────────────────────────────────────────┐
 │                                                               │
 │  Profile Configuration                                        │
 │                                                               │
 │  ┌─ Port ───────────────────────────────────────────────────┐ │
 │  │ COM3|                                                    │ │
 │  └──────────────────────────────────────────────────────────┘ │
 │                                                               │
 └───────────────────────────────────────────────────────────────┘
```

---

## 3. Dropdown State (After)

### 3.1 Variation: Opening Down
**Trigger**: Used when there is enough vertical space below
The background is dimmed (indicated by `.` markers). The `Port` field expands downward with a **Cyan** border.

```text
 ┌────────────────────────────────────────────────────────────────────────────
 │                                                                            
 │   Profile Configuration                                                    
 │                                                                            
 │  ┌─ Port ───────────────────────────────────────────────────┐              
 │  │ COM3                                                     │   
 │  ├──────────────────────────────────────────────────────────┤           
 │  │ COM1                                                    █│              
 │  │ COM4 (ESP32-S3)                                         ║│              
 │  │ /dev/ttyUSB0                                            ║│              
 │  └──────────────────────────────────────────────────────────┘              
 │                                                                            
 
```

### 3.2 Variation: Opening Up
**Trigger**: Used when space below is restricted. The list grows upward from the field's top border.

```text
 │  ┌─ Port ───────────────────────────────────────────────────┐              
 │  │ COM1                                                    █│              
 │  │ COM4 (ESP32-S3)                                         ║│              
 │  │ /dev/ttyUSB0                                            ║│              
 │  ├──────────────────────────────────────────────────────────┤           
 │  │ COM3                                                     │   
 │  └──────────────────────────────────────────────────────────┘
 │ 
 └───────────────────────────────────────────────────────────────────────────────────────────────────────────
 
```

---

## 4. Directional Logic
The widget determines its direction based on the `Rect` of the anchor field and the total terminal height.

```rust
// Pseudocode Logic
let needed_height = items.len().min(6);
let space_below = terminal_height - anchor.y;

if space_below >= needed_height {
    render_down(anchor, items);
} else {
    render_up(anchor, items);
}
```

## 5. Usage Best Practices
- **Consistency**: Use this overlay pattern for any field that has a pre-defined set of valid options (Ports, Profiles, Baud Rates).
- **Clearance**: Always use `f.render_widget(Clear, area)` before drawing the overlay to ensure the dimmed background doesn't bleed through.
- **Input Blocking**: While the dropdown is open, the `App` should ignore standard text input and only process `Up`, `Down`, `Enter`, and `Esc`.
