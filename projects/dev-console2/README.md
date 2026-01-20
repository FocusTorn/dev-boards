# Dev Console V2

A terminal user interface (TUI) application built with Rust using the Elm architecture pattern and Ratatui library.

## Features

- **Elm Architecture**: Clean separation of Model, Update, and View
- **Responsive Layout**: Adapts to terminal size changes
- **Component-Based UI**: Modular rendering system
- **Event Handling**: Keyboard input processing with message system

## UI Layout

The application displays 5 distinct components:

1. **Title Bar** - 3 rows high, thin bordered, centered "Title Bar" text
2. **Main Content** - Full width, remaining height, thin bordered, no label
3. **Bindings** - 1 row high, no border, left-justified "Bindings" text
4. **Divider** - Simple thin horizontal line
5. **Status Bar** - 1 row high, no border, left-justified "Status Bar" text

## Architecture

### Elm Architecture Components

- **Model**: Application state (`App` struct)
- **Message**: Events and actions (`Message` enum)
- **Update**: State transformation logic (`update` method)
- **View**: UI rendering (`view` method)

### File Structure

```
src/
├── main.rs          # Application entry point and main loop
├── app.rs           # Core application logic and UI rendering
└── terminal.rs      # Terminal initialization and management

specs/
├── ui-layout-spec.md      # Detailed UI layout specification
└── elm-architecture-spec.md # Elm architecture implementation guide
```

## Dependencies

- `ratatui` - Terminal UI framework
- `crossterm` - Cross-platform terminal handling
- `color-eyre` - Ergonomic error handling

## Usage

### Installation

```bash
cargo build --release
```

### Running

```bash
cargo run
```

### Controls

- `q` or `Esc` - Quit the application

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running in Development Mode

```bash
cargo run
```

## Extending the Application

### Adding New UI Components

1. Add a new rendering function in `app.rs`
2. Update the layout constraints in the `view` method
3. Add component state to `UiState` if needed
4. Add corresponding messages if the component is interactive

### Adding New Features

1. Define new `Message` variants in the enum
2. Handle messages in the `update` method
3. Update the `UiState` structure if needed
4. Modify the view to display the new state

### Example: Adding a New Message

```rust
#[derive(PartialEq, Debug)]
pub enum Message {
    Quit,
    NewFeature,  // Add new message
}

impl App {
    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Quit => {
                self.running = false;
            }
            Message::NewFeature => {
                // Handle new feature logic
            }
        }
        None
    }
}
```

## Testing

The application includes unit tests for core functionality:

```bash
cargo test
```

Tests cover:
- Application state management
- Message handling
- State transitions

## License

MIT License - see LICENSE file for details.
