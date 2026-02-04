use ratatui::layout::Rect;
use ratatui::widgets::{ListState, ScrollbarState};

#[derive(Debug)]
pub struct OverlayDropdown {
    pub title: String,
    pub items: Vec<String>,
    pub selected: usize,
    pub is_open: bool,
    pub max_shown: usize,
    pub scroll_state: ScrollbarState,
    pub list_state: ListState,
}

impl OverlayDropdown {
    pub fn new(title: String, items: Vec<String>, max_shown: usize) -> Self {
        let scroll_state = ScrollbarState::new(items.len().saturating_sub(max_shown));
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        
        Self {
            title,
            items,
            selected: 0,
            is_open: false,
            max_shown,
            scroll_state,
            list_state,
        }
    }

    pub fn calculate_layout(&self, anchor: Rect, terminal_height: u16) -> (Rect, bool) {
        let needed_height = self.items.len().min(self.max_shown) as u16;
        let space_below = terminal_height.saturating_sub(anchor.bottom());
        
        if space_below >= needed_height + 1 {
            let mut area = anchor;
            area.height = anchor.height + needed_height + 1;
            (area, true)
        } else {
            let mut area = anchor;
            area.y = anchor.y.saturating_sub(needed_height + 1);
            area.height = anchor.height + needed_height + 1;
            (area, false)
        }
    }
}
