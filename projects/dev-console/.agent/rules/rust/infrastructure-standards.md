---
globs: **/*.rs
alwaysApply: false
---

# dev-console Infrastructure Standards

## Infrastructure Management

### 1. Warning Resolution

**✅ CORRECT - Apply universal pattern**:

When resolving warnings for dev-console infrastructure modules (like `DashboardUpdateBatch`, `CommandConfig`, `ToolDetector`, `ProgressTracker`, etc.), apply the universal infrastructure code management pattern.

```rust
// Example: DashboardUpdateBatch (infrastructure for future use)
/// Batch dashboard state updates (for future use)
#[allow(dead_code)]
pub struct DashboardUpdateBatch {
    status_text: Option<Arc<str>>,
    output_lines: Vec<String>,
    // ...
}

// Example: CommandConfig (infrastructure for future use)
/// Command configuration builder (for future use)
#[allow(dead_code)]
pub struct CommandConfig {
    command: String,
    args: Vec<String>,
    // ...
}

// Example: ToolDetector trait (infrastructure for future use)
/// Trait for tool detection (dependency injection, for future use)
#[allow(dead_code)]
pub trait ToolDetector {
    fn detect_arduino_cli(&self, project_root: &PathBuf, env: &str) -> ToolInfo;
    // ...
}
```

**✅ CORRECT - Infrastructure Modules List**:

The following modules are intended for future integration and should be preserved:
- `DashboardUpdateBatch` - Batching dashboard state updates
- `CommandConfig` - Command configuration builder
- `CommandExecutor` - Command execution trait
- `ToolDetector` / `ToolManager` - Tool detection with dependency injection
- `ConfigValidationError` variants - Configuration validation errors
- `error_format` functions - Error formatting utilities
- `ProgressTracker` / `ProgressHistory` - Advanced progress tracking
- `LayoutCache` methods - Layout calculation caching
- `FieldEditorState` helper methods - Field editor utilities
- `StringInterner` methods - String interning utilities

Mark all these with `#[allow(dead_code)]` and "(for future use)" comments.

### Common Mistakes

- ❌ **Removing Infrastructure Code** - Do not remove dev-console infrastructure modules that will be integrated later.
- ❌ **Not Marking Infrastructure Code** - Do not leave infrastructure code without `#[allow(dead_code)]` and "(for future use)" comments.

### Checklist

- [ ] **Infrastructure Code Marked**: All dev-console infrastructure modules are marked with `#[allow(dead_code)]` and "(for future use)" comments.
- [ ] **Build Clean**: `cargo check` runs with zero warnings.
- [ ] **Documentation Clear**: Infrastructure code intent is documented with "(for future use)" comments.
