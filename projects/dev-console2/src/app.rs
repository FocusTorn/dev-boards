
    
    
use crate::commands::{self, ProgressUpdate};
use crate::config::{Config, ProfileConfig};
use crate::widgets::tab_bar::{TabBarItem, TabBarWidget};
use crate::widgets::progress_bar::ProgressBarWidget;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, Paragraph, BorderType},
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
    // Profile state
    pub profile_config: Option<ProfileConfig>,
    pub selected_profile_index: usize,
    pub profile_ids: Vec<String>,
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

        // Load profile configuration
        let mut profile_config: Option<ProfileConfig> = None;
        let mut profile_ids: Vec<String> = Vec::new();
        let mut status_text = "Ready".to_string();

        match crate::config::load_profile_config() {
            Ok(config) => {
                profile_ids = config.sketches.iter().map(|s| s.id.clone()).collect();
                profile_config = Some(config);
                status_text = format!("{} profiles loaded.", profile_ids.len());
            },
            Err(e) => {
                status_text = format!("[Error] Failed to load profiles: {}", e);
            }
        };

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
            status_text,
            profile_config,
            selected_profile_index: 0,
            profile_ids,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Message {
    Quit,
    SelectPreviousCommand,
    SelectNextCommand,
    ExecuteCommand,
    SelectPreviousProfile,
    SelectNextProfile,
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
            Message::SelectNextProfile => {
                if !self.profile_ids.is_empty() {
                    self.selected_profile_index = (self.selected_profile_index + 1) % self.profile_ids.len();
                }
            }
            Message::SelectPreviousProfile => {
                if !self.profile_ids.is_empty() {
                    self.selected_profile_index = if self.selected_profile_index > 0 {
                        self.selected_profile_index - 1
                    } else {
                        self.profile_ids.len() - 1
                    };
                }
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

    fn get_settings_from_profile(&self) -> crate::commands::Settings {
    if let (Some(profile_config), Some(profile_id)) = (&self.profile_config, self.profile_ids.get(self.selected_profile_index)) {
        // Find the sketch for this profile
        if let Some(sketch) = profile_config.sketches.iter().find(|s| s.id == *profile_id) {
            // Find the device and connection for this sketch
            let device = profile_config.devices.iter()
                .find(|d| d.id == sketch.device);
            let connection = profile_config.connections.iter()
                .find(|c| c.id == sketch.connection);
            
            if let (Some(device), Some(connection)) = (device, connection) {
                let sketch_path_buf = std::path::PathBuf::from(&sketch.path);
                
                let sketch_directory = sketch_path_buf.parent()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "".to_string()); // Fallback if parent is somehow not found

                let sketch_name = sketch_path_buf.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("sketch")
                    .to_string();
                
                return crate::commands::Settings {
                    sketch_directory,
                    sketch_name,
                    fqbn: device.fbqn.clone(),  // Using FBQN from YAML
                    board_model: device.board_model.clone(),
                    env: if connection.compiler == "arduino-cli" { "arduino" } else { "windows" }.to_string(),
                };
            }
        }
    }
    
    // Fallback to default settings
    crate::config::load_command_settings()
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
                let settings = self.get_settings_from_profile(); // Get settings from selected profile

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
        let inner_profile_area = profile_block.inner(profile_area);
        frame.render_widget(profile_block, profile_area);
        
        // Render profile combo box
        if !self.profile_ids.is_empty() {
            // Show current selection
            let current_profile = if self.selected_profile_index < self.profile_ids.len() {
                &self.profile_ids[self.selected_profile_index]
            } else {
                "No profiles"
            };
            
            let profile_text = if self.profile_ids.len() > 1 {
                format!("{} ({} of {})", current_profile, self.selected_profile_index + 1, self.profile_ids.len())
            } else {
                current_profile.to_string()
            };
            
            let profile_paragraph = Paragraph::new(profile_text)
                .style(Style::default().fg(Color::Cyan))
                .block(Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(" Sketch Profile ")
                    .title_style(Style::default().fg(Color::Yellow)));
            
            frame.render_widget(profile_paragraph, inner_profile_area);
        } else {
            // No profiles available
            let no_profile_text = Paragraph::new("No profiles found")
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(" Sketch Profile ")
                    .title_style(Style::default().fg(Color::Yellow)));
            
            frame.render_widget(no_profile_text, inner_profile_area);
        }

        
        
        
        
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
            .block(commands_block);
        
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
        frame.render_widget(output_paragraph.block(output_block), output_area);
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

