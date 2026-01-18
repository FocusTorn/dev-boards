# Documentation Index

Welcome to the ESP32-S3 Dev Console documentation. This comprehensive documentation suite covers all aspects of the application, from basic usage to advanced development.

## 📚 Documentation Structure

### 🚀 Getting Started
- **[Main README](../README.md)** - Project overview and quick start guide
- **[Development Setup](development.md)** - Set up your development environment
- **[Configuration Reference](configuration.md)** - Complete configuration guide

### 📖 User Guides
- **[HWND System Guide](guides/hwnd-system.md)** - Understanding UI element positioning
- **[Keybindings Guide](guides/keybindings.md)** - Keyboard shortcuts and customization
- **[Settings Storage Guide](guides/settings-storage.md)** - How settings work and are managed

### 🔧 Technical Documentation
- **[API Reference](api/README.md)** - Complete API documentation
- **[Examples](examples/README.md)** - Code examples and tutorials

## 🎯 Quick Navigation

### For Users
1. **New to the application?** Start with the [Main README](../README.md)
2. **Need to configure settings?** Read the [Configuration Reference](configuration.md)
3. **Want to customize keybindings?** Check the [Keybindings Guide](guides/keybindings.md)
4. **Having issues with settings?** See the [Settings Storage Guide](guides/settings-storage.md)

### For Developers
1. **Setting up development?** Follow the [Development Setup Guide](development.md)
2. **Understanding the codebase?** Read the [API Reference](api/README.md)
3. **Working with UI elements?** Study the [HWND System Guide](guides/hwnd-system.md)
4. **Looking for examples?** Browse the [Examples](examples/README.md)

## 🏗️ Architecture Overview

The dev-console is built with a modular architecture:

```
┌─────────────────────────────────────────────────────────────┐
│                    Main Application                          │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │   UI Layer  │  │ Event Layer │  │   Settings Layer    │ │
│  │             │  │             │  │                     │ │
│  │ • Dashboard │  │ • Keyboard  │  │ • SettingsManager   │ │
│  │ • Settings  │  │ • Mouse     │  │ • Profile Manager   │ │
│  │ • Layout    │  │ • Navigation│  │ • Validation        │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │Command Layer│  │Process Layer│  │   Utility Layer     │ │
│  │             │  │             │  │                     │ │
│  │ • Compile   │  │ • Process   │  │ • Path Utils        │ │
│  │ • Upload    │  │ • Monitor   │  │ • String Intern     │ │
│  │ • Monitor   │  │ • Cleanup   │  │ • Tool Detection    │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## 🔑 Key Concepts

### HWND System
- **Purpose**: Manages UI element positioning and layout
- **Key Files**: `src/constants.rs`, `src/layout_manager.rs`
- **Documentation**: [HWND System Guide](guides/hwnd-system.md)

### Settings Management
- **Purpose**: Thread-safe configuration with persistence
- **Key Files**: `src/settings_manager.rs`, `src/settings.rs`
- **Documentation**: [Settings Storage Guide](guides/settings-storage.md)

### Event Handling
- **Purpose**: Keyboard and mouse input processing
- **Key Files**: `src/event_handler.rs`, `src/main.rs`
- **Documentation**: [Keybindings Guide](guides/keybindings.md)

### Profile System
- **Purpose**: Save and load different configurations
- **Key Files**: `src/profile_manager.rs`, `src/profile_state.rs`
- **Documentation**: [Settings Storage Guide](guides/settings-storage.md)

## 📋 Common Tasks

### Adding a New Settings Field

1. **Update Settings struct** in `src/settings.rs`
2. **Add HWND constant** in `src/constants.rs`
3. **Update field configuration** in the field editor
4. **Add validation** if needed
5. **Update documentation**

### Adding a New Keybinding

1. **Add to configuration** in `config.yaml`
2. **Implement handler** in `src/event_handler.rs`
3. **Update documentation** in the [Keybindings Guide](guides/keybindings.md)

### Adding a New Tab

1. **Define tab** in `config.yaml`
2. **Create render function** in `src/render/`
3. **Add event handling** in `src/event_handler.rs`
4. **Update documentation**

## 🛠️ Development Workflow

### 1. Setup
```bash
# Clone and setup
git clone <repository>
cd dev-boards/projects/dev-console
cargo build
```

### 2. Development
```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Check code quality
cargo fmt
cargo clippy
```

### 3. Documentation
```bash
# Serve documentation locally
cargo doc --open

