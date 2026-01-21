// use crate::config::{self, Config};
// use crate::widgets::tab_bar::{TabBarAlignment, TabBarItem, TabBarStyle, TabBarWidget};
// use ratatui::{
//     layout::{Alignment, Constraint, Layout, Rect},
//     style::Color,
//     text::{Line, Span},
//     widgets::{Block, Borders, Paragraph},
//     Frame,
// };

// #[derive(Debug)]
// pub struct App {
//     pub running: bool,
//     #[allow(dead_code)] // Future UI state
//     pub ui_state: UiState,
//     pub tabs: Vec<TabBarItem>,
//     pub config: Config,
// }

// impl Default for App {
//     fn default() -> Self {
//         let config = config::load_config().unwrap();
//         let tabs = if let Some(main_tabs_config) =
//             config.tab_bars.iter().find(|t| t.id == "MainContentTabBar")
//         {
//             main_tabs_config
//                 .tabs
//                 .iter()
//                 .map(|t| TabBarItem {
//                     name: t.name.clone(),
//                     active: t.default == Some("active".to_string()),
//                 })
//                 .collect()
//         } else {
//             vec![]
//         };

//         Self {
//             running: true,
//             ui_state: UiState::default(),
//             tabs,
//             config,
//         }
//     }
// }

// /// UI-specific state
// #[derive(Debug, Default)]
// #[allow(dead_code)] // Future UI state
// pub struct UiState {
//     //>
//     // Future UI state can be added here
//     // e.g., active_panel: Option<PanelId>,
//     // selected_item: Option<usize>,
// } //<

// /// Application messages for state updates
// #[derive(PartialEq, Debug)]
// pub enum Message {
//     //>
//     Quit,
//     // Future messages can be added
//     // NavigateUp,
//     // NavigateDown,
//     // SelectItem,
// } //<

// impl App {
//     /// Update application state based on messages
//     pub fn update(&mut self, msg: Message) -> Option<Message> {
//         //>
//         match msg {
//             Message::Quit => {
//                 self.running = false;
//             } // Handle other messages
//         }
//         None // Return None unless chaining messages
//     } //<

//     /// Render the application UI
//     pub fn view(&mut self, frame: &mut Frame) {
//         //>
//         // Create vertical layout with 5 sections
//         let vertical_layout = Layout::vertical([
//             Constraint::Length(3), // Title Bar
//             Constraint::Min(0),    // Main Content
//             Constraint::Length(1), // Bindings
//             Constraint::Length(1), // Divider
//             Constraint::Length(1), // Status Bar
//         ]);

//         let [title_area, main_area, bindings_area, divider_area, status_area] =
//             vertical_layout.areas(frame.area());

//         // Render each component
//         self.render_title_bar(frame, title_area);
//         self.render_main_content(frame, main_area, Block::bordered());
//         self.render_bindings(frame, bindings_area);
//         self.render_divider(frame, divider_area);
//         self.render_status_bar(frame, status_area);
//     } //<

//     /// Render the title bar component
//     fn render_title_bar(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
//         //>
//         let block = Block::bordered()
//             .title("Title Bar")
//             .title_alignment(Alignment::Center);
//         frame.render_widget(block, area);
//     } //<

//    fn render_main_content(&self, frame: &mut Frame, area: Rect, _block: Block) {
//         // 1. Get widget and alignment in one step
//         if let Some((tab_bar, alignment)) = TabBarWidget::from_config(&self.config, &self.tabs, "MainContentTabBar") {
            
//             // 2. Handle the layout (the split automatically accounts for 1 or 2 lines)
//             let [header_area, body_area] = tab_bar.split_layout(area);

//             // 3. Render body and aligned tab bar
//             frame.render_widget(Block::bordered(), body_area);
//             tab_bar.render_aligned(header_area, alignment, frame.buffer_mut());
//         } else {
//             // Fallback if no tab config found
//             frame.render_widget(Block::bordered(), area);
//         }
//     }



    
    
//     /// Render the bindings display
//     fn render_bindings(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
//         //>
//         let paragraph = Paragraph::new("Bindings").alignment(Alignment::Left);
//         frame.render_widget(paragraph, area);
//     } //<

//     /// Render the divider line
//     fn render_divider(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
//         //>
//         let divider = Paragraph::new(Line::from(Span::raw("─".repeat(area.width as usize))));
//         frame.render_widget(divider, area);
//     } //<

//     /// Render the status bar
//     fn render_status_bar(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
//         //>
//         let paragraph = Paragraph::new("Status Bar").alignment(Alignment::Left);
//         frame.render_widget(paragraph, area);
//     } //<
// }






// #[cfg(test)]
// mod tests {
//     //>
//     use super::*;

//     #[test]
//     fn test_app_default_state() {
//         let app = App::default();
//         assert!(app.running);
//     }

