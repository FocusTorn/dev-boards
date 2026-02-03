use crate::commands::ProgressUpdate;
use crate::config::ProfileConfig;
mod executors;
mod system;
mod view;
mod ansi;
pub mod theme;

use crate::app::theme::Theme;

use crate::widgets::command_list::CommandListWidget;
use crate::widgets::file_browser::FileBrowser;
use crate::widgets::popup::Popup;
use crate::widgets::smooth_scrollbar::{
    ScrollBar, ScrollBarInteraction, ScrollCommand, ScrollEvent, ScrollLengths,
};
use crate::widgets::toast::ToastManager;
use crate::widgets::{WidgetOutcome, InteractiveWidget};
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

/// Semantic actions that can be triggered by user input or system events.
///>
/// This enum maps human-readable strings from configuration files to internal 
/// executable functions. It is the primary means of decoupling input 
/// (keys/mouse) from application behavior.
///<
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
    SettingsUp,
    SettingsDown,
    #[strum(serialize = "execute", serialize = "commands_execute")]
    Execute,
    ToggleFocus,
    ProfileNew,
    ProfileClone,
    ProfileDelete,
    ProfileSave,
    Cancel,
}

impl Action {
    /// Attempts to parse a string into an `Action` variant.
    pub fn from_str(s: &str) -> Option<Self> {
        <Self as std::str::FromStr>::from_str(s).ok()
    }
}

/// Category of hardware monitor currently active.
#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum MonitorType {
    Serial,
    Mqtt,
}

/// Modes for handling list selection/highlighting.
#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum DispatchMode {
    /// Action triggers only on Enter or Click.
    OnSelect,
    /// Action triggers as the user cycles through items with Up/Down.
    OnHighlight,
}

/// UI focus regions for keyboard navigation.
#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Focus {
    Sidebar,
    Content,
}

/// Represents the current state of a background task or monitoring process.
///>
/// The `Running` variant includes metrics used for progress smoothing and 
/// ETA calculation, while `Monitoring` tracks long-lived hardware connections.
///<
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

/// Spatial coordinates for primary UI regions.
///>
/// This structure is cached on the `App` struct and recalculated only when 
/// the terminal is resized, ensuring high-performance rendering.
///<
#[derive(Debug, Clone, Copy, Default)]
pub struct AppLayout {
    pub title: Rect,
    pub main: Rect,
    pub bindings: Rect,
    pub status_bar: Rect,
    pub profile: Rect,
    pub commands: Rect,
    pub status: Rect,
    pub output: Rect,
    pub settings: Option<SettingsLayout>,
}

/// Layout specifically for the Settings/Profiles tab.
#[derive(Debug, Clone, Copy, Default)]
pub struct SettingsLayout {
    pub sidebar: Rect,
    pub content: Rect,
    pub field_areas: [Rect; 4],
    pub icon_areas: [Option<Rect>; 4],
}

/// The central application state following the Elm Architecture (Model).
///>
/// `App` owns all UI components, background task channels, and configuration 
/// state. It acts as the coordinator between user input and hardware 
/// execution.
///<
#[derive(Debug)]
pub struct App {
    pub running: bool,
    config: crate::config::Config,
    tab_bar_map: HashMap<String, crate::config::TabBarConfig>,
    terminal_too_small: bool,
    commands: Vec<String>,
    selected_command_index: usize,
    hovered_command_index: Option<usize>,
    command_index_before_hover: Option<usize>,
    settings_categories: Vec<String>,
    selected_settings_category_index: usize,
    selected_field_index: usize,
    hovered_field_index: Option<usize>,
    field_index_before_hover: Option<usize>,
    icon_focused: bool,
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
    pub profile_config_path: String,
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
    pub dispatch_mode: DispatchMode,
    pub focus: Focus,

    modal: Option<Popup<FileBrowser>>,

    // Input state
    pub input: tui_input::Input,
    pub input_active: bool,
    pub serial_tx: Option<mpsc::Sender<crate::commands::SerialCommand>>,
    pub mqtt_tx: Option<mpsc::Sender<crate::commands::MqttCommand>>,

