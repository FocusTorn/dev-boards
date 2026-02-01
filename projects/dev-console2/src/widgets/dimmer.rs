use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
};

/// Muted grey for dimmed foregrounds.
pub const DIM_FG: Color = Color::Indexed(238);
/// Extremely dark grey for dimmed backgrounds.
pub const DIM_BG: Color = Color::Indexed(232);

/// Utility to desaturate and dim a buffer area.
/// Iterates over each cell in the specified area and mutes its colors.
pub fn apply_dimming(buf: &mut Buffer, area: Rect) {
    let intersection = area.intersection(buf.area);
    for y in intersection.top()..intersection.bottom() {
        for x in intersection.left()..intersection.right() {
            let cell = &mut buf[(x, y)];
            
            // Mute Foreground
            // If it's bright (Reset, White, or any colored text), force it to DIM_FG
            if cell.fg != Color::Black && cell.fg != DIM_BG {
                cell.set_fg(DIM_FG);
            }
            
            // Mute Background
            cell.set_bg(DIM_BG);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn test_apply_dimming() {
        let area = Rect::new(0, 0, 5, 5);
        let mut buf = Buffer::empty(area);
        
        // Setup some colored cells
        buf[(1, 1)].set_symbol("A").set_style(Style::default().fg(Color::Red).bg(Color::Blue));
        buf[(2, 2)].set_symbol("B").set_style(Style::default().fg(Color::Green).bg(Color::Yellow));
        
        // Apply dimming to a sub-section
        let dim_area = Rect::new(1, 1, 2, 2);
        apply_dimming(&mut buf, dim_area);
        
        // Verify dimmed cells
        assert_eq!(buf[(1, 1)].fg, DIM_FG);
        assert_eq!(buf[(1, 1)].bg, DIM_BG);
        assert_eq!(buf[(2, 2)].fg, DIM_FG);
        assert_eq!(buf[(2, 2)].bg, DIM_BG);
        
        // Verify non-dimmed cells remain unchanged (default is Reset)
        assert_eq!(buf[(0, 0)].fg, Color::Reset);
        assert_eq!(buf[(4, 4)].fg, Color::Reset);
    }
}