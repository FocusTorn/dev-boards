use crate::commands::ProgressUpdate;
use crate::config::ProfileConfig;
use crate::widgets::tab_bar::{TabBarItem, TabBarWidget};
use crate::widgets::progress_bar::ProgressBarWidget;
use crate::widgets::status_box::StatusBoxWidget;
use crate::widgets::output_box::OutputBoxWidget;
use crate::widgets::toast::{ToastManager, ToastWidget};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph, BorderType, ScrollbarState, Clear},
    Frame,
};
use arboard::Clipboard;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::RefCell;
use color_eyre::Result;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ActiveBox {
    None,
    Output,
    Status,
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub tabs: Vec<TabBarItem>,
    pub config: crate::config::Config,
    pub terminal_too_small: bool,
    pub commands: Vec<String>,
    pub selected_command_index: usize,
    pub output_lines: Vec<String>,
    pub output_scroll: u16,
    pub output_scrollbar_state: RefCell<ScrollbarState>,
    pub progress_is_running: bool,
    pub progress_percentage: f64,
    pub progress_stage: String,
    pub command_tx: mpsc::Sender<ProgressUpdate>,
    command_rx: mpsc::Receiver<ProgressUpdate>,
    pub status_text: String,
    pub toast_manager: ToastManager,
    pub profile_config: Option<ProfileConfig>,
    pub selected_profile_index: usize,
    pub profile_ids: Vec<String>,
    pub active_box: ActiveBox,
    pub cancel_signal: Arc<AtomicBool>,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = crate::config::load_config()?;
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

        let mut profile_config: Option<ProfileConfig> = None;
        let mut profile_ids: Vec<String> = Vec::new();
        let status_text;
        let mut output_lines = Vec::new();
        
        // Load toast widget configuration - Propagate error if it fails
        let toast_config = crate::config::load_widget_config()?;
        let toast_manager = ToastManager::new(toast_config);

        match crate::config::load_profile_config() {
            Ok(config) => {
                profile_ids = config.sketches.iter().map(|s| s.id.clone()).collect();
                profile_config = Some(config);
                status_text = format!("{} profiles loaded.", profile_ids.len());
            },
            Err(e) => {
                status_text = "[Error] Failed to load config.yaml".to_string();
                for line in format!("{}", e).lines() {
                    output_lines.push(line.to_string());
                }
            }
        };

        Ok(Self {
            running: true,
            tabs,
            config,
            terminal_too_small: false,
            commands,
            selected_command_index: 0,
            output_lines,
            output_scroll: 0,
            output_scrollbar_state: RefCell::new(ScrollbarState::default()),
            progress_is_running: false,
            progress_percentage: 0.0,
            progress_stage: String::new(),
            command_tx,
            command_rx,
            status_text,
            toast_manager,
            profile_config,
            selected_profile_index: 0,
            profile_ids,
            active_box: ActiveBox::None,
            cancel_signal: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Quit => self.running = false,
            Message::SelectNextCommand => {
                if self.active_box == ActiveBox::None {
                    self.selected_command_index = (self.selected_command_index + 1) % self.commands.len();
                }
            }
            Message::SelectPreviousCommand => {
                if self.active_box == ActiveBox::None {
                    self.selected_command_index = if self.selected_command_index > 0 {
                        self.selected_command_index - 1
                    } else {
                        self.commands.len() - 1
                    };
                }
            }
            Message::ExecuteCommand => {
                if self.active_box == ActiveBox::None {
                    let command = self.commands[self.selected_command_index].clone();
                    self.dispatch_command(&command);
                }
            }
            Message::SelectNextProfile => {
                if self.active_box == ActiveBox::None {
                    if !self.profile_ids.is_empty() {
                        self.selected_profile_index = (self.selected_profile_index + 1) % self.profile_ids.len();
                    }
                }
            }
            Message::SelectPreviousProfile => {
                if self.active_box == ActiveBox::None {
                    if !self.profile_ids.is_empty() {
                        self.selected_profile_index = if self.selected_profile_index > 0 {
                            self.selected_profile_index - 1
                        } else {
                            self.profile_ids.len() - 1
                        };
                    }
                }
            }
            Message::ScrollOutputUp => {
                match self.active_box {
                    ActiveBox::Output | ActiveBox::None => {
                        self.output_scroll = self.output_scroll.saturating_sub(1);
                    },
                    _ => {},
                }
            }
            Message::ScrollOutputDown => {
                match self.active_box {
                    ActiveBox::Output | ActiveBox::None => {
                        let max_scroll = self.output_lines.len().saturating_sub(1) as u16;
                        if self.output_scroll < max_scroll {
                            self.output_scroll = self.output_scroll.saturating_add(1);
                        }
                    },
                    _ => {},
                }
            }
            Message::CopyStatusText(text_to_copy) => {
                match Clipboard::new() {
                    Ok(mut clipboard) => {
                        if let Err(e) = clipboard.set_text(text_to_copy) {
                            self.status_text = format!("[Error] Failed to copy to clipboard: {}", e);
                            self.toast_manager.error("Failed to copy to clipboard");
                        } else {
                            self.toast_manager.success("Status text copied to clipboard.");
                        }
                    }
                    Err(e) => {
                        self.status_text = format!("[Error] Failed to initialize clipboard: {}", e);
                    }
                }
            }
            Message::CopyOutputText(text_to_copy) => {
                match Clipboard::new() {
                    Ok(mut clipboard) => {
                        if let Err(e) = clipboard.set_text(text_to_copy) {
                            self.status_text = format!("[Error] Failed to copy to clipboard: {}", e);
                            self.toast_manager.error("Failed to copy output to clipboard");
                        } else {
                            self.toast_manager.success("Output text copied to clipboard.");
                        }
                    }
                    Err(e) => {
                        self.status_text = format!("[Error] Failed to initialize clipboard: {}", e);
                    }
                }
            }
            Message::FocusBox(box_to_focus) => {
                self.active_box = box_to_focus;
            }
            Message::UnfocusBox => {
                if self.progress_is_running {
                    self.cancel_signal.store(true, Ordering::SeqCst);
                    self.push_output("[SYSTEM] Cancellation signal sent...");
                }
                self.active_box = ActiveBox::None;
            }
            Message::ShowToast(message) => {
                self.toast_manager.success(message);
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
                    self.push_output(&line);
                }
                ProgressUpdate::Percentage(p) => self.progress_percentage = p,
                ProgressUpdate::Stage(s) => self.progress_stage = s,
                ProgressUpdate::Completed => {
                    self.progress_is_running = false;
                    self.progress_percentage = 100.0;
                    self.status_text = "Command completed successfully.".to_string();
                    self.push_output("Command completed successfully.");
                }
                ProgressUpdate::Failed(e) => {
                    self.progress_is_running = false;
                    self.status_text = format!("[Error] {}", e);
                    self.push_output(&format!("[Error] Command failed: {}", e));
                }
            }
        }
    }

    fn get_settings_from_profile(&self) -> Result<crate::commands::Settings> {
        if let (Some(profile_config), Some(profile_id)) = (&self.profile_config, self.profile_ids.get(self.selected_profile_index)) {
            if let Some(sketch) = profile_config.sketches.iter().find(|s| s.id == *profile_id) {
                let device = profile_config.devices.iter()
                    .find(|d| d.id == sketch.device);
                let connection = profile_config.connections.iter()
                    .find(|c| c.id == sketch.connection);
                
                if let (Some(device), Some(connection)) = (device, connection) {
                    let sketch_path_buf = std::path::PathBuf::from(&sketch.path);
                    
                    let sketch_directory = sketch_path_buf.parent() 
                        .map(|p| p.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "".to_string());

                    let sketch_name = sketch_path_buf.file_stem() 
                        .and_then(|s| s.to_str())
                        .unwrap_or("sketch")
                        .to_string();
                    
                    return Ok(crate::commands::Settings {
                        sketch_directory,
                        sketch_name,
                        fqbn: device.fbqn.clone(),
                        board_model: device.board_model.clone(),
                        env: if connection.compiler == "arduino-cli" { "arduino" } else { "windows" }.to_string(),
                    });
                }
            }
        }
        crate::config::load_command_settings().map_err(|e| color_eyre::eyre::eyre!(e))
    }

    fn dispatch_command(&mut self, command: &str) {
        self.push_output(&format!("Executing '{}'...", command));
        self.progress_is_running = false;

        match command {
            "Compile" => {
                self.progress_is_running = true;
                self.progress_percentage = 0.0;
                self.progress_stage = "Initializing...".to_string();
                self.output_lines.clear();
                
                let tx = self.command_tx.clone();
                let cancel_signal = self.cancel_signal.clone();
                cancel_signal.store(false, Ordering::SeqCst);

                match self.get_settings_from_profile() {
                    Ok(settings) => {
                        std::thread::spawn(move || {
                            let callback = move |update| {
                                if tx.send(update).is_err() {
                                    return;
                                }
                            };
                            crate::commands::run_compile(&settings, cancel_signal, callback);
                        });
                    },
                    Err(e) => {
                         self.progress_is_running = false;
                         self.status_text = format!("[Error] {}", e);
                         self.push_output(&format!("[Error] Failed to get settings: {}", e));
                         self.toast_manager.error(&format!("Settings Error: {}", e));
                    }
                }
            }
            "Upload" => {}
            "Monitor-Serial" => {}
            "Monitor-MQTT" => {}
            "Clean" => {
                self.push_output("Cleaning project...");
                self.push_output("Done.");
            }
            "All" => {}
            "Help" => {
                self.push_output("Help:");
                self.push_output("- Use Up/Down arrows to select a command.");
                self.push_output("- Press Enter to execute.");
                self.push_output("- Press 'q' to quit.");
            }
            _ => {
                self.push_output(&format!("Unknown command: {}", command));
            }
        }
    }

    fn push_output(&mut self, text: &str) {
        for line in text.lines() {
            self.output_lines.push(line.to_string());
        }
    }

    pub fn check_terminal_size(&mut self, area: Rect) {
        self.terminal_too_small = area.width < self.config.application.min_width || 
                                 area.height < self.config.application.min_height;
    }

    pub fn view(&mut self, frame: &mut Frame) {
        self.check_terminal_size(frame.area());
        
        if self.terminal_too_small {
            self.render_terminal_too_small(frame);
            return;
        }

        let vertical_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(2),
        ]);

        let [title_area, main_area, bindings_area, status_bar_area] =
            vertical_layout.areas(frame.area());

        self.render_title_bar(frame, title_area);
        self.render_main_content(frame, main_area);
        self.render_bindings(frame, bindings_area);
        self.render_status_bar(frame, status_bar_area);
        
        // Handle Toast Expiry
        self.toast_manager.update();

        // Render toasts
        frame.render_widget(ToastWidget::new(&self.toast_manager), frame.area());
    }

    fn render_title_bar(&self, frame: &mut Frame, area: Rect) {
        let title_text = &self.config.application.title;
        let title_len = title_text.len();
        let total_width = area.width as usize;
        
        let line = if total_width <= title_len + 2 {
             Line::from(Span::styled(
                 title_text, 
                 Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
             ))
        } else {
            let available_dash_space = total_width.saturating_sub(title_len + 2);
            let left_dash_count = available_dash_space / 2;
            let right_dash_count = available_dash_space - left_dash_count;
            
            let left_dashes = "═".repeat(left_dash_count);
            let right_dashes = "═".repeat(right_dash_count);
            
            Line::from(vec![
                Span::styled(left_dashes, Style::default().fg(Color::White)),
                Span::styled(format!(" {} ", title_text), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(right_dashes, Style::default().fg(Color::White)),
            ])
        };
        
        frame.render_widget(Paragraph::new(line).alignment(Alignment::Center), area);
    }

    fn render_terminal_too_small(&self, frame: &mut Frame) {
        frame.render_widget(Clear, frame.area());
        
        let message = format!(
            "Terminal Too Small\nRequired: {}x{}
Current: {}x{}

Press 'q' to quit",
            self.config.application.min_width,
            self.config.application.min_height,
            frame.area().width,
            frame.area().height
        );
        
        let paragraph = Paragraph::new(message)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        
        frame.render_widget(paragraph, frame.area());
    }

    pub fn get_status_area(&self, total_area: Rect) -> Rect {
        let vertical_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(2),
        ]);
        let [_, main_area, _, _] = vertical_layout.areas(total_area);

        let mut effective_inner_area = main_area;
        if let Some(tab_config) = self.config.tab_bars.iter().find(|t| t.id == "MainContentTabBar") {
            let desired_height = if tab_config.style.as_deref() == Some("tabbed") { 2 } else { 1 };
            if main_area.height >= desired_height {
                effective_inner_area.y += desired_height.saturating_sub(1);
                effective_inner_area.height = effective_inner_area.height.saturating_sub(desired_height.saturating_sub(1));
            }
        }
        effective_inner_area = Block::bordered().inner(effective_inner_area);

        let [_, right_col_area] = Layout::horizontal([
            Constraint::Length(25),
            Constraint::Min(0),
        ])
        .areas(effective_inner_area);

        let [status_area, _] = Layout::vertical([
            Constraint::Length(4),
            Constraint::Min(0),
        ])
        .areas(right_col_area);

        status_area
    }
    
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
        
        let [status_area, output_area] = Layout::vertical([
            Constraint::Length(4),
            Constraint::Min(0),
        ])
        .areas(right_col_area);
        
        let profile_block = Block::bordered().title(" Profile ");
        let inner_profile_area = profile_block.inner(profile_area);
        frame.render_widget(profile_block, profile_area);
        
        if !self.profile_ids.is_empty() {
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
            let no_profile_text = Paragraph::new("No profiles found")
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(" Sketch Profile ")
                    .title_style(Style::default().fg(Color::Yellow)));
            
            frame.render_widget(no_profile_text, inner_profile_area);
        }

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
            .block(Block::bordered().title(" Commands "));
        
        frame.render_widget(command_list, commands_area);

        if self.progress_is_running {
            let progress_widget = ProgressBarWidget::new(
                "Status".to_string(),
                self.progress_percentage,
                self.progress_stage.clone(),
            );
            frame.render_widget(progress_widget, status_area);
        } else {
            let status_widget = StatusBoxWidget::new(&self.status_text, self.active_box == ActiveBox::Status);
            frame.render_widget(status_widget, status_area);
        }
        
        let mut scrollbar_state = self.output_scrollbar_state.borrow_mut();
        let output_widget = OutputBoxWidget::new(&self.output_lines, &mut scrollbar_state, self.output_scroll, self.active_box == ActiveBox::Output);
        frame.render_widget(output_widget, output_area);
    }

    fn render_bindings(&self, frame: &mut Frame, area: Rect) {
        let mut spans = Vec::new();
        
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
        
        if spans.is_empty() {
            spans.push(Span::raw(""));
        }
        
        let paragraph = Paragraph::new(Line::from(spans));
        frame.render_widget(paragraph, area);
    }
    
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let block = Block::new()
            .borders(ratatui::widgets::Borders::TOP)
            .border_style(Style::default().fg(Color::White));
        frame.render_widget(block, area);
        
        let bindings_width = if self.config.application.bindings.is_empty() {
            0
        } else {
            let bindings_text: String = self.config.application.bindings
                .iter()
                .map(|b| format!(" {} ", b.display))
                .collect::<String>();
            bindings_text.len() as u16
        };
        
        let text_area = Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: 1,
        };
        
        if text_area.height > 0 && text_area.width > 0 {
            let status_text_val = if self.config.application.status_bar.default_text.is_empty() {
                "Status: Ready".to_string()
            } else {
                self.config.application.status_bar.default_text.clone()
            };
            
            let status_spans = vec![
                Span::styled(
                    format!("{} ", status_text_val),
                    Style::default().fg(Color::White)
                )
            ];
            let status_paragraph = Paragraph::new(Line::from(status_spans));
            frame.render_widget(status_paragraph, text_area);
        }
        
        if !self.config.application.bindings.is_empty() && text_area.height > 0 && text_area.width > bindings_width {
            let mut bindings_spans = Vec::new();
            
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

#[derive(PartialEq, Debug, Clone)]
pub enum Message {
    Quit,
    SelectPreviousCommand,
    SelectNextCommand,
    ExecuteCommand,
    SelectPreviousProfile,
    SelectNextProfile,
    ScrollOutputUp,
    ScrollOutputDown,
    CopyStatusText(String),
    CopyOutputText(String),
    ShowToast(&'static str),
    FocusBox(ActiveBox),
    UnfocusBox,
}
