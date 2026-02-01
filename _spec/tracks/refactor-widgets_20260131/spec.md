# Specification: Reusable Popup and FileChooser Widgets

## 1. Purpose
Refactor existing components from `shared-rust` into high-quality, encapsulated widgets for `dev-console2`. The goal is to provide a generic modal system with a buffer-mutating "ghosting" dimmer and a portable file-browsing component.

## 2. Functional Requirements

### 2.1 Ghosting Dimmer
- Must implement a utility to iterate over the `ratatui::Buffer`.
- Must transform foreground colors to mid-greys (e.g., ANSI 238).
- Must transform background colors to deep greys (e.g., ANSI 232).
- Must preserve the underlying text/structure while visually desaturating it.

### 2.2 Generic Popup Container
- Must act as a wrapper for any content implementing a standard interaction pattern.
- Must handle centering logic based on percentage of the parent area.
- Must provide "Chrome": Borders, Title, and a clear distinction from the dimmed background.
- Must be responsible for calling the "Ghosting Dimmer" before rendering its content.

### 2.3 FileChooser Widget (FileBrowser)
- Must be a portable widget that can be used inside a `Popup` or as a standalone view.
- Must support directory navigation (Enter/ArrowRight to enter, ArrowLeft to exit).
- Must support a `..` parent entry at the top of every list.
- Must maintain a `history` stack for `Backspace` navigation to prior directories.
- Must support page navigation via `PageUp`/`PageDown`.
- Must support file selection with icons (📁 for directories, 📄 for files).
- Must return a `WidgetOutcome<PathBuf>` to communicate selections to the parent.

### 2.4 Interactive Settings UI
- Must support keyboard navigation (Arrows/Tab) between fields and action icons.
- Must support mouse hit detection for field selection and icon triggering.
- Must provide inline editing for text and numeric values using `tui_input`.
- Action icons (folder) must trigger the `FileBrowser` modal.

## 3. Success Criteria
- [x] `Dimmer` successfully grayscales the background without clearing it.
- [x] `Popup` can wrap a `Paragraph` and a `FileChooser` with the same logic.
- [x] `FileChooser` correctly navigates the local filesystem and returns a `PathBuf`.
- [x] All widgets include unit tests for logic (FS navigation, outcome generation).
- [x] No regression in existing TUI performance.
- [x] Modal system correctly blocks both keyboard and mouse input leakage to the background.
- [x] Mouse support is fully implemented for the `FileBrowser` (scroll, click selection).