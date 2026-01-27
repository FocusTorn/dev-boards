# Specification: Device Management & Profile UI (dc2)

## Overview
This track implements the "Device" section of a new, comprehensive **Profile Management Tab**. The UI will adopt a VS Code-style settings layout with a sidebar for category navigation and a main content area for granular configuration. This phase focuses on the "Device" category, rich port detection, and full CRUD operations for profiles.

## Functional Requirements

### 1. VS Code-Style Settings UI
- **Sidebar Navigation:** A left-hand list of categories (e.g., "Device", "MQTT", "Paths").
- **Main Content Area:** Displays settings for the selected category.
- **Header Actions:** 
    - **Profile Switcher:** A dropdown to select the active profile.
    - **CRUD Actions:** Buttons/shortcuts to "New Profile", "Clone Profile", and "Delete Profile".

### 2. Rich Port Autodetection & Selection
- **Autodetection:** Retrieve VID, PID, Manufacturer, Product Name, Serial, and Port Name.
- **"Quick Pick" Interface:**
    - Searchable list triggered by clicking the "Device" selector.
    - **Modal Mode:** Grayscale background dimming (using provided logic).
    - **In-Place Mode:** Standard dropdown integration.
- **Configuration Persistence:** Save selected port and metadata to `config.yaml`.

### 3. Profile Management Operations
- **Load:** Populate all fields (Baud, Display Name, Port) based on the selected profile.
- **Edit/Save:** In-place editing of values with a "Save Changes" footer/tab-bar action.
- **Create New:** Initialize a fresh profile with default values.
- **Clone:** Create a new profile by copying values from an existing one.
- **Delete:** Remove a profile (with a confirmation prompt).

### 4. Device Configuration Fields
- **Active:** Display Name, Port (Quick Pick), Baud Rate, Description.
- **Placeholders (Disabled):** Protocol (Serial/Telnet), Flow Control (RTS/DTR).

## Acceptance Criteria
- [ ] Profiles tab features a sidebar with a working "Device" category.
- [ ] User can create, clone, and delete profiles from the UI.
- [ ] Port selection uses a searchable "Quick Pick" with optional Modal dimming.
- [ ] Selected device metadata is correctly saved to the profile in `config.yaml`.
- [ ] 100% test coverage for CRUD logic and port detection.
- [ ] 80% overall coverage for the new UI components.

## Out of Scope
- Implementation of "MQTT" or "Paths" categories (logic for these will be added in future tracks).
