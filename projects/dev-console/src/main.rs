// ESP32-S3 Dev Console
// TUI application for managing ESP32-S3 development settings

// IMPORTS ------------------>> 

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Scrollbar, ScrollbarState, ScrollbarOrientation},
};
use std::io;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use serde::{Deserialize, Serialize};
use tui_components::{
    BaseLayout, BaseLayoutConfig, BaseLayoutResult,
    BindingConfig, StatusBarConfig,
    DimmingContext, RectRegistry, Popup, render_popup,
    TabBar, TabBarStyle, RectHandle,
    Toast, ToastType, render_toasts,
    TabBarConfigYaml, TabBarManager,
    get_box_by_name,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui_input::{Input, InputRequest};
use serialport::available_ports;

//--------------------------------------------------------<<

// Static handle names (HWND IDs)
const HWND_MAIN_CONTENT_BOX: &str = "hwndMainContentBox";
const HWND_MAIN_CONTENT_TAB_BAR: &str = "hwndMainContentTabBar";

// Settings file path
fn get_settings_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("dev-console")
        .join("settings.yaml")
}

// Settings data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings { //>
    sketch_directory: String,
    sketch_name: String,
    env: String,
    board_model: String,
    fqbn: String,
    port: String,
    baudrate: u32,
    create_log: bool,
} //<

impl Default for Settings { //>
    fn default() -> Self {
        Self {
            sketch_directory: "".to_string(),
            sketch_name: "".to_string(),
            env: "arduino".to_string(),
            board_model: "esp32-s3".to_string(),
            fqbn: "esp32:esp32:esp32s3".to_string(),
            port: "COM9".to_string(),
            baudrate: 115200,
            create_log: false,
        }
    }
} //<

impl Settings { //>
    fn load() -> Self {
        let path = get_settings_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(settings) = serde_yaml::from_str::<Settings>(&contents) {
                return settings;
            }
        }
        Self::default()
    }
    
    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = get_settings_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = serde_yaml::to_string(self)?;
        fs::write(&path, contents)?;
        Ok(())
    }
} //<

// Settings field editor state
#[derive(Debug, Clone)]
enum FieldEditorState { //>
    Selected {
        field_index: usize,
    },
    Editing {
        field_index: usize,
        input: Input,
    },
    Selecting {
        field_index: usize,
        selected_index: usize,
        options: Vec<String>,
    },
} //<

// Dashboard state
#[derive(Debug, Clone)]
struct DashboardState { //>
    commands: Vec<String>,
    selected_command: usize,
    status_text: String,
    output_lines: Vec<String>,
    output_scroll: usize,
    // Progress tracking
    is_running: bool,
    progress_percent: f64,
    progress_stage: String,
    current_file: String,
} //<

impl DashboardState { //>
    fn new() -> Self {
        Self {
            commands: vec![
                "Build".to_string(),
                "Compile".to_string(),
                "Progress".to_string(),
                "Progress-new".to_string(),
                "Upload".to_string(),
                "Upload_custom".to_string(),
                "Monitor".to_string(),
                "Clean".to_string(),
                "All".to_string(),
                "Help".to_string(),
            ],
            selected_command: 0,
            status_text: "Ready".to_string(),
            output_lines: Vec::new(),
            output_scroll: 0,
            is_running: false,
            progress_percent: 0.0,
            progress_stage: String::new(),
            current_file: String::new(),
        }
    }
    
    fn scroll_output_up(&mut self, amount: usize) {
        if self.output_scroll > 0 {
            self.output_scroll = self.output_scroll.saturating_sub(amount);
        }
    }
    
    fn scroll_output_down(&mut self, amount: usize) {
        let max_scroll = self.output_lines.len().saturating_sub(1);
        if self.output_scroll < max_scroll {
            self.output_scroll = (self.output_scroll + amount).min(max_scroll);
        }
    }
} //<

// Settings fields
struct SettingsFields { //>
    labels: Vec<&'static str>,
    getters: Vec<Box<dyn Fn(&Settings) -> String>>,
    setters: Vec<Box<dyn Fn(&mut Settings, String)>>,
} //<

impl SettingsFields { //>
    fn new() -> Self {
        let mut labels = Vec::new();
        let mut getters: Vec<Box<dyn Fn(&Settings) -> String>> = Vec::new();
        let mut setters: Vec<Box<dyn Fn(&mut Settings, String)>> = Vec::new();
        
        // Sketch Directory
        labels.push("Sketch Directory");
        getters.push(Box::new(|s| s.sketch_directory.clone()));
        setters.push(Box::new(|s, v| s.sketch_directory = v));
        
        // Sketch Name
        labels.push("Sketch Name");
        getters.push(Box::new(|s| s.sketch_name.clone()));
        setters.push(Box::new(|s, v| s.sketch_name = v));
        
        // Environment (arduino, esp-idf)
        labels.push("Environment");
        getters.push(Box::new(|s| s.env.clone()));
        setters.push(Box::new(|s, v| s.env = v));
        
        // Board Model (esp32-s3, ard-nano)
        labels.push("Board Model");
        getters.push(Box::new(|s| s.board_model.clone()));
        setters.push(Box::new(|s, v| s.board_model = v));
        
        // FQBN
        labels.push("FQBN");
        getters.push(Box::new(|s| s.fqbn.clone()));
        setters.push(Box::new(|s, v| s.fqbn = v));
        
        // Port
        labels.push("Port");
        getters.push(Box::new(|s| s.port.clone()));
        setters.push(Box::new(|s, v| s.port = v));
        
        // Baudrate
        labels.push("Baudrate");
        getters.push(Box::new(|s| s.baudrate.to_string()));
        setters.push(Box::new(|s, v| {
            if let Ok(b) = v.parse::<u32>() {
                s.baudrate = b;
            }
        }));
        
        Self {
            labels,
            getters,
            setters,
        }
    }
    
    fn get_value(&self, settings: &Settings, index: usize) -> String {
        (self.getters[index])(settings)
    }
    
    fn set_value(&self, settings: &mut Settings, index: usize, value: String) {
        (self.setters[index])(settings, value);
    }
    
    fn count(&self) -> usize {
        self.labels.len()
    }
    
    /// Check if a field is a dropdown field
    fn is_dropdown(&self, index: usize) -> bool {
        // Environment (index 2) and Port (index 5) are dropdowns
        index == 2 || index == 5
    }
    
    /// Get dropdown options for a field
    fn get_dropdown_options(&self, index: usize) -> Vec<String> {
        match index {
            2 => {
                // Environment dropdown
                vec!["arduino".to_string(), "esp-idf".to_string()]
            }
            5 => {
                // Port dropdown - detect available COM ports
                match available_ports() {
                    Ok(ports) => {
                        ports.into_iter()
                            .map(|p| {
                                // On Windows, ports are like "COM1", "COM9", etc.
                                p.port_name
                            })
                            .collect()
                    }
                    Err(_) => {
                        // Fallback to common ports if detection fails
                        vec!["COM1".to_string(), "COM3".to_string(), "COM5".to_string(), "COM7".to_string(), "COM9".to_string()]
                    }
                }
            }
            _ => vec![],
        }
    }
} //<

// Type aliases for cleaner code
type TabBarConfig = TabBarConfigYaml;

#[derive(Debug, Clone, Deserialize)]
struct AppConfig { //>
    application: ApplicationConfig,
    #[serde(rename = "tab_bars")]
    tab_bars: std::collections::HashMap<String, TabBarConfig>,
} //<

#[derive(Debug, Clone, Deserialize)]
struct ApplicationConfig { //>
    title: String,
    bindings: Vec<BindingConfigYaml>,
    status_bar: StatusBarConfigYaml,
} //<

#[derive(Debug, Clone, Deserialize)]
struct BindingConfigYaml { //>
    key: String,
    description: String,
} //<

#[derive(Debug, Clone, Deserialize)]
struct StatusBarConfigYaml { //>
    default_text: String,
    #[serde(default)]
    modal_text: Option<String>,
} //<

fn load_config(config_path: Option<PathBuf>) -> Result<AppConfig, Box<dyn std::error::Error>> { //>
    let path = config_path.unwrap_or_else(|| {
        let mut default_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        default_path.push("config.yaml");
        default_path
    });
    
    let contents = fs::read_to_string(&path)?;
    let config: AppConfig = serde_yaml::from_str(&contents)?;
    Ok(config)
} //<

// ┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
// │                                                 MAIN ENTRY POINT                                                 │
// └──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut popup: Option<Popup> = None;
    let mut toasts: Vec<Toast> = Vec::new();
    let mut registry = RectRegistry::new();
    
    // Load settings
    let mut settings = Settings::load();
    let settings_fields = SettingsFields::new();
    let mut field_editor_state = FieldEditorState::Selected { field_index: 0 };
    
    // Dashboard state
    let mut dashboard_state = DashboardState::new();
    
    // Shared dashboard state for thread communication
    let dashboard_arc = Arc::new(Mutex::new(dashboard_state.clone()));
    
    // Load configuration from YAML file
    let app_config = match load_config(None) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config.yaml: {}", e);
            eprintln!("Using default configuration");
            return Err(e);
        }
    };
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Convert YAML bindings > BaseLayoutConfig bindings
    let global_bindings: Vec<BindingConfig> = app_config.application.bindings
        .iter()
        .map(|b| BindingConfig {
            key: b.key.clone(),
            description: b.description.clone(),
        })
        .collect();
    
    // Create base layout configuration from YAML
    let config = BaseLayoutConfig {
        title: app_config.application.title.clone(),
        tabs: vec![],
        global_bindings,
        status_bar: StatusBarConfig {
            default_text: app_config.application.status_bar.default_text.clone(),
            modal_text: app_config.application.status_bar.modal_text.clone(),
        },
    };
    
    // Look up tab bar config by handle name (HWND)
    let tab_bar_config = app_config.tab_bars.values()
        .find(|config| config.hwnd == HWND_MAIN_CONTENT_TAB_BAR)
        .ok_or_else(|| format!(
            "Tab bar with hwnd '{}' not found in config. Available tab bars: {}",
            HWND_MAIN_CONTENT_TAB_BAR,
            app_config.tab_bars.keys().map(|k| k.as_str()).collect::<Vec<_>>().join(", ")
        ))?;
    
    // Create and initialize tab bar manager
    let main_content_tab_bar = TabBarManager::create(&mut registry, HWND_MAIN_CONTENT_TAB_BAR, tab_bar_config);
    
    // Store current tab bar instance for click detection
    let mut current_tab_bar: Option<(TabBar, RectHandle)> = None;
    
    // Get tab style for style switching
    let tab_bar_state = registry.get_tab_bar_state(main_content_tab_bar.handle())
        .expect("Tab bar state should be initialized");
    let tab_style = TabBarStyle::from_str(&tab_bar_state.config.style);
    
    let main_content_box_handle_name = HWND_MAIN_CONTENT_BOX;
    let mut original_anchor_metrics: Option<Rect> = None;
    
    // ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
    // │                                           MAIN LOOP                                            │
    // └────────────────────────────────────────────────────────────────────────────────────────────────┘ 
          
    loop {
        terminal.draw(|f| {
            let area = f.area();
            
            // Create dimming context based on popup state or dropdown selection
            let is_selecting = matches!(field_editor_state, FieldEditorState::Selecting { .. });
            let dimming = DimmingContext::new(popup.is_some() || is_selecting);
            
            // Render base layout
            let base_layout = BaseLayout::new(
                &config,
                None,
                &dimming,
            );
            let result: BaseLayoutResult = base_layout.render(f, area, &mut registry);
            let content_area = result.content_area;
            
            // Store original anchor metrics after first render
            if original_anchor_metrics.is_none() {
                if let Some(anchor_handle) = registry.get_handle(main_content_box_handle_name) {
                    if let Some(metrics) = registry.get_metrics(anchor_handle) {
                        original_anchor_metrics = Some(metrics.into());
                    }
                } else {
                    original_anchor_metrics = Some(content_area);
                }
            }
            
            // Restore anchor position
            if let (Some(anchor_handle), Some(original_metrics)) = (registry.get_handle(main_content_box_handle_name), original_anchor_metrics) {
                registry.update(anchor_handle, original_metrics);
            }
            
            // Initialize or update main content bounding box
            if let Some(handle) = registry.get_handle(main_content_box_handle_name) {
                registry.update(handle, content_area);
            } else {
                let _handle = registry.register(Some(main_content_box_handle_name), content_area);
            }

            // Prepare tab bar
            let tab_bar_result = main_content_tab_bar.prepare(&mut registry, Some(tab_style));
            
            // Render content block
            let render_area = if let Some(box_manager) = get_box_by_name(&registry, main_content_box_handle_name) {
                box_manager.prepare(&mut registry).unwrap_or(content_area)
            } else {
                content_area
            };
            
            // Render the content box border
            render_content(f, render_area, &dimming);
            
            // Render tab content
            if let Some(active_tab_idx) = registry.get_active_tab(main_content_tab_bar.handle()) {
                if let Some(tab_bar_state) = registry.get_tab_bar_state(main_content_tab_bar.handle()) {
                    if let Some(tab_config) = tab_bar_state.tab_configs.get(active_tab_idx) {
                        // Create nested area for tab content (x+1, y+1, width-2, height-2 to account for borders)
                        let nested_area = Rect {
                            x: render_area.x.saturating_add(1),
                            y: render_area.y.saturating_add(1),
                            width: render_area.width.saturating_sub(2),
                            height: render_area.height.saturating_sub(2),
                        };
                        
                        if tab_config.id == "settings" {
                            render_settings(f, nested_area, &settings, &settings_fields, &field_editor_state);
                        } else if tab_config.id == "dashboard" {
                            // Sync dashboard_state from Arc right before rendering
                            if let Ok(state) = dashboard_arc.lock() {
                                dashboard_state = state.clone();
                            }
                            render_dashboard(f, nested_area, &mut dashboard_state);
                        }
                    }
                }
            }

            // Render tab bar
            if let Some((tab_bar, anchor_handle, tab_bar_state)) = tab_bar_result {
                tab_bar.render_with_state(f, &mut registry, &tab_bar_state, Some(&dimming));
                
                // Store tab bar for click detection
                current_tab_bar = Some((tab_bar, anchor_handle));
            }
            
            // Render dropdown overlay if selecting (after all widgets so it appears on top)
            if let FieldEditorState::Selecting { field_index, selected_index, options } = &field_editor_state {
                // Store field area in a variable we can access
                // We'll calculate it the same way render_settings does
                if let Some(box_manager) = get_box_by_name(&registry, HWND_MAIN_CONTENT_BOX) {
                    if let Some(content_rect) = box_manager.metrics(&registry) {
                        // Calculate centered content area (same as in render_settings)
                        let min_width_pixels = 80;
                        let min_height_pixels = 21;
                        // Check if terminal is large enough - only render dropdown if it is
                        if content_rect.width >= min_width_pixels && content_rect.height >= min_height_pixels {
                            let content_width = (content_rect.width * 50 / 100).max(min_width_pixels).min(content_rect.width);
                            let content_height = (content_rect.height * 50 / 100).max(min_height_pixels).min(content_rect.height);
                            let content_x = content_rect.x + (content_rect.width.saturating_sub(content_width)) / 2;
                            let content_y = content_rect.y + (content_rect.height.saturating_sub(content_height)) / 2;
                            
                            // Ensure content_area doesn't exceed bounds
                            let content_area = Rect {
                                x: content_x.min(content_rect.x + content_rect.width),
                                y: content_y.min(content_rect.y + content_rect.height),
                                width: content_width.min(content_rect.width.saturating_sub(content_x.saturating_sub(content_rect.x))),
                                height: content_height.min(content_rect.height.saturating_sub(content_y.saturating_sub(content_rect.y))),
                            };
                            
                            // Split into top section (Sketch Directory, Sketch Name) and bottom section (Device/Connection)
                            let main_chunks = Layout::default()
                                .direction(Direction::Vertical)
                                .constraints([
                                    Constraint::Length(6), // Top section: 2 boxes (3 lines each)
                                    Constraint::Min(0),   // Bottom section: Device/Connection
                                ])
                                .split(content_area);
                            
                            // Calculate field area based on field_index
                            let field_area = if *field_index < 2 {
                                // Top full-width fields
                                let top_chunks = Layout::default()
                                    .direction(Direction::Vertical)
                                    .constraints([
                                        Constraint::Length(3), // Sketch Directory
                                        Constraint::Length(3), // Sketch Name
                                    ])
                                    .split(main_chunks[0]);
                                top_chunks[*field_index]
                            } else if *field_index < 5 {
                                // Device column (left)
                                let bottom_chunks = Layout::default()
                                    .direction(Direction::Horizontal)
                                    .constraints([
                                        Constraint::Percentage(50), // Device column
                                        Constraint::Percentage(50), // Connection column
                                    ])
                                    .split(main_chunks[1]);
                                
                                // Device section: Environment (2), Board Model (3), FQBN (4)
                                let section_inner = Block::default().borders(Borders::ALL).inner(bottom_chunks[0]);
                                let field_height = 3;
                                let field_offset = (*field_index - 2) as u16 * (field_height + 1);
                                Rect {
                                    x: section_inner.x + 1,
                                    y: section_inner.y + 1 + field_offset,
                                    width: section_inner.width.saturating_sub(2),
                                    height: field_height as u16,
                                }
                            } else {
                                // Connection column (right)
                                let bottom_chunks = Layout::default()
                                    .direction(Direction::Horizontal)
                                    .constraints([
                                        Constraint::Percentage(50), // Device column
                                        Constraint::Percentage(50), // Connection column
                                    ])
                                    .split(main_chunks[1]);
                                
                                // Connection section: Port (5), Baudrate (6)
                                let section_inner = Block::default().borders(Borders::ALL).inner(bottom_chunks[1]);
                                let field_height = 3;
                                let field_offset = (*field_index - 5) as u16 * (field_height + 1);
                                Rect {
                                    x: section_inner.x + 1,
                                    y: section_inner.y + 1 + field_offset,
                                    width: section_inner.width.saturating_sub(2),
                                    height: field_height as u16,
                                }
                            };
                            
                            // Calculate dropdown position (below the field)
                            let dropdown_height = (options.len() + 2).min(10) as u16;
                            let dropdown_area = Rect {
                                x: field_area.x,
                                y: field_area.y + field_area.height,
                                width: field_area.width,
                                height: dropdown_height,
                            };
                            
                            // Make sure dropdown fits in the frame
                            let adjusted_dropdown_area = if dropdown_area.y + dropdown_area.height > area.height {
                                // If doesn't fit below, show above
                                Rect {
                                    x: dropdown_area.x,
                                    y: field_area.y.saturating_sub(dropdown_area.height),
                                    width: dropdown_area.width,
                                    height: dropdown_height,
                                }
                            } else {
                                dropdown_area
                            };
                            
                            // Render dropdown
                            let mut items = Vec::new();
                            for (i, option) in options.iter().enumerate() {
                                let style = if i == *selected_index {
                                    Style::default()
                                        .fg(Color::Rgb(255, 215, 0))
                                        .add_modifier(Modifier::BOLD | Modifier::REVERSED)
                                } else {
                                    Style::default()
                                        .fg(Color::White)
                                };
                                items.push(ListItem::new(Line::from(vec![
                                    Span::styled(option.clone(), style),
                                ])));
                            }
                            
                            let list = List::new(items)
                                .block(Block::default()
                                    .borders(Borders::ALL)
                                    .border_style(Style::default().fg(Color::Rgb(255, 215, 0))));
                            f.render_widget(Clear, adjusted_dropdown_area);
                            f.render_widget(list, adjusted_dropdown_area);
                        }
                    }
                }
            }
            
            if let Some(ref popup) = popup {
                render_popup(f, area, popup);
            }
            
            // Filter out expired toasts
            let now = std::time::SystemTime::now();
            toasts.retain(|toast| {
                if let Ok(duration) = now.duration_since(toast.shown_at) {
                    duration.as_secs_f64() < 1.5
                } else {
                    false
                }
            });
            
            render_toasts(f, area, &toasts);
            
            // Handle cursor positioning for editing fields
            if let FieldEditorState::Editing { field_index, ref input } = field_editor_state {
                if let Some(active_tab_idx) = registry.get_active_tab(main_content_tab_bar.handle()) {
                    if let Some(tab_bar_state) = registry.get_tab_bar_state(main_content_tab_bar.handle()) {
                        if let Some(tab_config) = tab_bar_state.tab_configs.get(active_tab_idx) {
                            if tab_config.id == "settings" {
                                // Calculate cursor position based on new layout
                                if let Some(box_manager) = get_box_by_name(&registry, HWND_MAIN_CONTENT_BOX) {
                                    if let Some(content_rect) = box_manager.metrics(&registry) {
                                        // Check if terminal is large enough - don't position cursor if warning is shown
                                        let min_width_pixels = 80;
                                        let min_height_pixels = 21;
                                        
                                        // Only position cursor if terminal is large enough
                                        if content_rect.width >= min_width_pixels && content_rect.height >= min_height_pixels {
                                            // Calculate centered content area
                                            let content_width = (content_rect.width * 50 / 100).max(min_width_pixels);
                                            let content_height = (content_rect.height * 50 / 100).max(min_height_pixels);
                                            let content_x = content_rect.x + (content_rect.width.saturating_sub(content_width)) / 2;
                                            let content_y = content_rect.y + (content_rect.height.saturating_sub(content_height)) / 2;
                                            
                                            // Get inner area width for scroll calculation
                                            let inner_width = if field_index < 2 {
                                                // Top fields use full width minus borders
                                                (content_width.saturating_sub(2)) as usize
                                            } else if field_index < 5 {
                                                // Device section fields
                                                ((content_width / 2).saturating_sub(4)) as usize
                                            } else {
                                                // Connection section fields
                                                ((content_width / 2).saturating_sub(4)) as usize
                                            };
                                            
                                            // Calculate scroll offset and visual cursor position
                                            let scroll_offset = input.visual_scroll(inner_width);
                                            let visual_cursor = input.visual_cursor();
                                            
                                            let (cursor_x, cursor_y) = if field_index < 2 {
                                                // Top section: Sketch Directory (0) or Sketch Name (1)
                                                // Each field block: top border at y, text at y+1, bottom border at y+2
                                                let y_offset = if field_index == 0 { 0 } else { 3 };
                                                // Cursor position relative to visible text start
                                                let cursor_pos_in_view = visual_cursor.saturating_sub(scroll_offset);
                                                // content_y + y_offset (field top) + 1 (border) = text line
                                                (content_x + 1 + cursor_pos_in_view as u16, content_y + y_offset + 1)
                                            } else if field_index < 5 {
                                                // Device section: Environment (2), Board Model (3), FQBN (4)
                                                // Section block at content_y + 6, inner area at +1, fields start at +1
                                                // Each nested field: top at inner_y + field_offset, text at +1
                                                let section_y = content_y + 6; // After top section
                                                let field_offset = ((field_index - 2) * 4) as u16; // 3 lines per field + 1 spacing
                                                // Cursor position relative to visible text start
                                                let cursor_pos_in_view = visual_cursor.saturating_sub(scroll_offset);
                                                // section_y + 1 (section border) + 1 (inner start) + field_offset (field top) + 1 (field border) = text line
                                                (content_x + 1 + 1 + cursor_pos_in_view as u16, section_y + 1 + 1 + field_offset + 1)
                                            } else {
                                                // Connection section: Port (5), Baudrate (6)
                                                let section_y = content_y + 6; // After top section
                                                let section_x = content_x + content_width / 2; // Right column
                                                let field_offset = ((field_index - 5) * 4) as u16; // 3 lines per field + 1 spacing
                                                // Cursor position relative to visible text start
                                                let cursor_pos_in_view = visual_cursor.saturating_sub(scroll_offset);
                                                // section_y + 1 (section border) + 1 (inner start) + field_offset (field top) + 1 (field border) = text line
                                                (section_x + 1 + 1 + cursor_pos_in_view as u16, section_y + 1 + 1 + field_offset + 1)
                                            };
                                            
                                            // Set cursor position
                                            f.set_cursor_position((cursor_x, cursor_y));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })?;
        
        // ┌──────────────────────────────────────────────────────────────────────────────────────────────┐
        // │                              Handle events (keyboard and mouse)                              │
        // └──────────────────────────────────────────────────────────────────────────────────────────────┘                
        
        match crossterm::event::poll(std::time::Duration::from_millis(50)) {
            Ok(true) => {
                match event::read()? {
                    Event::Key(key) => {
                        if key.kind != KeyEventKind::Press {
                            continue;
                        }
                        
                        // Handle dashboard navigation
                        if let Some(active_tab_idx) = registry.get_active_tab(main_content_tab_bar.handle()) {
                            if let Some(tab_bar_state) = registry.get_tab_bar_state(main_content_tab_bar.handle()) {
                                if let Some(tab_config) = tab_bar_state.tab_configs.get(active_tab_idx) {
                                    if tab_config.id == "dashboard" {
                                        match key.code {
                                            KeyCode::Up | KeyCode::Char('k') => {
                                                if dashboard_state.selected_command > 0 {
                                                    dashboard_state.selected_command -= 1;
                                                    // Sync back to Arc
                                                    if let Ok(mut state) = dashboard_arc.lock() {
                                                        state.selected_command = dashboard_state.selected_command;
                                                    }
                                                }
                                                continue;
                                            }
                                            KeyCode::Down | KeyCode::Char('j') => {
                                                if dashboard_state.selected_command < dashboard_state.commands.len().saturating_sub(1) {
                                                    dashboard_state.selected_command += 1;
                                                    // Sync back to Arc
                                                    if let Ok(mut state) = dashboard_arc.lock() {
                                                        state.selected_command = dashboard_state.selected_command;
                                                    }
                                                }
                                                continue;
                                            }
                                            KeyCode::Enter => {
                                                // Execute selected command
                                                let command = dashboard_state.commands[dashboard_state.selected_command].clone();
                                                
                                                // Clear previous output
                                                dashboard_state.output_lines.clear();
                                                dashboard_state.output_scroll = 0;
                                                
                                                if command == "Progress" {
                                                    // For progress command, initialize progress state
                                                    dashboard_state.is_running = true;
                                                    dashboard_state.progress_percent = 0.0;
                                                    dashboard_state.progress_stage = "Initializing".to_string();
                                                    dashboard_state.current_file = String::new();
                                                    dashboard_state.status_text = format!("Running: {}", command);
                                                    dashboard_state.output_lines.push(format!("> {}", command));
                                                    
                                                    // Execute progress command in background thread
                                                    let settings_clone = settings.clone();
                                                    
                                                    // Update Arc with current state before spawning thread
                                                    if let Ok(mut state) = dashboard_arc.lock() {
                                                        *state = dashboard_state.clone();
                                                    }
                                                    
                                                    let dashboard_clone = dashboard_arc.clone();
                                                    
                                                    thread::spawn(move || {
                                                        execute_progress_command(dashboard_clone, settings_clone);
                                                    });
                                                } else if command == "Progress-new" {
                                                    // Rust-based progress command
                                                    dashboard_state.is_running = true;
                                                    dashboard_state.progress_percent = 0.0;
                                                    dashboard_state.progress_stage = "Initializing".to_string();
                                                    dashboard_state.current_file = String::new();
                                                    dashboard_state.status_text = format!("Running: {}", command);
                                                    dashboard_state.output_lines.push(format!("> {}", command));
                                                    
                                                    // Execute progress command in background thread
                                                    let settings_clone = settings.clone();
                                                    
                                                    // Update Arc with current state before spawning thread
                                                    if let Ok(mut state) = dashboard_arc.lock() {
                                                        *state = dashboard_state.clone();
                                                    }
                                                    
                                                    let dashboard_clone = dashboard_arc.clone();
                                                    
                                                    thread::spawn(move || {
                                                        execute_progress_rust(dashboard_clone, settings_clone);
                                                    });
                                                } else if command == "Build" || command == "Compile" || command == "Upload" {
                                                    // For Build, Compile, and Upload commands
                                                    dashboard_state.is_running = false; // No progress bar for these
                                                    dashboard_state.progress_stage = String::new();
                                                    dashboard_state.status_text = format!("Running: {}", command);
                                                    dashboard_state.output_lines.push(format!("> {}", command));
                                                    
                                                    // Update Arc with current state before spawning thread
                                                    if let Ok(mut state) = dashboard_arc.lock() {
                                                        *state = dashboard_state.clone();
                                                    }
                                                    
                                                    // Execute command in background thread
                                                    let settings_clone = settings.clone();
                                                    let dashboard_clone = dashboard_arc.clone();
                                                    let command_clone = command.clone();
                                                    
                                                    thread::spawn(move || {
                                                        execute_pmake_command(dashboard_clone, settings_clone, command_clone);
                                                    });
                                                } else {
                                                    // For other commands, use regular status
                                                    dashboard_state.is_running = false;
                                                    dashboard_state.progress_stage = String::new();
                                                    dashboard_state.status_text = format!("Running: {}", command);
                                                    dashboard_state.output_lines.push(format!("> {}", command));
                                                    dashboard_state.output_lines.push("Command execution not yet implemented".to_string());
                                                    
                                                    // Update Arc
                                                    if let Ok(mut state) = dashboard_arc.lock() {
                                                        *state = dashboard_state.clone();
                                                    }
                                                }
                                                
                                                continue;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                        
                        match field_editor_state {
                            FieldEditorState::Editing { field_index, ref mut input } => {
                                match key.code {
                                    KeyCode::Enter => {
                                        // Confirm edit
                                        settings_fields.set_value(&mut settings, field_index, input.value().to_string());
                                        if let Err(e) = settings.save() {
                                            toasts.push(Toast::new(
                                                format!("Failed to save settings: {}", e),
                                                ToastType::Error,
                                            ));
                                        } else {
                                            toasts.push(Toast::new(
                                                "Settings saved".to_string(),
                                                ToastType::Success,
                                            ));
                                        }
                                        field_editor_state = FieldEditorState::Selected { field_index };
                                    }
                                    KeyCode::Esc => {
                                        // Cancel edit
                                        field_editor_state = FieldEditorState::Selected { field_index };
                                    }
                                    KeyCode::Char(c) => {
                                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                                            // Handle Ctrl+key combinations
                                            match c {
                                                'a' => {
                                                    let _ = input.handle(InputRequest::GoToStart);
                                                }
                                                'e' => {
                                                    let _ = input.handle(InputRequest::GoToEnd);
                                                }
                                                _ => {}
                                            }
                                        } else {
                                            let _ = input.handle(InputRequest::InsertChar(c));
                                        }
                                    }
                                    KeyCode::Backspace => {
                                        let _ = input.handle(InputRequest::DeletePrevChar);
                                    }
                                    KeyCode::Delete => {
                                        let _ = input.handle(InputRequest::DeleteNextChar);
                                    }
                                    KeyCode::Left => {
                                        let _ = input.handle(InputRequest::GoToPrevChar);
                                    }
                                    KeyCode::Right => {
                                        let _ = input.handle(InputRequest::GoToNextChar);
                                    }
                                    KeyCode::Home => {
                                        let _ = input.handle(InputRequest::GoToStart);
                                    }
                                    KeyCode::End => {
                                        let _ = input.handle(InputRequest::GoToEnd);
                                    }
                                    _ => {}
                                }
                            }
                            FieldEditorState::Selected { field_index } => {
                                match key.code {
                                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                                        break;
                                    }
                                    KeyCode::Esc => {
                                        popup = None;
                                    }
                                    KeyCode::Enter => {
                                        // Check if field is a dropdown
                                        if settings_fields.is_dropdown(field_index) {
                                            // Open dropdown
                                            let options = settings_fields.get_dropdown_options(field_index);
                                            let current_value = settings_fields.get_value(&settings, field_index);
                                            // Find current selection index
                                            let selected_index = options.iter()
                                                .position(|opt| opt == &current_value)
                                                .unwrap_or(0);
                                            field_editor_state = FieldEditorState::Selecting {
                                                field_index,
                                                selected_index,
                                                options,
                                            };
                                        } else {
                                            // Start text editing
                                            let current_value = settings_fields.get_value(&settings, field_index);
                                            let mut input = Input::new(current_value);
                                            let _ = input.handle(InputRequest::GoToEnd);
                                            field_editor_state = FieldEditorState::Editing {
                                                field_index,
                                                input,
                                            };
                                        }
                                    }
                                    KeyCode::Up | KeyCode::Char('k') => {
                                        // Navigate up in field list
                                        if field_index > 0 {
                                            field_editor_state = FieldEditorState::Selected {
                                                field_index: field_index - 1,
                                            };
                                        }
                                    }
                                    KeyCode::Down | KeyCode::Char('j') => {
                                        // Navigate down in field list
                                        if field_index < settings_fields.count() - 1 {
                                            field_editor_state = FieldEditorState::Selected {
                                                field_index: field_index + 1,
                                            };
                                        }
                                    }
                                    KeyCode::Tab => {
                                        // Cycle to next field
                                        let next_index = (field_index + 1) % settings_fields.count();
                                        field_editor_state = FieldEditorState::Selected {
                                            field_index: next_index,
                                        };
                                    }
                                    KeyCode::Left | KeyCode::Char('h') => {
                                        if tab_style != TabBarStyle::BoxStatic && tab_style != TabBarStyle::TextStatic {
                                            main_content_tab_bar.navigate_previous(&mut registry);
                                        }
                                    }
                                    KeyCode::Right | KeyCode::Char('l') => {
                                        if tab_style != TabBarStyle::BoxStatic && tab_style != TabBarStyle::TextStatic {
                                            main_content_tab_bar.navigate_next(&mut registry);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            FieldEditorState::Selecting { field_index, ref mut selected_index, ref options } => {
                                match key.code {
                                    KeyCode::Enter => {
                                        // Confirm selection
                                        if *selected_index < options.len() {
                                            let selected_value = options[*selected_index].clone();
                                            settings_fields.set_value(&mut settings, field_index, selected_value);
                                            if let Err(e) = settings.save() {
                                                toasts.push(Toast::new(
                                                    format!("Failed to save settings: {}", e),
                                                    ToastType::Error,
                                                ));
                                            } else {
                                                toasts.push(Toast::new(
                                                    "Settings saved".to_string(),
                                                    ToastType::Success,
                                                ));
                                            }
                                        }
                                        field_editor_state = FieldEditorState::Selected { field_index };
                                    }
                                    KeyCode::Esc => {
                                        // Cancel selection
                                        field_editor_state = FieldEditorState::Selected { field_index };
                                    }
                                    KeyCode::Up | KeyCode::Char('k') => {
                                        // Navigate up in dropdown
                                        if *selected_index > 0 {
                                            *selected_index -= 1;
                                        } else {
                                            *selected_index = options.len().saturating_sub(1);
                                        }
                                    }
                                    KeyCode::Down | KeyCode::Char('j') => {
                                        // Navigate down in dropdown
                                        if *selected_index < options.len().saturating_sub(1) {
                                            *selected_index += 1;
                                        } else {
                                            *selected_index = 0;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Event::Mouse(mouse_event) => {
                        // Handle mouse scrolling for dashboard output
                        if let Some(active_tab_idx) = registry.get_active_tab(main_content_tab_bar.handle()) {
                            if let Some(tab_bar_state) = registry.get_tab_bar_state(main_content_tab_bar.handle()) {
                                if let Some(tab_config) = tab_bar_state.tab_configs.get(active_tab_idx) {
                                    if tab_config.id == "dashboard" {
                                        if let Some(box_manager) = get_box_by_name(&registry, HWND_MAIN_CONTENT_BOX) {
                                            if let Some(content_rect) = box_manager.metrics(&registry) {
                                                // Calculate output box area (column 2, bottom box)
                                                let nested_area = Rect {
                                                    x: content_rect.x.saturating_add(1),
                                                    y: content_rect.y.saturating_add(1),
                                                    width: content_rect.width.saturating_sub(2),
                                                    height: content_rect.height.saturating_sub(2),
                                                };
                                                
                                                let columns = Layout::default()
                                                    .direction(Direction::Horizontal)
                                                    .constraints([
                                                        Constraint::Percentage(50),
                                                        Constraint::Percentage(50),
                                                    ])
                                                    .split(nested_area);
                                                
                                                let column2_chunks = Layout::default()
                                                    .direction(Direction::Vertical)
                                                    .constraints([
                                                        Constraint::Length(3),
                                                        Constraint::Min(0),
                                                    ])
                                                    .split(columns[1]);
                                                
                                                let output_area = column2_chunks[1];
                                                
                                                // Check if mouse is over output area
                                                if mouse_event.column >= output_area.x && 
                                                   mouse_event.column < output_area.x + output_area.width &&
                                                   mouse_event.row >= output_area.y && 
                                                   mouse_event.row < output_area.y + output_area.height {
                                                    match mouse_event.kind {
                                                        MouseEventKind::ScrollUp => {
                                                            dashboard_state.scroll_output_up(3);
                                                        }
                                                        MouseEventKind::ScrollDown => {
                                                            dashboard_state.scroll_output_down(3);
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        if mouse_event.kind == MouseEventKind::Down(crossterm::event::MouseButton::Left) {
                            // Handle mouse clicks on tabs
                            if let Some((ref tab_bar, _handle)) = current_tab_bar {
                                let clicked_tab: Option<usize> = tab_bar.get_tab_at(mouse_event.column, mouse_event.row, Some(&registry));
                                if let Some(clicked_tab_idx) = clicked_tab {
                                    if tab_style != TabBarStyle::BoxStatic && tab_style != TabBarStyle::TextStatic {
                                        main_content_tab_bar.set_active(&mut registry, clicked_tab_idx);
                                    }
                                }
                            }
                            
                            // Handle mouse clicks on settings fields
                            if let Some(active_tab_idx) = registry.get_active_tab(main_content_tab_bar.handle()) {
                                if let Some(tab_bar_state) = registry.get_tab_bar_state(main_content_tab_bar.handle()) {
                                    if let Some(tab_config) = tab_bar_state.tab_configs.get(active_tab_idx) {
                                        if tab_config.id == "settings" {
                                            // Calculate which field was clicked based on mouse position
                                            if let Some(box_manager) = get_box_by_name(&registry, HWND_MAIN_CONTENT_BOX) {
                                                if let Some(content_rect) = box_manager.metrics(&registry) {
                                                    // Calculate centered content area
                                                    let min_width_pixels = 80;
                                                    let min_height_pixels = 21;
                                                    let content_width = (content_rect.width * 50 / 100).max(min_width_pixels);
                                                    let content_height = (content_rect.height * 50 / 100).max(min_height_pixels);
                                                    let content_x = content_rect.x + (content_rect.width.saturating_sub(content_width)) / 2;
                                                    let content_y = content_rect.y + (content_rect.height.saturating_sub(content_height)) / 2;
                                                    
                                                    // Check if click is within content area
                                                    if mouse_event.column >= content_x && mouse_event.column < content_x + content_width &&
                                                       mouse_event.row >= content_y && mouse_event.row < content_y + content_height {
                                                        
                                                        // Check top section (Sketch Directory, Sketch Name)
                                                        if mouse_event.row >= content_y && mouse_event.row < content_y + 6 {
                                                            let field_index = if mouse_event.row < content_y + 3 { 0 } else { 1 };
                                                            // Start editing directly on click
                                                            let current_value = settings_fields.get_value(&settings, field_index);
                                                            let mut input = Input::new(current_value);
                                                            let _ = input.handle(InputRequest::GoToEnd);
                                                            field_editor_state = FieldEditorState::Editing {
                                                                field_index,
                                                                input,
                                                            };
                                                        } else {
                                                            // Check bottom section (Device/Connection)
                                                            let section_y = content_y + 6;
                                                            let section_width = content_width / 2;
                                                            
                                                            // Check Device section (left)
                                                            if mouse_event.column >= content_x && mouse_event.column < content_x + section_width {
                                                                let relative_y = mouse_event.row.saturating_sub(section_y + 1);
                                                                let field_offset = relative_y / 4; // 3 lines per field + 1 spacing
                                                                if field_offset < 3 {
                                                                    let field_index = 2 + field_offset as usize; // Environment (2), Board Model (3), FQBN (4)
                                                                    if field_index < 5 {
                                                                        // Check if dropdown field
                                                                        if settings_fields.is_dropdown(field_index) {
                                                                            let options = settings_fields.get_dropdown_options(field_index);
                                                                            let current_value = settings_fields.get_value(&settings, field_index);
                                                                            let selected_index = options.iter()
                                                                                .position(|opt| opt == &current_value)
                                                                                .unwrap_or(0);
                                                                            field_editor_state = FieldEditorState::Selecting {
                                                                                field_index,
                                                                                selected_index,
                                                                                options,
                                                                            };
                                                                        } else {
                                                                            let current_value = settings_fields.get_value(&settings, field_index);
                                                                            let mut input = Input::new(current_value);
                                                                            let _ = input.handle(InputRequest::GoToEnd);
                                                                            field_editor_state = FieldEditorState::Editing {
                                                                                field_index,
                                                                                input,
                                                                            };
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            // Check Connection section (right)
                                                            else if mouse_event.column >= content_x + section_width && mouse_event.column < content_x + content_width {
                                                                let relative_y = mouse_event.row.saturating_sub(section_y + 1);
                                                                let field_offset = relative_y / 4; // 3 lines per field + 1 spacing
                                                                if field_offset < 2 {
                                                                    let field_index = 5 + field_offset as usize; // Port (5), Baudrate (6)
                                                                    if field_index < 7 {
                                                                        // Check if dropdown field
                                                                        if settings_fields.is_dropdown(field_index) {
                                                                            let options = settings_fields.get_dropdown_options(field_index);
                                                                            let current_value = settings_fields.get_value(&settings, field_index);
                                                                            let selected_index = options.iter()
                                                                                .position(|opt| opt == &current_value)
                                                                                .unwrap_or(0);
                                                                            field_editor_state = FieldEditorState::Selecting {
                                                                                field_index,
                                                                                selected_index,
                                                                                options,
                                                                            };
                                                                        } else {
                                                                            let current_value = settings_fields.get_value(&settings, field_index);
                                                                            let mut input = Input::new(current_value);
                                                                            let _ = input.handle(InputRequest::GoToEnd);
                                                                            field_editor_state = FieldEditorState::Editing {
                                                                                field_index,
                                                                                input,
                                                                            };
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Event::Resize(_, _) => {
                        // Terminal resize - will be handled on next draw
                    }
                    _ => {}
                }
            }
            Ok(false) => {
                // No event available
            }
            Err(_) => {
                // Error polling, continue anyway
            }
        }
    }
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    Ok(())
}

/// Render main content
fn render_content(f: &mut ratatui::Frame, area: Rect, dimming: &DimmingContext) {
    let content_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(dimming.border_color(true)));
    
    f.render_widget(content_block, area);
}

/// Render settings panel
fn render_settings(
    f: &mut ratatui::Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
) {
    // Check if terminal is too small (minimum size requirements)
    let min_width_pixels = 80;
    let min_height_pixels = 21;
    
    if area.width < min_width_pixels || area.height < min_height_pixels {
        // Terminal is too small - show warning message
        let warning_text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "⚠ Terminal Too Small",
                Style::default().fg(Color::Rgb(255, 215, 0)).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!("Minimum size required: {}x{}", min_width_pixels, min_height_pixels)),
            Line::from(format!("Current size: {}x{}", area.width, area.height)),
            Line::from(""),
            Line::from("Please resize your terminal to at least 80 columns by 21 rows."),
            Line::from(""),
            Line::from(Span::styled(
                "The form will appear automatically when the terminal is large enough.",
                Style::default().fg(Color::Cyan),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Press [q] to quit",
                Style::default().fg(Color::White),
            )),
        ];
        
        let warning_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 215, 0)))
            .title("Warning");
        
        let warning_para = Paragraph::new(warning_text)
            .block(warning_block)
            .alignment(Alignment::Center);
        
        // Center the warning message
        let warning_width = 60;
        let warning_height = 11; // Increased to accommodate new line
        let warning_x = area.x + (area.width.saturating_sub(warning_width)) / 2;
        let warning_y = area.y + (area.height.saturating_sub(warning_height)) / 2;
        
        let warning_area = Rect {
            x: warning_x,
            y: warning_y,
            width: warning_width.min(area.width),
            height: warning_height.min(area.height),
        };
        
        f.render_widget(warning_para, warning_area);
        return;
    }
    
    // Calculate content size: 50% of available space, but at least 80 pixels wide and 25 pixels tall
    let content_width = (area.width * 50 / 100).max(min_width_pixels).min(area.width);
    let content_height = (area.height * 50 / 100).max(min_height_pixels).min(area.height);
    // Center the content (no blank lines above/below)
    let content_x = area.x + (area.width.saturating_sub(content_width)) / 2;
    let content_y = area.y + (area.height.saturating_sub(content_height)) / 2;
    
    // Ensure content area doesn't exceed bounds
    let content_area = Rect {
        x: content_x.min(area.x + area.width),
        y: content_y.min(area.y + area.height),
        width: content_width.min(area.width.saturating_sub(content_x.saturating_sub(area.x))),
        height: content_height.min(area.height.saturating_sub(content_y.saturating_sub(area.y))),
    };
    
    // Ensure content_area is valid before splitting
    if content_area.width == 0 || content_area.height == 0 {
        return;
    }
    
    // Split into top section (Sketch Directory, Sketch Name) and bottom section (Device/Connection)
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Top section: 2 boxes (3 lines each)
            Constraint::Min(0),   // Bottom section: Device/Connection
        ])
        .split(content_area);
    
    // Render top section: Sketch Directory and Sketch Name
    let top_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Sketch Directory
            Constraint::Length(3), // Sketch Name
        ])
        .split(main_chunks[0]);
    
    render_full_width_field(f, top_chunks[0], settings, fields, editor_state, 0, "Sketch Directory");
    render_full_width_field(f, top_chunks[1], settings, fields, editor_state, 1, "Sketch Name");
    
    // Render bottom section: Device and Connection columns
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Device column
            Constraint::Percentage(50), // Connection column
        ])
        .split(main_chunks[1]);
    
    // Device section: Environment, Board Model, FQBN
    // Calculate Device section height (3 fields + 1 extra row)
    let device_height = render_section(f, bottom_chunks[0], settings, fields, editor_state, "Device", &[2, 3, 4], None);
    
    // Connection section: Port, Baud Rate - match Device height
    render_section(f, bottom_chunks[1], settings, fields, editor_state, "Connection", &[5, 6], Some(device_height));
}

/// Render a full-width field (for Sketch Directory and Sketch Name)
fn render_full_width_field(
    f: &mut ratatui::Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
    field_index: usize,
    title: &str,
) {
    // Ensure area is valid
    if area.width == 0 || area.height == 0 {
        return;
    }
    
    let is_selected = matches!(editor_state, FieldEditorState::Selected { field_index: idx } if *idx == field_index);
    let is_editing = matches!(editor_state, FieldEditorState::Editing { field_index: idx, .. } if *idx == field_index);
    let value = fields.get_value(settings, field_index);
    
    // Get inner area for text (accounting for borders)
    let inner_area = Block::default().borders(Borders::ALL).inner(area);
    let text_width = inner_area.width as usize;
    
    let (display_value, _scroll_offset) = if is_editing {
        if let FieldEditorState::Editing { input, .. } = editor_state {
            let scroll = input.visual_scroll(text_width);
            let value_str = input.value();
            // Get the visible portion of the text
            let chars: Vec<char> = value_str.chars().collect();
            let visible_start = scroll.min(chars.len());
            let visible_end = (visible_start + text_width).min(chars.len());
            let visible_text: String = chars[visible_start..visible_end].iter().collect();
            (visible_text, scroll)
        } else {
            (value, 0)
        }
    } else {
        (value, 0)
    };
    
    // Border color: #666666 (RGB 102, 102, 102) for box characters by default
    // Title color: white for text
    let border_color = if is_editing {
        Color::Rgb(255, 215, 0) // Gold when editing
    } else if is_selected {
        Color::Cyan // Cyan when selected
    } else {
        Color::Rgb(102, 102, 102) // #666666 by default
    };
    
    let title_color = Color::White;
    
    let text_color = if is_editing {
        Color::Rgb(255, 215, 0) // Gold when editing
    } else if is_selected {
        Color::Cyan // Cyan when selected
    } else {
        Color::White // White by default
    };
    
    let block = Block::default()
        .title(Span::styled(format!(" {} ", title), Style::default().fg(title_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0));
    
    let para = Paragraph::new(display_value)
        .style(Style::default().fg(text_color))
        .block(block);
    
    f.render_widget(para, area);
}

/// Render a section with nested fields (Device or Connection)
/// Returns the height used for the section
fn render_section(
    f: &mut ratatui::Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
    section_title: &str,
    field_indices: &[usize],
    target_height: Option<u16>,
) -> u16 {
    // Ensure area is valid
    if area.width == 0 || area.height == 0 {
        return 0;
    }
    
    // Border color: #666666 (RGB 102, 102, 102) for box characters
    let border_color = Color::Rgb(102, 102, 102);
    
    // Title color: white for text
    let title_color = Color::White;
    
    // Calculate field height (3 lines per field)
    let field_height = 3;
    let spacing = 1; // Spacing between fields
    let total_fields = field_indices.len();
    
    // Calculate exact height needed: (field_height * total_fields) + (spacing * (total_fields - 1)) + borders (2)
    let mut needed_height = (field_height * total_fields) + (spacing * total_fields.saturating_sub(1)) + 2;
    
    // If target_height is provided (for matching heights), use it
    // Otherwise, if this is Device section, add 1 extra row
    let final_height = if let Some(target) = target_height {
        target
    } else if section_title == "Device" {
        needed_height += 1; // Device box needs 1 extra row
        needed_height as u16
    } else {
        needed_height as u16
    };
    
    // Use the calculated height, not the full area
    let section_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: final_height.min(area.height),
    };
    
    // Outer section block with white title and gray border
    let section_block = Block::default()
        .title(Span::styled(format!(" {} ", section_title), Style::default().fg(title_color)))
        .borders(Borders::ALL)
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0))
        .border_style(Style::default().fg(border_color));
    
    // Inner area for nested fields
    let inner_area = section_block.inner(section_area);
    
    let mut y_offset = 1; // Start after top border
    
    for &field_index in field_indices {
        if field_index >= fields.count() {
            break;
        }
        
        let field_area = Rect {
            x: inner_area.x + 1,
            y: inner_area.y + y_offset,
            width: inner_area.width.saturating_sub(2),
            height: field_height as u16,
        };
        
        render_nested_field(f, field_area, settings, fields, editor_state, field_index);
        y_offset += field_height as u16 + spacing as u16; // Add spacing between fields
    }
    
    // Render the section block with calculated height
    f.render_widget(section_block, section_area);
    
    // Return the height used
    section_area.height
}

/// Render a nested field inside a section
fn render_nested_field(
    f: &mut ratatui::Frame,
    area: Rect,
    settings: &Settings,
    fields: &SettingsFields,
    editor_state: &FieldEditorState,
    field_index: usize,
) {
    // Ensure area is valid
    if area.width == 0 || area.height == 0 {
        return;
    }
    
    let label = fields.labels[field_index];
    let value = fields.get_value(settings, field_index);
    let is_selected = matches!(editor_state, FieldEditorState::Selected { field_index: idx } if *idx == field_index);
    let is_editing = matches!(editor_state, FieldEditorState::Editing { field_index: idx, .. } if *idx == field_index);
    let is_selecting = matches!(editor_state, FieldEditorState::Selecting { field_index: idx, .. } if *idx == field_index);
    
    // Get inner area for text (accounting for borders)
    let inner_area = Block::default().borders(Borders::ALL).inner(area);
    let text_width = inner_area.width as usize;
    
    // Render the field normally first
    let (display_value, _scroll_offset) = if is_editing {
        if let FieldEditorState::Editing { input, .. } = editor_state {
            let scroll = input.visual_scroll(text_width);
            let value_str = input.value();
            // Get the visible portion of the text
            let chars: Vec<char> = value_str.chars().collect();
            let visible_start = scroll.min(chars.len());
            let visible_end = (visible_start + text_width).min(chars.len());
            let visible_text: String = chars[visible_start..visible_end].iter().collect();
            (visible_text, scroll)
        } else {
            (value.clone(), 0)
        }
    } else {
        (value.clone(), 0)
    };
    
    // Border color: #666666 (RGB 102, 102, 102) for box characters by default
    // Title color: white for text
    let border_color = if is_editing || is_selecting {
        Color::Rgb(255, 215, 0) // Gold when editing or selecting
    } else if is_selected {
        Color::Cyan // Cyan when selected
    } else {
        Color::Rgb(102, 102, 102) // #666666 by default
    };
    
    let title_color = Color::White;
    
    let text_color = if is_editing || is_selecting {
        Color::Rgb(255, 215, 0) // Gold when editing or selecting
    } else if is_selected {
        Color::Cyan // Cyan when selected
    } else {
        Color::White // White by default
    };
    
    let block = Block::default()
        .title(Span::styled(format!(" {} ", label), Style::default().fg(title_color)))
        .borders(Borders::ALL)
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0))
        .border_style(Style::default().fg(border_color));
    
    let para = Paragraph::new(display_value)
        .style(Style::default().fg(text_color))
        .block(block);
    
    f.render_widget(para, area);
    
    // Note: Dropdown overlay is rendered in main loop after all widgets
    // to ensure it appears on top
}

/// Render dashboard panel
fn render_dashboard(
    f: &mut ratatui::Frame,
    area: Rect,
    dashboard_state: &mut DashboardState,
) {
    // Ensure area is valid
    if area.width == 0 || area.height == 0 {
        return;
    }
    
    // Calculate commands box width: longest command + 4 spaces
    let max_command_width = dashboard_state.commands
        .iter()
        .map(|cmd| cmd.len())
        .max()
        .unwrap_or(10);
    let commands_box_width = ((max_command_width + 4) as u16).min(area.width);
    
    // Split into two columns
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(commands_box_width), // Column 1: Commands (fixed width)
            Constraint::Min(0),                      // Column 2: Status and Output (remaining)
        ])
        .split(area);
    
    // Column 1: Command list
    let command_items: Vec<ListItem> = dashboard_state.commands
        .iter()
        .enumerate()
        .map(|(idx, cmd)| {
            let style = if idx == dashboard_state.selected_command {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::White)
            };
            ListItem::new(Line::from(Span::styled(cmd.clone(), style)))
        })
        .collect();
    
    let command_list = List::new(command_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(" Commands ", Style::default().fg(Color::White)))
            .border_style(Style::default().fg(Color::Rgb(102, 102, 102)))
            .padding(ratatui::widgets::Padding::new(1, 1, 0, 0)));
    
    f.render_widget(command_list, columns[0]);
    
    // Column 2: Split into status bar and output
    let column2_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Status bar (4 lines: border + text + progress bar + border)
            Constraint::Min(0),     // Output (remaining space)
        ])
        .split(columns[1]);
    
    // Status bar box - show progress bar if running progress command
    let status_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(" Status ", Style::default().fg(Color::White)))
        .border_style(Style::default().fg(Color::Rgb(102, 102, 102)))
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0));
    
    let status_inner = status_block.inner(column2_chunks[0]);
    
    if dashboard_state.is_running && !dashboard_state.progress_stage.is_empty() {
        // Show progress bar
        let progress_text = if dashboard_state.current_file.is_empty() {
            format!("{}: {:.1}%", dashboard_state.progress_stage, dashboard_state.progress_percent)
        } else {
            format!("{}: {:.1}% - {}", dashboard_state.progress_stage, dashboard_state.progress_percent, dashboard_state.current_file)
        };
        
        // Create progress bar
        let progress_width = status_inner.width as usize;
        let filled_width = ((progress_width as f64 * dashboard_state.progress_percent / 100.0) as usize).min(progress_width);
        let empty_width = progress_width.saturating_sub(filled_width);
        
        let progress_bar = format!(
            "{}{}",
            "█".repeat(filled_width),
            "░".repeat(empty_width)
        );
        
        let progress_lines = vec![
            Line::from(Span::styled(
                progress_text,
                Style::default().fg(Color::Cyan),
            )),
            Line::from(Span::styled(
                progress_bar,
                Style::default().fg(Color::Green),
            )),
        ];
        
        let status_para = Paragraph::new(progress_lines)
            .block(status_block)
            .style(Style::default().fg(Color::White));
        
        f.render_widget(status_para, column2_chunks[0]);
    } else {
        // Show regular status text
        let status_para = Paragraph::new(dashboard_state.status_text.clone())
            .block(status_block)
            .style(Style::default().fg(Color::White));
        
        f.render_widget(status_para, column2_chunks[0]);
    }
    
    // Output box with scrolling
    let output_area = column2_chunks[1];
    let output_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(" Output ", Style::default().fg(Color::White)))
        .border_style(Style::default().fg(Color::Rgb(102, 102, 102)))
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0));
    let output_inner = output_block.inner(output_area);
    
    // Calculate visible lines
    let visible_height = output_inner.height as usize;
    let total_lines = dashboard_state.output_lines.len();
    let max_scroll = total_lines.saturating_sub(visible_height.max(1));
    dashboard_state.output_scroll = dashboard_state.output_scroll.min(max_scroll);
    
    // Get visible lines
    let start_line = dashboard_state.output_scroll;
    let end_line = (start_line + visible_height).min(total_lines);
    
    let visible_lines: Vec<Line> = if dashboard_state.output_lines.is_empty() {
        vec![Line::from(Span::styled(
            "No output yet. Select a command to run.",
            Style::default().fg(Color::Rgb(128, 128, 128)),
        ))]
    } else {
        dashboard_state.output_lines[start_line..end_line]
            .iter()
            .map(|line| Line::from(line.clone()))
            .collect()
    };
    
    let output_para = Paragraph::new(visible_lines)
        .block(output_block.clone())
        .style(Style::default().fg(Color::White));
    
    f.render_widget(output_para, output_area);
    
    // Render scrollbar if there are more lines than visible
    if total_lines > visible_height {
        let mut scrollbar_state = ScrollbarState::new(total_lines)
            .position(dashboard_state.output_scroll);
        
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .thumb_symbol("█")
            .track_symbol(Some("│"));
        
        f.render_stateful_widget(scrollbar, output_area, &mut scrollbar_state);
    }
}

/// Execute progress command and parse output
fn execute_progress_command(
    dashboard: Arc<Mutex<DashboardState>>,
    settings: Settings,
) {
    // Build command: python pmake.py progress
    // First, find the project directory from settings
    let sketch_dir = PathBuf::from(&settings.sketch_directory);
    
    // Look for pmake.py in the sketch directory or parent
    let pmake_script = sketch_dir.join("pmake.py");
    let pmake_script_parent = sketch_dir.parent().map(|p| p.join("pmake.py"));
    
    let script_path = if pmake_script.exists() {
        pmake_script
    } else if let Some(parent_script) = pmake_script_parent {
        if parent_script.exists() {
            parent_script
        } else {
            {
                let mut state = dashboard.lock().unwrap();
                state.is_running = false;
                state.status_text = "Error: pmake.py not found".to_string();
                state.output_lines.push("Error: Could not find pmake.py script".to_string());
            }
            return;
        }
    } else {
        {
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.status_text = "Error: pmake.py not found".to_string();
            state.output_lines.push("Error: Could not find pmake.py script".to_string());
        }
        return;
    };
    
    // Add initial debug message
    {
        let mut state = dashboard.lock().unwrap();
        // Don't clear - append to existing "> Progress" line
        state.output_lines.push(format!("Executing: python {:?} progress", script_path));
        state.output_lines.push(format!("Working directory: {:?}", sketch_dir));
        state.output_lines.push(format!("Script exists: {}", script_path.exists()));
        state.output_lines.push("Starting command execution...".to_string());
        if state.output_lines.len() > 1 {
            state.output_scroll = state.output_lines.len().saturating_sub(1);
        }
    }
    
    // Find workspace root (where pyproject.toml with uv workspace config is)
    let workspace_root = sketch_dir
        .ancestors()
        .find(|path| {
            let pyproject = path.join("pyproject.toml");
            if pyproject.exists() {
                // Check if it's a UV workspace (has [tool.uv.workspace] or [tool.uv.sources])
                if let Ok(content) = fs::read_to_string(&pyproject) {
                    return content.contains("[tool.uv") || content.contains("[project]");
                }
            }
            false
        })
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| {
            // Fallback: assume workspace root is 4 levels up from sketch_dir
            // sketch_dir is typically: .../projects/esp32-s3__LB-Gold/sht21-solo
            // workspace root is: .../dev-boards
            sketch_dir.parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| sketch_dir.clone())
        });
    
    // Try to use uv run first (if available), otherwise use python with PYTHONPATH
    let mut cmd = if which::which("uv").is_ok() {
        let mut uv_cmd = Command::new("uv");
        uv_cmd.arg("run");
        uv_cmd.arg("python");
        uv_cmd.arg("-u"); // Unbuffered output
        uv_cmd
    } else {
        let mut py_cmd = Command::new("python");
        py_cmd.arg("-u"); // Unbuffered output
        // Set PYTHONPATH to include workspace root so py_makefile can be found
        let pythonpath = workspace_root.to_string_lossy().to_string();
        py_cmd.env("PYTHONPATH", &pythonpath);
        py_cmd
    };
    
    cmd.arg(&script_path);
    cmd.arg("progress");
    cmd.current_dir(&workspace_root); // Run from workspace root so UV can find packages
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.env("PYTHONUNBUFFERED", "1"); // Ensure unbuffered output
    
    // Add debug info about command
    {
        let mut state = dashboard.lock().unwrap();
        state.output_lines.push(format!("Workspace root: {:?}", workspace_root));
        state.output_lines.push(format!("Using UV: {}", which::which("uv").is_ok()));
    }
    
    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.status_text = format!("Error: {}", e);
            state.output_lines.push(format!("Failed to execute command: {}", e));
            return;
        }
    };
    
    // Read both stdout and stderr
    use std::io::{BufRead, BufReader};
    
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    
    // Spawn thread to read stderr
    let dashboard_stderr = dashboard.clone();
    if let Some(stderr) = stderr {
        let stderr_reader = BufReader::new(stderr);
        thread::spawn(move || {
            for line in stderr_reader.lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => break,
                };
                
                let line_trimmed = line.trim();
                if line_trimmed.is_empty() {
                    continue;
                }
                
                // Remove ANSI escape sequences from stderr
                let cleaned = remove_ansi_escapes(&line);
                let line_trimmed = cleaned.trim();
                
                if !line_trimmed.is_empty() {
                    // Add stderr to output (verbose compilation output might be here)
                    {
                        let mut state = dashboard_stderr.lock().unwrap();
                        state.output_lines.push(line_trimmed.to_string());
                        if state.output_lines.len() > 1 {
                            state.output_scroll = state.output_lines.len().saturating_sub(1);
                        }
                    }
                }
            }
        });
    }
    
    // Read stdout - alive_progress uses carriage returns, so we need to handle \r
    // Use read_until to read line-by-line, but also handle \r within lines
    if let Some(stdout) = stdout {
        let mut reader = BufReader::new(stdout);
        let mut line_buffer = Vec::new();
        
        loop {
            line_buffer.clear();
            match reader.read_until(b'\n', &mut line_buffer) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    // Convert bytes to string
                    let line = match String::from_utf8(line_buffer.clone()) {
                        Ok(s) => s,
                        Err(_) => continue, // Skip invalid UTF-8
                    };
                    
                    // Remove ANSI escape sequences
                    let cleaned = remove_ansi_escapes(&line);
                    
                    // Handle carriage returns - alive_progress uses \r to overwrite same line
                    let processed_line = if cleaned.contains('\r') {
                        // Split by \r and take the last part (most recent update)
                        cleaned.split('\r').last().unwrap_or(&cleaned).to_string()
                    } else {
                        cleaned
                    };
                    
                    let trimmed = processed_line.trim();
                    
                    if trimmed.is_empty() {
                        continue;
                    }
                    
                    // Check if this is a progress update
                    let is_progress = trimmed.contains('%') && (trimmed.contains("Compiling") || 
                        trimmed.contains("Initializing") || 
                        trimmed.contains("Linking") || 
                        trimmed.contains("Generating"));
                    
                    {
                        let mut state = dashboard.lock().unwrap();
                        
                        if is_progress {
                            // Parse and update progress
                            if trimmed.contains("Initializing") || trimmed.contains("initializing") {
                                state.progress_stage = "Initializing".to_string();
                            } else if trimmed.contains("Compiling") || trimmed.contains("compiling") {
                                state.progress_stage = "Compiling".to_string();
                            } else if trimmed.contains("Linking") || trimmed.contains("linking") {
                                state.progress_stage = "Linking".to_string();
                            } else if trimmed.contains("Generating") || trimmed.contains("generating") {
                                state.progress_stage = "Generating".to_string();
                            }
                            
                            if let Some(percent) = extract_percentage(&trimmed) {
                                state.progress_percent = percent;
                            }
                            
                            if let Some(file) = extract_current_file(&trimmed) {
                                state.current_file = file;
                            }
                            
                            // Update progress line (replace if last line was progress, otherwise add)
                            if !state.output_lines.is_empty() && state.output_lines.last().map(|s| s.contains('%')).unwrap_or(false) {
                                let last_idx = state.output_lines.len() - 1;
                                state.output_lines[last_idx] = trimmed.to_string();
                            } else {
                                state.output_lines.push(trimmed.to_string());
                            }
                        } else {
                            // Regular output line - always add it (this captures verbose compilation output)
                            state.output_lines.push(trimmed.to_string());
                        }
                        
                        if state.output_lines.len() > 1 {
                            state.output_scroll = state.output_lines.len().saturating_sub(1);
                        }
                    }
                }
                Err(_) => break,
            }
        }
    }
    
    // Wait for process to finish
    let exit_status = child.wait();
    
    // Add exit status to output
    {
        let mut state = dashboard.lock().unwrap();
        match exit_status {
            Ok(status) => {
                if status.success() {
                    state.output_lines.push("Command completed successfully".to_string());
                } else {
                    state.output_lines.push(format!("Command exited with code: {:?}", status.code()));
                }
            }
            Err(e) => {
                state.output_lines.push(format!("Error waiting for process: {}", e));
            }
        }
    }
    
    // Mark as complete
    {
        let mut state = dashboard.lock().unwrap();
        state.is_running = false;
        if state.progress_percent < 100.0 {
            state.progress_percent = 100.0;
        }
        state.status_text = "Complete".to_string();
    }
}

/// Extract percentage from a line
fn extract_percentage(line: &str) -> Option<f64> {
    // Look for patterns like "45.2%" or "45%"
    use regex::Regex;
    lazy_static::lazy_static! {
        static ref PERCENT_RE: Regex = Regex::new(r"(\d+\.?\d*)%").unwrap();
    }
    
    if let Some(captures) = PERCENT_RE.captures(line) {
        if let Ok(percent) = captures[1].parse::<f64>() {
            return Some(percent.min(100.0));
        }
    }
    None
}

/// Extract current file from a line
fn extract_current_file(line: &str) -> Option<String> {
    // Look for file patterns in alive_progress format: "Compiling - file.cpp [1/5 files] (45.2%)"
    // Pattern: stage - filename.ext [info]
    use regex::Regex;
    lazy_static::lazy_static! {
        // Match " - filename.ext" or "filename.ext" before brackets or parentheses
        static ref FILE_RE: Regex = Regex::new(r"(?:-\s+)?([^\s\[\]()]+\.(cpp|c|ino|S))").unwrap();
    }
    
    if let Some(captures) = FILE_RE.captures(line) {
        return Some(captures[1].to_string());
    }
    None
}

/// Execute pmake command (Build, Compile, Upload) and capture output
fn execute_pmake_command(
    dashboard: Arc<Mutex<DashboardState>>,
    settings: Settings,
    command: String,
) {
    // Build command: python pmake.py <command>
    let sketch_dir = PathBuf::from(&settings.sketch_directory);
    
    // Look for pmake.py in the sketch directory or parent
    let pmake_script = sketch_dir.join("pmake.py");
    let pmake_script_parent = sketch_dir.parent().map(|p| p.join("pmake.py"));
    
    let script_path = if pmake_script.exists() {
        pmake_script
    } else if let Some(parent_script) = pmake_script_parent {
        if parent_script.exists() {
            parent_script
        } else {
            {
                let mut state = dashboard.lock().unwrap();
                state.status_text = "Error: pmake.py not found".to_string();
                state.output_lines.push("Error: Could not find pmake.py script".to_string());
            }
            return;
        }
    } else {
        {
            let mut state = dashboard.lock().unwrap();
            state.status_text = "Error: pmake.py not found".to_string();
            state.output_lines.push("Error: Could not find pmake.py script".to_string());
        }
        return;
    };
    
    // Find workspace root
    let workspace_root = sketch_dir
        .ancestors()
        .find(|path| {
            let pyproject = path.join("pyproject.toml");
            if pyproject.exists() {
                if let Ok(content) = fs::read_to_string(&pyproject) {
                    return content.contains("[tool.uv") || content.contains("[project]");
                }
            }
            false
        })
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| {
            sketch_dir.parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| sketch_dir.clone())
        });
    
    // Map command name to pmake.py argument
    let pmake_arg = match command.as_str() {
        "Build" => "build",
        "Compile" => "compile",
        "Upload" => "upload",
        _ => {
            let mut state = dashboard.lock().unwrap();
            state.status_text = format!("Error: Unknown command: {}", command);
            state.output_lines.push(format!("Error: Unknown command: {}", command));
            return;
        }
    };
    
    // Try to use uv run first (if available), otherwise use python with PYTHONPATH
    let mut cmd = if which::which("uv").is_ok() {
        let mut uv_cmd = Command::new("uv");
        uv_cmd.arg("run");
        uv_cmd.arg("python");
        uv_cmd.arg("-u"); // Unbuffered output
        uv_cmd
    } else {
        let mut py_cmd = Command::new("python");
        py_cmd.arg("-u"); // Unbuffered output
        let pythonpath = workspace_root.to_string_lossy().to_string();
        py_cmd.env("PYTHONPATH", &pythonpath);
        py_cmd
    };
    
    cmd.arg(&script_path);
    cmd.arg(pmake_arg);
    cmd.current_dir(&workspace_root);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.env("PYTHONUNBUFFERED", "1");
    
    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            let mut state = dashboard.lock().unwrap();
            state.status_text = format!("Error: {}", e);
            state.output_lines.push(format!("Failed to execute command: {}", e));
            return;
        }
    };
    
    // Read both stdout and stderr
    use std::io::{BufRead, BufReader};
    
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    
    // Spawn thread to read stderr
    let dashboard_stderr = dashboard.clone();
    if let Some(stderr) = stderr {
        let stderr_reader = BufReader::new(stderr);
        thread::spawn(move || {
            for line in stderr_reader.lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => break,
                };
                
                let line_trimmed = line.trim();
                if line_trimmed.is_empty() {
                    continue;
                }
                
                {
                    let mut state = dashboard_stderr.lock().unwrap();
                    state.output_lines.push(format!("[stderr] {}", line));
                    if state.output_lines.len() > 1 {
                        state.output_scroll = state.output_lines.len().saturating_sub(1);
                    }
                }
            }
        });
    }
    
    // Read stdout line by line
    if let Some(stdout) = stdout {
        let reader = BufReader::new(stdout);
        
        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => break,
            };
            
            // Remove ANSI escape sequences
            let cleaned_line = remove_ansi_escapes(&line);
            let line_trimmed = cleaned_line.trim();
            
            if !line_trimmed.is_empty() {
                {
                    let mut state = dashboard.lock().unwrap();
                    state.output_lines.push(cleaned_line.clone());
                    if state.output_lines.len() > 1 {
                        state.output_scroll = state.output_lines.len().saturating_sub(1);
                    }
                }
            }
        }
    }
    
    // Wait for process to finish
    let exit_status = child.wait();
    
    // Update status based on exit code
    {
        let mut state = dashboard.lock().unwrap();
        match exit_status {
            Ok(status) => {
                if status.success() {
                    state.status_text = format!("{} completed successfully", command);
                    state.output_lines.push(format!("{} completed successfully", command));
                } else {
                    state.status_text = format!("{} failed with exit code: {:?}", command, status.code());
                    state.output_lines.push(format!("{} failed with exit code: {:?}", command, status.code()));
                }
            }
            Err(e) => {
                state.status_text = format!("Command execution error: {}", e);
                state.output_lines.push(format!("Command execution error: {}", e));
            }
        }
    }
}

/// Remove ANSI escape sequences from a string
fn remove_ansi_escapes(s: &str) -> String {
    use regex::Regex;
    lazy_static::lazy_static! {
        static ref ANSI_RE: Regex = Regex::new(r"\x1B\[[0-9;]*[a-zA-Z]").unwrap();
    }
    ANSI_RE.replace_all(s, "").to_string()
}

/// Execute progress command using Rust (direct arduino-cli call)
fn execute_progress_rust(
    dashboard: Arc<Mutex<DashboardState>>,
    settings: Settings,
) {
    use std::io::{BufRead, BufReader};
    
    // Compilation state tracking
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum CompileStage {
        Initializing,
        Compiling,
        Linking,
        Generating,
        Complete,
    }
    
    struct CompileState {
        stage: CompileStage,
        current_file: String,
        files_compiled: usize,
        total_files: usize,
        compile_lines_seen: std::collections::HashSet<String>,
        compiled_lines_seen: std::collections::HashSet<String>,
        start_time: std::time::Instant,
        compile_stage_start: Option<std::time::Instant>,
        link_stage_start: Option<std::time::Instant>,
        generate_stage_start: Option<std::time::Instant>,
    }
    
    impl CompileState {
        fn new() -> Self {
            Self {
                stage: CompileStage::Initializing,
                current_file: String::new(),
                files_compiled: 0,
                total_files: 0,
                compile_lines_seen: std::collections::HashSet::new(),
                compiled_lines_seen: std::collections::HashSet::new(),
                start_time: std::time::Instant::now(),
                compile_stage_start: None,
                link_stage_start: None,
                generate_stage_start: None,
            }
        }
        
        fn calculate_progress(&self) -> f64 {
            match self.stage {
                CompileStage::Initializing => {
                    let elapsed = self.start_time.elapsed().as_secs_f64();
                    (elapsed / 2.0).min(5.0).max(1.0)
                }
                CompileStage::Compiling => {
                    let compile_elapsed = self.compile_stage_start
                        .map(|t| t.elapsed().as_secs_f64())
                        .unwrap_or(0.0);
                    
                    if self.total_files > 0 {
                        let file_progress = self.files_compiled as f64 / self.total_files as f64;
                        let file_based = 5.0 + (file_progress * 60.0);
                        let time_based = 5.0 + (compile_elapsed * 2.0).min(60.0);
                        (file_based * 0.9 + time_based * 0.1).min(65.0)
                    } else {
                        5.0 + (compile_elapsed * 2.0).min(60.0)
                    }
                }
                CompileStage::Linking => {
                    let link_elapsed = self.link_stage_start
                        .map(|t| t.elapsed().as_secs_f64())
                        .unwrap_or(0.0);
                    65.0 + (link_elapsed * 5.0).min(25.0)
                }
                CompileStage::Generating => {
                    let gen_elapsed = self.generate_stage_start
                        .map(|t| t.elapsed().as_secs_f64())
                        .unwrap_or(0.0);
                    90.0 + (gen_elapsed * 3.0).min(9.9)
                }
                CompileStage::Complete => 100.0,
            }
        }
    }
    
    // Regex patterns for parsing (similar to Python parser)
    lazy_static::lazy_static! {
        static ref RE_COMPILE_COMMAND: regex::Regex = regex::Regex::new(
            r"@([^\s]+\.(cpp|c|ino|S))|([^\s/\\]+\.(cpp|c|ino|S))"
        ).unwrap();
        static ref RE_COMPILE_LINE: regex::Regex = regex::Regex::new(
            r"(?i)compiling\s+([^\s]+\.(cpp|c|ino|S))"
        ).unwrap();
        static ref RE_COMPILED_FILE: regex::Regex = regex::Regex::new(
            r"(?i)\.(cpp|c|ino|S)\.o|gcc-ar|compiled\s+[^\s]+\.(cpp|c|ino|S)|using previously compiled file"
        ).unwrap();
    }
    
    // Build arduino-cli command
    let sketch_dir = PathBuf::from(&settings.sketch_directory);
    let sketch_file = sketch_dir.join(&settings.sketch_name);
    let build_path = sketch_dir.join("build");
    
    // Find arduino-cli using same logic as Python version
    // Python: script_path.parent.parent.parent.parent / "Arduino" / "arduino-cli.exe"
    // Where script_path is pmake.py in sketch_dir
    // So from sketch_dir, go up 3 levels to workspace root, then into Arduino/
    let arduino_cli = if settings.env == "arduino" {
        // Try workspace root/Arduino/arduino-cli.exe (same as Python pmake.py)
        // sketch_dir is like: .../projects/esp32-s3__LB-Gold/sht21-solo
        // Go up 3 levels: projects -> dev-boards (workspace root)
        let workspace_path = sketch_dir
            .parent()  // esp32-s3__LB-Gold
            .and_then(|p| p.parent())  // projects
            .and_then(|p| p.parent())  // dev-boards (workspace root)
            .map(|p| p.join("Arduino").join("arduino-cli.exe"));
        
        // Check if it exists, otherwise try PATH
        if let Some(path) = workspace_path {
            if path.exists() {
                path
            } else {
                // Try PATH
                if which::which("arduino-cli").is_ok() {
                    PathBuf::from("arduino-cli")
                } else {
                    // Return the expected path anyway (will show better error)
                    path
                }
            }
        } else {
            // Fallback to PATH
            if which::which("arduino-cli").is_ok() {
                PathBuf::from("arduino-cli")
            } else {
                PathBuf::from("arduino-cli") // Will fail with better error message
            }
        }
    } else {
        // For esp-idf, would need different command
        PathBuf::from("arduino-cli")
    };
    
    // Build command arguments
    let mut cmd = Command::new(&arduino_cli);
    cmd.arg("compile");
    cmd.arg("--fqbn").arg(&settings.fqbn);
    cmd.arg("--build-path").arg(&build_path);
    cmd.arg("--verbose");
    cmd.arg(&sketch_file);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.current_dir(&sketch_dir);
    
    // Add initial message
    {
        let mut state = dashboard.lock().unwrap();
        state.output_lines.push(format!("Executing: {:?} compile --fqbn {} --verbose {:?}", 
            arduino_cli, settings.fqbn, sketch_file));
        state.output_lines.push(format!("Build path: {:?}", build_path));
        state.output_lines.push(format!("Arduino CLI path: {:?}", arduino_cli));
        state.output_lines.push(format!("Arduino CLI exists: {}", arduino_cli.exists()));
        state.is_running = true;
        state.progress_stage = "Initializing".to_string();
        state.progress_percent = 0.0;
    }
    
    // Check if arduino-cli exists (unless it's in PATH)
    if !arduino_cli.exists() && arduino_cli.to_string_lossy() != "arduino-cli" {
        let mut state = dashboard.lock().unwrap();
        state.is_running = false;
        state.status_text = format!("Error: arduino-cli not found at: {:?}", arduino_cli);
        state.output_lines.push(format!("Error: arduino-cli not found at: {:?}", arduino_cli));
        state.output_lines.push("Please ensure arduino-cli.exe is installed in the Arduino directory at the workspace root.".to_string());
        return;
    }
    
    // Spawn process
    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.status_text = format!("Error: Failed to start arduino-cli: {}", e);
            state.output_lines.push(format!("Error: Failed to start arduino-cli: {}", e));
            state.output_lines.push(format!("Tried path: {:?}", arduino_cli));
            if !arduino_cli.exists() && arduino_cli.to_string_lossy() != "arduino-cli" {
                state.output_lines.push("The arduino-cli executable was not found at the expected location.".to_string());
            }
            return;
        }
    };
    
    // Read stderr in separate thread
    let dashboard_stderr = dashboard.clone();
    if let Some(stderr) = child.stderr.take() {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let cleaned = remove_ansi_escapes(&line);
                    let trimmed = cleaned.trim();
                    if !trimmed.is_empty() {
                        let mut state = dashboard_stderr.lock().unwrap();
                        state.output_lines.push(trimmed.to_string());
                        if state.output_lines.len() > 1 {
                            state.output_scroll = state.output_lines.len().saturating_sub(1);
                        }
                    }
                }
            }
        });
    }
    
    // Read stdout and parse
    let mut compile_state = CompileState::new();
    
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        
        for line_result in reader.lines() {
            let line = match line_result {
                Ok(l) => l,
                Err(_) => break,
            };
            
            let cleaned = remove_ansi_escapes(&line);
            let line_lower = cleaned.to_lowercase();
            let trimmed = cleaned.trim();
            
            if trimmed.is_empty() {
                continue;
            }
            
            // Add to output
            {
                let mut state = dashboard.lock().unwrap();
                state.output_lines.push(trimmed.to_string());
                if state.output_lines.len() > 1 {
                    state.output_scroll = state.output_lines.len().saturating_sub(1);
                }
            }
            
            // Parse line for compilation state
            // Detect errors
            if line_lower.contains("error") || line_lower.contains("fatal") {
                // Error detected - already added to output
                continue;
            }
            
            // Detect stages
            if line_lower.contains("detecting libraries") || line_lower.contains("detecting library") {
                compile_state.stage = CompileStage::Compiling;
                if compile_state.compile_stage_start.is_none() {
                    compile_state.compile_stage_start = Some(std::time::Instant::now());
                }
            } else if line_lower.contains("generating function prototypes") || line_lower.contains("generating prototypes") {
                compile_state.stage = CompileStage::Compiling;
            } else if line_lower.contains("linking everything together") || (line_lower.contains("linking") && line_lower.contains("together")) {
                compile_state.stage = CompileStage::Linking;
                compile_state.current_file.clear();
                if compile_state.link_stage_start.is_none() {
                    compile_state.link_stage_start = Some(std::time::Instant::now());
                }
            } else if line_lower.contains("creating esp32") || line_lower.contains("creating image") || 
                      (line_lower.contains("esptool") && line_lower.contains("elf2image")) {
                compile_state.stage = CompileStage::Generating;
                compile_state.current_file.clear();
                if compile_state.generate_stage_start.is_none() {
                    compile_state.generate_stage_start = Some(std::time::Instant::now());
                }
            } else if line_lower.contains("sketch uses") || line_lower.contains("global variables use") {
                compile_state.stage = CompileStage::Complete;
                compile_state.current_file.clear();
            }
            
            // Detect compilation commands/files
            if line.contains("xtensa-esp32s3-elf-g++") || line.contains("xtensa-esp32s3-elf-gcc") {
                if line.contains("-c") {
                    compile_state.stage = CompileStage::Compiling;
                    if compile_state.compile_stage_start.is_none() {
                        compile_state.compile_stage_start = Some(std::time::Instant::now());
                    }
                    
                    if let Some(captures) = RE_COMPILE_COMMAND.captures(&line) {
                        if let Some(file_match) = captures.get(1).or_else(|| captures.get(3)) {
                            let file_path = file_match.as_str();
                            compile_state.current_file = file_path.to_string();
                            if !compile_state.compile_lines_seen.contains(trimmed) {
                                compile_state.compile_lines_seen.insert(trimmed.to_string());
                                compile_state.total_files = compile_state.compile_lines_seen.len();
                            }
                        }
                    }
                }
            } else if let Some(captures) = RE_COMPILE_LINE.captures(&line_lower) {
                if let Some(file_match) = captures.get(1) {
                    let file_path = file_match.as_str();
                    compile_state.current_file = file_path.to_string();
                    compile_state.stage = CompileStage::Compiling;
                    if compile_state.compile_stage_start.is_none() {
                        compile_state.compile_stage_start = Some(std::time::Instant::now());
                    }
                    if !compile_state.compile_lines_seen.contains(trimmed) {
                        compile_state.compile_lines_seen.insert(trimmed.to_string());
                        compile_state.total_files = compile_state.compile_lines_seen.len();
                    }
                }
            } else if RE_COMPILED_FILE.is_match(&line_lower) {
                if !compile_state.compiled_lines_seen.contains(trimmed) {
                    compile_state.compiled_lines_seen.insert(trimmed.to_string());
                    compile_state.files_compiled = compile_state.compiled_lines_seen.len();
                }
            }
            
            // Update dashboard state
            {
                let mut state = dashboard.lock().unwrap();
                state.progress_percent = compile_state.calculate_progress();
                
                match compile_state.stage {
                    CompileStage::Initializing => state.progress_stage = "Initializing".to_string(),
                    CompileStage::Compiling => state.progress_stage = "Compiling".to_string(),
                    CompileStage::Linking => state.progress_stage = "Linking".to_string(),
                    CompileStage::Generating => state.progress_stage = "Generating".to_string(),
                    CompileStage::Complete => state.progress_stage = "Complete".to_string(),
                }
                
                state.current_file = compile_state.current_file.clone();
            }
        }
    }
    
    // Wait for process to finish
    let exit_status = child.wait();
    
    {
        let mut state = dashboard.lock().unwrap();
        state.is_running = false;
        
        match exit_status {
            Ok(status) => {
                if status.success() {
                    state.progress_percent = 100.0;
                    state.progress_stage = "Complete".to_string();
                    state.status_text = "Compilation completed successfully".to_string();
                    state.output_lines.push("Compilation completed successfully".to_string());
                } else {
                    state.status_text = format!("Compilation failed with exit code: {:?}", status.code());
                    state.output_lines.push(format!("Compilation failed with exit code: {:?}", status.code()));
                }
            }
            Err(e) => {
                state.status_text = format!("Error waiting for process: {}", e);
                state.output_lines.push(format!("Error waiting for process: {}", e));
            }
        }
    }
}
