use crate::commands::ProgressUpdate;
use crate::config::ProfileConfig;
mod executors;
mod system;
mod view;
mod ansi;
pub mod theme;

use crate::app::theme::Theme;

use crate::widgets::tab_bar::{TabBarItem, TabBarWidget, TabBarAlignment};
use crate::widgets::command_list::CommandListWidget;
use crate::widgets::smooth_scrollbar::{ScrollBar, ScrollBarInteraction, ScrollCommand, ScrollLengths, ScrollEvent};
use crate::widgets::toast::{ToastManager};
use ratatui::{
    layout::{Constraint, Layout, Rect, Position},
    widgets::{Block},
};
use crossterm::event::{self, KeyCode, KeyModifiers, KeyEventKind};
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool};
use std::time::Instant;
use color_eyre::Result;

/// Semantic actions that can be triggered by user input or system events
use strum_macros::{EnumString, Display};

#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Action {
    Quit,
    NextProfile,
    PrevProfile,
    NextTab,
    PrevTab,
    ScrollLineUp,
    ScrollLineDown,
    ScrollPageUp,
    ScrollPageDown,
    #[strum(serialize = "scroll_top", serialize = "scroll_output_to_top")]
    ScrollOutputToTop,
    #[strum(serialize = "scroll_bottom", serialize = "scroll_output_to_bottom")]
    ScrollOutputToBottom,
    ToggleAutoscroll,
    ToggleInput,
    CopyStatus,
    CopyOutputVisible,
    CopyOutputFull,
    #[strum(serialize = "Compile")]
    Compile,
    #[strum(serialize = "Upload")]
    Upload,
    #[strum(serialize = "Monitor-Serial")]
    MonitorSerial,
    #[strum(serialize = "Monitor-MQTT")]
    MonitorMqtt,
    #[strum(serialize = "Clean")]
    Clean,
    CommandsUp,
    CommandsDown,
    #[strum(serialize = "execute", serialize = "commands_execute")]
    Execute,
    Cancel,
}

impl Action {
    pub fn from_str(s: &str) -> Option<Self> {
        <Self as std::str::FromStr>::from_str(s).ok()
    }
}


/// Represents the current state of a background task
#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum MonitorType {
    Serial,
    Mqtt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskState {
    Idle,
    Running {
        percentage: f64,
        visual_percentage: f64,
        last_percentage: f64,
        stage: String,
        start_time: Instant,
        last_updated: Instant,
        smoothed_eta: Option<f64>,
    },
    Monitoring {
        monitor_type: MonitorType,
        start_time: Instant,
    },
}

const MAX_OUTPUT_LINES: usize = 2000;

#[derive(Debug, Clone, Copy)]
pub struct AppLayout {
    pub title: Rect,
    pub main: Rect,
    pub bindings: Rect,
    pub status_bar: Rect,
    pub profile: Rect,
    pub commands: Rect,
    pub status: Rect,
    pub output: Rect,
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    tabs: Vec<TabBarItem>,
    config: crate::config::Config,
    tab_bar_map: HashMap<String, crate::config::TabBarConfig>,
    terminal_too_small: bool,
    commands: Vec<String>,
    selected_command_index: usize,
    hovered_command_index: Option<usize>,
    command_index_before_hover: Option<usize>,
    output_lines: Vec<String>,
    output_cached_lines: Vec<ratatui::text::Line<'static>>,
    output_scroll: u16,
    output_scroll_interaction: ScrollBarInteraction,
    output_autoscroll: bool,
    task_state: TaskState,
    command_tx: mpsc::Sender<ProgressUpdate>,
    command_rx: mpsc::Receiver<ProgressUpdate>,
    status_text: String,
    toast_manager: ToastManager,
    profile_config: Option<ProfileConfig>,
    selected_profile_index: usize,
    profile_ids: Vec<String>,
    cancel_signal: Arc<AtomicBool>,
    view_area: Rect,
    layout: AppLayout,
    pub theme: Theme,
    predictor: crate::commands::ProgressPredictor,
    last_raw_input: String,
    last_frame_time: Instant,
    pub should_redraw: bool,

