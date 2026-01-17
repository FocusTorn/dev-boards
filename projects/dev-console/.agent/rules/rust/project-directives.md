---
globs: **/*.rs
alwaysApply: true
---

# dev-console Context Router

| If I am working on... | Then I MUST read... |
| :--- | :--- |
| **Project Infrastructure / Defaults** | `projects/dev-console/.agent/rules/rust/infrastructure-standards.md` |
| **UI / Rendering / Widgets** | `.agent/rules/by-language/rust/tui-hwnd-system.md`<br>`.agent/rules/by-language/rust/tui/tui-performance.md` |
| **Input / Events / Loop** | `.agent/rules/by-language/rust/tui/tui-events.md` |
| **Architecture / State** | `.agent/rules/by-language/rust/rust-architecture.md`<br>`.agent/rules/by-language/rust/rust-refactoring.md` |
| **Patterns / Idioms** | `.agent/rules/by-language/rust/rust-idioms.md` |
| **Maintenance / Cleanup** | `.agent/rules/by-language/rust/rust-maintenance.md` |
| **Build / Test** | `.agent/rules/by-language/rust/build-and-test.md` |

## Execution Protocol

1.  **Identify Context**: Determine which row(s) apply to the current task.
2.  **Load Rules**: Read ONLY the files listed in the applicable rows.
3.  **Apply Strictness**: Follow the loaded rules exactly.
