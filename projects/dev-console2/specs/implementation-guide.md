# Implementation Guide

## Quick Start

This guide provides step-by-step instructions for implementing the dev-console-v2 TUI application.

## Prerequisites

- Rust 1.70+ installed
- Terminal that supports ANSI escape codes
- Basic understanding of Rust and terminal applications

## Step 1: Project Setup

The project structure has been created with:

```
dev-console2/
├── Cargo.toml              # Dependencies and project metadata
├── README.md               # Project documentation
├── src/
│   ├── main.rs            # Application entry point
│   ├── app.rs             # Core application logic
│   └── terminal.rs        # Terminal management
└── specs/
    ├── ui-layout-spec.md      # UI layout specification
    ├── elm-architecture-spec.md # Architecture details
    └── implementation-guide.md # This guide
```

## Step 2: Build and Run

### Build the Application

```bash
cd D:\_dev\_Projects\dev-boards\projects\dev-console2
cargo build
```

### Run the Application

```bash
cargo run
```

### Run Tests

```bash
cargo test
```

## Step 3: Understanding the Code

### Main Application Flow (`src/main.rs`)

1. **Initialization**: Sets up error handling and terminal
2. **Main Loop**: Continues while `app.running` is true
3. **Event Handling**: Maps keyboard input to messages
4. **State Updates**: Processes messages through the update function
5. **Rendering**: Draws the UI on each iteration

### Core Application Logic (`src/app.rs`)

1. **State Management**: `App` struct holds application state
2. **Message System**: `Message` enum defines all possible actions
3. **Update Function**: Transforms state based on messages
4. **View Function**: Renders UI components based on state
5. **Component Rendering**: Individual functions for each UI element

### Terminal Management (`src/terminal.rs`)

1. **Initialization**: Sets up raw terminal mode
2. **Cleanup**: Restores terminal to original state
3. **Panic Handling**: Ensures terminal is restored on crashes

## Step 4: UI Component Details

### Layout Structure

The UI uses a vertical layout with 5 sections:

```rust
let vertical_layout = Layout::vertical([
    Constraint::Length(3),  // Title Bar (3 rows)
    Constraint::Min(0),     // Main Content (flexible)
    Constraint::Length(1),  // Bindings (1 row)
    Constraint::Length(1),  // Divider (1 row)
    Constraint::Length(1),  // Status Bar (1 row)
]);
```

### Component Rendering

Each component has its own rendering function:

- `render_title_bar()` - Bordered box with centered title
- `render_main_content()` - Bordered empty area for future content
- `render_bindings()` - Text display for keyboard bindings
- `render_divider()` - Horizontal line separator
- `render_status_bar()` - Text display for status information

## Step 5: Adding New Features

### Adding a New Keyboard Shortcut

1. **Add Message Variant**:
```rust
#[derive(PartialEq, Debug)]
pub enum Message {
    Quit,
    Refresh,  // New message
}
```

2. **Map Key to Message**:
```rust
fn map_key_to_message(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Some(Message::Quit),
        KeyCode::Char('r') => Some(Message::Refresh),  // New mapping
        _ => None,
    }
}
```

3. **Handle Message in Update**:
```rust
pub fn update(&mut self, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            self.running = false;
        }
        Message::Refresh => {
            // Handle refresh logic
        }
    }
    None
}
```

### Adding a New UI Component

1. **Add Layout Constraint**:
```rust
let vertical_layout = Layout::vertical([
    Constraint::Length(3),  // Title Bar
    Constraint::Min(0),     // Main Content
    Constraint::Length(1),  // New Component
    Constraint::Length(1),  // Bindings
    Constraint::Length(1),  // Divider
    Constraint::Length(1),  // Status Bar
]);
```

2. **Add Rendering Function**:
```rust
fn render_new_component(&self, frame: &mut Frame, area: Rect) {
    let paragraph = Paragraph::new("New Component")
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}
```

3. **Call in View Method**:
```rust
let [title_area, main_area, new_area, bindings_area, divider_area, status_area] = 
    vertical_layout.areas(frame.area());

// ... existing components ...
self.render_new_component(frame, new_area);
// ... remaining components ...
```

## Step 6: Testing and Debugging

### Running Tests

```bash
cargo test
```

### Debug Mode

Add debug output to understand application flow:

```rust
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub ui_state: UiState,
}

// In update function:
pub fn update(&mut self, msg: Message) -> Option<Message> {
    println!("Processing message: {:?}", msg);  // Debug output
    match msg {
        // ... existing logic
    }
}
```

### Common Issues

1. **Terminal Not Restored**: Ensure panic hook is installed
2. **Layout Issues**: Check constraint values and area calculations
3. **Event Handling**: Verify key codes and event polling
4. **State Issues**: Check message handling and state updates

## Step 7: Performance Considerations

### Rendering Optimization

- Minimize expensive operations in the view function
- Use efficient layout calculations
- Avoid unnecessary string allocations

### Event Handling

- Use appropriate polling intervals (250ms is reasonable)
- Filter events properly (only handle key press events)
- Batch multiple updates when possible

## Step 8: Next Steps

### Potential Enhancements

1. **Interactive Elements**: Add lists, tables, or forms to main content
2. **Configuration**: Add config file support for customization
3. **Themes**: Implement color schemes and styling options
4. **Plugins**: Design extensible architecture for plugins
5. **Data Integration**: Connect to external data sources

### Advanced Features

1. **Multi-threading**: Background tasks with thread-safe communication
2. **State Persistence**: Save and restore application state
3. **Keyboard Macros**: Record and replay keyboard sequences
4. **Help System**: Interactive help and documentation
5. **Performance Monitoring**: Resource usage display

## Resources

- [Ratatui Documentation](https://docs.rs/ratatui/latest/ratatui/)
- [Elm Architecture Guide](https://ratatui.rs/concepts/application-patterns/the-elm-architecture/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Crossterm Documentation](https://docs.rs/crossterm/latest/crossterm/)
