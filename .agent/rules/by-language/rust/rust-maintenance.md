---
globs: **/*.rs

---

# Rust Maintenance Rules

## Code Cleanup & Warning Resolution

### **1. :: Understanding Deprecated vs Unused Code**

**✅ CORRECT - Investigate #[allow(dead_code)] purpose**:

When encountering `#[allow(dead_code)]` attributes, investigate the code's purpose:

1. **Deprecated code**: Keep until migration is complete, then remove
2. **Utility methods**: Remove `#[allow(dead_code)]` - they're part of the public API
3. **Truly unused code**: Remove the code entirely

**✅ CORRECT - Removing #[allow(dead_code)] from utility methods**:

```rust
// Before: Utility method marked as dead code
#[allow(dead_code)]
pub fn process_count(&self) -> usize {
    self.processes.lock().unwrap().len()
}

// After: Remove attribute - it's a useful API method
pub fn process_count(&self) -> usize {
    self.processes.lock().unwrap().len()
}
```

### **2. :: Infrastructure Code Management Pattern**

**✅ CORRECT - Categorize and document infrastructure code for future use**:

When resolving compiler warnings for infrastructure code, systematically categorize:

1. **Mark infrastructure code**: For infrastructure code, use:
   ```rust
   /// Module description (for future use)
   #[allow(dead_code)]
   pub struct InfrastructureStruct {
       // Will be used in future feature
   }
   ```

2. **Document intent**: Always add "(for future use)" comment alongside `#[allow(dead_code)]` to clarify intent:
   ```rust
   /// Function description (for future use)
   #[allow(dead_code)]
   pub fn infrastructure_function() {
       // Implementation for future integration
   }
   ```

3. **Prefix unused variables**: For variables intentionally unused in current implementation, prefix with `_`:
   ```rust
   fn handle_event(_key_modifiers: KeyModifiers, key_code: KeyCode) {
       // key_modifiers intentionally unused for now
   }
   ```

**❌ INCORRECT - Not categorizing infrastructure code**:

```rust
// Wrong: Leaving infrastructure code without allow attribute
pub struct DashboardUpdateBatch {
    // Warning: struct is never constructed
}
```

### **3. :: Systematic Warning Resolution Workflow**

**✅ CORRECT - Use systematic workflow for resolving multiple warnings**:

1. **Identify all warnings**: Run `cargo check`
2. **Categorize warnings**: Unused imports, unused structs, unused methods, etc.
3. **Apply fix pattern**:
   - **Unused imports** → Remove
   - **Unused infrastructure code** → `#[allow(dead_code)]` with "(for future use)" comment
   - **Unused variables** → Prefix with `_` or remove
4. **Verify resolution**: Run `cargo check` again

**✅ CORRECT - Batch processing pattern**:

```rust
// Step 1: Fix all unused imports
// Remove: use crate::unused_module;

// Step 2: Mark infrastructure code
#[allow(dead_code)] // For future use
pub struct InfrastructureStruct { }

// Step 3: Fix unused variables
fn handler(_unused_param: Type, used_param: Type) { }

// Step 4: Verify
// Run: cargo check
```

## Common Mistakes

### ❌ Warning Resolution Violations
- ❌ **Not Categorizing Infrastructure Code** - Don't leave infrastructure code without `#[allow(dead_code)]` and "(for future use)" comments
- ❌ **Removing Infrastructure Code** - Don't remove code that will be used in future features
- ❌ **Not Documenting Intent** - Don't use `#[allow(dead_code)]` without "(for future use)" comments
- ❌ **Fixing Warnings Randomly** - Don't fix warnings without systematic categorization
- ❌ **Not Verifying After Fixes** - Don't assume fixes work without running `cargo check`

## Checklist

- [ ] **Infrastructure Code Categorized**: All infrastructure code is marked with `#[allow(dead_code)]` and "(for future use)" comments
- [ ] **Truly Unused Code Removed**: Unused imports and truly unused code are removed
- [ ] **Unused Variables Prefixed**: Unused variables are prefixed with `_` or removed
- [ ] **Systematic Workflow**: Warnings are resolved using systematic categorization approach
- [ ] **Build Verified**: `cargo check` runs successfully with zero warnings