    // Input state
    pub input: tui_input::Input,
    pub input_active: bool,
    pub serial_tx: Option<mpsc::Sender<crate::commands::SerialCommand>>,
    pub mqtt_tx: Option<mpsc::Sender<crate::commands::MqttCommand>>,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = crate::config::load_config()?;
        
        let mut tab_bar_map: HashMap<String, crate::config::TabBarConfig> = HashMap::new();
        for tb in config.tab_bars.iter() {
            tab_bar_map.insert(tb.id.clone(), tb.clone());
        }

        let tabs = config.tab_bars.iter()
            .find(|t| t.id == "MainContentTabBar")
            .map(|c| c.tabs.iter().map(|t| TabBarItem {
                id: t.id.clone(),
                name: t.name.clone(),
                active: t.default == Some("active".to_string()),
            }).collect())
            .unwrap_or_default();

        let output_autoscroll = config.tab_bars.iter()
            .find(|t| t.id == "OutputPanelStaticOptions")
            .and_then(|t| t.tabs.iter().find(|tab| tab.id == "autoscroll"))
            .map(|tab| tab.default == Some("active".to_string()))
            .unwrap_or(true);

        let commands = vec![
            "Compile".to_string(),
            "Upload".to_string(),
            "Monitor-Serial".to_string(),
            "Monitor-MQTT".to_string(),
            "Clean".to_string(),
            "All".to_string(),
        ];

        let (command_tx, command_rx) = mpsc::channel();

        let mut profile_config: Option<ProfileConfig> = None;
        let mut profile_ids: Vec<String> = Vec::new();
        let mut initial_output = Vec::new();
        
        let toast_config = crate::config::load_widget_config()?;
        let toast_manager = ToastManager::new(toast_config);

        let initial_status = match crate::config::load_profile_config() {
            Ok(config) => {
                profile_ids = config.sketches.iter().map(|s| s.id.clone()).collect();
                profile_config = Some(config);
                format!("{} profiles loaded.", profile_ids.len())
            },
            Err(e) => {
                let msg = "[Error] Failed to load config.yaml".to_string();
                for line in format!("{}", e).lines() {
                    initial_output.push(line.to_string());
                }
                msg
            }
        };

        let app_theme = Theme::new(&config.theme);

