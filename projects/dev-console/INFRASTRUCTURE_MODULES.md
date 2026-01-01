# Infrastructure Modules for Future Use

This document describes infrastructure modules that have been created to support future development and optimizations. These modules are available but may not be fully integrated into the current codebase.

## Available Infrastructure Modules

### 1. `error_format.rs` - Error Message Formatting

**Purpose**: Standardizes error message formatting across the application.

**Available Functions**:
- `format_error(message: &str) -> String` - Format error messages consistently
- `format_warning(message: &str) -> String` - Format warning messages consistently
- `format_info(message: &str) -> String` - Format info messages consistently
- `format_success(message: &str) -> String` - Format success messages consistently
- `report_error(dashboard, message)` - Report error to dashboard state
- `report_warning(dashboard, message)` - Report warning to dashboard state
- `report_info(dashboard, message)` - Report info to dashboard state
- `report_success(dashboard, message)` - Report success to dashboard state
- `report_error_with_context(dashboard, message, context)` - Report error with context

**Usage Example**:
```rust
use crate::error_format::report_error;

report_error(dashboard_arc, "Failed to execute command");
```

**Status**: Module created, functions available for use. Currently used in error reporting paths.

---

### 2. `tool_detector.rs` - Dependency Injection for Tool Detection

**Purpose**: Centralizes arduino-cli, python, and uv detection logic with dependency injection support.

**Available Components**:
- `ToolDetector` trait - Interface for tool detection
- `DefaultToolDetector` - Default implementation
- `ToolManager<T>` - Manager that uses dependency injection
- `ToolInfo` struct - Information about detected tools

**Usage Example**:
```rust
use crate::tool_detector::{DefaultToolDetector, ToolManager};

let detector = DefaultToolDetector;
let mut manager = ToolManager::new(detector);
manager.detect_all(&project_root, "arduino");

let arduino_info = manager.arduino_cli(&project_root, "arduino");
if arduino_info.available {
    // Use arduino-cli
}
```

**Status**: Module created, ready for integration. Can replace scattered tool detection code in command modules.

---

### 3. `config_validation.rs` - Configuration Validation and Error Recovery

**Purpose**: Provides validation and error recovery for application configuration.

**Available Functions**:
- `validate_config(config: &AppConfig) -> Result<(), ConfigValidationError>` - Validate configuration
- `create_default_config() -> AppConfig` - Create default configuration
- `load_and_validate_config(path: Option<PathBuf>) -> Result<AppConfig, Error>` - Load and validate with error recovery

**Usage Example**:
```rust
use crate::config_validation::load_and_validate_config;

let config = load_and_validate_config(None)?; // Gracefully handles missing/invalid config
```

**Status**: Fully integrated into main.rs. Provides graceful error recovery for configuration loading.

---

### 4. `string_intern.rs` - String Interning for Performance

**Purpose**: Reduces string allocations for repeated status messages.

**Available Components**:
- `StringInterner` struct - String interner implementation
- `intern_string(s: &str) -> Arc<str>` - Global interner function
- `common` module - Pre-interned common status messages (READY, RUNNING, COMPLETED, etc.)

**Usage Example**:
```rust
use crate::string_intern::{intern_string, common};

// Intern a string
let status = intern_string("Running");

// Use pre-interned common strings
let ready = common::READY.clone();
```

**Status**: Fully integrated. DashboardState uses Arc<str> for status_text, progress_stage, and current_file fields.

---

### 5. `dashboard_batch.rs` - Batch Dashboard State Updates

**Purpose**: Reduces lock contention on Arc<Mutex<DashboardState>> by batching updates.

**Available Components**:
- `DashboardUpdateBatch` struct - Batch of updates to apply
- Methods: `set_status_text()`, `add_output_line()`, `set_progress_percent()`, etc.
- `apply(dashboard)` - Apply all batched updates in a single lock operation

**Usage Example**:
```rust
use crate::dashboard_batch::DashboardUpdateBatch;

let mut batch = DashboardUpdateBatch::new();
batch.set_status_text("Processing");
batch.add_output_line("Line 1".to_string());
batch.set_progress_percent(50.0);
batch.apply(dashboard_arc); // Single lock acquisition
```

**Status**: Module created, available for use. Can be integrated into command execution modules to reduce lock contention.

---

### 6. `commands/executor.rs` - Command Execution Infrastructure

**Purpose**: Provides unified command execution interface and builder pattern.

**Available Components**:
- `CommandExecutor` trait - Unified command execution interface
- `CommandConfig` builder - Fluent API for building Command instances

**Usage Example**:
```rust
use crate::commands::executor::{CommandExecutor, CommandConfig};

// Builder pattern
let cmd = CommandConfig::new("arduino-cli")
    .arg("compile")
    .arg("--fqbn")
    .arg(&fqbn)
    .working_dir(&sketch_dir)
    .env("VAR", "value")
    .build();
```

**Status**: Module created, CommandConfig builder available. CommandExecutor trait defined for future command implementations.

---

## Integration Recommendations

### High Priority
1. **tool_detector.rs** - Replace scattered tool detection in `progress_rust.rs`, `upload.rs`, and `pmake.rs` with ToolManager
2. **dashboard_batch.rs** - Integrate batch updates into command execution modules to reduce lock contention

### Medium Priority
3. **error_format.rs** - Use standardized error reporting throughout command modules
4. **commands/executor.rs** - Implement CommandExecutor trait for all command types

### Low Priority
5. **string_intern.rs** - Already integrated, consider using common pre-interned strings more extensively

---

## Notes

- All infrastructure modules are designed to be optional enhancements
- Modules can be integrated incrementally without breaking existing functionality
- Each module is self-contained and well-documented
- Performance benefits are realized when modules are fully integrated
