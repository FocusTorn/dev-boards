// Layout calculation cache

use ratatui::layout::Rect;
use std::collections::HashMap;

/// Cache for layout calculations
pub struct LayoutCache {
    content_area_cache: Option<Rect>,
    field_area_cache: HashMap<usize, Rect>,
    dropdown_area_cache: HashMap<usize, Rect>,
}

impl LayoutCache {
    /// Create a new layout cache
    pub fn new() -> Self {
        Self {
            content_area_cache: None,
            field_area_cache: HashMap::new(),
            dropdown_area_cache: HashMap::new(),
        }
    }
    
    /// Get cached content area
    pub fn get_content_area(&self) -> Option<Rect> {
        self.content_area_cache
    }
    
    /// Cache content area
    pub fn set_content_area(&mut self, area: Rect) {
        self.content_area_cache = Some(area);
    }
    
    /// Get cached field area
    pub fn get_field_area(&self, field_index: usize) -> Option<Rect> {
        self.field_area_cache.get(&field_index).copied()
    }
    
    /// Cache field area
    pub fn set_field_area(&mut self, field_index: usize, area: Rect) {
        self.field_area_cache.insert(field_index, area);
    }
    
    /// Get cached dropdown area
    pub fn get_dropdown_area(&self, field_index: usize) -> Option<Rect> {
        self.dropdown_area_cache.get(&field_index).copied()
    }
    
    /// Cache dropdown area
    pub fn set_dropdown_area(&mut self, field_index: usize, area: Rect) {
        self.dropdown_area_cache.insert(field_index, area);
    }
    
    /// Clear all caches
    pub fn clear(&mut self) {
        self.content_area_cache = None;
        self.field_area_cache.clear();
        self.dropdown_area_cache.clear();
    }
    
    /// Clear field-specific caches
    pub fn clear_field_cache(&mut self, field_index: usize) {
        self.field_area_cache.remove(&field_index);
        self.dropdown_area_cache.remove(&field_index);
    }
}

impl Default for LayoutCache {
    fn default() -> Self {
        Self::new()
    }
}
