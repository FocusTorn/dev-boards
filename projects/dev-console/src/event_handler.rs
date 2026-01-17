// Event handling module
// Handles all keyboard and mouse events

use crate::dashboard::DashboardState;
use crate::process_manager::ProcessManager;
use crate::command_helper::execute_command;
use crate::constants::HWND_MAIN_CONTENT_BOX;
use crate::field_editor::{FieldEditorState, SettingsFields};
use crate::layout_manager::LayoutManager;
use crate::settings_manager::SettingsManager;
use crate::profile_state::ProfileState;

use crossterm::event::{KeyCode, KeyModifiers, MouseEventKind};
use tui_input::{Input, InputRequest};
use tui_components::{TabBarManager, TabBar, TabBarStyle, Toast, ToastType, RectRegistry, get_box_by_name};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::sync::{Arc, Mutex};

/// Handle dashboard keyboard events
pub fn handle_dashboard_key_event(
    key_code: crossterm::event::KeyCode,
    dashboard: &Arc<Mutex<DashboardState>>,
    settings_manager: &SettingsManager,
    process_manager: Arc<ProcessManager>,
) -> bool {
    // Returns true if event was handled, false otherwise
    match key_code {
        crossterm::event::KeyCode::Esc => {
            // Cancel running command if one is active
            let is_running = {
                let state = dashboard.lock().unwrap();
                state.is_running
            };
            
            if is_running {
                process_manager.kill_all();
                let mut state = dashboard.lock().unwrap();
                state.is_running = false;
                state.set_status_text("Command cancelled");
                state.add_output_line("Command cancelled by user".to_string());
            }
            true
        }
        crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
            let mut state = dashboard.lock().unwrap();
            if state.selected_command > 0 {
                state.selected_command -= 1;
            }
            true
        }
        crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
            let mut state = dashboard.lock().unwrap();
            if state.selected_command < state.commands.len().saturating_sub(1) {
                state.selected_command += 1;
            }
            true
        }
        crossterm::event::KeyCode::Enter => {
            // Get command and latest settings
            let command = {
                let state = dashboard.lock().unwrap();
                state.commands[state.selected_command].clone()
            };
            
            // Reload settings from disk to ensure we have the absolute latest
            // (in case settings were updated via dropdown selection)
            let _ = settings_manager.reload();
            
            // Get latest settings from manager (always up-to-date)
            let settings = settings_manager.get();
            
            // Debug: Log settings being used for command
            {
                let mut state = dashboard.lock().unwrap();
                state.add_output_line(format!("[DEBUG] Command: {}", command));
                state.add_output_line(format!("[DEBUG] Sketch directory: '{}'", settings.sketch_directory));
                state.add_output_line(format!("[DEBUG] Sketch name: '{}'", settings.sketch_name));
            }
            
            // Execute command using helper (eliminates duplication)
            execute_command(&command, dashboard, settings, process_manager);
            
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

/// Result of handling a profile event
#[derive(Debug)]
pub enum ProfileEventResult {
    Continue,
    Toast(Toast),
    #[allow(dead_code)]
    RefreshProfiles,
    SaveProfile(String),
    LoadProfile(String),
}

/// Handle profile keyboard events
pub fn handle_profile_key_event(
    key_code: KeyCode,
    key_modifiers: KeyModifiers,
    profile_state: &ProfileState,
    settings_manager: &SettingsManager,
) -> ProfileEventResult {
    let is_active = *profile_state.is_active.lock().unwrap();
    
    match key_code {
        KeyCode::Char('p') | KeyCode::Char('P') if !key_modifiers.contains(KeyModifiers::CONTROL) => {
            // Toggle profile mode
            let mut active = profile_state.is_active.lock().unwrap();
            *active = !*active;
            if *active {
                // Refresh profiles when activating
                let _ = profile_state.refresh_profiles();
            } else {
                profile_state.clear_selection();
            }
            ProfileEventResult::Continue
        }
        KeyCode::Char('s') | KeyCode::Char('S') if !key_modifiers.contains(KeyModifiers::CONTROL) => {
            // Save current settings as profile - generate default name
            let settings = settings_manager.get();
            // Generate name like "sht21-solo-win" format: sketch_name-env-platform
            let platform = if cfg!(windows) { "win" } else if cfg!(unix) { "unix" } else { "other" };
            let default_name = if !settings.sketch_name.is_empty() {
                format!("{}-{}-{}", settings.sketch_name, settings.env, platform)
            } else {
                format!("profile-{}-{}", settings.env, platform)
            };
            ProfileEventResult::SaveProfile(default_name)
        }
        KeyCode::Char('l') | KeyCode::Char('L') if !key_modifiers.contains(KeyModifiers::CONTROL) => {
            // Load selected profile (works even when not active)
            if let Some(profile_name) = profile_state.get_selected_profile() {
                ProfileEventResult::LoadProfile(profile_name)
            } else {
                ProfileEventResult::Toast(Toast::new(
                    "No profile selected".to_string(),
                    ToastType::Error,
                ))
            }
        }
        KeyCode::Enter if is_active => {
            // Load selected profile
            if let Some(profile_name) = profile_state.get_selected_profile() {
                ProfileEventResult::LoadProfile(profile_name)
            } else {
                ProfileEventResult::Toast(Toast::new(
                    "No profile selected".to_string(),
                    ToastType::Error,
                ))
            }
        }
        KeyCode::Up | KeyCode::Char('k') if is_active => {
            profile_state.move_up();
            ProfileEventResult::Continue
        }
        KeyCode::Down | KeyCode::Char('j') if is_active => {
            profile_state.move_down();
            ProfileEventResult::Continue
        }
        _ => ProfileEventResult::Continue,
    }
}

/// Handle field editor keyboard events
pub fn handle_field_editor_key_event(
    key_code: KeyCode,
    key_modifiers: KeyModifiers,
    editor_state: &FieldEditorState,
    settings_manager: &SettingsManager,
    settings_fields: &SettingsFields,
    profile_state: &crate::profile_state::ProfileState,
    registry: &mut RectRegistry,
    main_content_tab_bar: &TabBarManager,
    tab_style: TabBarStyle,
) -> FieldEditorEventResult {
    match editor_state {
        FieldEditorState::Editing { field_index, input } => {
            handle_editing_key_event(key_code, key_modifiers, *field_index, input, settings_manager, settings_fields)
        }
        FieldEditorState::Selected { field_index } => {
            handle_selected_key_event(
                key_code,
                *field_index,
                settings_manager,
                settings_fields,
                profile_state,
                registry,
                main_content_tab_bar,
                tab_style,
            )
        }
        FieldEditorState::Selecting { field_index, selected_index, options } => {
            handle_selecting_key_event(key_code, *field_index, *selected_index, options, settings_manager, settings_fields)
        }
        FieldEditorState::ProfileSelecting { .. } => {
            // Enter and Esc are handled in the main loop
            FieldEditorEventResult::Continue
        }
    }
}

/// Handle keyboard events when editing a field
fn handle_editing_key_event(
    key_code: KeyCode,
    _key_modifiers: KeyModifiers,
    field_index: usize,
    input: &Input,
    settings_manager: &SettingsManager,
    settings_fields: &SettingsFields,
) -> FieldEditorEventResult {
    match key_code {
        KeyCode::Enter => {
            // Confirm edit - use SettingsManager to update and save atomically
            let value = input.value().to_string();
            match settings_manager.update(|settings| {
                settings_fields.set_value(settings, field_index, value);
            }) {
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
    settings_manager: &SettingsManager,
    settings_fields: &SettingsFields,
    profile_state: &crate::profile_state::ProfileState,
    registry: &mut RectRegistry,
    main_content_tab_bar: &TabBarManager,
    tab_style: TabBarStyle,
) -> FieldEditorEventResult {
    let settings = settings_manager.get(); // Get current settings
    match key_code {
        KeyCode::Char('q') | KeyCode::Char('Q') => FieldEditorEventResult::Exit,
        KeyCode::Enter => {
            // Check if field is a dropdown
            if settings_fields.is_dropdown(field_index) {
                // Open dropdown
                let options = settings_fields.get_dropdown_options(field_index, &settings);
                let current_value = settings_fields.get_value(&settings, field_index);
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
                let current_value = settings_fields.get_value(&settings, field_index);
                let mut input = Input::new(current_value);
                let _ = input.handle(InputRequest::GoToEnd);
                FieldEditorEventResult::StateChanged(FieldEditorState::Editing {
                    field_index,
                    input,
                })
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            profile_state.move_up();
            FieldEditorEventResult::Continue
        }
        KeyCode::Down | KeyCode::Char('j') => {
            profile_state.move_down();
            FieldEditorEventResult::Continue
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
    settings_manager: &SettingsManager,
    settings_fields: &SettingsFields,
) -> FieldEditorEventResult {
    match key_code {
        KeyCode::Enter => {
            // Confirm selection - use SettingsManager to update and save atomically
            if selected_index < options.len() {
                let selected_value = options[selected_index].clone();
                // Update settings and save
                match settings_manager.update(|settings| {
                    settings_fields.set_value(settings, field_index, selected_value.clone());
                }) {
                    Err(e) => FieldEditorEventResult::Toast(Toast::new(
                        format!("Failed to save settings: {}", e),
                        ToastType::Error,
                    )),
                    Ok(_) => {
                        // Verify the update was saved by reading it back
                        let saved_settings = settings_manager.get();
                        let saved_value = settings_fields.get_value(&saved_settings, field_index);
                        if saved_value != selected_value {
                            FieldEditorEventResult::Toast(Toast::new(
                                format!("Warning: Settings may not have saved correctly. Expected '{}', got '{}'", selected_value, saved_value),
                                ToastType::Error,
                            ))
                        } else {
                            FieldEditorEventResult::Toast(Toast::new("Settings saved".to_string(), ToastType::Success))
                        }
                    }
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
/// Works when hovering over the output panel (not just on scroll events)
/// Modifies the Arc directly to avoid overwriting state with stale local data
pub fn handle_dashboard_scroll(
    mouse_event: &crossterm::event::MouseEvent,
    dashboard_arc: &Arc<Mutex<DashboardState>>,
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
            
            // Column 1 width should match render/dashboard.rs (max_command_width + 4)
            let max_command_width = 15u16; // Conservative estimate matching most commands
            let commands_box_width = (max_command_width + 4).min(nested_area.width);

            let columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(commands_box_width),
                    Constraint::Min(0),
                ])
                .split(nested_area);
            
            let column2_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(4), // Match Constraint::Length(4) in dashboard.rs
                    Constraint::Min(0),
                ])
                .split(columns[1]);
            
            let output_area = column2_chunks[1];
            
            // Check if mouse is over output area (hover detection)
            let is_over_output = mouse_event.column >= output_area.x && 
                                 mouse_event.column < output_area.x + output_area.width &&
                                 mouse_event.row >= output_area.y && 
                                 mouse_event.row < output_area.y + output_area.height;
            
            if is_over_output {
                // Modify Arc directly to avoid overwriting state
                if let Ok(mut state) = dashboard_arc.lock() {
                    match mouse_event.kind {
                        MouseEventKind::ScrollUp => {
                            state.scroll_output_up(3);
                        }
                        MouseEventKind::ScrollDown => {
                            state.scroll_output_down(3);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

/// Handle mouse clicks on settings fields
pub fn handle_settings_field_click(
    mouse_event: &crossterm::event::MouseEvent,
    settings_manager: &SettingsManager,
    settings_fields: &SettingsFields,
    registry: &RectRegistry,
    main_content_tab_bar: &TabBarManager,
    layout_manager: &mut LayoutManager,
) -> Option<FieldEditorState> {
    let settings = settings_manager.get(); // Get current settings
    if let Some(active_tab_idx) = registry.get_active_tab(main_content_tab_bar.handle()) {
        if let Some(tab_bar_state) = registry.get_tab_bar_state(main_content_tab_bar.handle()) {
            if let Some(tab_config) = tab_bar_state.tab_configs.get(active_tab_idx) {
                if tab_config.id == "settings" {
                    if let Some(box_manager) = get_box_by_name(registry, HWND_MAIN_CONTENT_BOX) {
                        if let Some(content_rect) = box_manager.metrics(registry) {
                            let content_rect: Rect = content_rect.into();
                            
                            // Use LayoutManager for cached content area calculation
                            if let Some(content_area) = layout_manager.get_content_area(content_rect) {
                                let content_x = content_area.x;
                                let content_y = content_area.y;
                                let content_width = content_area.width;
                                
                                // Check if click is within content area
                                if mouse_event.column >= content_x && mouse_event.column < content_x + content_width &&
                                   mouse_event.row >= content_y && mouse_event.row < content_y + content_area.height {
                                    
                                    // Check top section (Sketch Directory, Sketch Name)
                                    if mouse_event.row >= content_y && mouse_event.row < content_y + 6 {
                                        let field_index = if mouse_event.row < content_y + 3 { 0 } else { 1 };
                                        // Check if dropdown field
                                        if settings_fields.is_dropdown(field_index) {
                                            let options = settings_fields.get_dropdown_options(field_index, &settings);
                                            let current_value = settings_fields.get_value(&settings, field_index);
                                            let selected_index = options.iter()
                                                .position(|opt| opt == &current_value)
                                                .unwrap_or(0);
                                            return Some(FieldEditorState::Selecting {
                                                field_index,
                                                selected_index,
                                                options,
                                            });
                                        } else {
                                            // Start editing directly on click
                                            let current_value = settings_fields.get_value(&settings, field_index);
                                            let mut input = Input::new(current_value);
                                            let _ = input.handle(InputRequest::GoToEnd);
                                            return Some(FieldEditorState::Editing {
                                                field_index,
                                                input,
                                            });
                                        }
                                    } else {
                                        // Check bottom section (Device | Connection | MQTT with 2 sub-columns)
                                        let section_y = content_y + 6;
                                        
                                        let bottom_columns = Layout::default()
                                            .direction(Direction::Horizontal)
                                            .constraints([
                                                Constraint::Percentage(25), // Device
                                                Constraint::Percentage(25), // Connection
                                                Constraint::Percentage(50), // MQTT (2 sub-columns)
                                            ])
                                            .split(Rect {
                                                x: content_x,
                                                y: section_y,
                                                width: content_width,
                                                height: content_area.height.saturating_sub(6),
                                            });
                                        
                                        // Helper to find which field in a section was clicked
                                        let find_field = |mouse_x: u16, mouse_y: u16, area: Rect, indices: &[usize]| -> Option<usize> {
                                            if mouse_x >= area.x && mouse_x < area.x + area.width &&
                                               mouse_y >= area.y && mouse_y < area.y + area.height {
                                                let relative_y = mouse_y.saturating_sub(area.y + 1); // +1 for top border
                                                let field_offset = (relative_y / 4) as usize; // 3 lines per field + 1 spacing
                                                if field_offset < indices.len() {
                                                    return Some(indices[field_offset]);
                                                }
                                            }
                                            None
                                        };

                                        // Check Device and Connection columns
                                        let clicked_field = find_field(mouse_event.column, mouse_event.row, bottom_columns[0], &[2, 3, 4])
                                            .or_else(|| find_field(mouse_event.column, mouse_event.row, bottom_columns[1], &[5, 6]));
                                        
                                        // Check MQTT section (2 sub-columns)
                                        let clicked_field = clicked_field.or_else(|| {
                                            let mqtt_columns = Layout::default()
                                                .direction(Direction::Horizontal)
                                                .constraints([
                                                    Constraint::Percentage(50), // Credentials
                                                    Constraint::Percentage(50), // Topics
                                                ])
                                                .split(bottom_columns[2]);
                                            
                                            find_field(mouse_event.column, mouse_event.row, mqtt_columns[0], &[7, 8, 9, 10])
                                                .or_else(|| find_field(mouse_event.column, mouse_event.row, mqtt_columns[1], &[11, 12, 13]))
                                        });

                                        if let Some(field_index) = clicked_field {
                                            // Check if dropdown field
                                            if settings_fields.is_dropdown(field_index) {
                                                let options = settings_fields.get_dropdown_options(field_index, &settings);
                                                let current_value = settings_fields.get_value(&settings, field_index);
                                                let selected_index = options.iter()
                                                    .position(|opt| opt == &current_value)
                                                    .unwrap_or(0);
                                                return Some(FieldEditorState::Selecting {
                                                    field_index,
                                                    selected_index,
                                                    options,
                                                });
                                            } else {
                                                let current_value = settings_fields.get_value(&settings, field_index);
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