        Ok(Self {
            running: true,
            tabs,
            config,
            tab_bar_map,
            terminal_too_small: false,
            commands,
            selected_command_index: 0,
            hovered_command_index: None,
            command_index_before_hover: None,
            output_lines: initial_output.clone(),
            output_cached_lines: initial_output.iter().map(|l| crate::app::ansi::parse_ansi_line(l)).collect(),
            output_scroll: 0,
            output_scroll_interaction: ScrollBarInteraction::new(),
            output_autoscroll,
            task_state: TaskState::Idle,
            command_tx,
            command_rx,
            status_text: initial_status,
            toast_manager,
            profile_config,
            selected_profile_index: 0,
            profile_ids,
            cancel_signal: Arc::new(AtomicBool::new(false)),
            view_area: Rect::default(),
            layout: AppLayout {
                title: Rect::default(),
                main: Rect::default(),
                bindings: Rect::default(),
                status_bar: Rect::default(),
                profile: Rect::default(),
                commands: Rect::default(),
                status: Rect::default(),
                output: Rect::default(),
            },
            theme: app_theme,
            predictor: crate::commands::ProgressPredictor::new(),
            last_raw_input: String::new(),
            last_frame_time: Instant::now(),
            should_redraw: true,
            input: tui_input::Input::default(),
            input_active: false,
            serial_tx: None,
            mqtt_tx: None,
        })
    }

    pub fn calculate_layout(&self, area: Rect) -> AppLayout {
        let vertical_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(2),
        ]);

        let [title, main, bindings, status_bar] = vertical_layout.areas(area);

        let mut inner_main = main;
        for id in &["MainContentTabBar"] {
            if let Some(tab_config) = self.config.tab_bars.iter().find(|t| t.id == *id) {
                if tab_config.alignment.vertical != Some(TabBarAlignment::Bottom) {
                    let consumed = TabBarWidget::config_consumed_height(&self.config, id);
                    if inner_main.height >= consumed {
                        inner_main.y += consumed;
                        inner_main.height = inner_main.height.saturating_sub(consumed);
                    }
                }
            }
        }
        inner_main = Block::bordered().inner(inner_main);

        let [left_col, right_col] = Layout::horizontal([
            Constraint::Length(25),
            Constraint::Min(0),
        ])
        .areas(inner_main);

        let [profile, commands] = Layout::vertical([
            Constraint::Length(10),
            Constraint::Min(0),
        ])
        .areas(left_col);

        let [status, output] = Layout::vertical([
            Constraint::Length(4),
            Constraint::Min(0),
        ])
        .areas(right_col);

        AppLayout {
            title,
            main,
            bindings,
            status_bar,
            profile,
            commands,
            status,
            output,
        }
    }

    
    
    
    
    
    /// Primary Update Loop - Decoupled from physical input interpretation
    pub fn update(&mut self, msg: Message) {
        self.should_redraw = true;
        
        match msg {
            
            // Raw Inputs -> Route through interpretation dispatcher
            Message::Key(key_event) => { //>
                self.dispatch_key(key_event);
            } //<
            Message::Mouse(mouse_event) => { //>
                self.dispatch_mouse(mouse_event);
            } //<
            Message::SystemUpdate(update) => { //>
                self.exec_system_update(update);
            } //<
            
            // System Actions
            Message::Resize(w, h) => { //>
                self.should_redraw = true;
                let new_area = Rect::new(0, 0, w, h);
                self.view_area = new_area;
                self.layout = self.calculate_layout(new_area);
                self.check_terminal_size(new_area);
                self.sync_autoscroll();
            } //<
       
        }
    }

    fn get_modifiers_display(&self, mods: KeyModifiers) -> String {
        let mut parts = Vec::new();
        if mods.contains(KeyModifiers::CONTROL) { parts.push("Ctrl"); }
        if mods.contains(KeyModifiers::ALT) { parts.push("Alt"); }
        if mods.contains(KeyModifiers::SHIFT) { parts.push("Shift"); }
        if parts.is_empty() { "None".to_string() } else { parts.join("+") }
    }

    /// Event Dispatcher: Physical Key -> Semantic Message
    fn dispatch_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press { return; }

        let mods_str = self.get_modifiers_display(key.modifiers);
        self.last_raw_input = format!("KEY: {:?} | MODS: [{}]", key.code, mods_str);
        self.should_redraw = true;

        if self.input_active {
            use tui_input::backend::crossterm::EventHandler;
            match key.code {
                KeyCode::Enter => {
                    self.exec_send_command();
                    self.input_active = false;
                    self.should_redraw = true;
                }
                KeyCode::Esc => {
                    if self.input_active {
                        self.input_active = false;
                        self.input.reset();
                    } else {
                        self.dispatch_command(Action::Cancel);
                    }
                    self.should_redraw = true;
                }
                _ => {
                    self.input.handle_event(&crossterm::event::Event::Key(key));
                    self.should_redraw = true;
                }
            }
            return;
        }

        // 1. Tab Bar Navigation Bindings
        if let Some(tab_bar) = self.tab_bar_map.get("MainContentTabBar") {
            for key_str in &tab_bar.navigation.left {
                if self.key_matches(key, key_str) { 
                    self.dispatch_command(Action::PrevTab); 
                    return; 
                }
            }
            for key_str in &tab_bar.navigation.right {
                if self.key_matches(key, key_str) { 
                    self.dispatch_command(Action::NextTab); 
                    return; 
                }
            }
            
            // 1.1 Tab-specific Semantic Bindings
            if let Some(active_tab) = self.tabs.iter().find(|tab| tab.active) {
                if let Some(bindings_config) = tab_bar.tab_bindings.get(&active_tab.id) {
                    for binding in &bindings_config.items {
                        for (phys_key, action_str) in &binding.triggers {
                            if self.key_matches(key, phys_key) {
                                if let Some(action) = Action::from_str(action_str) {
                                    self.dispatch_command(action);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }

        // 2. Config Bindings Match
        for binding in &self.config.application.bindings.items {
            for (phys_key, action_str) in &binding.triggers {
                if self.key_matches(key, phys_key) {
                    if let Some(action) = Action::from_str(action_str) {
                        self.dispatch_command(action);
                        return;
                    }
                }
            }
        }

    }

    /// Event Dispatcher: Physical Mouse -> Semantic Message
    pub fn dispatch_mouse(&mut self, mouse_event: event::MouseEvent) {
        // RAW LOGGING (Ignore Move noise)
        if mouse_event.kind != event::MouseEventKind::Moved {
            let mods_str = self.get_modifiers_display(mouse_event.modifiers);
            self.last_raw_input = format!("MOUSE: {:?} | MODS: [{}]", mouse_event.kind, mods_str);
            self.should_redraw = true;
        }

        let mouse_pos = Position::new(mouse_event.column, mouse_event.row);
        let layout = self.layout; // Use cached layout

        // 1. Tab Bar Widget Interactions (Encapsulated)
        
        // 1.1 Main Content Tabs
        if let Some(_) = self.tab_bar_map.get("MainContentTabBar") {
            if let Some((tab_bar, horiz, vert, off_x, off_y)) = TabBarWidget::from_config(&self.config, &self.tabs, "MainContentTabBar") {
                if let Some(tab_idx) = tab_bar.handle_mouse_event(layout.main, horiz, vert, off_x, off_y, mouse_event) {
                    for (i, tab) in self.tabs.iter_mut().enumerate() {
                        tab.active = i == tab_idx;
                    }
                    self.should_redraw = true;
                    return; 
                }
            }
        }

        // 1.2 Output Panel "Auto" Toggle
        let output_static_tabs = vec![TabBarItem { id: "autoscroll".to_string(), name: "Auto".to_string(), active: self.output_autoscroll }];
        if let Some(_) = self.tab_bar_map.get("OutputPanelStaticOptions") {
            if let Some((tab_bar, horiz, vert, off_x, off_y)) = TabBarWidget::from_config(&self.config, &output_static_tabs, "OutputPanelStaticOptions") {
                if tab_bar.handle_mouse_event(layout.output, horiz, vert, off_x, off_y, mouse_event).is_some() {
                    self.dispatch_command(Action::ToggleAutoscroll);
                    return;
                }
            }
        }

        // 2. Command List Mouse Interaction
        let command_list = CommandListWidget::new(&self.commands, self.selected_command_index, self.hovered_command_index);
        match command_list.handle_mouse_event(layout.commands, mouse_event) {
            Some(crate::widgets::command_list::CommandListInteraction::Click(idx)) => {
                // COMMIT: User clicked, so the change is permanent. Clear the rollback point.
                self.selected_command_index = idx;
                self.command_index_before_hover = None; 
                self.dispatch_command(Action::Execute);
                self.should_redraw = true;
            }
            Some(crate::widgets::command_list::CommandListInteraction::Hover(idx)) => {
                // CAPTURE: If this is the first move into the box, save current selection for rollback
                if self.hovered_command_index.is_none() {
                    self.command_index_before_hover = Some(self.selected_command_index);
                }
                
                if self.hovered_command_index != Some(idx) {
                    self.hovered_command_index = Some(idx);
                    self.should_redraw = true;
                }
            }
            None => {
                // ROLLBACK: If we have a saved index and we are leaving the box, restore it.
                if let Some(old_idx) = self.command_index_before_hover {
                    self.selected_command_index = old_idx;
                    self.command_index_before_hover = None;
                    self.should_redraw = true;
                }
                
                if self.hovered_command_index.is_some() {
                    self.hovered_command_index = None;
                    self.should_redraw = true;
                }
            }
        }

        match mouse_event.kind {
            event::MouseEventKind::Down(_) => { //>
                // Clipboard logic (decoupled via messages)
                if layout.status.contains(mouse_pos) && mouse_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.dispatch_command(Action::CopyStatus);
                    return;
                } else if layout.output.contains(mouse_pos) && mouse_event.modifiers.contains(KeyModifiers::CONTROL) {
                    let is_full = mouse_event.modifiers.contains(KeyModifiers::SHIFT);
                    if is_full {
                        self.dispatch_command(Action::CopyOutputFull);
                    } else {
                        self.dispatch_command(Action::CopyOutputVisible);
                    }
                    return;
                }

        // 3. Output Box Interaction
        let output_area = layout.output;
        let inner_output = Block::bordered().inner(output_area);
        
        if inner_output.contains(mouse_pos) {
            match mouse_event.kind {
                event::MouseEventKind::Down(event::MouseButton::Left) => {
                    // Check if we are clicking on the Scrollbar area (last column of inner)
                    let is_scrollbar_click = mouse_pos.x >= inner_output.right().saturating_sub(1);
                    let is_control_held = mouse_event.modifiers.contains(event::KeyModifiers::CONTROL);
                    
                    if !is_scrollbar_click && is_control_held {
                        self.exec_copy_output(false); // Copy visible text
                        return;
                    }
                }
                _ => {}
            }
        }

        // 4. Scrollbar interaction
                                let inner_output = Block::bordered().inner(layout.output);
                                let scrollbar = ScrollBar::vertical(ScrollLengths {
                                    content_len: self.output_lines.len() + 1,
                                    viewport_len: inner_output.height as usize,
                                }).offset(self.output_scroll as usize);
                                
                                match scrollbar.handle_event(inner_output, ScrollEvent::from(mouse_event), &mut self.output_scroll_interaction) {
                                    Some(ScrollCommand::SetOffset(next)) => {
                                        self.output_scroll = next as u16;
                                        self.output_autoscroll = false;
                                    }
                                    Some(ScrollCommand::ReachedBottom) => {
                                        self.output_autoscroll = true;
                                        self.sync_autoscroll();
                                    }
                                    None => {}
                                }
                            }
                            event::MouseEventKind::ScrollUp | event::MouseEventKind::ScrollDown | event::MouseEventKind::Drag(_) => {
                                let inner_output = Block::bordered().inner(layout.output);
                                let scrollbar = ScrollBar::vertical(ScrollLengths {
                                    content_len: self.output_lines.len() + 1,
                                    viewport_len: inner_output.height as usize,
                                }).offset(self.output_scroll as usize);
                                
                                match scrollbar.handle_event(inner_output, ScrollEvent::from(mouse_event), &mut self.output_scroll_interaction) {
                                    Some(ScrollCommand::SetOffset(next)) => {
                                        self.output_scroll = next as u16;
                                        self.output_autoscroll = false;
                                    }
                                    Some(ScrollCommand::ReachedBottom) => {
                                        self.output_autoscroll = true;
                                        self.sync_autoscroll();
                                    }
                                    None => {}
                                }
                            }            _ => {}
        
        }
    }

        fn key_matches(&self, key: event::KeyEvent, binding_key: &str) -> bool {

            let inner = binding_key.trim_matches(|c| c == '[' || c == ']');

            let parts: Vec<String> = inner.split('+').map(|s| s.to_lowercase()).collect();

            let mut req_mods = KeyModifiers::empty();

            let mut target = String::new();

            for part in &parts {

                match part.as_str() {

                    "alt" => req_mods.insert(KeyModifiers::ALT),

                    "ctrl" | "control" => req_mods.insert(KeyModifiers::CONTROL),

                    "shift" => req_mods.insert(KeyModifiers::SHIFT),

                    k => target = k.to_string(),

                }

            }

            let significant = KeyModifiers::SHIFT | KeyModifiers::CONTROL | KeyModifiers::ALT;

            if (key.modifiers & significant) != req_mods { return false; }

            

            match target.as_str() {

                "q" => matches!(key.code, KeyCode::Char('q') | KeyCode::Char('Q')),

                "enter" | "return" => matches!(key.code, KeyCode::Enter),

                "esc" | "escape" => matches!(key.code, KeyCode::Esc),

                "up" => matches!(key.code, KeyCode::Up),

                "down" => matches!(key.code, KeyCode::Down),

                "left" => matches!(key.code, KeyCode::Left),

                "right" => matches!(key.code, KeyCode::Right),

                "pgup" | "pageup" => matches!(key.code, KeyCode::PageUp),

                "pgdn" | "pagedown" => matches!(key.code, KeyCode::PageDown),

                "home" => matches!(key.code, KeyCode::Home),

                "end" => matches!(key.code, KeyCode::End),

                "backspace" => matches!(key.code, KeyCode::Backspace),

                "tab" => matches!(key.code, KeyCode::Tab),

                "delete" | "del" => matches!(key.code, KeyCode::Delete),

                _ => {

                    if target.len() == 1 {

                        let c = target.chars().next().unwrap();

                        matches!(key.code, KeyCode::Char(key_c) if key_c.to_ascii_lowercase() == c.to_ascii_lowercase())

                    } else { false }

                }

            }

        }

    

    fn get_settings_from_profile(&self) -> Result<crate::commands::Settings> {
        if let (Some(profile_config), Some(profile_id)) = (&self.profile_config, self.profile_ids.get(self.selected_profile_index)) {
            if let Some(sketch) = profile_config.sketches.iter().find(|s| s.id == *profile_id) {
                let device = profile_config.devices.iter().find(|d| d.id == sketch.device);
                let connection = profile_config.connections.iter().find(|c| c.id == sketch.connection);
                if let (Some(device), Some(connection)) = (device, connection) {
                    let sketch_path_buf = std::path::PathBuf::from(&sketch.path);
                    let sketch_directory = sketch_path_buf.parent().map(|p| p.to_string_lossy().into_owned()).unwrap_or_else(|| "".to_string());
                    let sketch_name = sketch_path_buf.file_stem().and_then(|s| s.to_str()).unwrap_or("sketch").to_string();
                    return Ok(crate::commands::Settings {
                        sketch_directory,
                        sketch_name,
                        fqbn: device.fbqn.clone(),
                        port: connection.port.clone(),
                        baudrate: connection.baudrate,
                        board_model: device.board_model.clone(),
                        env: if connection.compiler == "arduino-cli" { "arduino" } else { "windows" }.to_string(),
                    });
                }
            }
        }
        crate::config::load_command_settings().map_err(|e| color_eyre::eyre::eyre!(e))
    }

    fn dispatch_command(&mut self, action: Action) {
        // [INPUT DECOUPLING] All bindings now flow through here.
        // Append dispatch info to the raw physical info already in last_raw_input
        self.last_raw_input = format!("{} >> ACTION: {:?}", self.last_raw_input, action);
        self.should_redraw = true;

        match action {
            Action::Quit => self.exec_quit(),
            Action::NextProfile => self.exec_next_profile(),
            Action::PrevProfile => self.exec_prev_profile(),
            Action::NextTab => self.exec_next_tab(),
            Action::PrevTab => self.exec_prev_tab(),
            Action::ScrollLineUp => self.exec_scroll_line_up(),
            Action::ScrollLineDown => self.exec_scroll_line_down(),
            Action::ScrollPageUp => self.exec_scroll_page_up(),
            Action::ScrollPageDown => self.exec_scroll_page_down(),
            Action::ScrollOutputToTop => self.exec_scroll_top(),
            Action::ScrollOutputToBottom => self.exec_scroll_bottom(),
            Action::ToggleAutoscroll => self.exec_toggle_autoscroll(),
            Action::ToggleInput => self.exec_toggle_input(),
            Action::CopyStatus => self.exec_copy_status(),
            Action::CopyOutputVisible => self.exec_copy_output(false),
            Action::CopyOutputFull => self.exec_copy_output(true),
            Action::CommandsUp => self.exec_commands_up(),
            Action::CommandsDown => self.exec_commands_down(),
            Action::Execute => self.exec_execute_selected_command(),
            Action::Cancel => self.exec_cancel(),
            Action::Compile => self.exec_compile(),
            Action::Upload => self.exec_upload(),
            Action::MonitorSerial => self.exec_monitor_serial(),
            Action::MonitorMqtt => self.exec_monitor_mqtt(),
            Action::Clean => self.exec_clean(),
        }
    }
    
                                
    fn push_line(&mut self, line: String) {
        let cached = crate::app::ansi::parse_ansi_line(&line);
        self.output_lines.push(line);
        self.output_cached_lines.push(cached);

        if self.output_lines.len() > MAX_OUTPUT_LINES {
            let to_remove = self.output_lines.len() - MAX_OUTPUT_LINES;
            self.output_lines.drain(0..to_remove);
            self.output_cached_lines.drain(0..to_remove);
        }
        self.should_redraw = true;
        self.sync_autoscroll();
    }

    pub fn log(&mut self, kind: &str, message: &str) {
        let formatted = self.theme.format_message(kind, message);
        self.push_line(formatted);
    }

    pub fn sync_autoscroll(&mut self) {
        if self.output_autoscroll {
            let layout = self.calculate_layout(self.view_area);
            let total_count = self.output_lines.len() + 1;
            let visible_height = layout.output.height.saturating_sub(2) as usize;
            self.output_scroll = total_count.saturating_sub(visible_height) as u16;
        }
    }

    pub fn check_terminal_size(&mut self, area: Rect) {
        self.terminal_too_small = area.width < self.config.application.min_width || area.height < self.config.application.min_height;
    }

    /// Returns the ID of the currently selected sketch profile
    pub fn get_current_sketch_id(&self) -> Option<String> {
        self.profile_ids.get(self.selected_profile_index).cloned()
    }

    /// Loads history and creates a predictor with optimized weights
    fn train_predictor(&self) -> crate::commands::ProgressPredictor {
        let history_path = std::path::Path::new(".dev-console/progress_history.json");
        let manager = crate::commands::HistoryManager::load(history_path);
        let stats = self.get_current_sketch_id()
            .and_then(|id| manager.get_stats(&id));
        
        crate::commands::ProgressPredictor::with_stats(stats)
    }

    pub fn is_task_running(&self) -> bool {
        matches!(self.task_state, TaskState::Running { .. })
    }

    pub fn is_toast_animating(&self) -> bool {
        !self.toast_manager.toasts.is_empty()
    }

    /// Unified error reporting pipeline
    pub fn report_error(&mut self, e: impl std::fmt::Display) {
        let msg = format!("{}", e);
        self.status_text = format!("[Error] {}", msg);
        self.log("error", &msg);
        self.toast_manager.error(&msg);
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Message {
    Key(event::KeyEvent),
    Mouse(event::MouseEvent),
    SystemUpdate(ProgressUpdate),
    Resize(u16, u16),
}
