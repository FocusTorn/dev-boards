# Keybindings Guide

This guide explains how keybindings work in the dev-console and how to add, remove, or customize them. The application uses a flexible keybinding system that supports both global and context-specific shortcuts.

## Overview

The keybinding system is divided into three main categories:

1. **Global Bindings** - Available throughout the application
2. **Tab-Specific Bindings** - Active only in specific tabs
3. **Modal Bindings** - Active during specific modal states (editing, selecting, etc.)

## Configuration Structure

Keybindings are defined in `config.yaml`:

```yaml
application:
    # Global keyboard bindings
    bindings:
        - key: "[q]"
          description: "Quit"

tab_bars:
    main_content_tab_bar:
        # Tab-specific navigation
        navigation:
            left: ["Left"]
            right: ["Right"]

tab_content:
    - tab_id: "dashboard"
      bindings:
        - key: "[🡘 🡙]"
          description: "Navigate"
        - key: "[Enter]"
          description: "Execute Command"
    
    - tab_id: "settings"
      bindings:
        - key: "[🡙]"
          description: "Navigate Profiles"
        - key: "[Tab]"
          description: "Next field"
        - key: "[Shift+Tab]"
          description: "Previous field"
        - key: "[Enter/Click]"
          description: "Confirm/Edit field"
```

## Default Keybindings

### Global Bindings

| Key | Action | Context |
|-----|--------|---------|
| `q` | Quit application | Always available |
| `Esc` | Cancel/Exit | Modal states, running commands |

### Dashboard Tab

| Key | Action | Description |
|-----|--------|-------------|
| `↑` / `k` | Navigate up | Move to previous command |
| `↓` / `j` | Navigate down | Move to next command |
| `Enter` | Execute command | Run selected command |
| `Mouse Scroll` | Scroll output | Navigate command output |
| `Click Profile Selector` | Open profile selection | Switch development profile |

### Settings Tab

| Key | Action | Description |
|-----|--------|-------------|
| `↑` / `↓` | Navigate profiles | Move between profile list items |
| `Tab` | Next field | Move to next settings field |
| `Shift+Tab` | Previous field | Move to previous settings field |
| `Enter` | Edit field | Enter edit mode for selected field |
| `Click` | Select field | Click to select a field |
| `s` | Save profile | Save current settings as profile |
| `l` | Load profile | Load selected profile |
| `r` | Refresh profiles | Refresh profile list |

### Modal States

#### Field Editing Mode

| Key | Action | Description |
|-----|--------|-------------|
| `Enter` | Confirm | Save field changes |
| `Esc` | Cancel | Discard changes |
| `Text input` | Type | Enter text values |
| `Arrow keys` | Navigate | Move cursor in text |

#### Dropdown Selection Mode

| Key | Action | Description |
|-----|--------|-------------|
| `↑` / `↓` | Navigate options | Move between dropdown options |
| `Enter` | Select | Choose highlighted option |
| `Esc` | Cancel | Close dropdown without selection |

## Adding New Keybindings

### 1. Global Bindings

Add to `config.yaml` under `application.bindings`:

```yaml
application:
    bindings:
        - key: "[q]"
          description: "Quit"
        - key: "[Ctrl+s]"
          description: "Save settings"
        - key: "[F1]"
          description: "Show help"
```

### 2. Tab-Specific Bindings

Add to the appropriate tab in `tab_content`:

```yaml
tab_content:
    - tab_id: "dashboard"
      bindings:
        - key: "[🡘 🡙]"
          description: "Navigate"
        - key: "[Ctrl+r]"
          description: "Refresh commands"
```

### 3. Implement Handler Logic

Add the key handling logic in `src/event_handler.rs`:

