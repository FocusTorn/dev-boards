# Widget Registry: dev-console2

## Active Widgets

### `TabBarWidget`
- **Purpose**: Handles both top-level navigation and static toggle buttons.
- **Usage**: Configured via `build-config.yaml` under `tab_bars`. Supports horizontal/vertical alignment.

### `SelectionListWidget` (Refactored from `CommandListWidget`)
- **Purpose**: A general-purpose list selection tool for both Dashboard commands and Settings categories.
- **Usage**: 
  ```rust
  SelectionListWidget::new(&items, selected_index, hovered_index)
      .normal_style(Style::default().fg(Color::DarkGray))
      .highlight_style(self.theme.style("commands_highlight"))
  ```
- **Note**: Does not render its own borders; should be wrapped in a `Block` by the caller.
- **Interactions**:
    - **Keyboard**: Up/Down cycles the `selected_index`.
    - **Mouse**: `Moved` events update `hovered_index`, `Down(Left)` updates `selected_index`.
    - **Dispatcher Mode**: 
        - `DispatchMode::OnHighlight`: Triggers an action on *every* Up/Down (used for Settings category switching).
        - `DispatchMode::OnSelect`: Triggers an action only on Enter/Click (used for Dashboard command execution).

### `OutputBoxWidget`
- **Purpose**: Renders the scrollable terminal output.
- **Usage**: Supports ANSI parsing and integrated scrollbar logic.

### `ProgressBarWidget`
- **Purpose**: Displays progress for long-running tasks (Compile/Upload).
- **Usage**: Includes smoothing logic and ETA prediction.

### `ToastWidget`
- **Purpose**: Temporary informational or error notifications.
- **Usage**: Managed via `ToastManager`.

## Inactive / Future Widgets
- **`ModalDialogWidget`**: Planned for confirmation prompts (Overwrite profile, Delete).
- **`PortPickerWidget`**: A specialized "Quick Pick" implementation for serial port selection.
