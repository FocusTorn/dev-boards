// Event handling module
// Handles all keyboard and mouse events

use crate::dashboard::DashboardState;
use crate::process_manager::ProcessManager;
use crate::commands::{execute_upload_rust, execute_progress_rust};
use crate::constants::HWND_MAIN_CONTENT_BOX;
use crate::field_editor::{FieldEditorState, SettingsFields};
use crate::layout_utils::calculate_centered_content_area;
use crate::layout_cache::LayoutCache;

use crossterm::event::{KeyCode, KeyModifiers, MouseEventKind};
use tui_input::{Input, InputRequest};
use tui_components::{TabBarManager, TabBar, TabBarStyle, Toast, ToastType, RectRegistry, get_box_by_name};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::settings::Settings;

/// Handle dashboard keyboard events
pub fn handle_dashboard_key_event(
    key_code: crossterm::event::KeyCode,
    dashboard_state: &mut DashboardState,
    dashboard_arc: &Arc<Mutex<DashboardState>>,
    settings: Settings,
    process_manager_arc: Arc<ProcessManager>,
) -> bool {
    // Returns true if event was handled, false otherwise
    match key_code {
        crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
            if dashboard_state.selected_command > 0 {
                dashboard_state.selected_command -= 1;
                if let Ok(mut state) = dashboard_arc.lock() {
                    state.selected_command = dashboard_state.selected_command;
                }
            }
            true
        }
        crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
            if dashboard_state.selected_command < dashboard_state.commands.len().saturating_sub(1) {
                dashboard_state.selected_command += 1;
                if let Ok(mut state) = dashboard_arc.lock() {
                    state.selected_command = dashboard_state.selected_command;
                }
            }
            true
        }
        crossterm::event::KeyCode::Enter => {
            // Execute selected command
            let command = dashboard_state.commands[dashboard_state.selected_command].clone();
            
            // Clear previous output
            dashboard_state.output_lines.clear();
            dashboard_state.output_scroll = 0;
            
            if command == "Compile" {
                // Rust-based progress command
                dashboard_state.is_running = true;
                dashboard_state.progress_percent = 0.0;
                dashboard_state.set_progress_stage("Initializing");
                dashboard_state.set_current_file("");
                dashboard_state.set_status_text(&format!("Running: {}", command));
                dashboard_state.add_output_line(format!("> {}", command));
                
                // Update Arc with current state before spawning thread
                if let Ok(mut state) = dashboard_arc.lock() {
                    *state = dashboard_state.clone();
                }
                
                let dashboard_clone = dashboard_arc.clone();
                let settings_clone = settings;
                let process_manager_clone = process_manager_arc.clone();
                
                thread::spawn(move || {
                    execute_progress_rust(dashboard_clone, settings_clone, process_manager_clone);
                });
            } else if command == "Upload" {
                // Rust-based upload command
                dashboard_state.is_running = true;
                dashboard_state.progress_percent = 0.0;
                dashboard_state.set_progress_stage("Initializing");
                dashboard_state.set_current_file("");
                dashboard_state.set_status_text(&format!("Running: {}", command));
                dashboard_state.add_output_line(format!("> {}", command));
                
                // Update Arc with current state before spawning thread
                if let Ok(mut state) = dashboard_arc.lock() {
                    *state = dashboard_state.clone();
                }
                
                let dashboard_clone = dashboard_arc.clone();
                let settings_clone = settings;
                let process_manager_clone = process_manager_arc.clone();
                
                thread::spawn(move || {
                    execute_upload_rust(dashboard_clone, settings_clone, process_manager_clone);
                });
            } else {
                // For other commands, use regular status
                dashboard_state.is_running = false;
                dashboard_state.set_progress_stage("");
                dashboard_state.set_status_text(&format!("Running: {}", command));
                dashboard_state.add_output_line(format!("> {}", command));
                dashboard_state.add_output_line("Command execution not yet implemented".to_string());
                
                // Update Arc
                if let Ok(mut state) = dashboard_arc.lock() {
                    *state = dashboard_state.clone();
                }
            }
            
            true
        }
        _ => false,
    }
}

