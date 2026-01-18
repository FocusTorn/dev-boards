# Development Setup Guide

This guide provides comprehensive instructions for setting up a development environment for the dev-console project, including prerequisites, build processes, testing, and contribution guidelines.

## Prerequisites

### System Requirements

- **Operating System**: Windows 10+, macOS 10.14+, or Linux (Ubuntu 18.04+)
- **Terminal**: Modern terminal with ANSI support
- **Memory**: Minimum 4GB RAM (8GB recommended)
- **Disk Space**: 500MB for source code and dependencies

### Required Software

#### Rust Toolchain

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version

# Add required components
rustup component add rustfmt
rustup component add clippy
```

#### Development Tools

```bash
# Git (for version control)
git --version

# Make (for build automation)
make --version

# Optional: VS Code with Rust extensions
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
```

#### ESP32 Development Tools

```bash
# Arduino CLI
curl -fsSL https://raw.githubusercontent.com/arduino/arduino-cli/master/install.sh | sh

# PlatformIO CLI
pip install platformio

# ESP32 Toolchain (platform specific)
# Windows: Download from Espressif website
# macOS: brew install esp32
# Linux: Follow Espressif documentation
```

## Project Setup

### Clone the Repository

```bash
# Clone the repository
git clone <repository-url>
cd dev-boards/projects/dev-console

# Verify project structure
ls -la
```

### Dependencies

The project uses both external and internal dependencies:

#### External Dependencies (Cargo.toml)

```toml
[dependencies]
# Shared components
tui-components = { path = "../../_shared-resources/shared-rust/resources/components" }

# TUI dependencies
ratatui = "0.29.0"
crossterm = "0.29.0"

# Configuration parsing
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1.0"

# File I/O
dirs = "5.0"

# Input handling
tui-input = "0.8"

# Serial port detection
serialport = "4.5"

# Regex for parsing
regex = "1.10"
lazy_static = "1.5"

# Command detection
which = "6.0"

# MQTT client
mqttrs = "0.4"
```

#### Internal Dependencies

Ensure the shared resources are available:

```bash
# Check shared components path
ls -la ../../_shared-resources/shared-rust/resources/components

# If missing, initialize git submodules
git submodule update --init --recursive
```

### Build the Project

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests during build
cargo build --tests
```

## Development Workflow

### 1. Code Organization

```
src/
├── main.rs                 # Application entry point
├── app_state.rs           # Application state management
├── settings_manager.rs     # Settings management
├── settings.rs            # Settings data structures
├── field_editor.rs        # Field editing functionality
├── dashboard.rs           # Dashboard state and logic
├── event_handler.rs       # Event handling
├── layout_manager.rs      # Layout calculations
├── process_manager.rs     # Process management
├── profile_manager.rs     # Profile management
├── config.rs              # Configuration structures
├── constants.rs           # Application constants
├── commands/              # Command execution modules
│   ├── mod.rs
│   ├── compile_parser.rs
│   ├── upload.rs
│   ├── monitor_serial.rs
│   └── monitor_mqtt.rs
├── render/                # UI rendering modules
│   ├── mod.rs
│   ├── dashboard.rs
│   └── settings.rs
└── utils/                 # Utility modules
    ├── path_utils.rs
    ├── string_intern.rs
    └── tool_detector.rs
```

### 2. Running the Application

```bash
# Development mode
cargo run

# Release mode
cargo run --release

# With custom configuration
DEV_CONSOLE_CONFIG="debug-config.yaml" cargo run

# With debug logging
RUST_LOG="debug" cargo run
```

### 3. Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration

# Benchmark tests
cargo test --release
```

### 4. Code Quality

```bash
# Format code
cargo fmt

# Check code formatting
cargo fmt -- --check

# Run clippy lints
cargo clippy

# Run clippy with all features
cargo clippy --all-features

# Security audit
cargo audit
```

## Configuration for Development

### Development Configuration

Create `config-dev.yaml` for development:

```yaml
application:
    title: "ESP32-S3 Dev Console (DEV)"
    min_width: 80
    min_height: 21
    bindings:
        - key: "[q]"
          description: "Quit"
        - key: "[F5]"
          description: "Refresh UI"
    status_bar:
        default_text: "DEV MODE | [q] Quit | [F5] Refresh"
        modal_text: "DEV MODE - Modal"

