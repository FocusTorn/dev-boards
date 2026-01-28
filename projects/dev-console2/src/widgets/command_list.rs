use ratatui::{
    buffer::Buffer,
    layout::{Rect, Position},
    style::{Color, Style},
    text::{Span},
    widgets::{Block, Widget},
};
use crossterm::event::{MouseEvent, MouseButton, MouseEventKind, KeyModifiers};

/// Semantic result of a mouse interaction with the command list.
pub enum CommandListInteraction {
    Click(usize),
    Hover(usize),
}

/// A sidebar widget for selecting and executing development commands.
///>
/// Displays a vertical list of available actions (Compile, Upload, etc.) 
/// with integrated mouse support for hover-based highlighting and 
/// click-to-execute logic.
///<
pub struct CommandListWidget<'a> {
    commands: &'a [String],
    selected_index: usize,
    hovered_index: Option<usize>,
    border_style: Style,
    title_style: Style,
    highlight_style: Style,
}

impl<'a> CommandListWidget<'a> {
    /// Creates a new command list with initial selection and hover state.
    pub fn new(commands: &'a [String], selected_index: usize, hovered_index: Option<usize>) -> Self {
        Self { 
            commands, 
            selected_index, 
            hovered_index,
            border_style: Style::default(),
            title_style: Style::default(),
            highlight_style: Style::default().fg(Color::Cyan).bg(Color::Rgb(0, 40, 40)),
        }
    }

    /// Sets the style for the widget's surrounding block borders.
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    /// Sets the style for the "Commands" title text.
    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }

    /// Sets the style used for selected or hovered command entries.
    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    /// Identifies if a mouse event occurred over a specific command entry.
    ///>
    /// Maps physical screen coordinates to zero-based command indices and 
    /// returns semantic interactions (Click or Hover) based on the mouse button 
    /// and movement state.
    ///<
    pub fn handle_mouse_event(&self, area: Rect, mouse_event: MouseEvent) -> Option<CommandListInteraction> {
        let mouse_pos = Position::new(mouse_event.column, mouse_event.row);
        
        if !area.contains(mouse_pos) {
            return None;
        }

        let inner_area = Block::bordered().inner(area);
        let rel_y = mouse_event.row.saturating_sub(inner_area.y) as usize;
        
        if rel_y < self.commands.len() && inner_area.contains(mouse_pos) { //>
            match mouse_event.kind {
                MouseEventKind::Down(MouseButton::Left) => Some(CommandListInteraction::Click(rel_y)),
                MouseEventKind::Moved | MouseEventKind::Drag(MouseButton::Left) => Some(CommandListInteraction::Hover(rel_y)),
                _ => None,
            }
        } else {
            None
        } //< 
    }
}

impl<'a> Widget for CommandListWidget<'a> {
    /// Renders the command list with appropriate highlighting.
    ///>
    /// Prioritizes hover highlighting over the actual selection index to 
    /// provide visual feedback during mouse navigation.
    ///<
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Span::styled(" Commands ", self.title_style))
            .border_style(self.border_style);
        let inner_area = block.inner(area);
        block.render(area, buf);

        for (idx, cmd) in self.commands.iter().enumerate() { //>
            if idx >= inner_area.height as usize { break; }
            
            let item_y = inner_area.y + idx as u16;
            let item_area = Rect::new(inner_area.x, item_y, inner_area.width, 1);
            
            let is_selected = idx == self.selected_index;
            let is_hovered = self.hovered_index == Some(idx);
            
            let is_highlighted = if self.hovered_index.is_some() { //>
                is_hovered
            } else {
                is_selected
            }; //< 

            let style = if is_highlighted { //>
                self.highlight_style
            } else {
                Style::default().fg(Color::DarkGray)
            }; //< 

            if is_highlighted { //>
                // Fill background border to border
                for x in inner_area.left()..inner_area.right() {
                    buf[(x, item_y)].set_style(style);
                }
            } //< 

            buf.set_string(item_area.x + 1, item_y, format!(" {}", cmd), style);
        } //< 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn buffer_to_string(buffer: &Buffer) -> String {
        let mut result = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                result.push_str(&buffer[(x, y)].symbol());
            }
            result.push('\n');
        }
        result
    }

    #[test]
    fn test_command_list_render() {
        let backend = TestBackend::new(20, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        let commands = vec!["C1".to_string(), "C2".to_string()];
        
        terminal.draw(|f| {
            let widget = CommandListWidget::new(&commands, 0, None);
            f.render_widget(widget, f.area());
        }).unwrap();

        let buffer = terminal.backend().buffer();
        let s = buffer_to_string(buffer);
        assert!(s.contains("Commands"));
        assert!(s.contains("C1"));
        assert!(s.contains("C2"));
    }

    #[test]
    fn test_command_list_interaction() {
        let commands = vec!["C1".to_string()];
        let widget = CommandListWidget::new(&commands, 0, None);
        let area = Rect::new(0, 0, 20, 5);
        
        // Hover
        let event = MouseEvent {
            kind: MouseEventKind::Moved,
            column: 5,
            row: 1,
            modifiers: KeyModifiers::empty(),
        };
        let interaction = widget.handle_mouse_event(area, event).unwrap();
        if let CommandListInteraction::Hover(idx) = interaction {
            assert_eq!(idx, 0);
        } else { panic!(); } //>

        // Click
        let event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 5,
            row: 1,
            modifiers: KeyModifiers::empty(),
        };
        let interaction = widget.handle_mouse_event(area, event).unwrap();
        if let CommandListInteraction::Click(idx) = interaction {
            assert_eq!(idx, 0);
        } else { panic!(); }
    }

    #[test]
    fn test_command_list_highlighting() {
        let backend = TestBackend::new(20, 5);
        let mut terminal = Terminal::new(backend).unwrap();
        let commands = vec!["C1".to_string()];
        
        // Selected, not hovered
        terminal.draw(|f| {
            let widget = CommandListWidget::new(&commands, 0, None);
            f.render_widget(widget, f.area());
        }).unwrap();

        // Not selected, hovered
        terminal.draw(|f| {
            let widget = CommandListWidget::new(&commands, 1, Some(0));
            f.render_widget(widget, f.area());
        }).unwrap();
    }
}