/// Result of handling a field editor event
#[derive(Debug)]
pub enum FieldEditorEventResult {
    Continue,
    Exit,
    Toast(Toast),
    StateChanged(FieldEditorState),
}

/// Handle field editor keyboard events
pub fn handle_field_editor_key_event(
    key_code: KeyCode,
    key_modifiers: KeyModifiers,
    field_editor_state: &FieldEditorState,
    settings: &mut Settings,
    settings_fields: &SettingsFields,
    registry: &mut RectRegistry,
    main_content_tab_bar: &TabBarManager,
    tab_style: TabBarStyle,
) -> FieldEditorEventResult {
    match field_editor_state {
        FieldEditorState::Editing { field_index, input } => {
            handle_editing_key_event(key_code, key_modifiers, *field_index, input, settings, settings_fields)
        }
        FieldEditorState::Selected { field_index } => {
            handle_selected_key_event(key_code, *field_index, settings, settings_fields, registry, main_content_tab_bar, tab_style)
        }
        FieldEditorState::Selecting { field_index, selected_index, options } => {
            handle_selecting_key_event(key_code, *field_index, *selected_index, options, settings, settings_fields)
        }
    }
}

/// Handle keyboard events when editing a field
fn handle_editing_key_event(
    key_code: KeyCode,
    key_modifiers: KeyModifiers,
    field_index: usize,
    input: &Input,
    settings: &mut Settings,
    settings_fields: &SettingsFields,
) -> FieldEditorEventResult {
    match key_code {
        KeyCode::Enter => {
            // Confirm edit
            settings_fields.set_value(settings, field_index, input.value().to_string());
            match settings.save() {
                Err(e) => FieldEditorEventResult::Toast(Toast::new(
                    format!("Failed to save settings: {}", e),
                    ToastType::Error,
                )),
                Ok(_) => FieldEditorEventResult::Toast(Toast::new("Settings saved".to_string(), ToastType::Success)),
            }
        }
        KeyCode::Esc => {
            FieldEditorEventResult::StateChanged(FieldEditorState::Selected { field_index })
        }
        _ => FieldEditorEventResult::Continue,
    }
}