//     #[test]
//     fn test_quit_message() {
//         let mut app = App::default();
//         let result = app.update(Message::Quit);
//         assert!(!app.running);
//         assert!(result.is_none());
//     }

//     #[test]
//     fn test_message_equality() {
//         assert_eq!(Message::Quit, Message::Quit);
//     }
// } //<







//     /// Render main content area with custom TabBar widget
//     // fn render_main_content(&self, frame: &mut Frame, area: Rect, _block: Block) { //>
//     //     // 1. Determine style and height first
//     //     let tab_config = self
//     //         .config
//     //         .tab_bars
//     //         .iter()
//     //         .find(|t| t.id == "MainContentTabBar");

//     //     let style = tab_config
//     //         .and_then(|c| c.style.as_deref())
//     //         .map(|s| match s {
//     //             "tabbed" => TabBarStyle::Tab,
//     //             "boxed" => TabBarStyle::Boxed,
//     //             _ => TabBarStyle::Text,
//     //         })
//     //         .unwrap_or(TabBarStyle::Text);

//     //     // Height is 2 for "Tab" style (label + bottom connector), 1 for others
//     //     let header_height = if style == TabBarStyle::Tab { 2 } else { 1 };

//     //     // 2. Create a layout with Overlap(1) so the TabBar's bottom row
//     //     // is the Parent's top border.
//     //     let layout = Layout::vertical([Constraint::Length(header_height), Constraint::Min(0)])
//     //         .spacing(ratatui::layout::Spacing::Overlap(1));

//     //     let [header_area, body_area] = layout.areas(area);

//     //     // 3. Render the Body (the "Parent")
//     //     // We use MergeStrategy to ensure the top border respects the tabs above it
//     //     let body_block =
//     //         Block::bordered().merge_borders(ratatui::symbols::merge::MergeStrategy::Exact);
//     //     frame.render_widget(body_block, body_area);

//     //     // 4. Render the TabBar in the header area
//     //     if let Some(tab_config) = tab_config {
//     //         let color = match tab_config.color.as_deref() {
//     //             Some("cyan") => Color::Cyan,
//     //             _ => Color::White,
//     //         };

//     //         let tab_bar = TabBarWidget::new(&self.tabs).style(style).color(color);

//     //         // Calculate alignment within the header_area
//     //         let tab_width = tab_bar.estimate_width();
//     //         let alignment = match tab_config.alignment.horizontal.as_deref() {
//     //             Some("center") => TabBarAlignment::Center,
//     //             Some("right") => TabBarAlignment::Right,
//     //             _ => TabBarAlignment::Left,
//     //         };

//     //         let start_x = match alignment {
//     //             TabBarAlignment::Left => header_area.x + 1,
//     //             TabBarAlignment::Center => {
//     //                 header_area.x + (header_area.width.saturating_sub(tab_width)) / 2
//     //             }
//     //             TabBarAlignment::Right => {
//     //                 header_area.x + header_area.width.saturating_sub(tab_width) - 1
//     //             }
//     //         };

//     //         let final_tab_area = Rect {
//     //             x: start_x,
//     //             y: header_area.y,
//     //             width: tab_width,
//     //             height: header_height,
//     //         };

//     //         frame.render_widget(tab_bar, final_tab_area);
//     //     }
//     // } //<

//     // fn render_main_content(&self, frame: &mut Frame, area: Rect, _block: Block) { //>
       
       
//     //     let tab_config = self
//     //         .config
//     //         .tab_bars
//     //         .iter()
//     //         .find(|t| t.id == "MainContentTabBar");

            
            
            
//     //     // 1. Setup the Widget from config (Simplified Mapping)
//     //     let style = match tab_config.and_then(|c| c.style.as_deref()) {
//     //         Some("tabbed") => TabBarStyle::Tab,
//     //         Some("boxed") => TabBarStyle::Boxed,
//     //         _ => TabBarStyle::Text,
//     //     };

//     //     let tab_bar = TabBarWidget::new(&self.tabs)
//     //         .style(style)
//     //         .color(Color::Cyan); // Or map from config

            
            
            
//     //     // 2. Handle Layout (The "Padding" logic)
//     //     let [header_area, body_area] = Layout::vertical([
//     //         Constraint::Length(tab_bar.desired_height()),
//     //         Constraint::Min(0),
//     //     ])
//     //     .spacing(ratatui::layout::Spacing::Overlap(1))
//     //     .areas(area);

//     //     // 3. Render
//     //     frame.render_widget(Block::bordered(), body_area);

//     //     if let Some(config) = tab_config {
//     //         let alignment = match config.alignment.horizontal.as_deref() {
//     //             Some("center") => TabBarAlignment::Center,
//     //             Some("right") => TabBarAlignment::Right,
//     //             _ => TabBarAlignment::Left,
//     //         };

