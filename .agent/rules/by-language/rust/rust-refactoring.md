---
globs: **/*.rs

---

# Rust Refactoring Rules

## Verification

### **1. :: Syntax Verification During Large Refactorings**

**✅ CORRECT - Verify syntax after each structural change**:

When performing large refactorings that involve replacing code blocks or nested structures:

1. **Run cargo build after each structural change** to verify syntax before proceeding
2. **Verify brace matching** after each change by counting opening and closing braces
3. **Check code structure** around changed areas to identify mismatches early
4. **Fix syntax errors immediately** before making additional changes

**✅ CORRECT - Incremental verification pattern**:

```rust
// Step 1: Make structural change
if let Some(value) = get_value() {
    // Changed code block
}

// Step 2: Verify syntax immediately
// Run: cargo build

// Step 3: Only proceed if build succeeds
// Continue with next change
```

**❌ INCORRECT - Deferring syntax verification**:

```rust
// Wrong: Making multiple structural changes without verification
// Errors are harder to diagnose
```

### **2. :: Incremental Module Extraction Strategy**

**✅ CORRECT - Extract functionality incrementally**:

When refactoring large Rust files (like main.rs), extract functionality incrementally starting with simpler code paths before complex state-dependent logic:

```rust
// Step 1: Extract simple event handlers first (e.g., dashboard events, mouse scrolling)
// These have minimal state dependencies and are easier to extract

// Step 2: Progressively extract more complex logic (e.g., field editor state management)
// Verify borrowing checker compliance after each step using cargo build

// Step 3: Extract state-dependent logic using result enums for state transitions
// Use result enums to communicate state changes rather than requiring mutable borrows
```

**✅ CORRECT - Verification after each step**:

```rust
// After each extraction step:
// 1. Run cargo build to verify borrowing checker compliance
// 2. Fix any borrowing conflicts before proceeding
// 3. Only proceed to next extraction if build succeeds
```

**❌ INCORRECT - Extracting all functionality at once**:

```rust
// Wrong: Attempting to extract all event handling in single pass
// Leads to complex borrowing checker conflicts (E0499, E0502)
```

### **3. :: Enum-Based Type-Safe Refactoring**

**✅ CORRECT - Replace dynamic closures with enum-based accessors**:

When refactoring from dynamic closures (Box<dyn Fn>) to type-safe patterns, create an enum representing all possible field types and maintain backward compatibility:

```rust
// Enum for type-safe field access
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldId {
    FieldA = 0,
    FieldB = 1,
    // ...
}

impl FieldId {
    pub fn get_value(&self, data: &Data) -> String {
        match self {
            FieldId::FieldA => data.field_a.clone(),
            // ...
        }
    }
}
```

**✅ CORRECT - Maintain backward compatibility**:

1. Create new type-safe enum-based API
2. Provide backward-compatibility wrapper that uses enum internally
3. Convert indices/identifiers to enum variants in wrapper methods
4. Gradually migrate callers to new API

### **4. :: Verification Before Assumption**

**✅ CORRECT - Verify existing implementations before assuming work is needed**:

When refactoring tasks mention implementing optimizations or patterns, verify the codebase first:

1. Search codebase for existing patterns (lazy_static!, caching mechanisms, etc.)
2. Check if optimization is already implemented
3. If found, document as verified rather than reimplementing
4. If not found, proceed with implementation

**✅ CORRECT - Verification checklist**:
- Search for existing patterns (lazy_static!, caching, etc.)
- Check if optimization is complete
- Document verification results
```

## Common Mistakes

### ❌ Refactoring Violations
- ❌ **Deferring Syntax Verification** - Don't make multiple structural changes without verifying syntax
- ❌ **Assuming Brace Structure** - Don't assume replacing code blocks maintains correct brace structure
- ❌ **Not Counting Braces** - Don't skip brace matching verification after structural changes
- ❌ **Extracting All Functionality At Once** - Don't attempt to extract all functionality in a single pass
- ❌ **Breaking API Without Compatibility** - Don't remove existing APIs without providing backward-compatibility wrappers
- ❌ **Assuming Implementation Is Needed** - Don't implement optimizations without verifying they don't already exist

## Checklist

- [ ] **Syntax Verification**: cargo build is run after each structural change during refactoring
- [ ] **Brace Matching**: Brace structure is verified after code block replacements
- [ ] **Incremental Extraction**: Functionality is extracted incrementally with verification after each step
