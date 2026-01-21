
    
    
use crate::commands::{self, ProgressUpdate};
use crate::config::{Config};
use crate::widgets::tab_bar::{TabBarItem, TabBarWidget};
use crate::widgets::progress_bar::ProgressBarWidget;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, Paragraph},
    Frame,
};
use std::sync::mpsc;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub tabs: Vec<TabBarItem>,
    pub config: Config,
    pub terminal_too_small: bool,
    pub commands: Vec<String>,
    pub selected_command_index: usize,
    pub output_lines: Vec<String>,
    pub progress_is_running: bool,
    pub progress_percentage: f64,
    pub progress_stage: String,
    pub command_tx: mpsc::Sender<ProgressUpdate>,
    command_rx: mpsc::Receiver<ProgressUpdate>,
    pub status_text: String,
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

        let commands = vec![
            "Compile".to_string(),
            "Ready".to_string(),
            "Upload".to_string(),
            "Monitor-Serial".to_string(),
            "Monitor-MQTT".to_string(),
            "Clean".to_string(),
            "All".to_string(),
            "Help".to_string(),
        ];

        let (command_tx, command_rx) = mpsc::channel();

        Self {
            running: true,
            tabs,
            config,
            terminal_too_small: false,
            commands,
            selected_command_index: 0,
            output_lines: Vec::new(),
            progress_is_running: false,
            progress_percentage: 0.0,
            progress_stage: String::new(),
            command_tx,
            command_rx,
            status_text: "Ready".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Message {
    Quit,
    SelectPreviousCommand,
    SelectNextCommand,
    ExecuteCommand,
}

impl App {
    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Quit => self.running = false,
            Message::SelectNextCommand => {
                self.selected_command_index = (self.selected_command_index + 1) % self.commands.len();
            }
            Message::SelectPreviousCommand => {
                self.selected_command_index = if self.selected_command_index > 0 {
                    self.selected_command_index - 1
                } else {
                    self.commands.len() - 1
                };
            }
            Message::ExecuteCommand => {
                let command = self.commands[self.selected_command_index].clone();
                self.dispatch_command(&command);
            }
        }
        None
    }

    pub fn handle_command_updates(&mut self) {
        while let Ok(update) = self.command_rx.try_recv() {
            match update {
                ProgressUpdate::Status(status) => {
                    self.status_text = status;
                }
                ProgressUpdate::OutputLine(line) => {
                    self.output_lines.push(line);
                }
                ProgressUpdate::Percentage(p) => self.progress_percentage = p,
                ProgressUpdate::Stage(s) => self.progress_stage = s,
                ProgressUpdate::Completed => {
                    self.progress_is_running = false;
                    self.progress_percentage = 100.0;
                    self.status_text = "Command completed successfully.".to_string();
                    self.output_lines.push("Command completed successfully.".to_string());
                }
                ProgressUpdate::Failed(e) => {
                    self.progress_is_running = false;
                    self.status_text = format!("[Error] {}", e);
                    self.output_lines.push(format!("[Error] Command failed: {}", e));
                }
            }
        }
    }

    fn dispatch_command(&mut self, command: &str) {
        self.output_lines.push(format!("Executing '{}'...", command));
        
        // Default to not running progress
        self.progress_is_running = false;

        match command {
            "Compile" => {
                self.progress_is_running = true;
                self.progress_percentage = 0.0;
                self.progress_stage = "Initializing...".to_string();
                self.output_lines.clear(); // Clear previous output
                
                let tx = self.command_tx.clone();
                let settings = crate::config::load_command_settings(); // Get settings

                std::thread::spawn(move || {
                    let callback = move |update| {
                        if tx.send(update).is_err() {
                            // Main thread has likely shut down, exit thread
                            return;
                        }
                    };
                    commands::run_compile(&settings, callback);
                });
            }
            "Upload" => {
                // To be implemented
            }
            "Monitor-Serial" => {
                // To be implemented
            }
            "Monitor-MQTT" => {
                // To be implemented
            }
            "Clean" => {
                self.output_lines.push("Cleaning project...".to_string());
                self.output_lines.push("Done.".to_string());
            }
            "All" => {
                // To be implemented
            }
            "Help" => {
                self.output_lines.push("Help:".to_string());
                self.output_lines.push("- Use Up/Down arrows to select a command.".to_string());
                self.output_lines.push("- Press Enter to execute.".to_string());
                self.output_lines.push("- Press 'q' to quit.".to_string());
            }
            _ => {
                self.output_lines.push(format!("Unknown command: {}", command));
            }
        }
    }

    pub fn check_terminal_size(&mut self, area: Rect) { //>
        self.terminal_too_small = area.width < self.config.application.min_width || 
                                 area.height < self.config.application.min_height;
    } //<

    pub fn view(&mut self, frame: &mut Frame) { //>
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
    } //<

    fn render_title_bar(&self, frame: &mut Frame, area: Rect) { //>
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
    } //<

    fn render_terminal_too_small(&self, frame: &mut Frame) { //>
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
    } //<

    
    
    
    fn render_main_content(&self, frame: &mut Frame, area: Rect) {
        let inner_area = TabBarWidget::render_composite(
            &self.config,
            &self.tabs,
            &["MainContentTabBar"], 
            area,
            frame.buffer_mut()
        );

        
        
        
        let [left_col_area, right_col_area] = Layout::horizontal([
            Constraint::Length(25),
            Constraint::Min(0),
        ])
        .areas(inner_area);
        
        
        
        let [profile_area, commands_area] = Layout::vertical([
            Constraint::Length(10),
            Constraint::Length(10),
        ])
        .areas(left_col_area);
        
        
        
        
        
        // let [commands_area, right_col_area] = Layout::horizontal([
        //     Constraint::Length(18),
        //     Constraint::Min(0),
        // ])
        // .areas(inner_area);

        
        
        
        
        
        
        let [status_area, output_area] = Layout::vertical([
            Constraint::Length(4),
            Constraint::Min(0),
        ])
        .areas(right_col_area);
        
        
        let profile_block = Block::bordered().title(" Profile ");
        frame.render_widget(profile_block, profile_area);

        
        
        
        
        // --- Commands List ---
        let commands_block = Block::bordered().title(" Commands ");
        let command_items: Vec<ListItem> = self.commands
            .iter()
            .enumerate()
            .map(|(idx, cmd)| {
                let style = if idx == self.selected_command_index {
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                ListItem::new(Line::from(Span::styled(cmd.clone(), style)))
            })
            .collect();
    
        let command_list = List::new(command_items)
            .block(commands_block.padding(ratatui::widgets::Padding::uniform(1)));
        
        frame.render_widget(command_list, commands_area);

        
        
        
        
        // --- Status and Output Blocks ---
        if self.progress_is_running {
            let progress_widget = ProgressBarWidget::new(
                "Status".to_string(),
                self.progress_percentage,
                self.progress_stage.clone(),
            );
            frame.render_widget(progress_widget, status_area);
        } else {
            let status_block = Block::bordered()
                .title(" Status ")
                .padding(ratatui::widgets::Padding::uniform(1))
                .border_style(if self.status_text.starts_with("[Error]") {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default()
                });
            let inner_status_area = status_block.inner(status_area);
            frame.render_widget(status_block, status_area);

            let style = if self.status_text.starts_with("[Error]") {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            };
            let status_paragraph = Paragraph::new(self.status_text.as_str())
                .style(style)
                .wrap(ratatui::widgets::Wrap { trim: true });

            frame.render_widget(status_paragraph, inner_status_area);
        }
        
        let output_block = Block::bordered().title(" Output ");
        let output_paragraph = if self.output_lines.is_empty() {
            Paragraph::new("No output yet.").style(Style::default().fg(Color::DarkGray))
        } else {
            let lines: Vec<Line> = self.output_lines.iter().map(|line| Line::from(line.as_str())).collect();
            Paragraph::new(lines)
        };
        frame.render_widget(output_paragraph.block(output_block.padding(ratatui::widgets::Padding::uniform(1))), output_area);
    }

    
    
    
    fn render_bindings(&self, frame: &mut Frame, area: Rect) { //>
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
    } //<
    
    
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) { //>
        
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
    } //<
    
    
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