//     //         // Use our new helper that handles X-offset and Rect creation
//     //         tab_bar.render_aligned(header_area, alignment, frame.buffer_mut());
//     //     }
//     // } //<


    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
use crate::config::{Config};
use crate::widgets::tab_bar::{TabBarItem, TabBarWidget};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    symbols::merge::MergeStrategy, // Added for clean border joins
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub ui_state: UiState,
    pub tabs: Vec<TabBarItem>,
    pub config: Config,
}

impl Default for App {
    fn default() -> Self {
        let config = crate::config::load_config().unwrap();
        let tabs = config.tab_bars.iter()
            .find(|t| t.id == "MainContentTabBar")
            .map(|c| c.tabs.iter().map(|t| TabBarItem {
                name: t.name.clone(),
                active: t.default == Some("active".to_string()),
            }).collect())
            .unwrap_or_default();

        Self {
            running: true,
            ui_state: UiState::default(),
            tabs,
            config,
        }
    }
}

#[derive(Debug, Default)]
pub struct UiState {}

#[derive(PartialEq, Debug)]
pub enum Message {
    Quit,
}

impl App {
    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Quit => self.running = false,
        }
        None
    }

    pub fn view(&mut self, frame: &mut Frame) {
        let vertical_layout = Layout::vertical([
            Constraint::Length(3), // Title Bar
            Constraint::Min(0),    // Main Content
            Constraint::Length(1), // Bindings
            Constraint::Length(1), // Divider
            Constraint::Length(1), // Status Bar
        ]);

        let [title_area, main_area, bindings_area, divider_area, status_area] =
            vertical_layout.areas(frame.area());

        self.render_title_bar(frame, title_area);
        self.render_main_content(frame, main_area);
        self.render_bindings(frame, bindings_area);
        self.render_divider(frame, divider_area);
        self.render_status_bar(frame, status_area);
    }

    fn render_title_bar(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .title("Title Bar")
            .title_alignment(Alignment::Center);
        frame.render_widget(block, area);
    }

    
    
    
    
    
    /// Renders a block and automatically attaches one or more tab bars to its borders.
    /// The first ID in `tab_ids` determines the primary top-border layout.
    fn render_tabbed_block(&self, frame: &mut Frame, area: Rect, tab_ids: &[&str]) {
        let mut current_body_area = area;
        let mut active_widgets = Vec::new();

        // 1. Calculate layouts for all requested tab bars
        for id in tab_ids {
            if let Some((tab_bar, alignment)) = TabBarWidget::from_config(&self.config, &self.tabs, id) {
                // Determine how this specific tab bar affects the block's inner area
                let [header, body] = tab_bar.split_layout(current_body_area);
                
                // Update the area for the NEXT widget or the final block
                current_body_area = body;
                active_widgets.push((tab_bar, alignment, header));
            }
        }

        // 2. Render the actual Block (The "Parent")
        // Use MergeStrategy so all attached tab bars join seamlessly
        let block = Block::bordered().merge_borders(MergeStrategy::Exact);
        frame.render_widget(block, current_body_area);

        // 3. Render all tab bar decals on top of the block's borders
        for (widget, alignment, header_area) in active_widgets {
            widget.render_aligned(header_area, alignment, frame.buffer_mut());
        }
    }
    
    
    
    
    
    
    
    
    // fn render_main_content(&self, frame: &mut Frame, area: Rect) {
       
       
       
       
    //     // 1. Logic is now encapsulated in tab_bar.rs
    //     if let Some((tab_bar, alignment)) = TabBarWidget::from_config(&self.config, &self.tabs, "MainContentTabBar") {
            
    //         // 2. The split automatically handles 1-line vs 2-line styles
    //         let [header_area, body_area] = tab_bar.split_layout(area);

    //         // 3. Render body with MergeStrategy so the tabs connect to the border
    //         let body_block = Block::bordered()
    //             .merge_borders(MergeStrategy::Exact);
            
    //         frame.render_widget(body_block, body_area);
            
    //         // 4. Render the aligned tabs on top of the border
    //         tab_bar.render_aligned(header_area, alignment, frame.buffer_mut());
    //     } else {
    //         frame.render_widget(Block::bordered(), area);
    //     }
    
    
    
    // }

    fn render_main_content(&self, frame: &mut Frame, area: Rect) {
        
        
        self.render_tabbed_block(frame, area, &["MainContentTabBar", "MainContentStaticTabBar"]);
        
        
    }

    
        

        
        
    
    
    
    
    
    fn render_bindings(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Paragraph::new("Bindings"), area);
    }

    fn render_divider(&self, frame: &mut Frame, area: Rect) {
        let divider = Paragraph::new(Line::from(Span::raw("─".repeat(area.width as usize))));
        frame.render_widget(divider, area);
    }

    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Paragraph::new("Status Bar"), area);
    }
}
    