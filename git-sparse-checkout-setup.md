# Git Sparse Checkout Setup Guide

This guide shows how to set up partial folder syncing from a private remote repository to multiple local workspaces.

## Overview

- **Private Remote Repo (RR)**: Contains all folders (RR-Folder1 through RR-Folder5)
- **WS1-Root**: Needs only RR-Folder1, RR-Folder2, RR-Folder3
- **WS2-Root**: Needs only RR-Folder1, RR-Folder3, RR-Folder4

## Solution: Git Sparse Checkout

Git sparse checkout allows you to check out only specific directories from a repository.

## Setup Instructions

### For WS1-Root (local1)

```bash
# Navigate to your workspace
cd WS1-Root

# Clone the repository (without checking out files)
git clone --no-checkout <private-repo-url> local1
cd local1

# Enable sparse checkout
git sparse-checkout init --cone

# Configure which folders to check out
git sparse-checkout set RR-Folder1 RR-Folder2 RR-Folder3

# Check out the files
git checkout main  # or master, or your default branch

# Create symbolic links or rename folders as needed
# Option 1: Keep original names
# Option 2: Rename to match your local structure
mv RR-Folder1 L1-Folder1
mv RR-Folder2 L1-Folder2
mv RR-Folder3 L1-Folder3
```

### For WS2-Root (local2)

```bash
# Navigate to your workspace
cd WS2-Root

# Clone the repository (without checking out files)
git clone --no-checkout <private-repo-url> local2
cd local2

# Enable sparse checkout
git sparse-checkout init --cone

# Configure which folders to check out
git sparse-checkout set RR-Folder1 RR-Folder3 RR-Folder4

# Check out the files
git checkout main  # or master, or your default branch

# Create symbolic links or rename folders as needed
mv RR-Folder1 L2-Folder1
mv RR-Folder3 L2-Folder2
mv RR-Folder4 L2-Folder3
```

## Updating Folders

To sync changes from the remote repository:

```bash
cd local1  # or local2
git pull origin main
```

## Adding/Removing Folders

To change which folders are checked out:

```bash
# Add a folder
git sparse-checkout add RR-Folder5

# Remove a folder (and its contents)
git sparse-checkout set RR-Folder1 RR-Folder2  # This removes RR-Folder3

# List current sparse checkout paths
git sparse-checkout list
```

## Alternative: Using .git/info/sparse-checkout (Non-Cone Mode)

If you need more control, you can use non-cone mode:

```bash
git sparse-checkout init --no-cone
git sparse-checkout set RR-Folder1/ RR-Folder2/ RR-Folder3/
```

## Important Notes

1. **Folder Renaming**: If you rename folders locally (RR-Folder1 → L1-Folder1), Git will track them as renamed. You may want to use symbolic links instead if you need to maintain the original structure.

2. **Committing Changes**: You can commit changes back to the remote repo normally. Only the folders you've checked out will be in your working directory, but Git tracks all changes.

3. **Branch Switching**: When switching branches, sparse checkout settings are preserved.

4. **Multiple Remotes**: If you need to sync to multiple remotes, you can add additional remotes:
   ```bash
   git remote add upstream <another-repo-url>
   git fetch upstream
   ```

## Troubleshooting

- **If folders don't appear**: Make sure you've run `git checkout <branch>` after setting sparse checkout
- **To see all remote folders**: `git ls-tree -r --name-only HEAD`
- **To disable sparse checkout**: `git sparse-checkout disable`

