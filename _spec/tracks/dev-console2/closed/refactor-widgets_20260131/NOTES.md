# Implementation Notes - Refactor Widgets

- Using `Color::Indexed` for dimming to ensure high-fidelity grayscale on 256-color terminals.
- Encapsulation is prioritized: Widgets return outcomes, App handles messages.
- Popup is a generic stage; FileBrowser is the first major content actor.

## Lessons Learned

### 1. TDD Gaps: The Need for "Negative" Tests
Initial TDD focused heavily on functional "happy paths" (does the button work?). This led to regressions where background input was still leaking through modals. 
**Lesson**: Critical architectural constraints (like input blocking) must have explicit "Negative Tests" that assert actions do *not* happen under certain conditions.

### 2. Test Architecture: Modularization is Key
As the TUI complexity grew, the `tests.rs` file became bloated and hard to navigate. Refactoring tests into sub-modules (`mod profiles`, `mod dashboard`) significantly improved iteration speed by allowing targeted runs (e.g., `cargo test profiles`).

### 3. Stateful Render Handles
The `FileBrowser` initially struggled with `PageUp`/`PageDown` because it didn't know its own rendered height (which is determined by the `Rect` passed during the `view()` call). 
**Lesson**: For complex widgets, implement a `render_stateful(&mut self, ...)` method that captures and stores viewport dimensions during the draw pass to enable geometry-aware logic in the update pass.

### 4. Borrow Checker & Closures
Navigating directory structures involves mutating the `FileBrowser` state (clearing entries, updating `current_dir`). Doing this inside `if let Some(entry) = self.entries.get(...)` blocks causes borrow conflicts. 
**Lesson**: Always clone small, critical pieces of data (like directory names or paths) before initiating state mutation to satisfy the borrow checker.

### 5. Hit Detection & Padding
Adding UI padding (upper/left) must be mirrored exactly in `dispatch_mouse` hit detection logic. Even a 1-pixel offset can break the "feel" of a TUI.
**Lesson**: Use a shared `Layout` calculation or a dedicated helper to determine "Inner Areas" for both rendering and hit-testing to keep them synchronized.