```rust
// For dashboard keys
pub fn handle_dashboard_key_event(
    key_code: crossterm::event::KeyCode,
    dashboard: &Arc<Mutex<DashboardState>>,
    settings_manager: &SettingsManager,
    process_manager: Arc<ProcessManager>,
) -> bool {
    match key_code {
        // Existing handlers...
        
        // New keybinding
        KeyCode::Char('r') if key_modifiers.contains(KeyModifiers::CONTROL) => {
            // Handle Ctrl+R - refresh commands
            let mut state = dashboard.lock().unwrap();
            state.refresh_commands();
            true
        }
        
        _ => false,
    }
}
```

### 4. Handle Modifiers

For key combinations with modifiers:

```rust
use crossterm::event::{KeyCode, KeyModifiers};

match key_code {
    KeyCode::Char('s') if key_modifiers.contains(KeyModifiers::CONTROL) => {
        // Ctrl+S
        handle_save_settings();
        true
    }
    KeyCode::Char('z') if key_modifiers.contains(KeyModifiers::CONTROL) => {
        // Ctrl+Z
        handle_undo();
        true
    }
    _ => false,
}
```

## Removing Keybindings

### 1. Remove from Configuration

Simply remove the entry from `config.yaml`:

```yaml
# Before
application:
    bindings:
        - key: "[q]"
          description: "Quit"
        - key: "[Ctrl+s]"  # Remove this line
          description: "Save settings"

# After
application:
    bindings:
        - key: "[q]"
          description: "Quit"
```

### 2. Remove Handler Logic

Remove or comment out the corresponding handler code:

```rust
pub fn handle_dashboard_key_event(/* ... */) -> bool {
    match key_code {
        // Existing handlers...
        
        // Remove or comment this block
        /*
        KeyCode::Char('r') if key_modifiers.contains(KeyModifiers::CONTROL) => {
            handle_refresh_commands();
            true
        }
        */
        
        _ => false,
    }
}
```

## Advanced Keybinding Features

### 1. Context-Aware Bindings

Implement different behavior based on application state:

```rust
pub fn handle_key_event(
    key_code: KeyCode,
    key_modifiers: KeyModifiers,
    app_state: &AppState,
) -> bool {
    // Check current context
    let is_editing = matches!(app_state.field_editor_state, FieldEditorState::Editing { .. });
    let is_dashboard_active = is_dashboard_tab_active(&app_state);
    
    match key_code {
        KeyCode::Char('s') if !is_editing => {
            // Global save when not editing
            handle_global_save();
            true
        }
        KeyCode::Char('s') if is_editing => {
            // Different behavior when editing
            handle_field_save();
            true
        }
        _ => false,
    }
}
```

### 2. Dynamic Keybindings

Load keybindings dynamically based on user configuration:

```rust
pub struct KeyBindingManager {
    bindings: HashMap<String, Vec<KeyBinding>>,
}

impl KeyBindingManager {
    pub fn load_from_config(&mut self, config: &AppConfig) {
        // Load bindings from config
        for binding in &config.application.bindings {
            self.add_global_binding(binding);
        }
        
        for tab_content in &config.tab_content {
            self.add_tab_bindings(tab_content);
        }
    }
}
```

### 3. Keybinding Conflicts

Handle conflicts between different binding contexts:

```rust
pub fn resolve_key_conflict(
    key_code: KeyCode,
    modifiers: KeyModifiers,
    context: &BindingContext,
) -> Option<BindingAction> {
    // Priority order: Modal > Tab > Global
    
    // Check modal bindings first
    if let Some(action) = context.modal_bindings.get(&(key_code, modifiers)) {
        return Some(action.clone());
    }
    
    // Then tab-specific bindings
    if let Some(action) = context.tab_bindings.get(&(key_code, modifiers)) {
        return Some(action.clone());
    }
    
    // Finally global bindings
    context.global_bindings.get(&(key_code, modifiers)).cloned()
}
```

## Keybinding Best Practices

### 1. Use Standard Conventions

Follow common keybinding patterns:

