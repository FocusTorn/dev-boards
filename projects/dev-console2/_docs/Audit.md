# Dev-Console-v2 Architectural Audit & Optimization Log

## Section 1: Foundational Mandates

- **State Integrity**
  - **The Rule**: Represent complex states (like background tasks or multi-step processes) using Enums with data payloads rather than multiple independent primitive variables.
  - **Why**: This prevents "illegal states" (e.g., progress existing when no task is running) and simplifies match-based rendering.
  - **Prompt Addition**: Represent complex or multi-variable states using Enums with associated data payloads. This ensures state integrity and allows the UI to react predictably to transitions.

- **Constant Identity & Indexing**
  - **The Rule**: Hardcoded string IDs used for lookups or dispatching must be defined as constants or indexed into HashMaps during initialization.
  - **Why**: Repeated O(n) lookups in high-frequency loops (view/event) are wasteful.
  - **Prompt Addition**: Optimize high-frequency data lookups by indexing configuration or state into HashMaps during initialization. Avoid repeated O(n) searches in the render or event loop.

- **Allocation Minimization**
  - **The Rule**: Minimize heap allocations (like .clone() or .to_string()) within the main event loop and render pass.
  - **Why**: High-frequency input loops should avoid unnecessary pressure on the allocator.
  - **Prompt Addition**: Minimize heap allocations in the main loop. Use Enums, static references, or Cow types for internal command routing and dispatching to ensure high-performance input handling.

- **Widget Interactivity Delegation**
  - **The Rule**: Any widget that is interactive (Tabs, Buttons, Scrollbars) MUST encapsulate its own hit-box and collision logic in a `handle_event` method.
  - **Why**: This prevents the "Controller" (App) from having to "reach inside" the widget to know how wide a tab is or where a button sits.
  - **Prompt Addition**: Interactive UI components must implement a `handle_mouse_event` method. The main application loop should never calculate the internal geometry of a widget for click detection; it must delegate the event to the widget and respond to the returned semantic result.

- **Strict View/Render Immutability**
  - **The Rule**: The view function and any render implementation must be treated as a pure projection of state and remain strictly immutable (`&self`).
  - **Why**: Mutation inside a render pass creates "ghost" state changes that are hard to debug and lead to frame-sync issues.
  - **Prompt Addition**: The UI render pass must be strictly immutable. State-mutating logic must be moved to the `update` loop or reactive 'sync' methods. Maintenance logic must be encapsulated within the widget's own `render` trait implementation.

- **Reactive State Synchronization**
  - **The Rule**: Derived state (like offsets or progress filler) must be updated only when the source state changes, never via every-frame polling.
  - **Why**: Calculating derived properties 60 times a second is wasteful and hides logic from the dispatcher.
  - **Prompt Addition**: Avoid 'frame-based' state calculation. If a property depends on other state, create an explicit `sync_[property]` method and trigger it only upon data ingestion, user toggle, or resize events.

- **Modular Command Execution**
  - **The Rule**: Decouple the "When" (Dispatcher) from the "How" (Implementation) by extracting all semantic actions into a dedicated `executors` module.
  - **Why**: Huge match blocks in `mod.rs` become unreadable and hard to test.
  - **Prompt Addition**: All semantic actions must be extracted into a dedicated `executors` module. The main dispatcher should only act as a router that calls `self.exec_[action]()`.

- **System Event Translation**
  - **The Rule**: Logic that polls external sources (Channels, Sockets) must only act as a "Translator" between wire protocols and internal messages.
  - **Why**: Direct mutation of App state from background threads makes tracking state transitions nearly impossible.
  - **Prompt Addition**: Background task polling must be decoupled from state mutation. The poller should act as a translator that maps external events into internal application Messages.

- **Mandatory View Extraction**
  - **The Rule**: Rendering logic (all `view` and `render` functions) must be isolated in a dedicated `view.rs` module.
  - **Why**: Prevents `mod.rs` from becoming a "God Object" and clearly separates the logic layer from the projection layer.
  - **Prompt Addition**: Rendering logic must be isolated to its own projection layer. The App struct in `mod.rs` should remain focused on state definitions and lifecycle management.

- **Layout Caching**
  - **The Rule**: Application layout (`AppLayout`) must be cached on the `App` struct and recalculated only on `Resize` or structural change.
  - **Why**: Recalculating expensive geometry on every frame is inefficient and unnecessary.
  - **Prompt Addition**: Recalculate layout ONLY when the `view_area` (terminal size) changes or a state change occurs that alters the UI structure.

