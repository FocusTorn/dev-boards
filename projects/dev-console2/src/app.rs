
    
    
use crate::config::{Config};
use crate::widgets::tab_bar::{TabBarItem, TabBarWidget};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect, Margin},
    text::{Line, Span},
    widgets::{Block, Paragraph, Clear},
    Frame,
    style::{Color, Style},
};



#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub tabs: Vec<TabBarItem>,
    pub config: Config,
    pub terminal_too_small: bool,
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
            tabs,
            config,
            terminal_too_small: false,
        }
    }
}

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

    pub fn check_terminal_size(&mut self, area: Rect) {
        self.terminal_too_small = area.width < self.config.application.min_width || 
                                 area.height < self.config.application.min_height;
    }

    pub fn view(&mut self, frame: &mut Frame) {
        // Check terminal size first
        self.check_terminal_size(frame.area());
        
        if self.terminal_too_small {
            self.render_terminal_too_small(frame);
            return;
        }

        let vertical_layout = Layout::vertical([
            Constraint::Length(3), // Title Bar
            Constraint::Min(0),    // Main Content
            Constraint::Length(1), // Bindings
            Constraint::Length(2), // Status Bar
        ]);

        let [title_area, main_area, bindings_area, status_area] =
            vertical_layout.areas(frame.area());

        self.render_title_bar(frame, title_area);
        self.render_main_content(frame, main_area);
        self.render_bindings(frame, bindings_area);
        self.render_status_bar(frame, status_area);
    }

    fn render_title_bar(&self, frame: &mut Frame, area: Rect) {
        // Create bordered block
        let block = Block::bordered();
        frame.render_widget(block, area);
        
        // Get the inner area inside the border
        let inner_area = area.inner(Margin::new(0, 0));
        
        // Render centered title with white text inside the border
        let title = Paragraph::new(self.config.application.title.clone())
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));
        
        frame.render_widget(title, inner_area);
    }

    fn render_terminal_too_small(&self, frame: &mut Frame) {
        // Clear the entire area
        frame.render_widget(Clear, frame.area());
        
        let message = format!(
            "Terminal Too Small\nRequired: {}x{}\nCurrent: {}x{}\n\nPress 'q' to quit",
            self.config.application.min_width,
            self.config.application.min_height,
            frame.area().width,
            frame.area().height
        );
        
        let paragraph = Paragraph::new(message)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red).add_modifier(ratatui::style::Modifier::BOLD));
        
        frame.render_widget(paragraph, frame.area());
    }

    
    
    
    fn render_main_content(&self, frame: &mut Frame, area: Rect) {
        let inner_area = TabBarWidget::render_composite(
            &self.config,
            &self.tabs,
            &["MainContentTabBar"], 
            area,
            frame.buffer_mut()
        );

        let content = ratatui::widgets::Paragraph::new("Your content here...");
        frame.render_widget(content, inner_area);
    }

    
    
    
    fn render_bindings(&self, frame: &mut Frame, area: Rect) {
        let mut spans = Vec::new();
        
        // Find active tab and get its specific bindings
        if let Some(active_tab) = self.tabs.iter().find(|tab| tab.active) {
            if let Some(tab_bar) = self.config.tab_bars.iter()
                .find(|tb| tb.id == "MainContentTabBar") {
                if let Some(tab_bindings) = tab_bar.tab_bindings.get(&active_tab.name.to_lowercase()) {
                    for (i, binding) in tab_bindings.iter().enumerate() {
                        if i > 0 {
                            spans.push(Span::styled(" ", Style::default()));
                        }
                        
                        spans.push(Span::styled(
                            format!("{} {}", binding.key, binding.description),
                            Style::default().fg(Color::White)
                        ));
                    }
                }
            }
        }
        
        // If no tab-specific bindings, show empty
        if spans.is_empty() {
            spans.push(Span::raw(""));
        }
        
        let paragraph = Paragraph::new(Line::from(spans));
        frame.render_widget(paragraph, area);
    }

    
    
    
    
    
    
    
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        
        // Create a block with only white top border
        let block = Block::new()
            .borders(ratatui::widgets::Borders::TOP)
            .border_style(Style::default().fg(Color::White));
        frame.render_widget(block, area);
        
        // Calculate width needed for global bindings
        let bindings_width = if self.config.application.bindings.is_empty() {
            0
        } else {
            let bindings_text: String = self.config.application.bindings
                .iter()
                .map(|b| format!(" {} ", b.display))
                .collect::<String>();
            bindings_text.len() as u16
        };
        
        // Use the full area but position text below the top border
        let text_area = Rect {
            x: area.x,
            y: area.y + 1, // Start below the top border
            width: area.width,
            height: 1, // Ensure text takes only one line
        };
        
        // Render left side (status text) - use fallback if area is too small
        if text_area.height > 0 && text_area.width > 0 {
            let status_text = if self.config.application.status_bar.default_text.is_empty() {
                "Status: Ready".to_string()
            } else {
                self.config.application.status_bar.default_text.clone()
            };
            
            let status_spans = vec![
                Span::styled(
                    format!("{} ", status_text),
                    Style::default().fg(Color::White)
                )
            ];
            let status_paragraph = Paragraph::new(Line::from(status_spans));
            frame.render_widget(status_paragraph, text_area);
        }
        
        // Render right side (global bindings) if there's space
        if !self.config.application.bindings.is_empty() && text_area.height > 0 && text_area.width > bindings_width {
            let mut bindings_spans = Vec::new();
            
            // Add global bindings
            for (i, binding) in self.config.application.bindings.iter().enumerate() {
                if i > 0 {
                    bindings_spans.push(Span::styled(" ", Style::default()));
                }
                
                bindings_spans.push(Span::styled(
                    format!(" {} ", binding.display),
                    Style::default().fg(Color::White)
                ));
            }
            
            let bindings_area = Rect {
                x: text_area.x + text_area.width - bindings_width,
                y: text_area.y,
                width: bindings_width,
                height: text_area.height,
            };
            
            let bindings_paragraph = Paragraph::new(Line::from(bindings_spans));
            frame.render_widget(bindings_paragraph, bindings_area);
        }
    }
    
    
}









