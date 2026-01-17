---
globs: **/*.rs

---

# TUI Event Handling Rules

## Architecture

### **1. :: Event Loop Restructuring for Borrowing Management**

**✅ CORRECT - Use conditional guards and nested matches**:

When extracting event handling from main loops, restructure event matching to use conditional guards for specific key/state combinations first, then use nested matches for other cases:

```rust
// Outer match with conditional guards for specific combinations
match key.code {
    // Handle specific key/state combinations first (with immutable refs)
    KeyCode::Enter | KeyCode::Esc if matches!(app_state.field_editor_state, FieldEditorState::Editing { .. }) => {
        match handle_field_editor_key_event(
            key.code,
            key.modifiers,
            &app_state.field_editor_state,  // Immutable reference
            &mut app_state.settings,
            &app_state.settings_fields,
        ) {
            FieldEditorEventResult::StateChanged(new_state) => {
                app_state.field_editor_state = new_state;  // Update state
            }
            FieldEditorEventResult::Toast(toast) => {
                toasts.push(toast);
            }
            _ => {}
        }
    }
    
    // Other key codes use nested match with mutable borrow
    _ => {
        match &mut app_state.field_editor_state {
            FieldEditorState::Editing { ref mut input, .. } => {
                handle_editing_input(key.code, key.modifiers, input);  // Mutable borrow
            }
            FieldEditorState::Selected { .. } => {
                // Handle with immutable reference
                match handle_field_editor_key_event(...) { ... }
            }
            FieldEditorState::Selecting { ref mut selected_index, ref options, .. } => {
                handle_dropdown_navigation(key.code, selected_index, options);  // Mutable borrow
            }
        }
    }
}
```

**✅ CORRECT - Separate borrowing contexts**:

This pattern creates separate borrowing contexts:
- Conditional guards handle specific combinations by calling extracted functions with immutable references
- Nested matches allow mutable borrows for direct state modification in other cases

**✅ CORRECT - Localized mutable borrows**:

Nested matches localize mutable borrows to specific code paths:

```rust
// Mutable borrow is localized to this match arm
match &mut app_state.field_editor_state {
    FieldEditorState::Editing { ref mut input, .. } => {
        // input is mutably borrowed here only
        handle_editing_input(key.code, key.modifiers, input);
        // Borrow ends here
    }
    // Other arms use different borrowing patterns
}
```

**❌ INCORRECT - Uniform handling causing borrowing conflicts**:

```rust
// Wrong: Attempting to handle all key codes uniformly
match handle_field_editor_key_event(
    key.code,
    &app_state.field_editor_state,  // Immutable borrow
    &mut app_state.settings,         // Mutable borrow
) {
    FieldEditorEventResult::StateChanged(new_state) => {
        app_state.field_editor_state = new_state;  // E0502: cannot borrow as mutable
    }
}
```

**❌ INCORRECT - Not using conditional guards**:

```rust
// Wrong: Not handling specific combinations first
match &mut app_state.field_editor_state {
    FieldEditorState::Editing { .. } => {
        // All key codes handled here, causing conflicts
        handle_field_editor_key_event(...);  // Borrowing conflicts
    }
}
```


## Common Mistakes
- ❌ **Uniform Handling Causing Conflicts** - Don't attempt to handle all key codes uniformly when borrowing constraints differ
- ❌ **Not Using Conditional Guards** - Don't skip conditional guards for specific key/state combinations
- ❌ **Not Separating Borrowing Contexts** - Don't mix immutable and mutable borrows in the same match context

## Checklist

- [ ] **Event Loop Structure**: Event loops use conditional guards for specific key/state combinations
- [ ] **Borrowing Context Separation**: Immutable and mutable borrows are separated into different match contexts
- [ ] **Localized Borrows**: Mutable borrows are localized to specific match arms
