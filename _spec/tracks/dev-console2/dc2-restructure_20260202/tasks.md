# Tasks: dc2-restructure_20260202

## Phase 1: Widget Tiering (Elements)
- [ ] **T1.1**: Move `progress_bar.rs`, `status_box.rs`, `dimmer.rs`, `smooth_scrollbar.rs`, `popup.rs`, `tab_bar.rs`, `output_box.rs` to `src/widgets/elements/`.
- [ ] **T1.2**: Update `src/widgets/mod.rs` to export the `elements` module.
- [ ] **T1.3**: Fix all imports in `app/view.rs` and `app/mod.rs` to point to new element paths.
- [ ] **T1.4**: Verify `cargo build` passes.

## Phase 2: Smart Component: Toast
- [ ] **T2.1**: Relocate `toast.rs` logic into `src/widgets/components/toast/`.
- [ ] **T2.2**: Split into `state.rs` (Manager logic) and `view.rs` (Widget impl).
- [ ] **T2.3**: Create `src/widgets/components/toast/config.yaml` with default values.
- [ ] **T2.4**: Implement the `Component` trait for `ToastManager`.
- [ ] **T2.5**: Write a test ensuring `ToastManager::load_config` correctly parses the local YAML.

## Phase 3: Smart Component: FileBrowser
- [ ] **T3.1**: Relocate `file_browser.rs` to `src/widgets/components/file_browser/`.
- [ ] **T3.2**: Split into `state.rs`, `view.rs`, and `mod.rs`.
- [ ] **T3.3**: Implement `Component` trait for `FileBrowser`.
- [ ] **T3.4**: Verify unit tests for file selection still pass.

## Phase 4: Registry & Focus
- [ ] **T4.1**: Implement `register_components!` macro in `src/widgets/mod.rs`.
- [ ] **T4.2**: Implement `ComponentManager` with a `VecDeque` for focus.
- [ ] **T4.3**: Refactor `App::update` to delegate key events to `ComponentManager`.
- [ ] **T4.4**: Verify that opening a modal (FileBrowser) correctly traps input.

## Phase 5: App Clean-up
- [ ] **T5.1**: Extract `AppAction` to `src/app/actions.rs`.
- [ ] **T5.2**: Extract state initialization to `src/app/state.rs`.
- [ ] **T5.3**: Break `executors.rs` into `arduino.rs`, `serial.rs`, and `system.rs`.
- [ ] **T5.4**: Final verification: All tests pass and no file > 400 lines.
