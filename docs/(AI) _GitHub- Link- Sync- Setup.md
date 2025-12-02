---
alwaysApply: true
---

# GitHub Link and Sync Setup System

## **REFERENCE FILES**

### **Documentation References**

- **SOP_DOCS**: `docs/_SOP.md`
- **ARCHITECTURE_DOCS**: `docs/_Architecture.md`
- **PACKAGE_ARCHETYPES**: `docs/_Package-Archetypes.md`
- **SPARSE_CHECKOUT_GUIDE**: `git-sparse-checkout-setup.md`
- **SETUP_SCRIPT**: `setup-sparse-checkout.ps1`
- **SYNC_SCRIPT**: `sync-sparse-checkout.ps1`

### **AI Testing Documentation References**

- **AI_TESTING_BASE**: `docs/testing/(AI) _Strategy- Base- Testing.md`
- **AI_MOCKING_BASE**: `docs/testing/(AI) _Strategy- Base- Mocking.md`
- **AI_TROUBLESHOOTING**: `docs/testing/(AI) _Troubleshooting- Base.md`

---

## **CRITICAL EXECUTION DIRECTIVE**

**AI Agent Directive**: Implement and maintain GitHub link and sync setup system for selective folder synchronization from private remote repositories to multiple local workspaces using Git sparse checkout.

**MANDATORY EXECUTION PROTOCOL**:

1. **NO DEVIATION**: All rules must be followed exactly as written
2. **NO SKIPPING**: No steps may be skipped, abbreviated, or modified
3. **NO SELECTIVE COMPLIANCE**: All rules apply to all actions
4. **FAILURE TO COMPLY**: Violating these rules constitutes a critical protocol violation

---

## **GITHUB LINK AND SYNC SYSTEM**

### **System Overview**

The GitHub Link and Sync Setup System enables selective synchronization of specific folders from a private remote GitHub repository to multiple local workspaces. Each workspace can independently select which folders to sync and map them to local folder names.

### **Core Concepts**

#### **1. :: Remote Repository Structure**

**✅ CORRECT - Remote repository contains all available folders**:

```
Private Remote Repo (RR)
├── RR-Folder1/
├── RR-Folder2/
├── RR-Folder3/
├── RR-Folder4/
└── RR-Folder5/
```

**❌ INCORRECT - Remote repository structure assumptions**:

- Assuming all workspaces need all folders
- Assuming folder structure matches local structure
- Assuming folder names are consistent across workspaces

#### **2. :: Workspace Structure**

**✅ CORRECT - Each workspace defines its own folder mapping**:

```
WS1-Root/
└── local1/
    ├── L1-Folder1/ → synced from RR-Folder1
    ├── L1-Folder2/ → synced from RR-Folder2
    └── L1-Folder3/ → synced from RR-Folder3

WS2-Root/
└── local2/
    ├── L2-Folder1/ → synced from RR-Folder1
    ├── L2-Folder2/ → synced from RR-Folder3
    └── L2-Folder3/ → synced from RR-Folder4
```

**❌ INCORRECT - Workspace structure violations**:

- Cloning entire repository when only subsets are needed
- Using same folder names across all workspaces
- Not maintaining workspace-specific folder mappings

#### **3. :: Synchronization Mechanism**

**✅ CORRECT - Git sparse checkout for selective folder syncing**:

- Use `git sparse-checkout` to select specific folders
- Maintain independent sparse checkout configuration per workspace
- Support folder renaming/mapping during sync
- Enable bidirectional sync (pull and push)

**❌ INCORRECT - Synchronization violations**:

- Cloning entire repository for partial access
- Manual folder copying instead of Git-based sync
- Not maintaining Git history and metadata
- Losing ability to push changes back to remote

---

## **GITHUB LINK AND SYNC RULES**

### **1. :: Repository Initialization Rules**

#### **1.1. :: Sparse Checkout Initialization**

**✅ CORRECT - Initialize sparse checkout before checkout**:

```bash
# Clone without checking out files
git clone --no-checkout <remote-repo-url> <local-folder>

# Navigate to local folder
cd <local-folder>

# Initialize sparse checkout (cone mode recommended)
git sparse-checkout init --cone

# Configure folders to sync
git sparse-checkout set <folder1> <folder2> <folder3>

# Check out selected folders
git checkout <branch-name>
```

**❌ INCORRECT - Initialization violations**:

- Checking out files before configuring sparse checkout
- Using `--no-cone` mode without specific requirements
- Not specifying branch before checkout
- Skipping sparse checkout initialization

#### **1.2. :: Folder Mapping Rules**

**✅ CORRECT - Map remote folders to local names**:

```bash
# After checkout, rename folders to match local structure
mv RR-Folder1 L1-Folder1
mv RR-Folder2 L1-Folder2
mv RR-Folder3 L1-Folder3

# Git will track these as renames
git add -A
git commit -m "Map remote folders to local structure"
```

**✅ CORRECT - Alternative: Use symbolic links**:

