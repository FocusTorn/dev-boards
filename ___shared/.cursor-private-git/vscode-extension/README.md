# Cursor Rules Sync VSCode Extension

Optional VSCode/Cursor extension for integrating Cursor Rules sync with the source control panel.

## Features

- Automatically checks for shared repository updates
- Shows notifications when updates are available
- Provides commands to sync rules
- Integrates with Git source control panel

## Installation

1. Install dependencies:
   ```bash
   cd vscode-extension
   npm install
   ```

2. Compile TypeScript:
   ```bash
   npm run compile
   ```

3. Install extension in VSCode/Cursor:
   - Press `F5` to open extension development host
   - Or package and install: `vsce package` then install the `.vsix` file

## Usage

The extension automatically:
- Checks for updates when workspace opens
- Shows notifications when updates are available
- Provides "Sync Now" button in notifications

Manual commands:
- `Cursor Rules Sync: Sync` - Run sync script
- `Cursor Rules Sync: Check` - Check for updates

## Development

```bash
# Watch for changes
npm run watch

# Compile
npm run compile
```

## Requirements

- VSCode/Cursor 1.60.0+
- Python 3.6+
- Git
- The sync scripts must be available at `___shared/.cursor-private-git/scripts/`