# Check documentation coverage
cargo doc --document-private-items
```

## 📚 Learning Path

### Beginner (1-2 hours)
1. Read [Main README](../README.md) - 15 min
2. Try [Basic Usage Examples](examples/README.md#basic-usage-examples) - 30 min
3. Read [Configuration Reference](configuration.md) - 30 min
4. Experiment with settings - 15 min

### Intermediate (2-4 hours)
1. Study [HWND System Guide](guides/hwnd-system.md) - 45 min
2. Read [Keybindings Guide](guides/keybindings.md) - 30 min
3. Understand [Settings Storage Guide](guides/settings-storage.md) - 45 min
4. Try [Advanced Examples](examples/README.md#advanced-examples) - 60 min

### Advanced (4-8 hours)
1. Complete [Development Setup](development.md) - 60 min
2. Study [API Reference](api/README.md) - 90 min
3. Review [Integration Examples](examples/README.md#testing-examples) - 60 min
4. Contribute to the project - 90 min

## 🔍 Finding Information

### By Topic
- **UI/Layout**: [HWND System Guide](guides/hwnd-system.md)
- **Configuration**: [Configuration Reference](configuration.md)
- **Settings**: [Settings Storage Guide](guides/settings-storage.md)
- **Input**: [Keybindings Guide](guides/keybindings.md)
- **Development**: [Development Setup](development.md)
- **API**: [API Reference](api/README.md)

### By File
- **`main.rs`**: [API Reference - Main](api/README.md#mainrs)
- **`settings.rs`**: [Settings Storage Guide](guides/settings-storage.md#settings-structure)
- **`event_handler.rs`**: [Keybindings Guide](guides/keybindings.md#event-handling)
- **`constants.rs`**: [HWND System Guide](guides/hwnd-system.md#hwnd-identifiers)

### By Use Case
- **I want to...**
  - **Configure the application**: [Configuration Reference](configuration.md)
  - **Customize keyboard shortcuts**: [Keybindings Guide](guides/keybindings.md)
  - **Understand how settings work**: [Settings Storage Guide](guides/settings-storage.md)
  - **Modify the UI layout**: [HWND System Guide](guides/hwnd-system.md)
  - **Add new features**: [Development Setup](development.md)
  - **Use the API**: [API Reference](api/README.md)

## 🤝 Contributing

### Documentation Contributions
- **Found an error?** Please report it or submit a fix
- **Missing information?** Let us know what you need
- **Better examples?** Share your use cases
- **Translation?** Help us reach more users

### Code Contributions
1. **Read the [Development Setup](development.md) guide**
2. **Follow the contribution guidelines**
3. **Write tests for new features**
4. **Update documentation**

## 📞 Getting Help

### Self-Service
- **Search this documentation** - Use Ctrl+F to find specific topics
- **Check examples** - Most common use cases are covered
- **Review API reference** - Detailed function documentation

### Community
- **Issues**: Report bugs and request features
- **Discussions**: Ask questions and share ideas
- **Pull Requests**: Contribute code and documentation

### Quick Links
- **Project Repository**: [GitHub Repository]
- **Issue Tracker**: [GitHub Issues]
- **Discussions**: [GitHub Discussions]

---

**Last Updated**: January 2026  
**Version**: 0.1.0  
**Maintainers**: FocusTorn <FocusTorn@gmail.com>

This documentation is a living resource. If you find something missing or unclear, please let us know!