```bash
# Create symbolic links if original structure must be preserved
ln -s RR-Folder1 L1-Folder1
ln -s RR-Folder2 L1-Folder2
ln -s RR-Folder3 L1-Folder3
```

**❌ INCORRECT - Folder mapping violations**:

- Not maintaining mapping documentation
- Using inconsistent naming conventions
- Breaking Git tracking with manual file moves
- Not committing folder mappings

### **2. :: Synchronization Rules**

#### **2.1. :: Pull Synchronization**

**✅ CORRECT - Pull changes from remote**:

```bash
# Navigate to local repository
cd <local-folder>

# Fetch latest changes
git fetch origin

# Pull changes for current sparse checkout paths
git pull origin <branch-name>
```

**✅ CORRECT - Verify sparse checkout paths before sync**:

```bash
# List current sparse checkout configuration
git sparse-checkout list

# Verify paths match expected folders
# Then proceed with pull
```

**❌ INCORRECT - Pull synchronization violations**:

- Pulling without verifying sparse checkout paths
- Pulling entire repository instead of sparse paths
- Not handling merge conflicts properly
- Skipping fetch before pull

#### **2.2. :: Push Synchronization**

**✅ CORRECT - Push changes back to remote**:

```bash
# Stage changes in synced folders
git add <local-folder-name>/

# Commit changes
git commit -m "Update synced folders"

# Push to remote
git push origin <branch-name>
```

**✅ CORRECT - Verify changes before push**:

```bash
# Check status of sparse checkout paths
git status

# Review changes
git diff

# Then commit and push
```

**❌ INCORRECT - Push synchronization violations**:

- Pushing changes outside sparse checkout paths
- Not verifying changes affect only synced folders
- Pushing without proper commit messages
- Not handling push conflicts

### **3. :: Folder Management Rules**

#### **3.1. :: Adding Folders**

**✅ CORRECT - Add new folder to sparse checkout**:

```bash
# Add folder to sparse checkout
git sparse-checkout add <new-folder>

# Check out the new folder
git checkout <branch-name>

# Map to local name if needed
mv <new-folder> <local-folder-name>
```

**❌ INCORRECT - Adding folder violations**:

- Adding folder without updating documentation
- Not mapping new folder to local structure
- Adding folder that conflicts with existing structure

#### **3.2. :: Removing Folders**

**✅ CORRECT - Remove folder from sparse checkout**:

```bash
# Update sparse checkout to exclude folder
git sparse-checkout set <folder1> <folder2>  # Excludes folder3

# Remove local folder if no longer needed
rm -rf <local-folder-name>
```

**❌ INCORRECT - Removing folder violations**:

- Removing folder without backing up changes
- Not updating sparse checkout configuration
- Removing folder that other processes depend on

---

## **GITHUB LINK AND SYNC PATTERNS**

### **1. :: Workspace Setup Pattern**

**✅ CORRECT - Complete workspace setup pattern**:

```bash
# Step 1: Navigate to workspace root
cd <workspace-root>

# Step 2: Clone repository without checkout
git clone --no-checkout <remote-repo-url> <local-folder-name>

# Step 3: Navigate to local folder
cd <local-folder-name>

# Step 4: Initialize sparse checkout
git sparse-checkout init --cone

# Step 5: Configure folders to sync
git sparse-checkout set <remote-folder1> <remote-folder2> <remote-folder3>

# Step 6: Check out selected folders
git checkout <branch-name>

# Step 7: Map folders to local names
mv <remote-folder1> <local-folder1>
mv <remote-folder2> <local-folder2>
mv <remote-folder3> <local-folder3>

# Step 8: Commit folder mappings
git add -A
git commit -m "Map remote folders to local structure"
```

### **2. :: Sync Pattern**

**✅ CORRECT - Regular sync pattern**:

```bash
# Step 1: Navigate to local repository
cd <workspace-root>/<local-folder-name>

# Step 2: Verify sparse checkout paths
git sparse-checkout list

# Step 3: Fetch latest changes
git fetch origin

# Step 4: Pull changes
git pull origin <branch-name>

# Step 5: Handle any conflicts if needed
# (resolve conflicts, then commit)

# Step 6: Verify sync completed successfully
git status
```

### **3. :: Multi-Workspace Pattern**

**✅ CORRECT - Managing multiple workspaces**:

```bash
# Workspace 1 Setup
cd WS1-Root
git clone --no-checkout <remote-repo-url> local1
cd local1
git sparse-checkout init --cone
git sparse-checkout set RR-Folder1 RR-Folder2 RR-Folder3
git checkout main
mv RR-Folder1 L1-Folder1
mv RR-Folder2 L1-Folder2
mv RR-Folder3 L1-Folder3
git add -A && git commit -m "Map folders for WS1"

# Workspace 2 Setup
cd WS2-Root
git clone --no-checkout <remote-repo-url> local2
cd local2
git sparse-checkout init --cone
git sparse-checkout set RR-Folder1 RR-Folder3 RR-Folder4
git checkout main
mv RR-Folder1 L2-Folder1
mv RR-Folder3 L2-Folder2
mv RR-Folder4 L2-Folder3
git add -A && git commit -m "Map folders for WS2"
```