tab_bars:
    main_content_tab_bar:
        hwnd: "hwndMainContentTabBar"
        anchor: "hwndMainContentBox"
        alignment:
            vertical: "top"
            horizontal: "center"
            offset_x: 0
            offset_y: 0
        style: "tabbed"
        color: "magenta"
        min_tab_width: 8
        tab_tooltips: true
        navigation:
            left: ["Left"]
            right: ["Right"]
        tabs:
            - id: "dashboard"
              name: "Dashboard"
              default: "active"
            - id: "settings"
              name: "Settings"
            - id: "debug"
              name: "Debug"

tab_content:
    - tab_id: "dashboard"
      bindings:
        - key: "[Enter]"
          description: "Execute command"
    - tab_id: "settings"
      bindings:
        - key: "[Tab]"
          description: "Next field"
    - tab_id: "debug"
      bindings:
        - key: "[r]"
          description: "Reload config"
```

### Environment Setup

Create `.env` file for development:

```bash
# .env
RUST_LOG=debug
DEV_CONSOLE_CONFIG=config-dev.yaml
DEV_CONSOLE_LOG_LEVEL=debug
```

Load environment variables:

```bash
# Using direnv (recommended)
echo "dotenv" > .envrc
direnv allow

# Or manually
source .env
```

## Debugging

### 1. Debug Builds

```bash
# Build with debug symbols
cargo build

# Run with debugger
gdb target/debug/dev-console
# or on Windows with VS Code debugging
```

### 2. Logging

Add logging to your code:

```rust
// In main.rs or relevant module
use log::{debug, info, warn, error};

fn some_function() {
    debug!("Debug message: {:?}", some_data);
    info!("Info message");
    warn!("Warning message");
    error!("Error message: {}", error);
}
```

Configure logging in `Cargo.toml`:

```toml
[dependencies]
log = "0.4"
env_logger = "0.10"
```

Initialize logging:

```rust
fn main() {
    env_logger::init();
    // ... rest of main
}
```

### 3. Common Debugging Scenarios

#### UI Layout Issues

```rust
// Add debug output for layout calculations
eprintln!("[DEBUG] Content area: {:?}", content_area);
eprintln!("[DEBUG] Field rect: {:?}", field_rect);
```

#### Event Handling

```rust
// Log events for debugging
match event::read()? {
    Event::Key(key) => {
        eprintln!("[DEBUG] Key event: {:?} with modifiers: {:?}", key.code, key.modifiers);
    }
    Event::Mouse(mouse) => {
        eprintln!("[DEBUG] Mouse event: {:?} at ({}, {})", mouse.kind, mouse.column, mouse.row);
    }
    _ => {}
}
```

#### Settings Issues

```rust
// Debug settings loading
eprintln!("[DEBUG] Settings path: {:?}", get_settings_path());
eprintln!("[DEBUG] Loaded settings: {:?}", settings);
```

## Testing

### Unit Tests

Create unit tests in the same file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_default() {
        let settings = Settings::default();
        assert_eq!(settings.env, "arduino");
        assert_eq!(settings.baudrate, 115200);
    }

    #[test]
    fn test_layout_calculation() {
        let rect = Rect::new(0, 0, 100, 50);
        let content = calculate_centered_content_area(rect);
        assert!(content.is_some());
    }
}
```

### Integration Tests

Create tests in `tests/` directory:

```rust
// tests/integration_test.rs
use dev_console::*;

#[test]
fn test_full_workflow() {
    // Test complete user workflow
    let app_state = AppState::new();
    // ... test implementation
}
```

### Mock Testing

Create mocks for external dependencies:

```rust
#[cfg(test)]
mod mocks {
    use super::*;
    
    pub struct MockSettingsManager {
        settings: Settings,
    }
    
    impl MockSettingsManager {
        pub fn new(settings: Settings) -> Self {
            Self { settings }
        }
    }
    
    impl SettingsManagerTrait for MockSettingsManager {
        fn get(&self) -> Settings {
            self.settings.clone()
        }
        
        fn update<F>(&self, f: F) -> Result<(), Box<dyn std::error::Error>>
        where
            F: FnOnce(&mut Settings),
        {
            // Mock implementation
            Ok(())
        }
    }
}
```

