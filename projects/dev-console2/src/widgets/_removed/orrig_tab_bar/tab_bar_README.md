# TabBarWidget

A highly configurable and interactive tab bar component for Ratatui-based TUIs. The `TabBarWidget` supports multiple visual themes, dynamic alignment, and integrated mouse hit detection, making it suitable for both primary navigation and secondary toggle controls.

## Key Features

- **Thematic Variety**: Choose from multiple styles including traditional Tabs, Boxed highlights, and minimalist Text.
- **Dynamic Alignment**: Position tabs at the Top, Bottom, Left, Center, or Right of their parent area.
- **Interactive Hit Detection**: Built-in support for mouse click detection, mapping screen coordinates back to tab indices.
- **Composite Rendering**: Specialized helper for managing layout-defining tab bars that consume header or footer space.
- **ANSI Color Support**: Full support for standard and indexed ANSI colors, decoupled from the main theme engine for flexibility.

## Tech Stack

- **Language**: Rust (Edition 2021)
- **TUI Framework**: `ratatui`
- **Terminal Backend**: `crossterm` (for mouse events)
- **Serialization**: `serde` (for configuration mapping)

## Getting Started

### 1. Define Tab Items
Tabs are represented by the `TabBarItem` struct, which tracks the display name, unique ID, and active state.

```rust
use crate::widgets::tab_bar::TabBarItem;

let tabs = vec![
    TabBarItem { id: "dash".into(), name: "Dashboard".into(), active: true },
    TabBarItem { id: "logs".into(), name: "Logs".into(), active: false },
];
```

### 2. Instantiate the Widget
Use the builder pattern to configure the widget's appearance.

```rust
use crate::widgets::tab_bar::{TabBarWidget, TabBarStyle};
use ratatui::style::Color;

let widget = TabBarWidget::new(&tabs)
    .style(TabBarStyle::Boxed)
    .color(Color::Gray)
    .active_color(Some(Color::Cyan))
    .min_tab_width(10);
```

### 3. Render
Render the widget into a specific area of the buffer.

```rust
// Standard render
frame.render_widget(widget, area);

// Aligned render with offsets
widget.render_aligned(
    area,
    TabBarAlignment::Center,
    TabBarAlignment::Top,
    0, 0,
    frame.buffer_mut()
);
```

---

## Architecture

### Directory Structure
```
src/widgets/
└── tab_bar.rs       # Complete implementation including tests and parsers
```

### State Management (`TabBarItem`)
The widget is stateless during the render pass, receiving a slice of `TabBarItem` that defines the current "truth" of the UI.
- `id`: Used for semantic identification in event loops.
- `name`: The text displayed on the tab.
- `active`: Boolean flag driving the highlight logic.

### Visual Themes (`TabBarStyle`)
- `Tab`: Two-line style with decorative `╭───╮` / `╯   ╰` borders. Requires `desired_height() == 2`.
- `Boxed`: Single-line style using `[` and `]` delimiters.
- `Text`: Minimalist style using spacing and bold modifiers.
- `BoxStatic` / `TextStatic`: Variants designed for persistent toggles that use "negate" colors for inactive states.

### Hit Detection Logic
The `handle_mouse_event` method abstracts the complexity of alignment and offsets:
1. Calculates the `aligned_area` based on current settings.
2. Verifies if the mouse click (Crossterm `MouseEvent`) is within the widget's bounds.
3. Iterates through the item widths to determine which specific tab was struck.
4. Returns the `usize` index of the tab, or `None`.

---

## Implementation Examples

### 1. Primary Tabbed Header
Used as the main navigation element in `dev-console-v2`. It uses `render_composite` to automatically "carve out" space from the main application area.

**Usage in `view.rs`:**
```rust
TabBarWidget::render_composite(
    &self.config,
    &self.tabs,
    &["MainContentTabBar"],
    layout.main,
    frame.buffer_mut(),
);
```

**How it works:**
- `render_composite` looks up the configuration by ID.
- It calculates `config_consumed_height` to determine if space should be taken from the Top or Bottom.
- It renders the `Block::bordered()` container.
- It renders the `TabBarWidget` into the reserved header/footer space.

### 2. Output Panel Auto-Scroll Toggle
A minimalist use case demonstrating "Static" styles for boolean toggles.

**Usage in `mod.rs` (Hit Detection):**
```rust
let output_static_tabs = vec![TabBarItem {
    id: "autoscroll".into(),
    name: "Auto".into(),
    active: self.output_autoscroll,
}];

if let Some((tab_bar, horiz, vert, off_x, off_y)) = TabBarWidget::from_config(
    &self.config, &output_static_tabs, "OutputPanelStaticOptions"
) {
    if tab_bar.handle_mouse_event(layout.output, horiz, vert, off_x, off_y, mouse_event).is_some() {
        self.dispatch_command(Action::ToggleAutoscroll);
    }
}
```

**How it works:**
- A transient `TabBarItem` is created representing the current `bool` state.
- The configuration `OutputPanelStaticOptions` typically uses `TabBarStyle::TextStatic` or `BoxStatic`.
- When clicked, it toggles the application state, which then flows back into the next render pass.

---

## Configuration Reference

The widget is typically driven by `config.yaml`. Example structure:

```yaml
tab_bars:
  - id: MainContentTabBar
    style: tab
    alignment:
      horizontal: center
      vertical: top
      offset_y: -1 # Pull up into the border
    tabs:
      - id: dashboard
        name: DASHBOARD
        default: active
      - id: profiles
        name: PROFILES
```

| Property | Description | Default |
|----------|-------------|---------|
| `style` | `tab`, `boxed`, `text`, `box_static`, `text_static` | `text` |
| `alignment` | Horizontal and Vertical positioning rules | `center`, `top` |
| `color` | Base foreground color (Crayola-style names) | `white` |
| `min_tab_width` | Ensures tabs aren't too small for mouse hits | `0` |
| `offset_x/y` | Fine-grained coordinate adjustment | `0` |

---

## Troubleshooting

### Mouse Hits Not Registering
- **Check Alignment**: Ensure the `aligned_area` passed to `handle_mouse_event` matches the one used in `render`. Using `render_aligned` or `render_composite` handles this automatically.
- **Terminal Width**: If the terminal is too narrow, the `estimate_width` might exceed available space, causing the hit-box to clip or wrap unpredictably.

### Tab Style Rendering Issues
- **Height Constraints**: The `Tab` style requires a height of at least 2. If rendered in a 1-height area, it will automatically fallback to the single-line `build_tab_line`.
- **Z-Index/Overlays**: Ensure the tab bar isn't being overwritten by a `Block` border. Use `offset_y: -1` to render the tab bar *on top* of a parent border.