```rust
// Good: Standard patterns
KeyCode::Char('q') => quit(),
KeyCode::Char('s') if ctrl => save(),
KeyCode::Char('z') if ctrl => undo(),
KeyCode::Char('y') if ctrl => redo(),

// Avoid: Unusual combinations
KeyCode::Char('x') if alt && shift => weird_action(),
```

### 2. Provide Visual Feedback

Update the status bar to show available keybindings:

```yaml
status_bar:
    default_text: "Status: Ready | [q] Quit | [Ctrl+S] Save | [F1] Help"
    modal_text: "Editing: [Enter] Save | [Esc] Cancel"
```

### 3. Handle Edge Cases

Consider terminal compatibility:

```rust
// Handle different terminal behaviors
match key_code {
    KeyCode::F(1) => {
        // Some terminals may not capture F1 properly
        show_help();
    }
    KeyCode::Char('?') => {
        // Alternative help key
        show_help();
    }
    _ => {}
}
```

## Testing Keybindings

### 1. Unit Tests

Test keybinding handlers:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dashboard_navigation() {
        let dashboard = Arc::new(Mutex::new(DashboardState::new()));
        let settings = SettingsManager::load();
        let process_manager = Arc::new(ProcessManager::new());
        
        // Test up arrow
        let handled = handle_dashboard_key_event(
            KeyCode::Up, &dashboard, &settings, process_manager.clone()
        );
        assert!(handled);
        
        // Verify state change
        let state = dashboard.lock().unwrap();
        assert_eq!(state.selected_command, 0); // Should not go below 0
    }
}
```

### 2. Integration Tests

Test keybinding in context:

```rust
#[test]
fn test_edit_mode_keybindings() {
    let mut app_state = AppState::new();
    app_state.field_editor_state = FieldEditorState::Editing { 
        field_index: 0, 
        input: Input::new("test") 
    };
    
    // Test Enter key in edit mode
    let result = handle_field_editor_key_event(
        KeyCode::Enter, 
        KeyModifiers::empty(),
        &app_state.field_editor_state,
        // ... other parameters
    );
    
    assert!(matches!(result, FieldEditorEventResult::StateChanged(_)));
}
```

## Troubleshooting

### 1. Key Not Working

Check if the key is being captured:

```rust
// Add debug logging
eprintln!("[DEBUG] Key event: {:?} with modifiers: {:?}", key_code, key_modifiers);

// Verify handler is being called
match key_code {
    KeyCode::Char('x') => {
        eprintln!("[DEBUG] X key detected, handling...");
        handle_x_key();
        true
    }
    _ => false,
}
```

### 2. Conflicting Bindings

Check binding priority and context:

```rust
// Log binding resolution
eprintln!("[DEBUG] Checking key: {:?} in context: {:?}", key_code, context);
if let Some(action) = resolve_key_conflict(key_code, modifiers, &context) {
    eprintln!("[DEBUG] Resolved to action: {:?}", action);
}
```

### 3. Terminal Issues

Some terminals may not capture certain keys:

```rust
// Test terminal capabilities
use crossterm::event::{self, Event};

fn test_terminal_support() {
    println!("Press keys to test terminal support (ESC to exit):");
    
    loop {
        if event::poll(Duration::from_millis(100)) {
            match event::read() {
                Ok(Event::Key(key)) => {
                    println!("Key: {:?} Modifiers: {:?}", key.code, key.modifiers);
                    if key.code == KeyCode::Esc {
                        break;
                    }
                }
                _ => {}
            }
        }
    }
}
```

## Summary

The keybinding system provides a flexible foundation for user interaction. By following these guidelines, you can effectively add, remove, and customize keybindings to create an intuitive user experience.

Key takeaways:
- Use the configuration file for declarative keybinding definitions
- Implement handler logic in the appropriate event handlers
- Consider context and priority when resolving conflicts
- Follow standard conventions for better usability
- Test thoroughly across different terminals and platforms
