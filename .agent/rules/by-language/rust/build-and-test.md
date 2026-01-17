---
globs: **/*.rs

---
# Rust Build and Test Rules

## Build Protocol

1. **Dev Build Only**: Use `cargo build` (not `--release`) for all development tasks.
2. **Zero Warnings**: Fix ALL warnings immediately. Build is only successful if clean.

**AI Agent Directive**: Follow Rust build and test rules exactly for all Rust development tasks.

**MANDATORY EXECUTION PROTOCOL**:

1. **NO DEVIATION**: All Rust build rules must be followed exactly as written
2. **NO SKIPPING**: No steps may be skipped, abbreviated, or modified
3. **NO SELECTIVE COMPLIANCE**: All rules apply to all Rust development activities
4. **FAILURE TO COMPLY**: Violating these rules constitutes a critical protocol violation

## Build Commands

### **1. :: Dev Build Only**

**CRITICAL ENFORCEMENT**: The AI agent MUST use `cargo build` (NOT `cargo check`) for all Rust build verification. `cargo check` is NOT acceptable as a substitute for `cargo build` when verifying code changes. The agent MUST run `cargo build` after making any Rust code changes.

**✅ CORRECT - Use dev build for development**:

```bash
# Always use dev build for development
cargo build

# Run the application
cargo run
```

**❌ INCORRECT - Don't use release build during development**:

```bash
# Wrong: Don't use release build during development
cargo build --release
```

**❌ INCORRECT - Don't use cargo check instead of cargo build**:

```bash
# Wrong: Don't use cargo check for verification
cargo check  # This is NOT acceptable for build verification

# Correct: Always use cargo build
cargo build  # This is the required command
```

**Rationale**: 
- Dev builds are faster and provide better error messages
- Release builds are for final distribution only
- `cargo build` is the required command for verification; `cargo check` does not satisfy the build verification requirement

### **2. :: Error and Warning Resolution**

**CRITICAL ENFORCEMENT**: After making ANY Rust code changes, the AI agent MUST: (1) Run `cargo build`, (2) Fix ALL warnings immediately, (3) Run `cargo build` again to verify zero warnings, (4) Do NOT mark task complete until build succeeds with zero warnings.

**CRITICAL ENFORCEMENT**: After running `cargo build`, the AI agent MUST immediately fix ALL warnings before marking the task complete. Warnings are NOT acceptable even if the build succeeds. The build is only considered successful when it completes with zero warnings (or only intentionally allowed warnings with `#[allow(...)]` attributes).

**✅ CORRECT - Fix all errors and warnings**:

After running `cargo build`, the AI agent MUST:

1. **Fix all compilation errors** - No exceptions
2. **Fix all warnings IMMEDIATELY** - Warnings must be addressed before proceeding. Use one of these approaches:
   - Remove unused code if not needed
   - Use the code if it's intended to be used
   - Add `#[allow(dead_code)]` or `#[allow(unused)]` attributes only if code is intentionally kept for future use
   - Prefix unused variables with `_` (e.g., `_config` instead of `config`)

3. **Verify build succeeds with zero warnings** - Run `cargo build` again to confirm no warnings remain
4. **Task incomplete if warnings exist** - Do NOT mark task as complete if warnings are present in build output

**MANDATORY VERIFICATION CHECKPOINT**: After running `cargo build`, the AI agent MUST verify:
- "Are there any warnings in the build output?"
- "Have I fixed all warnings before proceeding?"

If warnings exist and are not fixed, the task is INCOMPLETE and the agent MUST NOT proceed.

**✅ CORRECT - Handling unused code**:

```rust
// Option 1: Remove if not needed
// struct UnusedStruct { ... }  // Remove this

// Option 2: Use if intended
let config = load_config();  // Actually use it

// Option 3: Prefix with underscore if intentionally unused
struct AppState {
    _config: Config,  // Intentionally unused for now
    // ...
}

// Option 4: Allow attribute for future use
#[allow(dead_code)]
struct GitStatus {
    // Will be used in future feature
}
```

**❌ INCORRECT - Ignoring warnings**:

```rust
// Wrong: Leaving warnings unfixed
struct AppState {
    config: Config,  // Warning: field is never read
    // ...
}

// Wrong: Using release build to hide warnings
cargo build --release  // Don't do this
```

### **3. :: Build Verification**

**✅ CORRECT - Verify build after fixes**:

```bash
# After fixing errors/warnings, verify
cargo build

# Should show: "Finished `dev` profile [unoptimized + debuginfo] target(s)"
# With no errors and no warnings (or only intentionally allowed warnings)
```

**❌ INCORRECT - Not verifying after fixes**:

```bash
# Wrong: Assuming fixes work without verification
# Always run cargo build after making changes
```


## Common Mistakes

- ❌ **Using Release Build** - Don't use `cargo build --release` during development
- ❌ **Ignoring Warnings** - Don't leave warnings unfixed
- ❌ **Not Verifying** - Don't skip verification after fixing errors/warnings

## Checklist

- [ ] **Dev Build Used**: Only `cargo build` (not `--release`) used for development
- [ ] **All Errors Fixed**: All compilation errors resolved
- [ ] **All Warnings Fixed**: All warnings addressed (removed, used, or intentionally allowed)
- [ ] **Build Verified**: `cargo build` runs successfully with no errors/warnings
