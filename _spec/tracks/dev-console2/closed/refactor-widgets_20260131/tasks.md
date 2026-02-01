# Tasks: Widget Refactor

## Phase 1: Foundations (Red-Green-Refactor)

- [x] **Task 1.1: Define WidgetOutcome**
    - [x] Create `WidgetOutcome` enum in `src/widgets/mod.rs`.
    - [x] Add unit tests for outcome conversion/matching.
- [x] **Task 1.2: Implement Ghosting Dimmer**
    - [x] Create `src/widgets/dimmer.rs`.
    - [x] **Test**: Verify that applying dimming to a buffer with `Color::Red` changes the `fg` to the specified grey index.
    - [x] Implement `apply_dimming` logic.
- [x] **Task 1.3: Create Generic Popup Container**
    - [x] Create `src/widgets/popup.rs`.
    - [x] Implement centering logic and "Chrome" rendering.
    - [x] **Test**: Verify `Popup` correctly calculates its `Rect` based on parent percentage.

## Phase 2: The FileBrowser (Strict TDD)

- [x] **Task 2.1: FileBrowser State & Sorting**
    - [x] **RED**: Write test mocking directory entries and asserting correct sorting (dirs first, then alphabetic).
    - [x] **GREEN**: Implement `FileEntry` struct and `load_directory` sorting logic.
    - [x] **REFACTOR**: Clean up sorting implementation.
- [x] **Task 2.2: FileBrowser Interaction**
    - [x] **RED**: Write tests for `handle_key`:
        - [x] Navigation (Down/Up) updates index.
        - [x] `Enter` on a directory navigates in.
        - [x] `Enter` on a file returns `WidgetOutcome::Confirmed`.
        - [x] `Esc` returns `WidgetOutcome::Canceled`.
    - [x] **GREEN**: Implement `handle_key` logic to pass tests.
    - [x] **REFACTOR**: Dry up navigation logic.
- [x] **Task 2.3: FileBrowser Rendering**
    - [x] **RED**: Write test verifying `FileBrowser` renders expected icons and names into a buffer.
    - [x] **GREEN**: Implement `Widget` trait for `FileBrowser`.
    - [x] **REFACTOR**: Align styling with project theme.

## Phase 3: Integration (Strict TDD)

- [x] **Task 3.1: App State Integration**
    - [x] **RED**: Write test in `app/tests.rs` verifying that if `app.modal` is `Some`, key events are routed to it instead of the main view.
    - [x] **GREEN**: Update `App::update` to delegate to modal.
- [x] **Task 3.2: View Rendering Integration**
    - [x] **RED**: Write test verifying that `App::view` calls `apply_dimming` and renders the popup when `app.modal` is active.
    - [x] **GREEN**: Update `App::view` to render modal stack.
- [x] **Task 3.3: Polish & Verification**
    - [x] **RED**: Write integration test: Open modal -> Select file -> Verify App state updated.
    - [x] **GREEN**: Final wiring and verification.
    - [x] **REFACTOR**: Final code cleanup.

## Phase 4: Settings UI Enhancements (TDD)

- [x] **Task 4.1: Setting Field Navigation**
    - [x] **RED**: Write test verifying that `App` can track a `selected_field_index` and navigate between fields in the Settings tab.
    - [x] **GREEN**: Add `selected_field_index` to `App` and implement navigation logic.
- [x] **Task 4.2: Refined Rendering & Styling**
    - [x] **RED**: Update test to verify `📁` has no border and matches dashboard input style.
    - [x] **GREEN**: Remove icon border and align colors with dashboard input.
- [x] **Task 4.3: Unified Navigation (Tab & Arrows)**
    - [x] **RED**: Write test for `Tab` cycling through fields *and* the folder icon.
    - [x] **GREEN**: Implement `Tab`/`Shift+Tab` and refine Arrow logic to include icon focus.
- [x] **Task 4.4: Mouse Selection & Hit Detection**
    - [x] **RED**: Write test verifying that clicking a field's area updates `selected_field_index`.
    - [x] **GREEN**: Implement hit detection in `dispatch_mouse` for settings fields and the folder icon.
- [x] **Task 4.5: Inline Editing Logic**
    - [x] **RED**: Write test verifying that pressing Enter on a field activates `input_active` and populates `self.input`.
    - [x] **GREEN**: Implement "Edit Mode" for setting fields.
- [x] **Task 4.6: Action Triggering**
    - [x] **RED**: Write test verifying that clicking the 📁 icon or pressing Enter/Space on it opens the `FileBrowser`.
    - [x] **GREEN**: Final wiring of triggers.

## Phase 5: FileBrowser Advanced Navigation (TDD)

- [x] **Task 5.1: Hierarchical Arrow Navigation**
    - [x] **RED**: Write test verifying `ArrowRight` on a directory navigates in, and `ArrowLeft` navigates up.
    - [x] **GREEN**: Implement arrow-based hierarchy navigation.
- [x] **Task 5.2: History-based Back Navigation**
    - [x] **RED**: Write test verifying that entering a directory pushes to history, and `Backspace` pops from history to return to the *prior* directory.
    - [x] **GREEN**: Implement `history` stack and `Backspace` logic.
- [x] **Task 5.3: Page Navigation**
    - [x] **RED**: Write test for `PageUp`/`PageDown` moving the selection by a page (or fixed increment if height is dynamic).
    - [x] **GREEN**: Implement `PageUp`/`PageDown` logic using `last_visible_height`.
- [x] **Task 5.4: Verification & Polish**
    - [x] **RED**: Verify all navigation keys work in concert without conflicts.
    - [x] **GREEN**: Final build and test.
