# Quick Start Guide

## First Time Setup

### 1. Install Dependencies

```bash
pip install -r requirements.txt
```

Or install PyYAML directly:
```bash
pip install pyyaml
```

### 2. Initialize Shared Repository

Navigate to your project root and run:

```bash
# Windows
python ___shared\.cursor-private-git\scripts\setup-cursor-sync --init-git

# Linux/macOS
./___shared/.cursor-private-git/scripts/setup-cursor-sync --init-git
```

This will:
- Detect your project name
- Create the shared repository structure
- Scan for existing `.cursor/` directories
- Create configuration file
- Sync existing rules to shared repo (or vice versa)
- Initialize Git repository

### 3. Configure Git Remote (Optional)

If you want to push/pull from a remote repository:

```bash
cd ___shared/.cursor-private-git
git remote add origin <your-repo-url>
git push -u origin main
```

## Daily Usage

### Check for Updates

```bash
python ___shared\.cursor-private-git\scripts\check-cursor-rules-updates
```

Exit code 0 = up to date, 1 = updates available

### Sync from Shared Repo to Project

```bash
python ___shared\.cursor-private-git\scripts\sync-cursor-rules --direction=from
```

### Sync from Project to Shared Repo

```bash
python ___shared\.cursor-private-git\scripts\sync-cursor-rules --direction=to
```

### Sync Both Directions

```bash
python ___shared\.cursor-private-git\scripts\sync-cursor-rules --direction=both
```

### Dry Run (Preview Changes)

```bash
python ___shared\.cursor-private-git\scripts\sync-cursor-rules --dry-run
```

## Workflow Example

### Scenario: Project 1 modifies a rule

1. Edit `.cursor/rules/formatting/markdown.mdc`
2. Test your changes
3. Sync to shared repo:
   ```bash
   python sync-cursor-rules --direction=to
   ```
4. Changes are committed and pushed to shared repo

### Scenario: Project 2 receives updates

1. Check for updates:
   ```bash
   python check-cursor-rules-updates
   ```
2. If updates available, sync:
   ```bash
   python sync-cursor-rules --direction=from
   ```
3. Review changelog
4. Resolve any conflicts if prompted
5. Changes appear in VSCode/Cursor source control panel

## Adding a New Project

1. Navigate to new project root
2. Run setup:
   ```bash
   python ___shared\.cursor-private-git\scripts\setup-cursor-sync
   ```
3. Review configuration in `.cursor-sync-config.yaml`
4. Start syncing!

## Troubleshooting

### Scripts not found

Make sure you're running from project root, or use full paths:
```bash
python d:\_dev\_Projects\dev-boards\___shared\.cursor-private-git\scripts\sync-cursor-rules
```

### Permission denied (Linux/macOS)

Make scripts executable:
```bash
chmod +x ___shared/.cursor-private-git/scripts/*
```

### PyYAML not found

Install dependencies:
```bash
pip install pyyaml
```

### Git not found

Install Git and ensure it's in your PATH.

## Next Steps

- Read the full [README.md](README.md) for detailed documentation
- Configure conflict resolution in `.cursor-sync-config.yaml`
- Set up VSCode extension (optional) for automatic update notifications




