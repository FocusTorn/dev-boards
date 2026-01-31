# Product Guidelines: Dev Boards Workspace

## 1. UI/UX Philosophy (dev-console-v2)
- **Aesthetic:** Modern, interactive dashboard. Use bold colors, progress bars, and dynamic elements to provide a rich visual experience.
- **Layout:** Use a tabbed and sectioned interface to organize complex data into manageable views.
- **Notification Hierarchy:**
    - **Transient:** Use **Toasts** for non-blocking information and quick feedback.
    - **Persistent:** Use the **Status Bars** (top/bottom) for real-time state updates (e.g., connection status, active profile).
    - **Critical:** Use **Modal Interrupts** (to be ported from original dev-console) for blocking errors that require immediate user acknowledgement.

## 2. Cross-Platform Architecture
- **Environment Agnostic:** Design systems to work seamlessly on Windows 11 and Debian/Linux.
- **Hardware Abstraction:** 
    - Use config.yaml for immediate OS-specific path and port management.
    - Transition toward **Abstraction Traits** in Rust to unify hardware access and isolate platform-specific logic.
- **File Systems:** Use OS-agnostic path handling to ensure portability.

## 3. Observability & Debugging
- **Internal Logging:** Maintain a hybrid of file-based logging and internal event tracking.
- **Real-time Debugging:** Work toward a dedicated **Debug Tab** or panel within the TUI to monitor hardware communication and application state without leaving the interface.

## 4. Development & Documentation Standards
- **Small Blocks:** Enforce the principle of small, focused code blocks and functions to improve maintainability.
- **Self-Documentation:** Use expressive naming to make code intent clear.
- **Why-Focused Docstrings:** Every public and internal function, struct, and enum must have a docstring (using `///` in Rust) that explains the "Why" and "How" rather than just the "What".
- **Folding Markers:** 
    - Multi-line docstrings must use `///>` after the title and `///<` before the code to support folding.
    - All existing folding arrows (`//>`, `//<`, `==>>`, etc.) must be preserved and correctly indented.
- **Clean Attributes:** Keep attributes clean; remove all comments from `#[allow(dead_code)]` or similar declarations.
- **Architecture Guides:** Maintain high-level Markdown documentation to serve as a persistent reference for system design and project context.

## 5. Performance & Stability
- **Allocation Minimization:** Minimize heap allocations (clones, strings) in high-frequency loops (view/update cycles).
- **Caching:** Use caching mechanisms (e.g., ANSI line caching) to reduce render-pass latency and processing overhead.
- **Reactive Synchronization:** Ensure derived state is only updated when source data changes.
- **Crash Resilience:** Always install terminal panic hooks in TUI applications to prevent terminal corruption.
