use ratatui::{
    buffer::Buffer,
    layout::{Rect, Position},
    style::{Color, Style},
    widgets::Widget,
};
use crossterm::event::{KeyEvent, MouseEvent, MouseButton, MouseEventKind};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollCommand {
    SetOffset(usize),
    ReachedBottom,
}

#[derive(Debug, Clone, Copy)]
pub enum ScrollEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

impl From<crossterm::event::Event> for ScrollEvent {
    fn from(event: crossterm::event::Event) -> Self {
        match event {
            crossterm::event::Event::Key(k) => Self::Key(k),
            crossterm::event::Event::Mouse(m) => Self::Mouse(m),
            crossterm::event::Event::Resize(w, h) => Self::Resize(w, h),
            _ => Self::Resize(0, 0),
        }
    }
}

impl From<MouseEvent> for ScrollEvent {
    fn from(m: MouseEvent) -> Self {
        Self::Mouse(m)
    }
}

#[derive(Debug, Default, Clone)]
pub struct ScrollBarInteraction {
    pub is_dragging: bool,
    pub drag_start_y: u16,
    pub drag_start_offset: usize,
    pub thumb_grab_offset_rows: f64, // Where on the thumb (in rows) the mouse is held
}

impl ScrollBarInteraction {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollLengths {
    pub content_len: usize,
    pub viewport_len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum ScrollBarArrows {
    None,
    Top,
    Bottom,
    Both,
}

pub struct ScrollBar {
    lengths: ScrollLengths,
    offset: usize,
    track_style: Style,
    thumb_style: Style,
    arrows: ScrollBarArrows,
}

impl ScrollBar {
    pub fn vertical(lengths: ScrollLengths) -> Self {
        Self {
            lengths,
            offset: 0,
            track_style: Style::default().bg(Color::Rgb(45, 45, 45)),
            thumb_style: Style::default().fg(Color::White).bg(Color::Rgb(45, 45, 45)),
            arrows: ScrollBarArrows::None,
        }
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn track_style(mut self, style: Style) -> Self {
        self.track_style = style;
        self
    }

    pub fn thumb_style(mut self, style: Style) -> Self {
        self.thumb_style = style;
        self
    }

    pub fn handle_event(
        &self,
        area: Rect,
        event: ScrollEvent,
        interaction: &mut ScrollBarInteraction,
    ) -> Option<ScrollCommand> {
        let mouse_event = match event {
            ScrollEvent::Mouse(m) => m,
            _ => return None,
        };

        if self.lengths.content_len <= self.lengths.viewport_len {
            return None;
        }

        let mouse_pos = Position::new(mouse_event.column, mouse_event.row);
        let height = area.height as f64;
        let max_offset = (self.lengths.content_len.saturating_sub(self.lengths.viewport_len)) as f64;
        
        // Proportional thumb height
        let thumb_height = (self.lengths.viewport_len as f64 / self.lengths.content_len as f64 * height).max(1.0);
        let travel_dist = height - thumb_height;

        match mouse_event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if area.contains(mouse_pos) {
                    let relative_y = (mouse_pos.y.saturating_sub(area.y)) as f64;
                    let current_thumb_top = (self.offset as f64 / max_offset) * travel_dist;
                    
                    // If clicking ON the thumb, start dragging without jumping
                    if relative_y >= current_thumb_top && relative_y <= current_thumb_top + thumb_height {
                        interaction.is_dragging = true;
                        interaction.drag_start_y = mouse_pos.y;
                        interaction.drag_start_offset = self.offset;
                        interaction.thumb_grab_offset_rows = relative_y - current_thumb_top;
                    } else {
                        // Clicked track: jump to center the thumb at mouse
                        let new_top = (relative_y - thumb_height / 2.0).clamp(0.0, travel_dist);
                        let new_offset = if travel_dist > 0.0 {
                            (new_top / travel_dist * max_offset) as usize
                        } else { 0 };
                        
                        interaction.is_dragging = true;
                        interaction.drag_start_y = mouse_pos.y;
                        interaction.drag_start_offset = new_offset;
                        interaction.thumb_grab_offset_rows = thumb_height / 2.0;
                        
                        if new_offset >= max_offset as usize {
                            return Some(ScrollCommand::ReachedBottom);
                        }
                        return Some(ScrollCommand::SetOffset(new_offset));
                    }
                }
            }
            MouseEventKind::Up(MouseButton::Left) => {
                interaction.is_dragging = false;
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if interaction.is_dragging && travel_dist > 0.0 {
                    let relative_y = (mouse_pos.y.saturating_sub(area.y)) as f64;
                    let new_top = (relative_y - interaction.thumb_grab_offset_rows).clamp(0.0, travel_dist);
                    let new_offset = (new_top / travel_dist * max_offset) as usize;
                    
                    if new_offset >= max_offset as usize {
                        return Some(ScrollCommand::ReachedBottom);
                    }
                    return Some(ScrollCommand::SetOffset(new_offset));
                }
            }
            MouseEventKind::ScrollUp => {
                return Some(ScrollCommand::SetOffset(self.offset.saturating_sub(3)));
            }
            MouseEventKind::ScrollDown => {
                let max_offset = self.lengths.content_len.saturating_sub(self.lengths.viewport_len);
                let next_offset = self.offset.saturating_add(3);
                if next_offset >= max_offset {
                    return Some(ScrollCommand::ReachedBottom);
                }
                return Some(ScrollCommand::SetOffset(next_offset));
            }
            _ => {}
        }
        None
    }
}

impl Widget for &ScrollBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 || self.lengths.content_len == 0 {
            return;
        }

