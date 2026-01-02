// Layout Manager - Centralized layout calculations with caching

use crate::layout_cache::LayoutCache;
use crate::layout_utils::calculate_centered_content_area;
use ratatui::layout::Rect;

/// Centralized layout manager
/// Provides cached layout calculations to eliminate duplication
pub struct LayoutManager {
    cache: LayoutCache,
}

impl LayoutManager {
    /// Create a new layout manager
    pub fn new() -> Self {
        Self {
            cache: LayoutCache::new(),
        }
    }
    
    /// Get content area with caching
    /// This is the single implementation used everywhere
    pub fn get_content_area(&mut self, content_rect: Rect) -> Option<Rect> {
        self.cache.get_content_area()
            .filter(|cached| {
                cached.width == content_rect.width && cached.height == content_rect.height
            })
            .or_else(|| {
                calculate_centered_content_area(content_rect).map(|area| {
                    self.cache.set_content_area(area);
                    area
                })
            })
    }
    
    /// Get the underlying cache (for advanced usage)
    #[allow(dead_code)]
    pub fn cache_mut(&mut self) -> &mut LayoutCache {
        &mut self.cache
    }
    
    /// Clear the cache (useful after terminal resize)
    #[allow(dead_code)]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}
