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
- **Code Structure:** Prioritize small, focused blocks and functions to keep files manageable.
- **Self-Documentation:** Use expressive naming to make code intent clear.
- **Doc-Comments:** Employ heavy documentation (/// in Rust) to explain the "why" behind complex logic and hardware interactions.
- **Architecture Guides:** Maintain high-level Markdown documentation to serve as a persistent reference for system design and project context.
