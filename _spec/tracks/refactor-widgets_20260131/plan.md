# Implementation Plan: Reusable Widgets

## 1. Architectural Overview
We are moving from hardcoded, view-specific logic to an **Encapsulated Component** model. Each widget will manage its own internal state and return an `Outcome` enum to the parent.

## 2. Component Breakdown

### 2.1 The "Ghosting" Utility (`src/widgets/dimmer.rs`)
- **Logic**: Pure function `apply_dimming(buf: &mut Buffer, area: Rect)`.
- **Styling**: Uses `Color::Indexed` (232-255 range) to ensure compatibility with most terminals while providing a high-fidelity "disabled" look.

### 2.2 The Outcome Pattern (`src/widgets/mod.rs`)
- Define `WidgetOutcome<T>` to standardize how `App` receives data from widgets.
- This prevents widgets from needing to know about `App::Message`.

### 2.3 The Popup Stage (`src/widgets/popup.rs`)
- **Structure**: `Popup<T>` wrapper.
- **Centering**: Manual `Rect` calculation to avoid dependencies on external layout helpers where possible.
- **Rendering Sequence**:
    1. Render parent UI.
    2. Apply `dimmer`.
    3. `Clear` popup area.
    4. Render `T`.

### 2.4 The FileBrowser (`src/widgets/file_browser.rs`)
- **State**: `current_dir: PathBuf`, `entries: Vec<FileEntry>`, `selected: usize`.
- **Navigation**:
    - `handle_key(Enter)` on Dir -> `self.load_dir(new_path)`.
    - `handle_key(Enter)` on File -> `WidgetOutcome::Confirmed(path)`.
    - `handle_key(Esc)` -> `WidgetOutcome::Canceled`.

## 3. Integration Strategy
- Add `mod dimmer`, `mod popup`, and `mod file_browser` to `widgets/mod.rs`.
- Update `App` to hold an `Option<Popup<FileBrowser>>`.
- Route keys to the modal if it exists.
