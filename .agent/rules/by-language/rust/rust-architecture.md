---
globs: **/*.rs

---

# Rust Architecture Rules

## State Management

### **1. :: Result Enums for State Transition Communication**

**✅ CORRECT - Use result enums to communicate state changes**:

When extracting functions that need to modify application state, use result enums to communicate state changes instead of requiring mutable borrows:

```rust
// Define result enum for state transitions
pub enum FieldEditorEventResult {
    Continue,
    Exit,
    Toast(Toast),
    StateChanged(FieldEditorState),
}

// Extracted handler returns result enum
pub fn handle_field_editor_key_event(
    key_code: KeyCode,
    current_state: &FieldEditorState,  // Immutable reference
    settings: &mut Settings,
) -> FieldEditorEventResult {
    // Handler logic that determines state change
    FieldEditorEventResult::StateChanged(new_state)
}

// Caller updates state based on result
match handle_field_editor_key_event(key_code, &app_state.field_editor_state, &mut app_state.settings) {
    FieldEditorEventResult::StateChanged(new_state) => {
        app_state.field_editor_state = new_state;  // Caller owns the state
    }
    FieldEditorEventResult::Toast(toast) => {
        toasts.push(toast);
    }
    _ => {}
}
```

**✅ CORRECT - Decoupling state access from state modification**:

Result enums allow handlers to take immutable references to current state while still enabling state changes:

```rust
// Handler takes immutable reference
pub fn handle_event(current_state: &State) -> EventResult {
    // Can read current_state immutably
    // Returns result indicating what change should occur
}
```

**❌ INCORRECT - Requiring mutable borrows in extracted functions**:

```rust
// Wrong: Trying to pass multiple mutable borrows simultaneously
pub fn handle_event(
    state: &mut FieldEditorState,  // Mutable borrow
    settings: &mut Settings,       // Another mutable borrow
) {
    // May cause E0502: cannot borrow as mutable because it is also borrowed as immutable
}
```

**❌ INCORRECT - Not using result enums for state transitions**:

```rust
// Wrong: Extracted function directly modifies state, causing borrowing conflicts
pub fn handle_event(state: &mut State) {
    state.field = new_value;  // Requires mutable borrow, conflicts with caller's borrows
}
```

### **2. :: State Centralization with AppState Pattern**

**✅ CORRECT - Combine owned and shared state in AppState struct**:

Create a central `AppState` struct that holds both owned values and `Arc<Mutex<T>>` references:

```rust
pub struct AppState {
    // Owned state fields (for single-threaded access)
    pub settings: Settings,
    pub field_editor_state: FieldEditorState,
    pub dashboard_state: DashboardState,
    
    // Arc<Mutex<T>> fields (for shared mutable state)
    pub dashboard_arc: Arc<Mutex<DashboardState>>,
    pub process_manager_arc: Arc<ProcessManager>,
    
    // Utility fields
    pub layout_cache: LayoutCache,
}

impl AppState {
    pub fn new() -> Self {
        // Initialization logic
    }
    
    // Sync owned state from Arc<Mutex> when needed
    pub fn sync_dashboard_state(&mut self) {
        if let Ok(locked_state) = self.dashboard_arc.lock() {
            self.dashboard_state = locked_state.clone();
        }
    }
}
```

**✅ CORRECT - Simplified function signatures**:

AppState simplifies function signatures:

```rust
// Before: Multiple state parameters
fn process_event(
    settings: &mut Settings,
    field_editor_state: &mut FieldEditorState,
    dashboard_arc: Arc<Mutex<DashboardState>>,
    layout_cache: &mut LayoutCache,
) { }

// After: Single AppState parameter
fn process_event(app_state: &mut AppState) {
    // Access all state through app_state
}
```

**❌ INCORRECT - Mixing individual state variables**:

```rust
// Wrong: Maintaining separate state variables instead of centralizing
let mut settings = Settings::load();
let mut field_editor_state = FieldEditorState::new_selected(0);
// Multiple variables passed around individually
```

## Module Architecture

### **3. :: Separation of Concerns (TUI Architecture)**

**✅ CORRECT - Separate event handling, UI coordination, and state management**:

When refactoring large main.rs files, separate concerns into focused modules:

```rust
// event_handler.rs - All event handling logic
// Handles keyboard, mouse, state transitions
pub fn handle_key_event(...) -> EventResult { ... }

// ui_coordinator.rs - All UI rendering coordination
// Handles layout calculation, widget rendering, cursor positioning
pub fn render_ui<B: Backend>(...) { ... }

// app_state.rs - Centralized application state
// Holds all data structs and state enums
pub struct AppState { ... }

// main.rs - Orchestration only
// - Initializes TUI
// - Creates AppState
// - Calls event_handler functions
// - Calls ui_coordinator functions
```

**✅ CORRECT - Reduced main.rs complexity**:

After module separation, main.rs becomes primarily an orchestrator that connects the components without implementing business logic.

## Common Mistakes

### ❌ State Management Violations
- ❌ **Requiring Mutable Borrows in Extracted Functions** - Don't pass multiple mutable borrows to extracted functions
- ❌ **Not Using Result Enums** - Don't modify state directly in extracted functions without result enums
- ❌ **Mixing Individual State Variables** - Don't maintain separate state variables instead of centralizing in AppState
- ❌ **Not Providing Sync Methods** - Don't create AppState without methods to sync owned state from Arc<Mutex>
- ❌ **Mixing Concerns in Modules** - Don't put event handling in UI coordinator or rendering in event handler
- ❌ **Keeping Logic in main.rs** - Don't keep event handling or rendering logic in main.rs after module extraction

## Checklist

- [ ] **Result Enums**: State transitions are communicated via result enums instead of mutable borrows
- [ ] **State Centralization**: Application state is centralized in AppState struct with both owned and Arc<Mutex> fields
- [ ] **State Sync Methods**: AppState provides methods to sync owned state from Arc<Mutex> references
