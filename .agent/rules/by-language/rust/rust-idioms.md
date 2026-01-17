---
globs: **/*.rs

---

# Rust Idioms & Coding Patterns

## Type Conversion Patterns

### **1. :: Library API Type Conversion**

**✅ CORRECT - Convert library wrapper types explicitly**:

When using library APIs that return wrapper types (like `RectMetrics`), explicitly convert to the expected type (like `Rect`) before passing to functions:

```rust
// Library returns Option<RectMetrics>
if let Some(content_rect) = box_manager.metrics(&registry) {
    // Convert to expected type explicitly
    let content_rect: Rect = content_rect.into();
    
    // Now can use with functions expecting Rect
    calculate_layout(content_rect);
}
```

**✅ CORRECT - Check for conversion methods**:

When encountering type mismatch errors with library API return types:
1. Check if the library provides a conversion method (`.into()`, `From` trait, or explicit conversion)
2. Convert wrapper types to their expected types explicitly before use
3. Verify the conversion is available before assuming types are compatible

**❌ INCORRECT - Assuming wrapper types are compatible**:

```rust
// Wrong: Assuming RectMetrics and Rect are compatible
if let Some(content_rect) = box_manager.metrics(&registry) {
    // Error: type mismatch - RectMetrics vs Rect
    calculate_layout(content_rect);  // Won't compile
}
```

### **2. :: Option Handling Patterns**

**✅ CORRECT - Use or_else() for Option chaining**:

When chaining Option operations where the fallback also returns an Option, use `or_else()`:

```rust
let result = cached_value
    .filter(|cached| cached.is_valid())
    .or_else(|| {
        // Fallback returns Option<T>
        calculate_value().map(|value| {
            cache.set(value);
            value
        })
    });
```

**✅ CORRECT - Use unwrap_or_else() for direct values**:

When the fallback returns the inner type directly, use `unwrap_or_else()`:

```rust
let value = option_value.unwrap_or_else(|| {
    // Fallback returns T directly, not Option<T>
    default_value()
});
```

**❌ INCORRECT - Using unwrap_or_else() with Option-returning closures**:

```rust
// Wrong: unwrap_or_else() expects closure returning T, not Option<T>
let result = option_value.unwrap_or_else(|| {
    if condition {
        Some(value)
    } else {
        return;  // Type error: can't return () when expecting T
    }
});
```

## Process Management

### **3. :: Native Rust Process Management**

**✅ CORRECT - Store Child handles directly**:

When managing child processes in Rust, store `Child` handles (or tuples of PID and Child) in process management structures:

```rust
use std::process::{Child, Command};

pub struct ProcessManager {
    processes: Arc<Mutex<Vec<(u32, Child)>>>,  // Store (PID, Child) tuples
}

impl ProcessManager {
    pub fn register(&self, child: Child) {
        if let Ok(mut processes) = self.processes.lock() {
            let pid = child.id();
            processes.push((pid, child));
        }
    }
    
    pub fn cleanup(&self) {
        let mut processes_guard = self.processes.lock().unwrap();
        for (_pid, mut child) in processes_guard.drain(..) {
            let _ = child.kill();  // Use native Rust kill
        }
    }
}
```

**✅ CORRECT - Use Child::kill() for termination**:

Use `Child::kill()` for process termination instead of platform-specific external commands:

```rust
pub fn cleanup(&self) {
    for (_pid, mut child) in self.processes.drain(..) {
        let _ = child.kill();  // Native Rust API
    }
}
```

**❌ INCORRECT - Storing only PIDs**:

```rust
// Wrong: Storing only PIDs requires external commands
pub struct ProcessManager {
    processes: Arc<Mutex<Vec<u32>>>,  // Only PIDs
}
```

**❌ INCORRECT - Using external commands for termination**:

```rust
// Wrong: Using external commands instead of native Rust APIs
Command::new("kill").arg(pid.to_string()).output();  // Less reliable, platform-specific
```

## Common Mistakes

### ❌ Type Conversion Violations
- ❌ **Assuming Wrapper Types Are Compatible** - Don't assume library wrapper types can be used directly
- ❌ **Skipping Type Conversion** - Don't skip explicit conversion when types don't match

### ❌ Option Handling Violations
- ❌ **Wrong Method for Option Chaining** - Don't use `unwrap_or_else()` when fallback returns Option
- ❌ **Wrong Method for Direct Values** - Don't use `or_else()` when fallback returns inner type directly

### ❌ Process Management Violations
- ❌ **Storing Only PIDs** - Don't store only PIDs when Child handles are available
- ❌ **Using External Commands** - Don't use external commands when native Rust APIs are available

## Checklist

- [ ] **Type Conversion**: Library wrapper types are explicitly converted to expected types
- [ ] **Option Handling**: Correct method (or_else vs unwrap_or_else) is used based on return type
- [ ] **Process Management**: Child handles are stored directly and Child::kill() is used for termination
- [ ] **Native APIs Preferred**: Native Rust APIs are used instead of external commands when available
