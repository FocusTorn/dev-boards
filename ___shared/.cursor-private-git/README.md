# Cursor Rules Shared Repository System

A Git-aware sync system for sharing Cursor rules across multiple projects using a shared private repository.

## Overview

This system allows you to:
- Share Cursor rules (`commands/`, `rules/`) across multiple projects
- Keep rules synchronized via Git operations
- See changes in VSCode/Cursor source control panel naturally
- Handle conflicts with configurable resolution strategies
- Work cross-platform (Windows, Linux, macOS)

## Architecture

```
Project Structure:
├── .cursor/                          # Project's Cursor rules (Git-tracked)
│   ├── commands/                     # Mapped to shared repo
│   └── rules/                        # Mapped to shared repo
│
└── ___shared/
    └── .cursor-private-git/          # Shared repo (Git repository)
        ├── .git/
        ├── commands/                  # Shared commands
        ├── rules/                     # Shared rules
        ├── .cursor-sync-config.yaml   # Mapping configuration
        └── scripts/
            ├── sync-cursor-rules      # Main sync script
            ├── check-cursor-rules-updates  # Change detection
            └── setup-cursor-sync      # Setup script
```

## Quick Start

### 1. Initial Setup

Run the setup script in your project root:

```bash
# Windows
python ___shared\.cursor-private-git\scripts\setup-cursor-sync --init-git

# Linux/macOS
./___shared/.cursor-private-git/scripts/setup-cursor-sync --init-git
```

This will:
- Detect your project name
- Create shared repository structure
- Scan for `.cursor/` directories
- Create YAML configuration with mappings
- Sync existing `.cursor/` to shared repo (or vice versa)
- Initialize Git repository (if `--init-git` is used)

### 2. Daily Usage

**Sync changes from shared repo to project:**
```bash
python ___shared\.cursor-private-git\scripts\sync-cursor-rules --direction=from
```

**Sync changes from project to shared repo:**
```bash
python ___shared\.cursor-private-git\scripts\sync-cursor-rules --direction=to
```

**Sync both directions:**
```bash
python ___shared\.cursor-private-git\scripts\sync-cursor-rules --direction=both
```

**Check for updates (lightweight):**
```bash
python ___shared\.cursor-private-git\scripts\check-cursor-rules-updates
```

**Dry run (see what would change):**
```bash
python ___shared\.cursor-private-git\scripts\sync-cursor-rules --dry-run
```

## Configuration

The configuration file `.cursor-sync-config.yaml` defines how rules are synced:

```yaml
projects:
  dev-boards:
    mappings:
      - source: ".cursor/commands"
        target: "commands"
        sync_direction: "bidirectional"
      - source: ".cursor/rules/formatting"
        target: "rules/formatting"
        sync_direction: "bidirectional"
    sync_mode: "manual"  # manual or auto
    conflict_resolution: "prompt"  # prompt, source_wins, target_wins, merge
    git_remote: "origin"
    project_root: "."
```

### Configuration Options

- **mappings**: List of directory/file mappings between project and shared repo
  - `source`: Path in project (relative to project root)
  - `target`: Path in shared repo (relative to shared repo root)
  - `sync_direction`: `bidirectional`, `from_shared`, or `to_shared`

- **sync_mode**: `manual` (default) or `auto` (future: automatic syncing)

- **conflict_resolution**: How to handle conflicts
  - `prompt`: Ask user which version to keep
  - `source_wins`: Keep local version
  - `target_wins`: Keep remote version
  - `merge`: Attempt automatic merge (with fallback to prompt)

- **git_remote**: Git remote name for shared repo (default: `origin`)

- **project_root**: Relative path to project root from shared repo location (default: `.`)

## Workflow Examples

### Project 1 modifies a rule

1. Edit `.cursor/rules/formatting/markdown.mdc`
2. Commit to project Git (normal workflow)
3. Run sync to push to shared repo:
   ```bash
   python sync-cursor-rules --direction=to
   ```
4. Script commits to shared repo and pushes to remote

### Project 2 receives updates

1. Check for updates:
   ```bash
   python check-cursor-rules-updates
   ```
2. If updates available, sync:
   ```bash
   python sync-cursor-rules --direction=from
   ```
