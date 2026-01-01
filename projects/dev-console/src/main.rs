// ESP32-S3 Dev Console
// TUI application for managing ESP32-S3 development settings

// MODULES ------------------>> 

mod settings;
mod field_editor;
mod dashboard;
mod config;
mod render;
mod commands;
mod process_manager;

//--------------------------------------------------------<<
// IMPORTS ------------------>> 

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem},
};
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use tui_components::{
    BaseLayout, BaseLayoutConfig, BaseLayoutResult,
    BindingConfig, StatusBarConfig,
    DimmingContext, RectRegistry, Popup, render_popup,
    TabBar, TabBarStyle, RectHandle,
    Toast, ToastType, render_toasts,
    TabBarManager,
    get_box_by_name,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui_input::{Input, InputRequest};

// Module imports
use settings::Settings;
use field_editor::{FieldEditorState, SettingsFields};
use dashboard::DashboardState;
use config::load_config;

use commands::{execute_upload_rust, execute_progress_rust};

use render::{render_content, render_settings, render_dashboard};
use process_manager::ProcessManager;

//--------------------------------------------------------<<

const HWND_MAIN_CONTENT_BOX: &str = "hwndMainContentBox";
const HWND_MAIN_CONTENT_TAB_BAR: &str = "hwndMainContentTabBar";


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
    
    // Process manager for tracking and cleaning up child processes
    let process_manager = ProcessManager::new();
    let process_manager_arc = Arc::new(process_manager);
    
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
                                                
                                                if command == "Compile" { //>
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
                                                    let process_manager_clone = process_manager_arc.clone();
                                                    
                                                    thread::spawn(move || {
                                                        execute_progress_rust(dashboard_clone, settings_clone, process_manager_clone);
                                                    });
                                                } //<
                                                else if command == "Upload" { //>
                                                    // Rust-based upload command
                                                    dashboard_state.is_running = true;
                                                    dashboard_state.progress_percent = 0.0;
                                                    dashboard_state.progress_stage = "Initializing".to_string();
                                                    dashboard_state.current_file = String::new();
                                                    dashboard_state.status_text = format!("Running: {}", command);
                                                    dashboard_state.output_lines.push(format!("> {}", command));
                                                    
                                                    // Execute upload command in background thread
                                                    let settings_clone = settings.clone();
                                                    
                                                    // Update Arc with current state before spawning thread
                                                    if let Ok(mut state) = dashboard_arc.lock() {
                                                        *state = dashboard_state.clone();
                                                    }
                                                    
                                                    let dashboard_clone = dashboard_arc.clone();
                                                    let process_manager_clone = process_manager_arc.clone();
                                                    
                                                    thread::spawn(move || {
                                                        execute_upload_rust(dashboard_clone, settings_clone, process_manager_clone);
                                                    });
                                                } //<
                                                
                                                // Orriginal PMake calls
                                                // else if command == "Progress-Py" { //>
                                                //     // For progress command, initialize progress state
                                                //     dashboard_state.is_running = true;
                                                //     dashboard_state.progress_percent = 0.0;
                                                //     dashboard_state.progress_stage = "Initializing".to_string();
                                                //     dashboard_state.current_file = String::new();
                                                //     dashboard_state.status_text = format!("Running: {}", command);
                                                //     dashboard_state.output_lines.push(format!("> {}", command));
                                                    
                                                //     // Execute progress command in background thread
                                                //     let settings_clone = settings.clone();
                                                    
                                                //     // Update Arc with current state before spawning thread
                                                //     if let Ok(mut state) = dashboard_arc.lock() {
                                                //         *state = dashboard_state.clone();
                                                //     }
                                                    
                                                //     let dashboard_clone = dashboard_arc.clone();
                                                //     let process_manager_clone = process_manager_arc.clone();
                                                    
                                                //     thread::spawn(move || {
                                                //         execute_progress_command(dashboard_clone, settings_clone, process_manager_clone);
                                                //     });
                                                // }  //<
                                                // else if command == "Build" || command == "Compile-py" || command == "Upload-py" { //>
                                                //     // For Build, Compile, and Upload commands
                                                //     dashboard_state.is_running = false; // No progress bar for these
                                                //     dashboard_state.progress_stage = String::new();
                                                //     dashboard_state.status_text = format!("Running: {}", command);
                                                //     dashboard_state.output_lines.push(format!("> {}", command));
                                                    
                                                //     // Update Arc with current state before spawning thread
                                                //     if let Ok(mut state) = dashboard_arc.lock() {
                                                //         *state = dashboard_state.clone();
                                                //     }
                                                    
                                                //     // Execute command in background thread
                                                //     let settings_clone = settings.clone();
                                                //     let dashboard_clone = dashboard_arc.clone();
                                                //     let command_clone = command.clone();
                                                //     let process_manager_clone = process_manager_arc.clone();
                                                    
                                                //     thread::spawn(move || {
                                                //         execute_pmake_command(dashboard_clone, settings_clone, command_clone, process_manager_clone);
                                                //     });
                                                // } //<
                                                
                                                else { //>
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
                                                } //<
                                                
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
    
    // Cleanup: Kill any running child processes before exiting
    process_manager_arc.cleanup();
    
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