---

## **GITHUB LINK AND SYNC ANTI-PATTERNS**

### **❌ Architecture Anti-Patterns**

- ❌ **Full Repository Clone** - Cloning entire repository when only subsets are needed
- ❌ **Manual File Copying** - Copying files manually instead of using Git sync
- ❌ **No Folder Mapping** - Not maintaining documentation of folder mappings
- ❌ **Inconsistent Naming** - Using different naming conventions across workspaces
- ❌ **Missing Sparse Checkout** - Not using sparse checkout for selective syncing

### **❌ Synchronization Anti-Patterns**

- ❌ **Pull Without Fetch** - Pulling without fetching latest changes first
- ❌ **No Conflict Handling** - Not handling merge conflicts during sync
- ❌ **Push Without Verification** - Pushing changes without verifying sparse checkout paths
- ❌ **Sync Entire Repo** - Syncing entire repository instead of sparse paths
- ❌ **No Status Verification** - Not verifying sync status after operations

### **❌ Folder Management Anti-Patterns**

- ❌ **Add Without Mapping** - Adding folders without mapping to local structure
- ❌ **Remove Without Backup** - Removing folders without backing up changes
- ❌ **No Documentation** - Not documenting folder additions/removals
- ❌ **Break Git Tracking** - Moving files in ways that break Git tracking

---

## **GITHUB LINK AND SYNC QUALITY GATES**

### **Setup Quality Gates**

- [ ] **Sparse Checkout Initialized** - Sparse checkout properly initialized before checkout
- [ ] **Folders Configured** - All required folders configured in sparse checkout
- [ ] **Folder Mapping Complete** - All remote folders mapped to local names
- [ ] **Git Tracking Active** - All mapped folders tracked by Git
- [ ] **Documentation Updated** - Folder mappings documented

### **Sync Quality Gates**

- [ ] **Sparse Paths Verified** - Sparse checkout paths verified before sync
- [ ] **Fetch Completed** - Latest changes fetched from remote
- [ ] **Pull Successful** - Pull operation completed without errors
- [ ] **Conflicts Resolved** - All merge conflicts resolved if present
- [ ] **Status Verified** - Git status verified after sync

### **Folder Management Quality Gates**

- [ ] **Mapping Documented** - Folder additions/removals documented
- [ ] **Git Tracking Maintained** - Git tracking maintained for all operations
- [ ] **No Conflicts** - Folder operations don't conflict with existing structure
- [ ] **Backup Created** - Backups created before removing folders

---

## **GITHUB LINK AND SYNC SUCCESS METRICS**

After implementing proper GitHub link and sync setup:

- ✅ **Selective Sync** - Only required folders synced to each workspace
- ✅ **Independent Workspaces** - Each workspace maintains independent folder selection
- ✅ **Bidirectional Sync** - Changes can be pulled from and pushed to remote
- ✅ **Git History Preserved** - Full Git history and metadata maintained
- ✅ **Folder Mapping Maintained** - Consistent folder mapping across operations
- ✅ **Conflict Resolution** - Merge conflicts handled properly during sync
- ✅ **Documentation Complete** - All folder mappings and configurations documented

---

## **GITHUB LINK AND SYNC VIOLATION PREVENTION**

### **Natural Stops**

- **MANDATORY**: Full repository clone detected → "Use sparse checkout for selective syncing"
- **MANDATORY**: Missing sparse checkout initialization → "Initialize sparse checkout before checkout"
- **MANDATORY**: No folder mapping documentation → "Document all folder mappings"
- **MANDATORY**: Pull without fetch → "Fetch before pull for latest changes"
- **MANDATORY**: Push without verification → "Verify sparse checkout paths before push"
- **MANDATORY**: Folder operations without backup → "Create backup before removing folders"

### **Pattern Recognition**

- Repository structure → Determines sparse checkout configuration
- Workspace requirements → Determines folder selection and mapping
- Sync operations → Determines pull/push procedures
- Folder management → Determines add/remove procedures
- Conflict resolution → Determines merge conflict handling

---

## **EXECUTION PRIORITY MATRIX**

### **CRITICAL PRIORITY (Execute immediately)**

- Sparse checkout initialization verification
- Folder mapping documentation validation
- Sync operation conflict detection
- Git tracking verification

### **HIGH PRIORITY (Execute before proceeding)**

- Folder selection validation
- Sparse checkout path verification
- Pull/push operation verification
- Folder mapping consistency check

### **MEDIUM PRIORITY (Execute during normal operation)**

- Sync status monitoring
- Folder addition/removal validation
- Documentation updates
- Backup creation verification

### **LOW PRIORITY (Execute when time permits)**

- Performance optimization
- Workspace structure optimization
- Advanced conflict resolution strategies

---

## **DYNAMIC MANAGEMENT NOTE**

This document is optimized for AI agent internal processing and may be updated dynamically based on operational needs and pattern recognition. The structure prioritizes AI agent compliance and effectiveness over traditional documentation practices.

