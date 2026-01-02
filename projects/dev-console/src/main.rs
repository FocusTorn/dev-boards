// ESP32-S3 Dev Console
// TUI application for managing ESP32-S3 development settings

// MODULES ------------------>> 

mod settings;
mod settings_manager;
mod field_editor;
mod dashboard;
mod dashboard_batch;
mod config;
mod config_validation;
mod error_format;
mod tool_detector;
mod string_intern;
mod render;
mod commands;
mod command_helper;
mod process_manager;
mod constants;
mod path_utils;
mod layout_utils;
mod app_state;
mod layout_cache;
mod layout_manager;
mod event_handler;
mod ui_coordinator;
mod progress_tracker;
mod progress_history;

//--------------------------------------------------------<<
// IMPORTS ------------------>> 

use crate::layout_manager::LayoutManager;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::Rect,
};
use std::io;
use tui_components::{
    BaseLayoutConfig,
    BindingConfig, StatusBarConfig,
    DimmingContext, RectRegistry, Popup,
    TabBar, TabBarStyle, RectHandle,
    Toast,
    TabBarManager,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

// Module imports
use app_state::AppState;
use config_validation::load_and_validate_config;
use constants::*;
use event_handler::{
    handle_dashboard_key_event,
    handle_dashboard_scroll,
    handle_field_editor_key_event,
    handle_editing_input,
    handle_dropdown_navigation,
    handle_settings_field_click,
    handle_tab_click,
    FieldEditorEventResult,
};
use ui_coordinator::{render_ui, handle_cursor_positioning};
use field_editor::FieldEditorState;

//--------------------------------------------------------<<


// ┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
// │                                                 MAIN ENTRY POINT                                                 │
// └──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let popup: Option<Popup> = None;
    let mut toasts: Vec<Toast> = Vec::new();
    let mut registry = RectRegistry::new();
    
    // Initialize application state
    let mut app_state = AppState::new();
    
    // Load and validate configuration from YAML file (with error recovery)
    let app_config = load_and_validate_config(None)?;
    
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
    let mut layout_manager = LayoutManager::new();
    
    // ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
    // │                                           MAIN LOOP                                            │
    // └────────────────────────────────────────────────────────────────────────────────────────────────┘ 
          
    loop {
        terminal.draw(|f| {
            let area = f.area();
            
            // Create dimming context based on popup state or dropdown selection
            let is_selecting = matches!(app_state.field_editor_state, FieldEditorState::Selecting { .. });
            let dimming = DimmingContext::new(popup.is_some() || is_selecting);
            
            // Render UI using coordinator
            render_ui(
                f,
                area,
                &config,
                &dimming,
                &mut registry,
                main_content_box_handle_name,
                &mut original_anchor_metrics,
                &mut layout_manager,
                &main_content_tab_bar,
                tab_style,
                &app_state.settings,
                &app_state.settings_fields,
                &app_state.field_editor_state,
                &app_state.dashboard,
                &popup,
                &toasts,
                &mut current_tab_bar,
            );
            
            // Handle cursor positioning for editing fields
            handle_cursor_positioning(
                f,
                &app_state.field_editor_state,
                &registry,
                &main_content_tab_bar,
                main_content_box_handle_name,
                &mut layout_manager,
            );
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
                                        // SettingsManager always has latest values - no reload needed
                                        if handle_dashboard_key_event(
                                            key.code,
                                            &app_state.dashboard,
                                            &app_state.settings,
                                            app_state.process_manager.clone(),
                                        ) {
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Handle field editor events
                                match key.code {
                            KeyCode::Enter | KeyCode::Esc if matches!(app_state.field_editor_state, FieldEditorState::Editing { .. }) => {
                                match handle_field_editor_key_event(
                                    key.code,
                                    key.modifiers,
                                    &app_state.field_editor_state,
                                    &app_state.settings,
                                    &app_state.settings_fields,
                                    &mut registry,
                                    &main_content_tab_bar,
                                    tab_style,
                                ) {
                                    FieldEditorEventResult::StateChanged(new_state) => {
                                        app_state.field_editor_state = new_state;
                                    }
                                    FieldEditorEventResult::Toast(toast) => {
                                        toasts.push(toast);
                                        if let FieldEditorState::Editing { field_index, .. } = app_state.field_editor_state {
                                            app_state.field_editor_state = FieldEditorState::Selected { field_index };
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            KeyCode::Enter | KeyCode::Esc if matches!(app_state.field_editor_state, FieldEditorState::Selecting { .. }) => {
                                match handle_field_editor_key_event(
                                    key.code,
                                    key.modifiers,
                                    &app_state.field_editor_state,
                                    &app_state.settings,
                                    &app_state.settings_fields,
                                    &mut registry,
                                    &main_content_tab_bar,
                                    tab_style,
                                ) {
                                    FieldEditorEventResult::StateChanged(new_state) => {
                                        app_state.field_editor_state = new_state;
                                    }
                                    FieldEditorEventResult::Toast(toast) => {
                                        toasts.push(toast);
                                        if let FieldEditorState::Selecting { field_index, .. } = app_state.field_editor_state {
                                            app_state.field_editor_state = FieldEditorState::Selected { field_index };
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {
                                match &mut app_state.field_editor_state {
                                    FieldEditorState::Editing { ref mut input, .. } => {
                                        handle_editing_input(key.code, key.modifiers, input);
                                    }
                                    FieldEditorState::Selected { .. } => {
                                        match handle_field_editor_key_event(
                                            key.code,
                                            key.modifiers,
                                            &app_state.field_editor_state,
                                            &app_state.settings,
                                            &app_state.settings_fields,
                                            &mut registry,
                                            &main_content_tab_bar,
                                            tab_style,
                                        ) {
                                            FieldEditorEventResult::Exit => {
                                                break;
                                            }
                                            FieldEditorEventResult::StateChanged(new_state) => {
                                                app_state.field_editor_state = new_state;
                                            }
                                            FieldEditorEventResult::Toast(toast) => {
                                                toasts.push(toast);
                                            }
                                            _ => {}
                                        }
                                    }
                                    FieldEditorState::Selecting { ref mut selected_index, ref options, .. } => {
                                        handle_dropdown_navigation(key.code, selected_index, options);
                                    }
                                }
                            }
                        }
                    }
                    Event::Mouse(mouse_event) => {
                        // Handle mouse scrolling for dashboard output
                        // Only process scroll events, not all mouse movement
                        if matches!(mouse_event.kind, MouseEventKind::ScrollUp | MouseEventKind::ScrollDown) {
                            if let Some(active_tab_idx) = registry.get_active_tab(main_content_tab_bar.handle()) {
                                if let Some(tab_bar_state) = registry.get_tab_bar_state(main_content_tab_bar.handle()) {
                                    if let Some(tab_config) = tab_bar_state.tab_configs.get(active_tab_idx) {
                                        if tab_config.id == "dashboard" {
                                            // Modify Arc directly (no local copy anymore)
                                            handle_dashboard_scroll(&mouse_event, &app_state.dashboard, &registry);
                                        }
                                    }
                                }
                            }
                        }
                        
                        if mouse_event.kind == MouseEventKind::Down(crossterm::event::MouseButton::Left) {
                            // Handle mouse clicks on tabs
                            handle_tab_click(
                                &mouse_event,
                                &current_tab_bar,
                                &mut registry,
                                &main_content_tab_bar,
                                tab_style,
                            );
                            
                            // Handle mouse clicks on settings fields
                            if let Some(new_state) = handle_settings_field_click(
                                &mouse_event,
                                &app_state.settings,
                                &app_state.settings_fields,
                                &registry,
                                &main_content_tab_bar,
                                &mut layout_manager,
                            ) {
                                app_state.field_editor_state = new_state;
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
    app_state.process_manager.cleanup();
    
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
