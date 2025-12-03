# Update Cursor Rules Shared Repository System Plan

## Changes to Document

### 1. Update Configuration Format

- Change from JSON to YAML format
- Structure YAML config to be broken down by project with each project's specific configuration
- Store config as `.cursor-sync-config.yaml` in `___shared/.cursor-private-git/` (source controlled)
- Each project section defines its own mappings and sync preferences

### 2. Cross-Platform Script Implementation

- Use Python for all sync scripts (cross-platform, works on both Windows and Unix/Linux/Debian)
- Scripts use extensionless names (e.g., `sync-from-shared`, `sync-to-shared`, `setup-cursor-sync`)
- Scripts are executable on both platforms (shebang `#!/usr/bin/env python3` for Unix, Windows can run via `python script-name`)
- Scripts handle platform-specific path differences internally
- All scripts located in `___shared/.cursor-private-git/scripts/`
- **No need for separate `.sh` and `.ps1` versions** - single Python script works on both platforms
- **No need for wrapper scripts** - extensionless Python scripts with shebangs are sufficient

### 3. Initial Sync Handling

- Update documentation to clarify that existing `.cursor/` directory is not a problem
- When config is set up, it will either:
- Sync existing `.cursor/` contents to the shared repository (if first time setup)
- Sync shared repository contents down to `.cursor/` (if shared repo has newer content)
- Remove concerns about "handling existing .cursor directory" from questions section

## Files to Update

- `dev-boards/___shared/rule-shared-repo-plan.md` - Update all sections to reflect:
- YAML config format with project breakdown
- Cross-platform Python scripts with extensionless names (single script per function, no separate versions)
- Updated initial sync approach
- Remove outdated questions about existing .cursor directory

## Specific Section Updates

### Section 2: Create Project Mapping Configuration

- Change from `.cursor-sync-config.json` to `.cursor-sync-config.yaml`
- Update to show project-specific structure
- Clarify it's source controlled in shared repo

### Section 3: Implement Sync Scripts

- Update to show single extensionless Python scripts (e.g., `sync-from-shared`, `sync-to-shared`, `setup-cursor-sync`)
- Remove references to `.sh` and `.ps1` versions
- Remove references to wrapper scripts
- Update script locations to `___shared/.cursor-private-git/scripts/`
- Clarify that Python scripts with shebangs work natively on Unix and can be run with `python script-name` on Windows

### Section 7: Documentation and Usage Scripts

- Update helper scripts to be single extensionless Python scripts
- Remove references to `.sh` and `.ps1` versions
- Remove references to wrapper scripts

### Technical Considerations

- Remove "PowerShell Scripts: Primary automation (Windows-focused)"
- Remove references to separate script versions for different platforms
- Add cross-platform Python script considerations
- Clarify that extensionless Python scripts eliminate the need for platform-specific versions

### Files to Create/Modify Section

- Update to show YAML config instead of JSON
- Show single extensionless Python scripts (no `.sh` or `.ps1` versions)
- Remove references to wrapper scripts
- List scripts as: `sync-from-shared`, `sync-to-shared`, `setup-cursor-sync` (extensionless Python scripts)

### Questions to Resolve Section

- Remove question #2 about handling existing `.cursor/` directory (resolved)
- Remove question #3 about config being version controlled (resolved - it is)
- Update remaining questions if needed