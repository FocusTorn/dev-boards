# ESP32-S3 Dev Console

A powerful Terminal User Interface (TUI) application for managing ESP32-S3 development workflows, built with Rust and the ratatui framework.

## Overview

The ESP32-S3 Dev Console provides a comprehensive interface for managing embedded development tasks including compilation, uploading, monitoring, and debugging ESP32-S3 devices. The application features a modern, responsive TUI with support for profiles, real-time monitoring, and extensible command execution.

## Features

- **Dashboard Interface**: Central hub for executing development commands
- **Settings Management**: Comprehensive configuration system with profiles
- **Real-time Monitoring**: Serial and MQTT monitoring capabilities
- **Profile System**: Save and load different development configurations
- **Modern TUI**: Responsive terminal interface with mouse support
- **Extensible Architecture**: Modular design for easy customization

## Quick Start

### Prerequisites

- Rust 1.70+ 
- ESP32-S3 development tools (Arduino CLI, platformio, etc.)
- Windows, Linux, or macOS terminal

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd dev-boards/projects/dev-console

# Build and run
cargo run --release
```

### Basic Usage

1. **Launch the application**: `cargo run --release`
2. **Navigate tabs**: Use arrow keys or click to switch between Dashboard and Settings
3. **Configure settings**: Set up your sketch directory, board model, and connection parameters
4. **Execute commands**: Navigate the dashboard and press Enter to execute commands
5. **Monitor output**: View real-time output in the dashboard area

## Documentation Structure

- [User Guides](docs/guides/) - Comprehensive user documentation
- [API Reference](docs/api/) - Technical API documentation  
- [Examples](docs/examples/) - Code examples and tutorials
- [Configuration](docs/configuration.md) - Configuration reference
- [Development](docs/development.md) - Development setup and contributing

## Key Documentation Topics

### HWND System
Learn how the window handle (HWND) system manages UI element positioning and layout:
- [HWND System Guide](docs/guides/hwnd-system.md)

### Keybindings
Understand and customize keyboard shortcuts:
- [Keybindings Guide](docs/guides/keybindings.md)

### Settings Storage
Learn how settings are stored, managed, and persisted:
- [Settings Storage Guide](docs/guides/settings-storage.md)

## Architecture

The application is built with a modular architecture:

- **Main Loop**: Event-driven main application loop (`src/main.rs`)
- **UI Components**: Modular TUI components using the `tui-components` crate
- **Settings Management**: Centralized settings system with persistence
- **Command Execution**: Extensible command framework with process management
- **Profile System**: Configuration profiles for different development environments

## Configuration

The application uses YAML configuration files:

- `config.yaml` - Main application configuration
- Settings are stored in user's config directory: `~/.config/dev-console/settings.yaml`

## Development

See [Development Guide](docs/development.md) for:

- Setting up the development environment
- Understanding the codebase architecture
- Contributing guidelines
- Testing procedures

## License

MIT License - see LICENSE file for details.

## Support

For issues, questions, or contributions:

1. Check the documentation in the `docs/` directory
2. Review existing issues in the project repository
3. Create a new issue with detailed information

---

**Note**: This is part of the larger dev-boards project for embedded development tooling.
