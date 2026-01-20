# Elm Architecture Specification for Dev Console V2

## Architecture Overview

The application follows the Elm Architecture (TEA) pattern with three core components:
1. **Model** - Application state
2. **Update** - State transformation logic
3. **View** - UI rendering function

## Core Components

### 1. Model Structure

```rust
#[derive(Debug, Default)]
pub struct Model {
    // Application state
    pub running_state: RunningState,
    // UI state
    pub ui_state: UiState,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default)]
pub struct UiState {
    // Future UI state can be added here
    // e.g., active_panel, selected_item, etc.
}
```

### 2. Message System

```rust
#[derive(PartialEq)]
pub enum Message {
    // Input handling
    Quit,
    // Future messages can be added
    // e.g., NavigateUp, NavigateDown, SelectItem, etc.
}
```

### 3. Update Function

```rust
pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            model.running_state = RunningState::Done;
        }
        // Handle other messages
    }
    None // Return None unless chaining messages
}
```

### 4. View Function

```rust
pub fn view(model: &mut Model, frame: &mut Frame) {
    // Layout creation
    let vertical_layout = Layout::vertical([
        Length(3),   // Title Bar
        Min(0),      // Main Content
        Length(1),   // Bindings
        Length(1),   // Divider
        Length(1),   // Status Bar
    ]);
    
    let [title_area, main_area, bindings_area, divider_area, status_area] = 
        vertical_layout.areas(frame.area());
    
    // Render each component
    render_title_bar(frame, title_area);
    render_main_content(frame, main_area);
    render_bindings(frame, bindings_area);
    render_divider(frame, divider_area);
    render_status_bar(frame, status_area);
}
```

## Application Flow

### Main Loop Pattern

```rust
fn main() -> color_eyre::Result<()> {
    // Initialize terminal
    let mut terminal = init_terminal()?;
    let mut model = Model::default();
    
    // Main application loop
    while model.running_state != RunningState::Done {
        // Render current view
        terminal.draw(|f| view(&mut model, f))?;
        
        // Handle events and map to Message
        let mut current_msg = handle_event(&model)?;
        
        // Process updates (allow message chaining)
        while let Some(msg) = current_msg {
            current_msg = update(&mut model, msg);
        }
    }
    
    // Cleanup
    restore_terminal()?;
    Ok(())
}
```

### Event Handling

```rust
fn handle_event(_: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return Ok(map_key_to_message(key));
            }
        }
    }
    Ok(None)
}

fn map_key_to_message(key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Some(Message::Quit),
        // Future key mappings
        _ => None,
    }
}
```

## Component Rendering Functions

### Individual Component Functions

```rust
fn render_title_bar(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title("Title Bar")
        .title_alignment(Alignment::Center);
    frame.render_widget(block, area);
}

fn render_main_content(frame: &mut Frame, area: Rect) {
    let block = Block::bordered();
    frame.render_widget(block, area);
}

fn render_bindings(frame: &mut Frame, area: Rect) {
    let paragraph = Paragraph::new("Bindings")
        .alignment(Alignment::Left);
    frame.render_widget(paragraph, area);
}

fn render_divider(frame: &mut Frame, area: Rect) {
    let divider = Divider::default();
    frame.render_widget(divider, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect) {
    let paragraph = Paragraph::new("Status Bar")
        .alignment(Alignment::Left);
    frame.render_widget(paragraph, area);
}
```

## State Management Principles

### Immutability Preference
- The update function prefers immutability but can use mutable references for performance
- State changes are predictable and deterministic
- Each message produces a consistent state transition

### Message Chaining
- Update function can return `Option<Message>` for state machine patterns
- Enables complex state transitions and automated workflows
- Maintains single-threaded execution model

### Pure View Function
- View function is side-effect free (except for frame rendering)
- Given the same model state, produces identical UI output
- Enables predictable UI rendering and easier debugging

## Extensibility Points

### Adding New UI Components
1. Add new rendering function
2. Update layout constraints
3. Add component state to `UiState` if needed
4. Add corresponding messages if interactive

### Adding New Features
1. Define new `Message` variants
2. Handle messages in `update` function
3. Update `UiState` structure if needed
4. Modify view to display new state

### Error Handling
- Uses `color_eyre` for ergonomic error handling
- Terminal initialization/cleanup with proper error propagation
- Panic hooks for safe terminal restoration

## Testing Strategy

### Unit Tests
- Test update function logic
- Test message mapping
- Test state transitions

### Integration Tests
- Test full application lifecycle
- Test terminal initialization/cleanup
- Test event handling pipeline

### Visual Testing
- Test layout rendering
- Test component positioning
- Test responsive behavior