3. Script pulls changes, shows changelog, handles conflicts
4. Updates `.cursor/` files
5. VSCode/Cursor shows updated files in source control panel

## Scripts Reference

### sync-cursor-rules

Main sync script that handles bidirectional syncing.

**Usage:**
```bash
sync-cursor-rules [--dry-run] [--direction=from|to|both] [--project-name=NAME]
```

**Options:**
- `--dry-run`: Show what would be synced without making changes
- `--direction`: Sync direction (`from`, `to`, or `both`)
- `--project-name`: Override detected project name

**Features:**
- Pulls changes from shared repo Git repository
- Updates local `.cursor/` mapped directories
- Shows changelog of what changed
- Detects and handles conflicts
- Commits and pushes changes to shared repo

### check-cursor-rules-updates

Lightweight script to check if shared repo has updates.

**Usage:**
```bash
check-cursor-rules-updates [--project-name=NAME] [--json] [--quiet]
```

**Options:**
- `--project-name`: Override detected project name
- `--json`: Output results as JSON
- `--quiet`: Quiet mode - only exit code (0 = up to date, 1 = updates available)

**Exit Codes:**
- `0`: Shared repository is up to date
- `1`: Updates available
- `2`: Error (e.g., shared repo not found)

**Integration:**
Can be called by VSCode/Cursor extensions or Git hooks to check for updates.

### setup-cursor-sync

Initial setup script for new projects.

**Usage:**
```bash
setup-cursor-sync [--project-name=NAME] [--init-git] [--shared-repo-path=PATH]
```

**Options:**
- `--project-name`: Override detected project name
- `--init-git`: Initialize Git repository in shared repo
- `--shared-repo-path`: Override shared repo path

**What it does:**
- Detects project name
- Creates shared repository structure
- Scans for `.cursor/` directories
- Creates/updates YAML configuration
- Handles initial sync (project → shared or shared → project)

## Cross-Platform Support

All scripts work on Windows, Linux, and macOS:

- **Windows**: Run with `python script-name` or make executable
- **Linux/macOS**: Make executable with `chmod +x script-name`, then run `./script-name`
- Scripts use `pathlib` for cross-platform path handling
- Shebang `#!/usr/bin/env python3` for Unix compatibility

## Git Integration

The shared repository is a Git repository. Changes flow through Git operations:

1. **To Shared Repo**: Changes in `.cursor/` → copied to shared repo → committed → pushed
2. **From Shared Repo**: Changes pulled → copied to `.cursor/` → shown in source control

VSCode/Cursor source control panel naturally shows:
- Files that need to be synced
- Remote changes available
- Local changes ready to push

## Conflict Handling

When conflicts occur (local changes vs. remote changes), the system handles them based on configuration:

- **prompt**: Ask user which version to keep (interactive)
- **source_wins**: Keep local version automatically
- **target_wins**: Keep remote version automatically
- **merge**: Attempt automatic merge (falls back to prompt if merge fails)

## Troubleshooting

### "Could not find shared repository"

Make sure you're running scripts from the project root, or specify `--shared-repo-path`.

### "Git is not installed or not in PATH"

Install Git and ensure it's in your system PATH.

### "Project not found in configuration"

Run `setup-cursor-sync` to add your project to the configuration.

### "Merge conflicts detected"

Resolve conflicts manually in the shared repo, then re-run sync.

### Scripts not executable (Linux/macOS)

Make scripts executable:
```bash
chmod +x ___shared/.cursor-private-git/scripts/*
```

## Adding New Projects

1. Navigate to new project root
2. Run setup script:
   ```bash
   python ___shared\.cursor-private-git\scripts\setup-cursor-sync
   ```
3. Review configuration in `.cursor-sync-config.yaml`
4. Start syncing!

## VSCode/Cursor Extension (Optional)

A VSCode/Cursor extension can integrate with the source control panel:
- Calls `check-cursor-rules-updates` on Git refresh
- Shows notification: "X rule files updated in shared repo"
- Provides button to run `sync-cursor-rules`
- Shows shared repo status alongside project Git status

See `vscode-extension/` directory for extension code (if created).

## Requirements

- Python 3.6+
- Git
- PyYAML (`pip install pyyaml`)

## License

[Your License Here]

## Contributing

[Your Contributing Guidelines Here]




