---
globs: **/*.rs

---

# TUI Performance Rules

## Optimization Patterns

### **1. :: Layout Calculation Caching**

**✅ CORRECT - Cache expensive layout calculations with validity checks**:

When performing expensive layout calculations that are called frequently (e.g., on every render frame), implement caching with validity verification:

```rust
use std::collections::HashMap;
use ratatui::layout::Rect;

pub struct LayoutCache {
    content_area_cache: Option<Rect>,
    field_area_cache: HashMap<usize, Rect>,
    dropdown_area_cache: HashMap<usize, Rect>,
}

impl LayoutCache {
    pub fn get_content_area(&self) -> Option<Rect> {
        self.content_area_cache
    }
    
    pub fn set_content_area(&mut self, area: Rect) {
        self.content_area_cache = Some(area);
    }
    
    // Use with validity check
    pub fn get_valid_content_area(&self, current_rect: Rect) -> Option<Rect> {
        self.content_area_cache
            .filter(|cached| {
                // Verify cache is still valid (same dimensions)
                cached.width == current_rect.width && cached.height == current_rect.height
            })
    }
}
```

**✅ CORRECT - Verify cache validity before use**:

Always compare current parameters to cached parameters before using cached values:

```rust
// Before expensive calculation
if let Some(cached_area) = layout_cache.get_content_area()
    .filter(|cached| {
        // Verify cache is still valid
        cached.width == content_rect.width && cached.height == content_rect.height
    })
{
    // Use cached value
    use_layout(cached_area);
} else {
    // Recalculate and cache
    let calculated = expensive_calculation(content_rect);
    layout_cache.set_content_area(calculated);
    use_layout(calculated);
}
```

**✅ CORRECT - Cache invalidation on parameter change**:

Clear or update cache when parameters change:

```rust
impl LayoutCache {
    pub fn clear(&mut self) {
        self.content_area_cache = None;
        self.field_area_cache.clear();
        self.dropdown_area_cache.clear();
    }
    
    pub fn clear_field_cache(&mut self, field_index: usize) {
        self.field_area_cache.remove(&field_index);
        self.dropdown_area_cache.remove(&field_index);
    }
}
```

### **2. :: Cache Validity Criteria**

**✅ CORRECT - Cache validity checks**:

Cache validity should be based on the parameters that affect the calculation:

```rust
// For layout calculations, check dimensions
cached.width == current.width && cached.height == current.height

// For field-specific calculations, check field index and dimensions
cached.field_index == current.field_index && 
cached.dimensions == current.dimensions

// For dropdown calculations, check field index, options count, and frame size
cached.field_index == current.field_index &&
cached.options_count == current.options_count &&
cached.frame_height == current.frame_height
```

**✅ CORRECT - Granular cache invalidation**:

Invalidate only the specific cache entries that are affected by changes:

```rust
// Terminal resized - invalidate all dimension-based caches
layout_cache.clear();

// Field changed - invalidate only that field's cache
layout_cache.clear_field_cache(field_index);

// Options changed - invalidate dropdown cache for that field
layout_cache.clear_dropdown_cache(field_index);
```

**❌ INCORRECT - Overly broad validity checks**:

```rust
// Wrong: Checking parameters that don't affect the calculation
cached.timestamp == current.timestamp  // Timestamp doesn't affect layout
```

**❌ INCORRECT - Invalidating entire cache for small changes**:

```rust
// Wrong: Clearing all caches when only one field changed
layout_cache.clear();  // Should only clear field-specific cache
```


## Common Mistakes
- ❌ **Caching Without Validity Checks** - Don't use cached values without verifying they're still valid
- ❌ **Not Invalidating Cache** - Don't keep stale cache when parameters change
- ❌ **Caching Cheap Calculations** - Don't cache calculations that are inexpensive or rarely called
- ❌ **Overly Broad Validity Checks** - Don't check parameters that don't affect the calculation
- ❌ **Invalidating Entire Cache** - Don't clear all caches when only specific entries are affected

## Checklist

- [ ] **Cache Implementation**: Expensive calculations are cached with appropriate data structures
- [ ] **Validity Checks**: Cache validity is verified by comparing relevant parameters before use
- [ ] **Cache Invalidation**: Cache is cleared or updated when parameters change
- [ ] **Granular Invalidation**: Only affected cache entries are invalidated, not entire cache
- [ ] **Performance Benefit**: Caching provides measurable performance improvement (80%+ reduction in redundant calculations)