## Performance Optimization

### 1. Profile the Application

```bash
# Install profiling tools
cargo install cargo-flamegraph

# Generate flamegraph
cargo flamegraph --bin dev-console

# CPU profiling
perf record --call-graph=dwarf cargo run --release
perf report
```

### 2. Memory Profiling

```bash
# Memory profiling
valgrind --tool=massif cargo run
ms_print massif.out.*
```

### 3. Optimization Tips

```rust
// Use string interning for frequently used strings
use crate::string_intern::intern_string;

let id = intern_string("frequently_used_string");

// Cache expensive calculations
use std::collections::HashMap;

struct LayoutCache {
    cache: HashMap<(u16, u16), Rect>,
}

// Avoid allocations in hot paths
use std::borrow::Cow;

fn process_text(text: &str) -> Cow<str> {
    if text.contains("special") {
        Cow::Owned(text.replace("special", "processed"))
    } else {
        Cow::Borrowed(text)
    }
}
```

## Contributing

### 1. Code Style

Follow Rust conventions:

```rust
// Use rustfmt for formatting
cargo fmt

// Use clippy for linting
cargo clippy -- -D warnings

// Naming conventions
pub struct SettingsManager {  // PascalCase for types
    settings: Arc<Mutex<Settings>>,  // snake_case for fields
}

pub fn get_settings_path() -> PathBuf {  // snake_case for functions
    // ...
}
```

### 2. Commit Messages

Follow conventional commits:

```
feat: Add MQTT monitoring support
fix: Resolve layout calculation bug
docs: Update configuration documentation
test: Add integration tests for profile management
refactor: Simplify event handling logic
```

### 3. Pull Request Process

1. **Create feature branch**
   ```bash
   git checkout -b feature/new-feature
   ```

2. **Make changes and test**
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

3. **Commit changes**
   ```bash
   git add .
   git commit -m "feat: Add new feature"
   ```

4. **Push and create PR**
   ```bash
   git push origin feature/new-feature
   ```

### 4. Code Review Checklist

- [ ] Code follows style guidelines
- [ ] Tests are included and passing
- [ ] Documentation is updated
- [ ] No clippy warnings
- [ ] Performance impact considered
- [ ] Security implications reviewed

## Release Process

### 1. Version Bumping

Update version in `Cargo.toml`:

```toml
[package]
name = "dev-console"
version = "0.2.0"  # Bump version
```

### 2. Changelog

Update `CHANGELOG.md`:

```markdown
## [0.2.0] - 2024-01-XX

### Added
- MQTT monitoring support
- Profile management system
- Enhanced configuration options

### Fixed
- Layout calculation bugs
- Memory leak in event handling

### Changed
- Improved error handling
- Updated dependencies
```

### 3. Build and Test

```bash
# Full test suite
cargo test --all-features

# Build release
cargo build --release

# Test release binary
./target/release/dev-console --version
```

### 4. Tag and Release

```bash
# Create tag
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin v0.2.0

# Create release on GitHub/GitLab
```

## Troubleshooting

### Common Development Issues

#### 1. Build Failures

```bash
# Clean build
cargo clean
cargo build

# Update dependencies
cargo update

# Check Rust version
rustup update
```

#### 2. Terminal Issues

```bash
# Check terminal capabilities
infocmp $TERM

# Test terminal colors
for i in {0..255}; do
    printf "\x1b[48;5;%sm%3d\e[0m " "$i" "$i"
    if (( i == 15 )) || (( i > 15 )) && (( (i-15) % 6 == 0 )); then
        printf "\n"
    fi
done
```

#### 3. Dependency Issues

```bash
# Check shared resources
ls -la ../../_shared-resources/shared-rust/resources/components

# Rebuild shared components
cd ../../_shared-resources/shared-rust/resources/components
cargo build
```

### Getting Help

- **Documentation**: Check `docs/` directory
- **Issues**: Create GitHub issue with detailed information
- **Discussions**: Use GitHub Discussions for questions
- **Logs**: Include debug logs when reporting issues

This development setup guide provides everything needed to start developing, testing, and contributing to the dev-console project.