    pub output_button_bar: crate::widgets::components::button_bar::button_bar::ButtonBar,
    pub main_tab_bar: crate::widgets::components::tabbed_bar::tabbed_bar::TabbedBar,
    pub profile_id_button_bar: crate::widgets::components::button_bar::button_bar::ButtonBar,
}

impl App {
    /// Initializes a new application state from configuration files.
    ///>
    /// Loads the main UI configuration, hardware profiles, and widget 
    /// settings. If configuration loading fails, errors are captured and 
    /// displayed in the initial output log.
    ///<
    pub fn new() -> Result<Self> {
        let config = crate::config::load_config()?;
        
        let mut tab_bar_map: HashMap<String, crate::config::TabBarConfig> = HashMap::new();
        for tb in config.tab_bars.iter() {
            tab_bar_map.insert(tb.id.clone(), tb.clone());
        }

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

        let mut output_button_bar = crate::widgets::components::button_bar::button_bar::ButtonBar::new("OutputPanelStaticOptions")?;
        output_button_bar.set_active("autoscroll", output_autoscroll);

        let mut main_tab_bar = crate::widgets::components::tabbed_bar::tabbed_bar::TabbedBar::new("MainContentTabBar")?;
        
        // Find initial active tab from config
        if let Some(tab_config) = config.tab_bars.iter().find(|t| t.id == "MainContentTabBar") {
            if let Some(active_id) = tab_config.tabs.iter().find(|t| t.default == Some("active".to_string())).map(|t| t.id.clone()) {
                main_tab_bar.set_active(&active_id);
            }
        }

        let profile_id_button_bar = crate::widgets::components::button_bar::button_bar::ButtonBar::new("ProfileIDButtonBar")?;

        Ok(Self {
            running: true,
            config,
            tab_bar_map,
            terminal_too_small: false,
            commands,
            selected_command_index: 0,
            hovered_command_index: None,
            command_index_before_hover: None,
            settings_categories: vec![
                "Device".to_string(),
                "MQTT".to_string(),
                "Paths".to_string(),
            ],
            selected_settings_category_index: 0,
            selected_field_index: 0,
            hovered_field_index: None,
            field_index_before_hover: None,
            icon_focused: false,
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
            profile_config_path: "config.yaml".to_string(),
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
                settings: None,
            },
            theme: app_theme,
            predictor: crate::commands::ProgressPredictor::new(),
            last_raw_input: String::new(),
            last_frame_time: Instant::now(),
            should_redraw: true,
            dispatch_mode: DispatchMode::OnSelect,
            focus: Focus::Sidebar,
            modal: None,
            input: tui_input::Input::default(),
            input_active: false,
            serial_tx: None,
            mqtt_tx: None,
            output_button_bar,
            main_tab_bar,
            profile_id_button_bar,
        })
    }

    /// Recalculates the geometry of all UI regions based on available area.
    ///>
    /// This method partitions the terminal into semantic blocks (Title, Sidebar, 
    /// Output, etc.) while respecting widget-specific height requirements 
    /// defined in the configuration.
    ///< 
    pub fn calculate_layout(&self, area: Rect) -> AppLayout {
        let vertical_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(2),
        ]);

        let [title, main, bindings, status_bar] = vertical_layout.areas(area);

        let active_tab_id = self.main_tab_bar.get_active_id().unwrap_or_else(|| "dashboard".to_string());

        if active_tab_id == "profiles" {
            let inner_main = self.main_tab_bar.get_content_area(main);
            let settings = self.calculate_settings_layout(inner_main);
            AppLayout {
                title,
                main,
                bindings,
                status_bar,
                profile: Rect::default(),
                commands: Rect::default(),
                status: Rect::default(),
                output: Rect::default(),
                settings: Some(settings),
            }
        } else {
            let inner_main = self.main_tab_bar.get_content_area(main);
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
                settings: None,
            }
        }
    }

    /// Calculates the partitioning for the Settings/Profiles tab.
    pub fn calculate_settings_layout(&self, area: Rect) -> SettingsLayout {
        let [sidebar, spacer, content] = Layout::horizontal([
            Constraint::Length(25),
            Constraint::Length(3), // 3 column gap
            Constraint::Min(0),
        ])
        .areas(area);

        let vertical_layout = Layout::vertical([
            Constraint::Length(1),  // Alignment offset
            Constraint::Length(2),  // Header
            Constraint::Min(0),     // Settings fields
        ]);
        let chunks = vertical_layout.split(content);
        let fields_area = chunks[2];

        let settings_layout = Layout::vertical([
            Constraint::Length(5), // Field 0
            Constraint::Length(5), // Field 1
            Constraint::Length(5), // Field 2
            Constraint::Length(5), // Field 3
            Constraint::Min(0),
        ]);
        let field_chunks = settings_layout.split(fields_area);

        let mut field_areas = [Rect::default(); 4];
        let mut icon_areas = [None; 4];

        for i in 0..4 {
            let row_area = field_chunks[i];
            let row_layout = Layout::vertical([
                Constraint::Length(1), // Label
                Constraint::Length(1), // Description
                Constraint::Length(3), // Input row
            ]);
            let row_chunks = row_layout.split(row_area);
            let input_row = row_chunks[2];

            let horizontal_chunks = Layout::horizontal([
                Constraint::Percentage(50), // Input width
                Constraint::Length(4),      // Action Icon space
                Constraint::Min(0),         // Spacer
            ]).split(input_row);

            field_areas[i] = horizontal_chunks[0];
            if i == 1 { // Sketch Path has icon
                icon_areas[i] = Some(horizontal_chunks[1]);
            }
        }

        SettingsLayout {
            sidebar,
            content,
            field_areas,
            icon_areas,
        }
    }

    /// Primary Update Loop - Decoupled from physical input interpretation.
    ///>
    /// Processes messages from user input or system updates and transitions 
    /// the application state accordingly.
    ///< 
    pub fn update(&mut self, msg: Message) {
        self.should_redraw = true;
        
        match msg {
            // Raw Inputs -> Route through interpretation dispatcher
            Message::Key(key_event) => {
                self.dispatch_key(key_event);
            }
            Message::Mouse(mouse_event) => {
                self.dispatch_mouse(mouse_event);
            }
            Message::SystemUpdate(update) => {
                self.exec_system_update(update);
            }
            
            // System Actions
            Message::Resize(w, h) => {
                self.should_redraw = true;
                let new_area = Rect::new(0, 0, w, h);
                self.view_area = new_area;
                self.layout = self.calculate_layout(new_area);
                self.check_terminal_size(new_area);
                self.sync_autoscroll();
            }
        }
    }

    /// Formats key modifiers into a display string (e.g., "Ctrl+Shift").
    fn get_modifiers_display(&self, mods: KeyModifiers) -> String {
        let mut parts = Vec::new();
        if mods.contains(KeyModifiers::CONTROL) { parts.push("Ctrl"); }
        if mods.contains(KeyModifiers::ALT) { parts.push("Alt"); }
        if mods.contains(KeyModifiers::SHIFT) { parts.push("Shift"); }
        if parts.is_empty() { "None".to_string() } else { parts.join("+") }
    }

    /// Event Dispatcher: Translates physical keyboard input into semantic actions.
    ///>
    /// Handles input field capture, global hotkeys, and context-sensitive 
    /// bindings defined in `build-config.yaml`.
    ///< 
    fn dispatch_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press { return; }

        let mods_str = self.get_modifiers_display(key.modifiers);
        self.last_raw_input = format!("KEY: {:?} | MODS: [{}]", key.code, mods_str);
        self.should_redraw = true;

        if self.input_active {
            use tui_input::backend::crossterm::EventHandler;
            match key.code {
                KeyCode::Enter => {
                    let active_tab_id = self.main_tab_bar.get_active_id().unwrap_or_else(|| "dashboard".to_string());
                    
                    if active_tab_id == "profiles" {
                        self.exec_settings_finish_edit();
                    } else {
                        self.exec_send_command();
                    }
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

        // 0. Modal Handling (Priority)
        if let Some(modal) = &mut self.modal {
            match modal.handle_key(key) {
                WidgetOutcome::Consumed | WidgetOutcome::Changed(_) => {
                    return;
                }
                WidgetOutcome::Confirmed(path) => {
                    // Update settings based on context
                    let active_tab_id = self.main_tab_bar.get_active_id().unwrap_or_else(|| "dashboard".to_string());
                    
                    if active_tab_id == "profiles" && self.selected_field_index == 1 {
                        let path_str = path.to_string_lossy().to_string();
                        let current_profile_id = self.get_current_sketch_id();
                        if let (Some(config), Some(profile_id)) = (&mut self.profile_config, current_profile_id) {
                            if let Some(sketch) = config.sketches.iter_mut().find(|s| s.id == profile_id) {
                                sketch.path = path_str.clone();
                                self.log("info", &format!("Updated sketch path to: {}", path_str));
                            }
                        }
                    } else {
                        let msg = format!("Confirmed path: {:?}", path);
                        self.log("info", &msg);
                    }
                    self.modal = None;
                    return;
                }
                WidgetOutcome::Canceled => {
                    self.modal = None;
                    return;
                }
                WidgetOutcome::None => {}
            }
        }

        let active_tab_id = self.main_tab_bar.get_active_id().unwrap_or_else(|| "dashboard".to_string());
        
        // 1. Tab-specific Override (e.g. Profiles navigation)
        if active_tab_id == "profiles" {
            if self.key_matches(key, "[Up]") {
                self.dispatch_command(Action::SettingsUp);
                return;
            }
            if self.key_matches(key, "[Down]") {
                self.dispatch_command(Action::SettingsDown);
                return;
            }
            if self.key_matches(key, "[Left]") || self.key_matches(key, "[Shift+Tab]") {
                self.last_raw_input = format!("{} >> ACTION: SettingsPrevField", self.last_raw_input);
                self.exec_settings_prev_field();
                return;
            }
            if self.key_matches(key, "[Right]") || self.key_matches(key, "[Tab]") {
                self.last_raw_input = format!("{} >> ACTION: SettingsNextField", self.last_raw_input);
                self.exec_settings_next_field();
                return;
            }
            if self.key_matches(key, "[Enter]") && self.focus == Focus::Content {
                if self.icon_focused {
                    self.last_raw_input = format!("{} >> ACTION: SettingsAction", self.last_raw_input);
                    self.exec_settings_action();
                } else {
                    self.last_raw_input = format!("{} >> ACTION: SettingsEdit", self.last_raw_input);
                    self.exec_settings_edit();
                }
                return;
            }
            if self.key_matches(key, "f") && self.focus == Focus::Content {
                self.last_raw_input = format!("{} >> ACTION: SettingsAction", self.last_raw_input);
                self.exec_settings_action();
                return;
            }
        }

        // 2. Hardcoded Tab handling for focus switching (Fallback)
        if self.key_matches(key, "[Tab]") {
            self.dispatch_command(Action::ToggleFocus);
            return;
        }

        // 3. Tab Bar Navigation Bindings
        if let Some(tab_bar_config) = self.tab_bar_map.get("MainContentTabBar") {
            for key_str in &tab_bar_config.navigation.left {
                if self.key_matches(key, key_str) { 
                    self.dispatch_command(Action::PrevTab); 
                    return; 
                }
            }
            for key_str in &tab_bar_config.navigation.right {
                if self.key_matches(key, key_str) { 
                    self.dispatch_command(Action::NextTab); 
                    return; 
                }
            }
            
            // 1.1 Tab-specific Semantic Bindings
            if let Some(active_tab_id) = self.main_tab_bar.get_active_id() {
                if let Some(bindings_config) = tab_bar_config.tab_bindings.get(&active_tab_id) {
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

    /// Event Dispatcher: Translates physical mouse input into semantic actions.
    ///>
    /// Coordinates interactions between modular widgets (TabBars, Scrollbars, 
    /// CommandLists) and provides global hover/click detection for status 
    /// and output regions.
    ///< 
    pub fn dispatch_mouse(&mut self, mouse_event: event::MouseEvent) {
        // 0. Handle Modal Mouse Input (Priority)
        if let Some(modal) = &mut self.modal {
            match modal.handle_mouse(mouse_event, self.view_area) {
                WidgetOutcome::Consumed | WidgetOutcome::Changed(_) => {
                    self.should_redraw = true;
                    return;
                }
                WidgetOutcome::Confirmed(path) => {
                    let active_tab_id = self.main_tab_bar.get_active_id().unwrap_or_else(|| "dashboard".to_string());
                    
                    if active_tab_id == "profiles" && self.selected_field_index == 1 {
                        let path_str = path.to_string_lossy().to_string();
                        let current_profile_id = self.get_current_sketch_id();
                        if let (Some(config), Some(profile_id)) = (&mut self.profile_config, current_profile_id) {
                            if let Some(sketch) = config.sketches.iter_mut().find(|s| s.id == profile_id) {
                                sketch.path = path_str.clone();
                                self.log("info", &format!("Updated sketch path to: {}", path_str));
                            }
                        }
                    }
                    self.modal = None;
                    self.should_redraw = true;
                    return;
                }
                WidgetOutcome::Canceled => {
                    self.modal = None;
                    self.should_redraw = true;
                    return;
                }
                WidgetOutcome::None => {
                    // Block base UI if mouse is outside modal but modal is open? 
                    // Usually yes, modals are "capture-all"
                    return;
                }
            }
        }

        // RAW LOGGING (Ignore Move noise)
        if mouse_event.kind != event::MouseEventKind::Moved {
            let mods_str = self.get_modifiers_display(mouse_event.modifiers);
            self.last_raw_input = format!("MOUSE: {:?} | MODS: [{}]", mouse_event.kind, mods_str);
            self.should_redraw = true;
        }

        let mouse_pos = Position::new(mouse_event.column, mouse_event.row);
        let layout = self.layout;

        // 1. Tab Bar Widget Interactions
        if let WidgetOutcome::Confirmed(id) = self.main_tab_bar.handle_mouse(mouse_event, layout.main) {
            self.main_tab_bar.set_active(&id);
            // Recalculate layout for the new tab
            self.layout = self.calculate_layout(self.view_area);
            self.should_redraw = true;
            return; 
        }

        // 1.1 Settings Tab Hit Detection
        let active_tab_id = self.main_tab_bar.get_active_id().unwrap_or_else(|| "dashboard".to_string());

        if active_tab_id == "profiles" {
            if let Some(settings_layout) = layout.settings {
                // Sidebar Category Selection
                if settings_layout.sidebar.contains(mouse_pos) {
                    let sidebar_inner = Block::bordered().inner(settings_layout.sidebar);
                    if sidebar_inner.contains(mouse_pos) {
                        let relative_y = mouse_pos.y.saturating_sub(sidebar_inner.y);
                        if (relative_y as usize) < self.settings_categories.len() {
                            self.selected_settings_category_index = relative_y as usize;
                            self.focus = Focus::Sidebar;
                            self.selected_field_index = 0;
                            self.icon_focused = false;
                            self.should_redraw = true;
                            return;
                        }
                    }
                }

                // Content Field Selection
                if settings_layout.content.contains(mouse_pos) {
                    let mut found_hit = false;
                    for i in 0..4 {
                        // Check Button Bar hit if it's the first field (Profile ID)
                        if i == 0 {
                            if let WidgetOutcome::Confirmed(btn_id) = self.profile_id_button_bar.handle_mouse(mouse_event, settings_layout.field_areas[0]) {
                                match btn_id.as_str() {
                                    "save" => self.dispatch_command(Action::ProfileSave),
                                    "delete" => self.dispatch_command(Action::ProfileDelete),
                                    _ => {}
                                }
                                return;
                            }
                        }

                        // Check Field Click/Hover
                        if settings_layout.field_areas[i].contains(mouse_pos) {
                            found_hit = true;
                            if let event::MouseEventKind::Down(event::MouseButton::Left) = mouse_event.kind {
                                self.selected_field_index = i;
                                self.icon_focused = false;
                                self.focus = Focus::Content;
                                self.field_index_before_hover = None;
                                self.exec_settings_edit();
                            } else {
                                // Hover
                                if self.hovered_field_index.is_none() {
                                    self.field_index_before_hover = Some(self.selected_field_index);
                                }
                                if self.hovered_field_index != Some(i) {
                                    self.hovered_field_index = Some(i);
                                    self.selected_field_index = i;
                                    self.icon_focused = false;
                                    self.should_redraw = true;
                                }
                            }
                            break;
                        }
                        // Check Icon Click/Hover
                        if let Some(icon_area) = settings_layout.icon_areas[i] {
                            if icon_area.contains(mouse_pos) {
                                found_hit = true;
                                if let event::MouseEventKind::Down(event::MouseButton::Left) = mouse_event.kind {
                                    self.selected_field_index = i;
                                    self.icon_focused = true;
                                    self.focus = Focus::Content;
                                    self.field_index_before_hover = None;
                                    self.exec_settings_action();
                                } else {
                                    // Hover
                                    if self.hovered_field_index.is_none() {
                                        self.field_index_before_hover = Some(self.selected_field_index);
                                    }
                                    if self.hovered_field_index != Some(i) || !self.icon_focused {
                                        self.hovered_field_index = Some(i);
                                        self.selected_field_index = i;
                                        self.icon_focused = true;
                                        self.should_redraw = true;
                                    }
                                }
                                break;
                            }
                        }
                    }

                    if !found_hit {
                        if let Some(old_idx) = self.field_index_before_hover {
                            self.selected_field_index = old_idx;
                            self.field_index_before_hover = None;
                            self.icon_focused = false;
                            self.should_redraw = true;
                        }
                        if self.hovered_field_index.is_some() {
                            self.hovered_field_index = None;
                            self.should_redraw = true;
                        }
                    }
                    if found_hit { return; }
                }
            }
        }

        // 1.2 Output Panel "Auto" Toggle
        if let WidgetOutcome::Confirmed(id) = self.output_button_bar.handle_mouse(mouse_event, layout.output) {
            if id == "autoscroll" {
                self.dispatch_command(Action::ToggleAutoscroll);
                return;
            }
        }

        // 2. Command List Mouse Interaction
        let command_list = CommandListWidget::new(&self.commands, self.selected_command_index, self.hovered_command_index);
        match command_list.handle_mouse_event(layout.commands, mouse_event) {
            Some(crate::widgets::command_list::CommandListInteraction::Click(idx)) => {
                self.selected_command_index = idx;
                self.command_index_before_hover = None; 
                self.dispatch_command(Action::Execute);
                self.should_redraw = true;
            }
            Some(crate::widgets::command_list::CommandListInteraction::Hover(idx)) => {
                if self.hovered_command_index.is_none() {
                    self.command_index_before_hover = Some(self.selected_command_index);
                }
                if self.hovered_command_index != Some(idx) {
                    self.hovered_command_index = Some(idx);
                    self.should_redraw = true;
                }
            }
            None => {
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

        // 3. Status/Output Box Region Interactivity
        match mouse_event.kind {
            event::MouseEventKind::Down(_) => {
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
            }
            _ => {}
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

    /// Checks if a physical key event matches a string binding (e.g., "[Ctrl+Q]").
    fn key_matches(&self, key: event::KeyEvent, binding_key: &str) -> bool {
        let binding_lower = binding_key.to_lowercase();
        let inner = binding_lower.trim_matches(|c| c == '[' || c == ']');
        let parts: Vec<String> = inner.split('+').map(|s| s.to_string()).collect();
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
            "pgup" | "pg_up" | "pageup" | "page_up" => matches!(key.code, KeyCode::PageUp),
            "pgdn" | "pg_dn" | "pgdown" | "pagedown" | "page_down" => matches!(key.code, KeyCode::PageDown),
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

    /// Resolves the current hardware settings based on the active profile.
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

    /// Dispatches a semantic action to its corresponding executor.
    ///>
    /// This is the final stage of input routing, ensuring all commands flow 
    /// through a single centralized bottleneck for logging and state tracking.
    ///< 
    fn dispatch_command(&mut self, action: Action) {
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
            Action::SettingsUp => self.exec_settings_up(),
            Action::SettingsDown => self.exec_settings_down(),
            Action::Execute => self.exec_execute_selected_command(),
            Action::ToggleFocus => self.exec_toggle_focus(),
            Action::ProfileNew => self.exec_profile_new(),
            Action::ProfileClone => self.exec_profile_clone(),
            Action::ProfileDelete => self.exec_profile_delete(),
            Action::ProfileSave => self.exec_profile_save(),
            Action::Cancel => self.exec_cancel(),
            Action::Compile => self.exec_compile(),
            Action::Upload => self.exec_upload(),
            Action::MonitorSerial => self.exec_monitor_serial(),
            Action::MonitorMqtt => self.exec_monitor_mqtt(),
            Action::Clean => self.exec_clean(),
        }
    }
    
    /// Internal helper for adding lines to the output buffer.
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

    /// Adds a themed message to the application log.
    pub fn log(&mut self, kind: &str, message: &str) {
        let formatted = self.theme.format_message(kind, message);
        self.push_line(formatted);
    }

    /// Recalculates output scroll offset if autoscroll is enabled.
    pub fn sync_autoscroll(&mut self) {
        if self.output_autoscroll {
            let layout = self.calculate_layout(self.view_area);
            let total_count = self.output_lines.len() + 1;
            let visible_height = layout.output.height.saturating_sub(2) as usize;
            self.output_scroll = total_count.saturating_sub(visible_height) as u16;
        }
    }

    /// Updates internal flag if terminal dimensions fall below minimums.
    pub fn check_terminal_size(&mut self, area: Rect) {
        self.terminal_too_small = area.width < self.config.application.min_width || area.height < self.config.application.min_height;
    }

    /// Returns the ID of the currently selected sketch profile.
    pub fn get_current_sketch_id(&self) -> Option<String> {
        self.profile_ids.get(self.selected_profile_index).cloned()
    }

    /// Loads history and creates a predictor with optimized weights.
    fn train_predictor(&self) -> crate::commands::ProgressPredictor {
        let history_path = std::path::Path::new(".dev-console/progress_history.json");
        let manager = crate::commands::HistoryManager::load(history_path);
        let stats = self.get_current_sketch_id()
            .and_then(|id| manager.get_stats(&id));
        
        crate::commands::ProgressPredictor::with_stats(stats)
    }

    /// Returns true if a build or upload task is currently executing.
    pub fn is_task_running(&self) -> bool {
        matches!(self.task_state, TaskState::Running { .. })
    }

    /// Returns true if any toast notifications are currently visible.
    pub fn is_toast_animating(&self) -> bool {
        !self.toast_manager.toasts.is_empty()
    }

    /// Unified error reporting pipeline for status bars, logs, and toasts.
    pub fn report_error(&mut self, e: impl std::fmt::Display) {
        let msg = format!("{}", e);
        self.status_text = format!("[Error] {}", msg);
        self.log("error", &msg);
        self.toast_manager.error(&msg);
    }

    /// Returns the number of settings fields in the currently selected category.
    fn get_active_settings_field_count(&self) -> usize {
        let category = self.settings_categories.get(self.selected_settings_category_index)
            .map(|s| s.as_str())
            .unwrap_or("");
        
        match category {
            "Device" => 4, // Profile ID, Sketch Path, Serial Port, Baud Rate
            "MQTT" => 5,   // Host, Port, Client ID, Username, Password (approx)
            "Paths" => 1,  // config.yaml path
            _ => 0,
        }
    }
}

/// Events that drive application state transitions.
#[derive(PartialEq, Debug, Clone)]
pub enum Message {
    Key(event::KeyEvent),
    Mouse(event::MouseEvent),
    SystemUpdate(ProgressUpdate),
    Resize(u16, u16),
}

#[cfg(test)]
mod tests;