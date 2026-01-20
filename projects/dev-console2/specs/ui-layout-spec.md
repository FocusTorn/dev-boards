# Dev Console V2 - TUI Specification

## Overview
A terminal user interface (TUI) application built with Rust using the Elm architecture pattern and Ratatui library for rendering.

## UI Layout Specification

### Component 1: Title Bar
- **Position**: Top of screen, full width
- **Height**: 3 rows high
- **Border**: Thin bordered box
- **Content**: Centered text displaying "Title Bar"
- **Implementation**: `Block::bordered().title("Title Bar")` with centered alignment

### Component 2: Main Content Area
- **Position**: Below title bar, full width
- **Height**: Remaining height (excluding other components)
- **Border**: Thin bordered box
- **Content**: No label, empty content area for future expansion
- **Implementation**: `Block::bordered()` without title

### Component 3: Bindings Display
- **Position**: Above status bar, full width
- **Height**: 1 row high
- **Border**: No border
- **Content**: Left-justified text displaying "Bindings"
- **Implementation**: `Paragraph::new("Bindings")` with left alignment

### Component 4: Divider Line
- **Position**: Between bindings and status bar
- **Height**: 1 row high
- **Border**: Simple thin divider line
- **Content**: Horizontal line spanning full width
- **Implementation**: `Divider::default()` or custom line widget

### Component 5: Status Bar
- **Position**: Bottom of screen, full width
- **Height**: 1 row high
- **Border**: No border
- **Content**: Left-justified text displaying "Status Bar"
- **Implementation**: `Paragraph::new("Status Bar")` with left alignment

## Visual Layout

```
┌─ Title Bar (centered, 3 rows high, thin border) ──────────────────────┐
│                                                                     │
│                        Title Bar                                    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
┌─ Main Content Area (full width, remaining height, thin border) ────┐
│                                                                     │
│                                                                     │
│                                                                     │
│                  (empty content area)                              │
│                                                                     │
│                                                                     │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
Bindings (left justified, 1 row, no border)
───────────────────────────────────────────────────────────────────────
Status Bar (left justified, 1 row, no border)
```

## Layout Constraints

Using Ratatui's Layout system with vertical constraints:
1. **Title Bar**: `Length(3)` - Fixed 3 rows
2. **Main Content**: `Min(0)` - Minimum 0 rows, takes remaining space
3. **Bindings**: `Length(1)` - Fixed 1 row
4. **Divider**: `Length(1)` - Fixed 1 row
5. **Status Bar**: `Length(1)` - Fixed 1 row

## Responsive Design

- The layout automatically adjusts to terminal size changes
- All components maintain their relative positions
- Main content area expands/contracts based on available terminal height
- Full-width components adapt to terminal width changes

## Future Extensibility

The main content area is designed to accommodate:
- Multiple panels or widgets
- Dynamic content based on application state
- Interactive elements (lists, tables, forms)
- Real-time data display
