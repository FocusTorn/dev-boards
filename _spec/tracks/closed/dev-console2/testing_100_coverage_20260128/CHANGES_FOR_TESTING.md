# Codebase & Test Alignment Report

This document outlines changes made to the `dev-console-v2` codebase to accommodate testing requirements, and adjustments made to test expectations to match intended application behavior.

## 1. Architectural Changes for Mocking
- **Traits Extraction**: Created `src/commands/traits.rs` to define `CommandRunner`, `ChildProcess`, `FileSystem`, and `SerialProvider`.
- **Dependency Injection**: Modified `run_compile`, `run_upload`, and `SerialMonitor` to accept trait objects instead of concrete standard library types. This allows `mockall` to intercept system-level calls during unit tests.

## 2. Codebase Adjustments (To Match Test/Logic Expectations)
- **Clipping Logic in `TabBarWidget`**: The `get_aligned_area` function was updated to strictly clip returned `Rect`s within the parent area. Previously, positive offsets could push the widget out of the terminal buffer bounds, causing panics during rendering.
- **Action Case Sensitivity**: Standardized the `Action` enum deserialization. Tests revealed that `Action::from_str` was sensitive to case (e.g., `clean` vs `Clean`). The codebase was adjusted to use consistent CamelCase for semantic actions matching the YAML configuration.
- **YAML Enum Variants**: Fixed `TabBarStyle` and `TabBarAlignment` to use standard `CamelCase` strings in YAML to match `serde` defaults, as `strum`'s lowercase serialization was causing mismatches in manual test configurations.

## 3. Test Adjustments (To Match Codebase Reality)
- **Toast Duration Expansion**: Unit tests initially used very short durations (0.1s) for toasts. Due to the high speed of the test runner, the `Instant::now()` markers were occasionally expiring before the `render` pass could capture them. Test durations were increased to 10s to ensure visual stability during verification.
- **Buffer Verification Helper**: Since `ratatui::Buffer` cannot be directly stringified, added `buffer_content` helpers to extract raw symbols. This allowed assertions like `assert!(s.contains("Compiling"))` to work against the low-level TUI buffer.
- **Physical Key Matching**: Fixed tests to explicitly set `KeyEventKind::Press`. On Windows, `crossterm` events default to `Release` if not specified, which caused the `App::dispatch_key` logic (which guards for `Press`) to ignore test inputs.
- **Scrollbar Clipping**: Adjusted mouse coordinate expectations in `test_scrollbar_interaction`. The scrollbar logic uses `Rect::inner` to account for borders, meaning click targets must be offset by 1 to hit the active track.

## Phase 3: UI & Widget Components
- **Extraction of `OutputBoxWidget`**: The output panel logic in `view.rs` was heavily coupled with the main `App` state. This was extracted into a standalone `OutputBoxWidget` in `src/widgets/output_box.rs` to allow unit testing of output rendering, autoscroll behavior, and input box visibility without requiring a full `App` instance.
- **Manual `Default` for `ToastConfig`**: Discovered that `#[derive(Default)]` on `ToastConfig` was initializing `duration_seconds` to `0.0`, ignoring the `#[serde(default = "...")]` values intended for configuration loading. This caused rendering tests to immediately expire and hide toasts. Implemented a manual `Default` trait to ensure tests start with visible durations.
- **Layout Initialization in Tests**: Tests in `app/tests.rs` were failing to render UI elements because `create_test_app` initialized a default (0-sized) `AppLayout`. Updated the helper to perform an initial `calculate_layout` pass so `view()` calls have valid `Rect` areas to draw into.
- **Terminal Size Verification**: Adjusted `test_app_view_terminal_too_small` to match the specific text formatting used in `view.rs`. The test was initially looking for generic error strings, while the code uses a multi-line "Terminal Too Small" layout.
- **UTF-8 Character Verification**: Standardized the use of `█` and `═` symbols in `assert!` statements. Mismatched encoding during file writes (ANSI vs UTF-8) previously caused these assertions to fail due to symbol corruption.
