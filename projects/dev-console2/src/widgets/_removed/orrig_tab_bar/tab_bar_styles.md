# Tab Bar Styles and States Documentation

## Overview

The tab bar widget supports multiple visual styles and interaction types, each designed for specific use cases in terminal user interfaces.

## Style Types

### 1. Tab Style (`"tabbed"`)
**Visual Appearance:**
```
  ╭──────╮
──╯ Tab1 ╰─ Tab 2 ─ Tab 3 ──
```

**Characteristics:**
- Curved brackets (`╯` and `╰`) around the active tab
- Decorative top line (`╭─────╮`) above the active tab
- Only the active tab has special formatting
- Requires 2 lines of vertical space (top decorative line + tab line)
- Creates a "file folder tab" visual metaphor

**Best For:**
- Primary navigation in main application windows
- When you want to emphasize the active tab prominently
- Applications where the tab metaphor is central to the UI

**State Support:**
- `active` boolean flag for highlighting the current tab
- Navigation via keyboard (PgUp/PgDown, arrow keys, etc.)

---

### 2. Boxed Style (`"boxed"`)
**Visual Appearance:**
```
──[ Tab 1 ]─ Tab 2 ─ Tab 3 ──
```

**Characteristics:**
- Square brackets (`[` and `]`) around the active tab
- Only the active tab has special formatting
- Requires 1 line of vertical space
- Clean, minimalist appearance
- Less visually intrusive than tabbed style

**Best For:**
- Secondary navigation or tool palettes
- When you want a subtle active indicator
- Professional/utility applications
- Interfaces with multiple tab bars

**State Support:**
- `active` boolean flag for highlighting the current tab
- Navigation via keyboard

---

### 3. Text Style (`"text"`)
**Visual Appearance:**
```
─ Tab 1 ─ Tab 2 ─ Tab 3 ─
```

**Characteristics:**
- No special formatting for any tabs
- All tabs appear as plain text with separators
- Requires 1 line of vertical space
- Most minimal visual footprint
- Relies on positioning/color to indicate active state

**Best For:**
- Status indicators or non-interactive tab lists
- When tabs are purely informational
- Minimalist interfaces
- Display-only scenarios

**State Support:**
- `active` boolean flag (typically used for color emphasis)
- Navigation via keyboard

---

### 4. Box Static Style (`"box_static"`)
**Visual Appearance:**
```
──[ Tab 1 ]─[ Tab 2 ]─[ Tab 3 ]──
```

**Characteristics:**
- All tabs have square brackets
- No concept of "active" tab - all appear equal
- Requires 1 line of vertical space
- Uniform appearance across all tabs
- Static, non-interactive display

**Best For:**
- Display mode indicators
- Status bars showing multiple states
- Non-interactive information display
- When you want to show multiple options without hierarchy

**State Support:**
- **State-based coloring** when `type: "state"`
- **Button interaction** when `type: "button"`
- No navigation (static display)

---

### 5. Text Static Style (`"text_static"`)
**Visual Appearance:**
```
─ Tab 1 ─ Tab 2 ─ Tab 3 ─
```

**Characteristics:**
- All tabs appear as plain text with separators
- No active tab concept
- Requires 1 line of vertical space
- Most minimal static display
- Uniform appearance

**Best For:**
- Simple status displays
- Non-interactive information
- Minimalist interfaces
- When you want to show options without visual hierarchy

**State Support:**
- **State-based coloring** when `type: "state"`
- **Button interaction** when `type: "button"`
- No navigation (static display)

---

## Interaction Types

### Navigation Type (Default)
**Applicable Styles:** `tabbed`, `boxed`, `text`

**Configuration:**
```yaml
navigation:
  left: ["PgUp", "Left", "h"]
  right: ["PgDown", "Right", "l"]
```

**Behavior:**
- Single active tab that can be changed
- Keyboard navigation between tabs
- Active tab highlighted with style-specific formatting
- Tab switching updates application state

**Use Case:**
- Primary application navigation
- Content switching
- Multi-panel interfaces

---

### State Type
**Applicable Styles:** `box_static`, `text_static`

**Configuration:**
```yaml
type: "state"
colors:
  active: "green"    # Currently active/selected state
  negate: "red"      # Inactive/disabled state  
  disabled: null       # Use default color for disabled
```

**Behavior:**
- Each tab can have independent state
- Colors indicate state without changing tab structure
- No navigation or active tab concept
- States: `active`, `negate`, `disabled`, `default`

**Use Case:**
- Status indicators (connection status, build states, etc.)
- Multi-state display panels
- When you need to show conditions rather than navigation

---

### Button Type (NEW)
**Applicable Styles:** `box_static`, `text_static`

**Configuration:**
```yaml
type: "button"
colors:
  active: null        # Normal button appearance
  hover: "cyan"       # Mouse hover color
  # negate: "red"    # Optional: disabled/pressed state
on_click: "functionToExecute"  # Action to trigger
```

**Behavior:**
- Each tab acts as an independent button
- Click/tap triggers specific actions
- Visual feedback on hover (and optional press states)
- No navigation or single-active concept
- Each button can execute different functions

**Use Case:**
- Tool palettes with actions
- Quick action bars
- When tabs represent commands rather than content
- Interactive toolbars
- Status panels with actionable items

---

## Configuration Examples

### Navigation Tab Bar
```yaml
- id: "MainContentTabBar"
  style: "tabbed"
  color: "cyan"
  navigation:
    left: ["PgUp"]
    right: ["PgDown"]
  tabs:
    - id: "dashboard"
      name: "Dashboard"
      default: "active"
    - id: "settings"
      name: "Settings"
```

### State-Based Static Bar
```yaml
- id: "StatusBar"
  style: "box_static"
  type: "state"
  colors:
    active: "green"
    negate: "red"
  tabs:
    - id: "connected"
      name: "Connected"
      default: "active"
    - id: "disconnected"
      name: "Disconnected"
```

### Button-Based Static Bar
```yaml
- id: "ToolBar"
  style: "text_static"
  type: "button"
  colors:
    hover: "cyan"
  tabs:
    - id: "build"
      name: "Build"
      on_click: "buildProject"
    - id: "test"
      name: "Test"
      on_click: "runTests"
```

---

## Implementation Notes

### Height Requirements
- **Tab style:** 2 lines (decorative top + tab line)
- **All other styles:** 1 line

### Layout Integration
- Tab style requires special layout handling to overlap with parent borders
- Static styles can be rendered as simple overlays
- Button type requires mouse event handling

### Color Mapping
- Colors use ratatui color names: `"cyan"`, `"green"`, `"red"`, etc.
- `null` values use default terminal colors
- State colors override the main `color` property

### Navigation Defaults
- If no `navigation` section is provided:
  - Left: `["Left", "h"]`
  - Right: `["Right", "l"]
