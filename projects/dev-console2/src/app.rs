use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::Color,
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};
use crate::widgets::tab_bar::{TabBarWidget, TabBarItem, TabBarStyle, TabBarAlignment};

/// Application state following Elm architecture
#[derive(Debug)]
pub struct App {
    pub running: bool,
    #[allow(dead_code)] // Future UI state
    pub ui_state: UiState,
    pub tabs: Vec<TabBarItem>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            ui_state: UiState::default(),
            tabs: vec![
                TabBarItem { name: "Dashboard".to_string(), active: true },
                TabBarItem { name: "Projects".to_string(), active: false },
                TabBarItem { name: "Settings".to_string(), active: false },
                TabBarItem { name: "Help".to_string(), active: false },
            ],
        }
    }
}

/// UI-specific state
#[derive(Debug, Default)]
#[allow(dead_code)] // Future UI state
pub struct UiState {
    // Future UI state can be added here
    // e.g., active_panel: Option<PanelId>,
    // selected_item: Option<usize>,
}

/// Application messages for state updates
#[derive(PartialEq, Debug)]
pub enum Message {
    Quit,
    // Future messages can be added
    // NavigateUp,
    // NavigateDown,
    // SelectItem,
}


impl App {
    /// Update application state based on messages
    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Quit => {
                self.running = false;
            }
            // Handle other messages
        }
        None // Return None unless chaining messages
    }
    
    /// Render the application UI
    pub fn view(&mut self, frame: &mut Frame) {
        // Create vertical layout with 5 sections
        let vertical_layout = Layout::vertical([
            Constraint::Length(3),  // Title Bar
            Constraint::Min(0),     // Main Content
            Constraint::Length(1),  // Bindings
            Constraint::Length(1),  // Divider
            Constraint::Length(1),  // Status Bar
        ]);
        
        let [title_area, main_area, bindings_area, divider_area, status_area] = 
            vertical_layout.areas(frame.area());
        
        // Render each component
        self.render_title_bar(frame, title_area);
        self.render_main_content(frame, main_area, Block::bordered());
        self.render_bindings(frame, bindings_area);
        self.render_divider(frame, divider_area);
        self.render_status_bar(frame, status_area);
    }
    
    /// Render the title bar component
    fn render_title_bar(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let block = Block::bordered()
            .title("Title Bar")
            .title_alignment(Alignment::Center);
        frame.render_widget(block, area);
    }
    
    /// Render main content area with custom TabBar widget
    fn render_main_content(&self, frame: &mut Frame, area: ratatui::layout::Rect, block: Block) {
        // First render the bounding box
        frame.render_widget(&block, area);
        
        // Create inner area for the tab bar (inside the bounding box)
        let inner_area = block.inner(area);
        
        // Create custom TabBar widget and configure it
        let tab_bar = TabBarWidget::new(&self.tabs)
            .style(TabBarStyle::Tab)
            .alignment(TabBarAlignment::Center)
            .color(Color::Cyan)
            .area(inner_area);
        
        // Render the custom TabBar widget inside the bounding box
        frame.render_widget(tab_bar, area);
    }
    
    /// Render the bindings display
    fn render_bindings(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let paragraph = Paragraph::new("Bindings")
            .alignment(Alignment::Left);
        frame.render_widget(paragraph, area);
    }
    
    /// Render the divider line
    fn render_divider(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let divider = Paragraph::new(Line::from(Span::raw("─".repeat(area.width as usize))));
        frame.render_widget(divider, area);
    }
    
    /// Render the status bar
    fn render_status_bar(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let paragraph = Paragraph::new("Status Bar")
            .alignment(Alignment::Left);
        frame.render_widget(paragraph, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_default_state() {
        let app = App::default();
        assert!(app.running);
    }
    
    #[test]
    fn test_quit_message() {
        let mut app = App::default();
        let result = app.update(Message::Quit);
        assert!(!app.running);
        assert!(result.is_none());
    }
    
    #[test]
    fn test_message_equality() {
        assert_eq!(Message::Quit, Message::Quit);
    }
}