/// Handle keyboard events when a field is selected
fn handle_selected_key_event(
    key_code: KeyCode,
    field_index: usize,
    settings: &Settings,
    settings_fields: &SettingsFields,
    registry: &mut RectRegistry,
    main_content_tab_bar: &TabBarManager,
    tab_style: TabBarStyle,
) -> FieldEditorEventResult {
    match key_code {
        KeyCode::Char('q') | KeyCode::Char('Q') => FieldEditorEventResult::Exit,
        KeyCode::Enter => {
            // Check if field is a dropdown
            if settings_fields.is_dropdown(field_index) {
                // Open dropdown
                let options = settings_fields.get_dropdown_options(field_index);
                let current_value = settings_fields.get_value(settings, field_index);
                let selected_index = options.iter()
                    .position(|opt| opt == &current_value)
                    .unwrap_or(0);
                FieldEditorEventResult::StateChanged(FieldEditorState::Selecting {
                    field_index,
                    selected_index,
                    options,
                })
            } else {
                // Start text editing
                let current_value = settings_fields.get_value(settings, field_index);
                let mut input = Input::new(current_value);
                let _ = input.handle(InputRequest::GoToEnd);
                FieldEditorEventResult::StateChanged(FieldEditorState::Editing {
                    field_index,
                    input,
                })
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if field_index > 0 {
                FieldEditorEventResult::StateChanged(FieldEditorState::Selected {
                    field_index: field_index - 1,
                })
            } else {
                FieldEditorEventResult::Continue
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if field_index < settings_fields.count() - 1 {
                FieldEditorEventResult::StateChanged(FieldEditorState::Selected {
                    field_index: field_index + 1,
                })
            } else {
                FieldEditorEventResult::Continue
            }
        }
        KeyCode::Tab => {
            let next_index = (field_index + 1) % settings_fields.count();
            FieldEditorEventResult::StateChanged(FieldEditorState::Selected {
                field_index: next_index,
            })
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if tab_style != TabBarStyle::BoxStatic && tab_style != TabBarStyle::TextStatic {
                main_content_tab_bar.navigate_previous(registry);
            }
            FieldEditorEventResult::Continue
        }
        KeyCode::Right | KeyCode::Char('l') => {
            if tab_style != TabBarStyle::BoxStatic && tab_style != TabBarStyle::TextStatic {
                main_content_tab_bar.navigate_next(registry);
            }
            FieldEditorEventResult::Continue
        }
        _ => FieldEditorEventResult::Continue,
    }
}

/// Handle keyboard events when selecting from a dropdown (for Enter/Esc only)
fn handle_selecting_key_event(
    key_code: KeyCode,
    field_index: usize,
    selected_index: usize,
    options: &Vec<String>,
    settings: &mut Settings,
    settings_fields: &SettingsFields,
) -> FieldEditorEventResult {
    match key_code {
        KeyCode::Enter => {
            // Confirm selection
            if selected_index < options.len() {
                let selected_value = options[selected_index].clone();
                settings_fields.set_value(settings, field_index, selected_value);
                match settings.save() {
                    Err(e) => FieldEditorEventResult::Toast(Toast::new(
                        format!("Failed to save settings: {}", e),
                        ToastType::Error,
                    )),
                    Ok(_) => FieldEditorEventResult::Toast(Toast::new("Settings saved".to_string(), ToastType::Success)),
                }
            } else {
                FieldEditorEventResult::StateChanged(FieldEditorState::Selected { field_index })
            }
        }
        KeyCode::Esc => {
            FieldEditorEventResult::StateChanged(FieldEditorState::Selected { field_index })
        }
        _ => FieldEditorEventResult::Continue,
    }
}

/// Handle editing input events (characters, backspace, etc.)
pub fn handle_editing_input(
    key_code: KeyCode,
    key_modifiers: KeyModifiers,
    input: &mut Input,
) {
    match key_code {
        KeyCode::Char(c) => {
            if key_modifiers.contains(KeyModifiers::CONTROL) {
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

/// Handle dropdown navigation
pub fn handle_dropdown_navigation(
    key_code: KeyCode,
    selected_index: &mut usize,
    options: &Vec<String>,
) {
    match key_code {
        KeyCode::Up | KeyCode::Char('k') => {
            if *selected_index > 0 {
                *selected_index -= 1;
            } else {
                *selected_index = options.len().saturating_sub(1);
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if *selected_index < options.len().saturating_sub(1) {
                *selected_index += 1;
            } else {
                *selected_index = 0;
            }
        }
        _ => {}
    }
}

/// Handle mouse scrolling for dashboard output
pub fn handle_dashboard_scroll(
    mouse_event: &crossterm::event::MouseEvent,
    dashboard_state: &mut DashboardState,
    registry: &RectRegistry,
) {
    if let Some(box_manager) = get_box_by_name(registry, HWND_MAIN_CONTENT_BOX) {
        if let Some(content_rect) = box_manager.metrics(registry) {
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

/// Handle mouse clicks on settings fields
pub fn handle_settings_field_click(
    mouse_event: &crossterm::event::MouseEvent,
    settings: &Settings,
    settings_fields: &SettingsFields,
    registry: &RectRegistry,
    main_content_tab_bar: &TabBarManager,
    layout_cache: &mut LayoutCache,
) -> Option<FieldEditorState> {
    if let Some(active_tab_idx) = registry.get_active_tab(main_content_tab_bar.handle()) {
        if let Some(tab_bar_state) = registry.get_tab_bar_state(main_content_tab_bar.handle()) {
            if let Some(tab_config) = tab_bar_state.tab_configs.get(active_tab_idx) {
                if tab_config.id == "settings" {
                    if let Some(box_manager) = get_box_by_name(registry, HWND_MAIN_CONTENT_BOX) {
                        if let Some(content_rect) = box_manager.metrics(registry) {
                            let content_rect: Rect = content_rect.into();
                            
                            // Use cached content area or calculate and cache it
                            if let Some(content_area) = layout_cache.get_content_area()
                                .filter(|cached| {
                                    cached.width == content_rect.width && cached.height == content_rect.height
                                })
                                .or_else(|| {
                                    calculate_centered_content_area(content_rect).map(|area| {
                                        layout_cache.set_content_area(area);
                                        area
                                    })
                                })
                            {
                                let content_x = content_area.x;
                                let content_y = content_area.y;
                                let content_width = content_area.width;
                                
                                // Check if click is within content area
                                if mouse_event.column >= content_x && mouse_event.column < content_x + content_width &&
                                   mouse_event.row >= content_y && mouse_event.row < content_y + content_area.height {
                                    
                                    // Check top section (Sketch Directory, Sketch Name)
                                    if mouse_event.row >= content_y && mouse_event.row < content_y + 6 {
                                        let field_index = if mouse_event.row < content_y + 3 { 0 } else { 1 };
                                        // Start editing directly on click
                                        let current_value = settings_fields.get_value(settings, field_index);
                                        let mut input = Input::new(current_value);
                                        let _ = input.handle(InputRequest::GoToEnd);
                                        return Some(FieldEditorState::Editing {
                                            field_index,
                                            input,
                                        });
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
                                                        let current_value = settings_fields.get_value(settings, field_index);
                                                        let selected_index = options.iter()
                                                            .position(|opt| opt == &current_value)
                                                            .unwrap_or(0);
                                                        return Some(FieldEditorState::Selecting {
                                                            field_index,
                                                            selected_index,
                                                            options,
                                                        });
                                                    } else {
                                                        let current_value = settings_fields.get_value(settings, field_index);
                                                        let mut input = Input::new(current_value);
                                                        let _ = input.handle(InputRequest::GoToEnd);
                                                        return Some(FieldEditorState::Editing {
                                                            field_index,
                                                            input,
                                                        });
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
                                                        let current_value = settings_fields.get_value(settings, field_index);
                                                        let selected_index = options.iter()
                                                            .position(|opt| opt == &current_value)
                                                            .unwrap_or(0);
                                                        return Some(FieldEditorState::Selecting {
                                                            field_index,
                                                            selected_index,
                                                            options,
                                                        });
                                                    } else {
                                                        let current_value = settings_fields.get_value(settings, field_index);
                                                        let mut input = Input::new(current_value);
                                                        let _ = input.handle(InputRequest::GoToEnd);
                                                        return Some(FieldEditorState::Editing {
                                                            field_index,
                                                            input,
                                                        });
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
    None
}

/// Handle mouse clicks on tabs
pub fn handle_tab_click(
    mouse_event: &crossterm::event::MouseEvent,
    current_tab_bar: &Option<(TabBar, tui_components::RectHandle)>,
    registry: &mut RectRegistry,
    main_content_tab_bar: &TabBarManager,
    tab_style: TabBarStyle,
) {
    if let Some((ref tab_bar, _handle)) = current_tab_bar {
        let clicked_tab: Option<usize> = tab_bar.get_tab_at(mouse_event.column, mouse_event.row, Some(registry));
        if let Some(clicked_tab_idx) = clicked_tab {
            if tab_style != TabBarStyle::BoxStatic && tab_style != TabBarStyle::TextStatic {
                main_content_tab_bar.set_active(registry, clicked_tab_idx);
            }
        }
    }
}
