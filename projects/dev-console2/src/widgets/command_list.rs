/// Sidebar command list with mouse interactivity.
///>
/// This widget renders the list of available build and management actions. 
/// It supports both keyboard selection and mouse-based hover/click 
/// interactions, providing a responsive sidebar experience.
///<
use ratatui::{
    buffer::Buffer,
    layout::{Rect, Position},
    style::{Color, Style},
    text::{Span},
    widgets::{Block, Widget},
};
use crossterm::event::{MouseEvent, MouseButton, MouseEventKind};

/// A stateful widget representing the list of executable commands.
pub struct CommandListWidget<'a> {
    commands: &'a [String],
    selected_index: usize,
    hovered_index: Option<usize>,
    border_style: Style,
    title_style: Style,
    highlight_style: Style,
}

impl<'a> CommandListWidget<'a> {
    /// Creates a new CommandList with provided selection and hover states.
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

    /// Sets the visual style for the component borders.
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    /// Sets the visual style for the "Commands" title text.
    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }

    /// Sets the background and text style for the active/hovered item.
    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    /// Detects and returns user interactions with the sidebar.
///>
    /// This follows the 'Widget Interactivity Delegation' mandate by 
    /// encapsulating hit-box detection and coordinate mapping within 
    /// the widget itself.
    ///<
    pub fn handle_mouse_event(&self, area: Rect, mouse_event: MouseEvent) -> Option<CommandListInteraction> {
        let mouse_pos = Position::new(mouse_event.column, mouse_event.row);
        
        if !area.contains(mouse_pos) {
            return None;
        }

        let inner_area = Block::bordered().inner(area);
        let rel_y = mouse_event.row.saturating_sub(inner_area.y) as usize;
        
        if rel_y < self.commands.len() && inner_area.contains(mouse_pos) {
            match mouse_event.kind {
                MouseEventKind::Down(MouseButton::Left) => Some(CommandListInteraction::Click(rel_y)),
                MouseEventKind::Moved | MouseEventKind::Drag(MouseButton::Left) => Some(CommandListInteraction::Hover(rel_y)),
                _ => None,
            }
        } else {
            None
        }
    }
}

/// Result of a mouse interaction with the command list.
pub enum CommandListInteraction {
    /// User clicked a specific command.
    Click(usize),
    /// User moved the mouse over a specific command.
    Hover(usize),
}

impl<'a> Widget for CommandListWidget<'a> {
    /// Renders the sidebar into the terminal buffer.
///>
    /// Handles the drawing of the bordered block, the command strings, 
    /// and the full-width background highlight for the active item.
    ///<
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Span::styled(" Commands ", self.title_style))
            .border_style(self.border_style);
        let inner_area = block.inner(area);
        block.render(area, buf);

        for (idx, cmd) in self.commands.iter().enumerate() {
            if idx >= inner_area.height as usize { break; }
            
            let item_y = inner_area.y + idx as u16;
            let item_area = Rect::new(inner_area.x, item_y, inner_area.width, 1);
            
            let is_selected = idx == self.selected_index;
            let is_hovered = self.hovered_index == Some(idx);
            
            // Mouse hover takes precedence over keyboard selection for highlighting
            let is_highlighted = if self.hovered_index.is_some() {
                is_hovered
            } else {
                is_selected
            };

            let style = if is_highlighted {
                self.highlight_style
            } else {
                Style::default().fg(Color::DarkGray)
            };

            if is_highlighted {
                // Fill the entire row background for a modern 'row selector' look
                for x in inner_area.left()..inner_area.right() {
                    buf[(x, item_y)].set_style(style);
                }
            }

            buf.set_string(item_area.x + 1, item_y, format!(" {}", cmd), style);
        }
    }
}