# North Star: dev-console2 End Goals

## High-Level Vision
To create a "VS Code-like" experience for development board management, where complex configurations are handled via a modular, keyboard-driven interface that remains lightweight and performant.

## Key Strategic Goals
- **Generic Navigation System**: Move away from hardcoded sidebar categories towards a dynamic system that can accommodate future modules (MQTT, Paths, OTA).
- **Interwoven Keybindings**: Establish a standard where `PgUp/PgDown` handles high-level tabs, `Up/Down` handles sub-navigation (sidebars), and `Tab` handles field-level focus.
- **Centralized Theming**: Migrate widget-level style overrides to a comprehensive theme configuration in `build-config.yaml`.
- **Mock-Driven Development**: Maintain 100% entry-point coverage by using robust mocks for serial hardware and filesystem interactions.
