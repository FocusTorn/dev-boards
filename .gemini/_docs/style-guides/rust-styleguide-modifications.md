# Rust-Specific Style Guide Overrides

This document defines the strict mandates for Rust code within this workspace, supplementing the base `rust.md` guide and the universal `general-styleguide-modifications.md`.

## 1. Dead Code Attributes
- **The Rule**: Do not add comments to the `#[allow(dead_code)]` attribute.
- **Action**: If you see `// Available for future use` or similar comments next to this attribute, remove them.
- **Correct Format**:
  ```rust
  #[allow(dead_code)]
  pub fn internal_util() { ... }
  ```

---

## 2. Universal Docstrings (Universal Documentation Mandate)
- **The Rule**: Every public and internal function, struct, and enum must have a "Why-focused" docstring using `///`.
- **Folding Requirement**:
    - Add `///>` immediately after the first concise title line of the docstring.
    - Add `///<` between the last line of the docstring and the start of the code block.
- **Example Usage**:
  ```rust
  /// Installs a custom panic hook to prevent terminal corruption.
  ///>
  /// If the application crashes, this hook ensures the terminal is restored to 
  /// its original state before the error message is printed, preventing the 
  /// "frozen terminal" state common in failed TUI apps.
  ///<
  #[allow(dead_code)]
  pub fn install_panic_hook() { ... }
  ```

---

## 3. TUI & Performance Optimizations

### State Integrity
- **The Rule**: Represent complex states (like background tasks or multi-step processes) using Enums with data payloads rather than multiple independent primitive variables.
- **Why**: This prevents "illegal states" and simplifies match-based rendering.

### Constant Identity & Indexing
- **The Rule**: Hardcoded string IDs used for lookups or dispatching must be defined as constants or indexed into HashMaps during initialization.
- **Why**: Avoids wasteful O(n) searches in high-frequency render/event loops.

### Allocation Minimization
- **The Rule**: Minimize heap allocations (like `.clone()` or `.to_string()`) within the main event loop and render pass.
- **Why**: Reduces pressure on the allocator during high-frequency input handling.

### Widget Interactivity Delegation
- **The Rule**: Interactive widgets (Tabs, Buttons, Scrollbars) MUST encapsulate its own hit-box logic in a `handle_event` method.
- **Why**: Prevents the App controller from needing to calculate internal widget geometry.

### Strict View/Render Immutability
- **The Rule**: The view function and render implementation must be treated as a pure projection of state and remain strictly immutable (`&self`).
- **Why**: Mutation inside render passes creates hard-to-debug "ghost" state changes and frame-sync issues.

### Reactive State Synchronization
- **The Rule**: Derived state (like offsets or progress filler) must be updated only when the source state changes, never via every-frame polling.
- **Why**: Calculating derived properties 60 times a second is wasteful.

### Modular Command Execution
- **The Rule**: Decouple the "When" (Dispatcher) from the "How" (Implementation) by extracting semantic actions into a dedicated `executors` module.

### System Event Translation
- **The Rule**: Polling logic for external sources (Channels, Sockets) must act as a "Translator" that maps external events into internal application Messages.

### Mandatory View Extraction
- **The Rule**: Rendering logic (all `view` and `render` functions) must be isolated in a dedicated `view.rs` module.

### Layout Caching
- **The Rule**: Recalculate layout ONLY when the terminal size (`view_area`) changes or a structural UI change occurs.

### Layout Encapsulation
- **The Rule**: Widgets must own the knowledge of their spatial requirements through helper methods like `consumed_height()`.

### Direct Execution
- **The Rule**: The dispatcher should call executors directly, eliminating internal "Message Chaining."

### Granular Dirty Flags
- **The Rule**: Distinguish between 'visual updates' (re-rendering) and 'structural changes' (re-calculating layout). Only trigger expensive geometry math when necessary.
