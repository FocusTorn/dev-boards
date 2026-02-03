# Implementation Plan: dc2-restructure_20260202

## Architecture Overview
The restructure follows a **Modular Layered Architecture** with a **Registry-based Component Model**.
- **View Layer**: Decentralized into "Dumb" Elements and "Smart" Components.
- **Management Layer**: `ComponentManager` handles state lifecycle and input delegation.
- **Domain Layer**: Decomposed executors for side-effects.

## Component Design

### 1. The `Component` Trait
- **Location**: `src/widgets/mod.rs`
- **Responsibilities**:
    - `on_tick(&mut self)`: Time-based state updates.
    - `handle_event(&mut self, event: Event) -> ComponentAction`: Input handling.
    - `render(&mut self, area: Rect, buf: &mut Buffer)`: Self-rendering logic.
    - `load_config(&mut self)`: Loading local YAML.

### 2. The Registry Macro
- **REUSE**: Leverage `InteractiveWidget` patterns but expand with macro generation.
- **Functionality**: Generates `enum Component { Toast(ToastManager), ... }` and `impl Component { ... }`.

### 3. ComponentManager
- **State**: `active_components: HashMap<Id, Component>`, `focus_stack: Vec<Id>`.
- **Interaction**: Handles the focus stack and routes all TUI events to the top component.

## Data Flow
1. **Input**: `main.rs` -> `App::handle_input` -> `ComponentManager::handle_input`.
2. **Logic**: `ComponentManager` delegates to the focused component.
3. **Outcome**: Component returns `ComponentAction`. If `Global`, it's mapped to `AppAction` and returned to the main loop.
4. **Rendering**: `App::view` -> `ComponentManager::render`.

## Phase Breakdown

### Phase 1: Foundation & Elements
1. Move existing stateless widgets to `src/widgets/elements/`.
2. Update all imports to `ratatui::prelude`.
3. Standardize elements to accept style/data structs rather than raw primitives.

### Phase 2: Smart Component Encapsulation
1. Refactor `Toast` into `src/widgets/components/toast/` (mod, state, view, config).
2. Create local `config.yaml` for `Toast`.
3. Implement the `Component` trait for `ToastManager`.
4. Repeat for `FileBrowser`, `CommandList`, and `SelectionList`.

### Phase 3: The Registry & Manager
1. Implement `register_components!` macro.
2. Implement `ComponentManager` in `src/app/components.rs`.
3. Integrate `ComponentManager` into `App` struct.

### Phase 4: App Decomposition
1. Create `src/app/actions.rs` and `src/app/state.rs`.
2. Move `update` logic to domain-specific modules.
3. Refactor `executors.rs` into smaller domain modules.

## Verification Tasks
- [ ] Run `cargo test` to ensure existing widget logic is preserved after the move.
- [ ] Verify `Toast` still fades out correctly using the new `on_tick` delegation.
- [ ] Manual test: Open `FileBrowser` and verify it captures all input until closed.