        let track_bg = self.track_style.bg.unwrap_or(Color::Rgb(45, 45, 45));
        let thumb_fg = self.thumb_style.fg.unwrap_or(Color::White);

        // 1. Draw Background Track
        for y in area.top()..area.bottom() {
            buf[(area.x, y)].set_style(self.track_style).set_symbol(" ");
        }

        if self.lengths.content_len <= self.lengths.viewport_len {
            return;
        }

        // 2. Render Arrows if requested
        let mut scroll_area = area;
        match self.arrows {
            ScrollBarArrows::Top => {
                buf[(area.x, area.y)].set_symbol("↑").set_style(self.thumb_style);
                scroll_area.y += 1;
                scroll_area.height = scroll_area.height.saturating_sub(1);
            }
            ScrollBarArrows::Bottom => {
                buf[(area.x, area.bottom() - 1)].set_symbol("↓").set_style(self.thumb_style);
                scroll_area.height = scroll_area.height.saturating_sub(1);
            }
            ScrollBarArrows::Both => {
                buf[(area.x, area.y)].set_symbol("↑").set_style(self.thumb_style);
                buf[(area.x, area.bottom() - 1)].set_symbol("↓").set_style(self.thumb_style);
                scroll_area.y += 1;
                scroll_area.height = scroll_area.height.saturating_sub(2);
            }
            ScrollBarArrows::None => {}
        }

        if scroll_area.height == 0 { return; }

        // 3. High-precision thumb bounds (in sub-cells)
        let total_sub = (scroll_area.height * 8) as f64;
        let max_offset = (self.lengths.content_len.saturating_sub(self.lengths.viewport_len)) as f64;
        
        let scroll_ratio = if max_offset > 0.0 { self.offset as f64 / max_offset } else { 0.0 };
        let thumb_ratio = self.lengths.viewport_len as f64 / self.lengths.content_len as f64;
        
        let thumb_h_sub = (thumb_ratio * total_sub).max(8.0);
        let travel_sub = total_sub - thumb_h_sub;
        
        let start_sub = scroll_ratio * travel_sub;
        let end_sub = start_sub + thumb_h_sub;

        let blocks = [" ", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

        for y in 0..scroll_area.height {
            let cell_top = (y * 8) as f64;
            let cell_bottom = ((y + 1) * 8) as f64;
            
            let intersect_t = start_sub.max(cell_top);
            let intersect_b = end_sub.min(cell_bottom);
            
            if intersect_b > intersect_t {
                let cell = &mut buf[(scroll_area.x, scroll_area.y + y)];
                let h_filled = intersect_b - intersect_t;
                
                if h_filled >= 7.9 {
                    cell.set_symbol("█").set_style(self.thumb_style);
                } else {
                    let is_at_top = intersect_t > cell_top;
                    let is_at_bottom = intersect_b < cell_bottom;
                    
                    if is_at_top && !is_at_bottom {
                        let idx = (h_filled.round() as usize).min(7);
                        cell.set_symbol(blocks[idx]).set_style(self.thumb_style);
                    } else if !is_at_top && is_at_bottom {
                        let h_empty = 8.0 - h_filled;
                        let idx = (h_empty.round() as usize).min(7);
                        let inv_style = Style::default().fg(track_bg).bg(thumb_fg);
                        cell.set_symbol(blocks[idx]).set_style(inv_style);
                    } else {
                        let idx = (h_filled.round() as usize).min(7);
                        cell.set_symbol(blocks[idx]).set_style(self.thumb_style);
                    }
                }
            }
        }
    }
}