- **Layout Encapsulation**
  - **The Rule**: Widgets must own the knowledge of their spatial requirements through helper methods like `consumed_height()`.
  - **Why**: Hardcoding magic offsets in the layout engine makes the UI fragile and hard to refactor.
  - **Prompt Addition**: Instead of hardcoding height adjustments in the layout engine, widgets should provide helper methods that the layout engine calls to determine spatial offsets.

- **Direct Execution**
  - **The Rule**: The dispatcher should call executors directly, eliminating internal "Message Chaining."
  - **Why**: Mapping a Key to a Message just to map that Message to a Function creates "Message Ping-Pong" and increases complexity.
  - **Prompt Addition**: Eliminate 'Command Messages' for internal state changes. Messages should be reserved strictly for external events (Key/Mouse/System) that need to enter the system.

- **Granular Dirty Flags**
  - **The Rule**: Distinguish between 'visual updates' (re-rendering) and 'structural changes' (re-calculating layout geometry).
  - **Why**: Updating text content is cheap; recalculating the entire layout tree is expensive.
  - **Prompt Addition**: Implement granular dirty flags. Distinguish between 'visual redraws' and 'structural layout recalculations.' Only trigger expensive geometry math when necessary.

---

## Section 2: Session Optimizations (Jan 24, 2026)

- **MSVC Toolchain Migration**
  - **The Change**: Migrated the entire build environment from `x86_64-pc-windows-gnu` to the native `x86_64-pc-windows-msvc` toolchain.
  - **Why**: MSVC is the standard for Windows, providing superior stability and native support for C-libraries like `ring` without external `gcc` dependencies.
  - **Status**: Completed; verified with successful native compilation of `ring`, `serialport`, and `rumqttc`.

- **Automated Toolchain Setup**
  - **The Change**: Integrated `winget` automation to install Visual Studio 2022 Build Tools and the C++ Workload directly from the CLI.
  - **Why**: Simplifies the developer onboarding process and ensures the environment is correctly configured without manual web downloads.
  - **Status**: Implemented and used to successfully restore the missing build environment.

- **Custom High-Precision ScrollBar**
  - **The Change**: Implemented a local `ScrollBar` widget using 1/8-block sub-cell glyphs and thumb-relative dragging.
  - **Why**: Resolved versioning conflicts with `tui-scrollbar` and achieved ultra-smooth visual feedback that follows the mouse without jumping.
  - **Status**: Implemented Mandate #1; fully functional with mouse wheel, dragging, and keyboard support.

- **Liquid Progress & ETA Unification**
  - **The Change**: Unified `CompileState` and `ProgressPredictor` to use identical learned stage weights and average durations from `progress_history.json`.
  - **Why**: Eliminated progress "jumps" (e.g. 30% to 90%) by ensuring each stage ends exactly where the next begins based on historical averages.
  - **Status**: Implemented Mandate #3; animations now use frame-independent exponential decay for fluidity.

- **Automated Action Mapping**
  - **The Change**: Replaced manual `match` blocks for Action strings with a compile-time `strum` mapping system.
  - **Why**: Enables "auto-mapping" of YAML strings to Rust Enums, allowing flexible aliases (like `execute` vs `commands_execute`) without manual maintenance.
  - **Status**: Implemented Mandate #9; cleaner code and more flexible configuration.

- **Two-Tier Interactive Command List**
  - **The Change**: Implemented a sophisticated hover/selection system where hovered items receive vibrant highlights while the active selection turns dim gray.
  - **Why**: Provides clear visual feedback for both mouse and keyboard interactions, adhering to modern TUI UX standards.
  - **Status**: Implemented Mandate #1 & #10; background highlights now span border-to-border for a "pro" feel.

- **Integrated Monitor Communication**
  - **The Change**: Implemented full Serial and MQTT monitors with background processing and interactive command fields.
  - **Why**: Extends the console from a build tool to a complete device management suite, allowing real-time interaction with ESP32/Arduino boards.
  - **Status**: Implemented Mandate #5; handles background reading/writing and automatic configuration from profiles.

- **Resilient Configuration Loading**
  - **The Change**: Refactored `ApplicationConfig` to use `#[serde(default)]` and fallback helpers for all non-critical metadata.
  - **Why**: Prevents application crashes and parsing errors when optional fields are missing from `build-config.yaml`.
  - **Status**: Completed; ensures the app is robust against user configuration errors.
