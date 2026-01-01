// String interning module
// Reduces string allocations for repeated status messages

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// String interner for repeated status messages
pub struct StringInterner {
    interned: HashMap<String, Arc<str>>,
}

impl StringInterner {
    /// Create a new string interner
    pub fn new() -> Self {
        Self {
            interned: HashMap::new(),
        }
    }
    
    /// Intern a string, returning a reference-counted string slice
    /// If the string has been interned before, returns the existing Arc
    pub fn intern(&mut self, s: &str) -> Arc<str> {
        if let Some(interned) = self.interned.get(s) {
            interned.clone()
        } else {
            let arc: Arc<str> = Arc::from(s);
            self.interned.insert(s.to_string(), arc.clone());
            arc
        }
    }
    
    /// Clear all interned strings (useful for memory management)
    pub fn clear(&mut self) {
        self.interned.clear();
    }
    
    /// Get the number of interned strings
    pub fn len(&self) -> usize {
        self.interned.len()
    }
}

/// Global string interner (thread-safe)
lazy_static::lazy_static! {
    static ref GLOBAL_INTERNER: Mutex<StringInterner> = Mutex::new(StringInterner::new());
}

/// Intern a string using the global interner
pub fn intern_string(s: &str) -> Arc<str> {
    GLOBAL_INTERNER.lock().unwrap().intern(s)
}

/// Intern common status messages
pub mod common {
    use super::intern_string;
    use std::sync::Arc;
    
    lazy_static::lazy_static! {
        pub static ref READY: Arc<str> = intern_string("Ready");
        pub static ref RUNNING: Arc<str> = intern_string("Running");
        pub static ref COMPLETED: Arc<str> = intern_string("Completed");
        pub static ref FAILED: Arc<str> = intern_string("Failed");
        pub static ref ERROR: Arc<str> = intern_string("Error");
        pub static ref SUCCESS: Arc<str> = intern_string("Success");
    }
}