//>
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


    
//<    
    



    // /// Renders a block and automatically attaches one or more tab bars to its borders.
    // /// The first ID in `tab_ids` determines the primary top-border layout.
    // fn render_tabbed_block(&self, frame: &mut Frame, area: Rect, tab_ids: &[&str]) {
    //     let mut current_body_area = area;
    //     let mut active_widgets = Vec::new();

    //     // 1. Calculate layouts for all requested tab bars
    //     for id in tab_ids {
    //         if let Some((tab_bar, alignment)) = TabBarWidget::from_config(&self.config, &self.tabs, id) {
    //             // Determine how this specific tab bar affects the block's inner area
    //             let [header, body] = tab_bar.split_layout(current_body_area);
                
    //             // Update the area for the NEXT widget or the final block
    //             current_body_area = body;
    //             active_widgets.push((tab_bar, alignment, header));
    //         }
    //     }

    //     // 2. Render the actual Block (The "Parent")
    //     // Use MergeStrategy so all attached tab bars join seamlessly
    //     let block = Block::bordered().merge_borders(MergeStrategy::Exact);
    //     frame.render_widget(block, current_body_area);

    //     // 3. Render all tab bar decals on top of the block's borders
    //     for (widget, alignment, header_area) in active_widgets {
    //         widget.render_aligned(header_area, alignment, frame.buffer_mut());
    //     }
    // }
    
    // fn render_main_content(&self, frame: &mut Frame, area: Rect) {
    //     self.render_tabbed_block(frame, area, &["MainContentTabBar", "MainContentStaticTabBar"]);
    // }
