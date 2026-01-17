---
trigger: always_on
---
# Environment Variable Rules

## **CRITICAL EXECUTION DIRECTIVE**

**AI Agent Directive**: Follow environment variable rules exactly for all scripts and tools.

**MANDATORY EXECUTION PROTOCOL**:

1. **NO DEVIATION**: All rules must be followed exactly as written
2. **NO SKIPPING**: No steps may be skipped, abbreviated, or modified
3. **NO SELECTIVE COMPLIANCE**: All rules apply to all activities
4. **FAILURE TO COMPLY**: Violating these rules constitutes a critical protocol violation

## **ENVIRONMENT VARIABLE USAGE**

### **1. :: WORKSPACE_ROOT Environment Variable**

**✅ CORRECT - Use WORKSPACE_ROOT environment variable**:

The `WORKSPACE_ROOT` environment variable must be set to the absolute path of the workspace root directory. This variable is used by scripts and tools to locate workspace resources and shared packages.

**✅ CORRECT - Setting WORKSPACE_ROOT in workspace settings**:

Add `WORKSPACE_ROOT` to your workspace settings file:

**VS Code** (`.vscode/settings.json`):
```json
{
    "WORKSPACE_ROOT": "D:\\_dev\\_Projects\\dev-boards"
}
```

**Cursor** (`.cursor/settings.json`):
```json
{
    "WORKSPACE_ROOT": "D:\\_dev\\_Projects\\dev-boards"
}
```

**✅ CORRECT - Setting WORKSPACE_ROOT as shell environment variable**:

```bash
# Windows (PowerShell)
$env:WORKSPACE_ROOT = "D:\_dev\_Projects\dev-boards"

# Linux/Unix
export WORKSPACE_ROOT="/path/to/dev-boards"
```

**✅ CORRECT - Accessing WORKSPACE_ROOT in Python scripts**:

```python
import os
from pathlib import Path

# Get workspace root from environment variable
root_path = os.environ.get('WORKSPACE_ROOT')
if not root_path:
    print("ERROR: WORKSPACE_ROOT environment variable not set.", file=sys.stderr)
    print()
    print("  Please add WORKSPACE_ROOT to your workspace settings file:")
    print("    .vscode/settings.json (VS Code)")
    print("    .cursor/settings.json (Cursor)")
    sys.exit(1)

_workspace_root = Path(root_path)
```

**❌ INCORRECT - Auto-detecting workspace root as fallback**:

```python
# Wrong: Auto-detecting workspace root when WORKSPACE_ROOT is not set
root_path = os.environ.get('WORKSPACE_ROOT')
if not root_path:
    # Wrong: Don't auto-detect - require explicit configuration
    current = Path(__file__).resolve()
    for parent in current.parents:
        if (parent / "pyproject.toml").exists():
            root_path = str(parent)
            break
```

**❌ INCORRECT - Using relative paths or hardcoded paths**:

```python
# Wrong: Using relative paths
_workspace_root = Path(__file__).parent.parent.parent

# Wrong: Hardcoding paths
_workspace_root = Path("D:/_dev/_Projects/dev-boards")
```

### **2. :: Environment Variable Requirements**

**✅ CORRECT - Scripts must require WORKSPACE_ROOT**:

All scripts that need workspace root access must:

1. **Check for WORKSPACE_ROOT**: Always check if `WORKSPACE_ROOT` environment variable is set
2. **Provide Clear Error**: If not set, display clear error message with instructions
3. **Exit with Error Code**: Exit with non-zero exit code if `WORKSPACE_ROOT` is not set
4. **No Auto-Detection**: Never implement auto-detection as a fallback

**✅ CORRECT - Error message format**:

```python
if not root_path:
    print("ERROR: WORKSPACE_ROOT environment variable not set.", file=sys.stderr)
    print()
    print("  Please add WORKSPACE_ROOT to your workspace settings file:")
    print("    .vscode/settings.json (VS Code)")
    print("    .cursor/settings.json (Cursor)")
    print()
    print("  Add the following setting:")
    print('    "WORKSPACE_ROOT": "D:\\\\_dev\\\\_Projects\\\\dev-boards"')
    print()
    print("  Or set it as an environment variable in your shell.")
    sys.exit(1)
```

**❌ INCORRECT - Auto-detection fallback**:

```python
# Wrong: Implementing auto-detection when WORKSPACE_ROOT is not set
if not root_path:
    # Auto-detect by walking up directory tree
    current = Path(__file__).resolve()
    for parent in current.parents:
        if (parent / "pyproject.toml").exists():
            root_path = str(parent)
            break
```

**❌ INCORRECT - Silent failure or default values**:

```python
# Wrong: Using default value or current directory
root_path = os.environ.get('WORKSPACE_ROOT', os.getcwd())

# Wrong: Continuing without workspace root
if not root_path:
    print("Warning: WORKSPACE_ROOT not set, using current directory")
    root_path = os.getcwd()
```

## **ANTI-PATTERNS**

### **❌ Environment Variable Violations**

- ❌ **Auto-Detection Fallback** - Don't implement auto-detection when WORKSPACE_ROOT is not set
- ❌ **Silent Failure** - Don't continue execution if WORKSPACE_ROOT is not set
- ❌ **Default Values** - Don't use default values or current directory as fallback
- ❌ **Hardcoded Paths** - Don't hardcode workspace root paths in scripts
- ❌ **Relative Paths** - Don't use relative paths to determine workspace root

## **QUALITY GATES**

- [ ] **WORKSPACE_ROOT Required**: Scripts check for WORKSPACE_ROOT environment variable
- [ ] **Clear Error Messages**: Error messages provide instructions for setting WORKSPACE_ROOT
- [ ] **No Auto-Detection**: No auto-detection fallback is implemented
- [ ] **Proper Exit Codes**: Scripts exit with non-zero code if WORKSPACE_ROOT is not set
- [ ] **Workspace Settings**: WORKSPACE_ROOT is configured in workspace settings file

## **SUCCESS METRICS**

After implementing proper environment variable usage:

- ✅ **Explicit Configuration** - WORKSPACE_ROOT is explicitly set in workspace settings
- ✅ **Clear Error Messages** - Users receive clear instructions when WORKSPACE_ROOT is missing
- ✅ **No Auto-Detection** - Scripts require explicit configuration, no fallback detection
- ✅ **Consistent Behavior** - All scripts use the same environment variable for workspace root
- ✅ **Cross-Platform Support** - Workspace root resolution works on Windows, Linux, and Unix
