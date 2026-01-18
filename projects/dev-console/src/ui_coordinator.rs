// UI rendering coordination module
// Handles UI rendering logic and layout management

use crate::render::{render_content, render_settings, render_dashboard};
use crate::field_editor::{FieldEditorState, SettingsFields};
use crate::dashboard::DashboardState;
use crate::layout_manager::LayoutManager;
use crate::profile_state::ProfileState;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame,
};
use tui_components::{
    BaseLayout, BaseLayoutConfig, BaseLayoutResult,
    DimmingContext, RectRegistry, Popup, render_popup,
    TabBar, TabBarStyle, RectHandle,
    Toast, render_toasts,
    TabBarManager,
    get_box_by_name,
    TabBarItem, TabBarAlignment, TabBarPosition,
};
use std::sync::{Arc, Mutex};

/// Render the main UI
pub fn render_ui(
    f: &mut Frame,
    area: Rect,
    config: &BaseLayoutConfig,
    dimming: &DimmingContext,
    registry: &mut RectRegistry,
    main_content_box_handle_name: &str,
    original_anchor_metrics: &mut Option<Rect>,
    layout_manager: &mut LayoutManager,
    main_content_tab_bar: &TabBarManager,
    tab_style: TabBarStyle,
    settings_manager: &crate::settings_manager::SettingsManager,
    settings_fields: &SettingsFields,
    field_editor_state: &FieldEditorState,
    profile_state: &ProfileState,
    dashboard_arc: &Arc<Mutex<DashboardState>>,
    popup: &Option<Popup>,
    toasts: &Vec<Toast>,
    current_tab_bar: &mut Option<(TabBar, RectHandle)>,
    tab_content_configs: &Vec<crate::config::TabContentConfigYaml>,
) {
    // Find active tab ID to inject contextual bindings
    let active_tab_id = if let Some(active_tab_idx) = registry.get_active_tab(main_content_tab_bar.handle()) {
        if let Some(tab_bar_state) = registry.get_tab_bar_state(main_content_tab_bar.handle()) {
            tab_bar_state.tab_configs.get(active_tab_idx).map(|t| t.id.clone())
        } else {
            None
        }
    } else {
        None
    };
    
    // Create contextual config with tab-specific bindings
    let mut contextual_config = config.clone();
    if let Some(ref tab_id) = active_tab_id {
        if let Some(content_config) = tab_content_configs.iter().find(|c| &c.tab_id == tab_id) {
            for b in &content_config.bindings {
                contextual_config.global_bindings.push(tui_components::BindingConfig {
                    key: b.key.clone(),
                    description: b.description.clone(),
                });
            }
        }
    }
    
    // Render base layout with contextual config
    let base_layout = BaseLayout::new(
        &contextual_config,
        active_tab_id.as_deref(),
        dimming,
    );
    let result: BaseLayoutResult = base_layout.render(f, area, registry);
    let content_area = result.content_area;
    
    // Store original anchor metrics after first render
    if original_anchor_metrics.is_none() {
        if let Some(anchor_handle) = registry.get_handle(main_content_box_handle_name) {
            if let Some(metrics) = registry.get_metrics(anchor_handle) {
                *original_anchor_metrics = Some(metrics.into());
            }
        } else {
            *original_anchor_metrics = Some(content_area);
        }
    }
    
    // Restore anchor position
    if let (Some(anchor_handle), Some(original_metrics)) = (registry.get_handle(main_content_box_handle_name), original_anchor_metrics) {
        registry.update(anchor_handle, *original_metrics);
    }
    
    // Initialize or update main content bounding box
    if let Some(handle) = registry.get_handle(main_content_box_handle_name) {
        registry.update(handle, content_area);
    } else {
        let _handle = registry.register(Some(main_content_box_handle_name), content_area);
    }

    // Prepare tab bar
    let tab_bar_result = main_content_tab_bar.prepare(registry, Some(tab_style));
    
    // Render content block
    let render_area = if let Some(box_manager) = get_box_by_name(registry, main_content_box_handle_name) {
        box_manager.prepare(registry).unwrap_or(content_area)
    } else {
        content_area
    };
    
    // Render the content box border
    render_content(f, render_area, dimming);
    
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
                    let settings = settings_manager.get(); // Get current settings
                    render_settings(f, nested_area, &settings, settings_fields, field_editor_state, profile_state, registry, dimming);
                } else if tab_config.id == "dashboard" {
                    // Render dashboard directly from Arc to avoid cloning
                    if let Ok(mut state) = dashboard_arc.lock() {
                        render_dashboard(f, nested_area, &mut *state, profile_state, registry, dimming);
                    }
                }
            }
        }
    }

    // Render tab bar
    if let Some((tab_bar, anchor_handle, tab_bar_state)) = tab_bar_result {
        tab_bar.render_with_state(f, registry, &tab_bar_state, Some(dimming));
        
        // Store tab bar for click detection
        *current_tab_bar = Some((tab_bar, anchor_handle));

        // Render Profile Selector as a boxed static tab bar component aligned to the top right
        if let Some(metrics) = registry.get_metrics(anchor_handle) {
            let anchor_rect: Rect = metrics.into();
            let profile_name_opt = profile_state.active_profile_name.lock().unwrap().clone();
            let display_name = match profile_name_opt {
                Some(ref name) => name.clone(),
                None => "No Profile".to_string(),
            };
            
            let is_active = profile_name_opt.is_some();
            let color = if is_active { Color::Cyan } else { Color::White };
            
            let profile_item = TabBarItem {
                name: display_name,
                active: is_active,
                state: None,
            };
            
            let profile_tab_bar = TabBar::new(
                vec![profile_item],
                TabBarStyle::BoxStatic,
                TabBarAlignment::Right,
            )
            .with_position(TabBarPosition::TopOf(anchor_rect))
            .with_color(color);
            
            profile_tab_bar.render_with_registry_and_handle(f, Some(registry), Some(crate::constants::HWND_PROFILE_SELECTOR), Some(dimming));
        }
    }
    
    // Render dropdown overlay if selecting
    render_dropdown_overlay(
        f,
        area,
        field_editor_state,
        registry,
    );
    
    // Render popup
    if let Some(ref popup) = popup {
        render_popup(f, area, popup);
    }
    
    // Render toasts
    render_toasts(f, area, toasts);
}

/// Render dropdown overlay when selecting a field
fn render_dropdown_overlay(
    f: &mut Frame,
    area: Rect,
    field_editor_state: &FieldEditorState,
    registry: &RectRegistry,
) {
    // Render dropdown overlay if selecting
    match field_editor_state {
        FieldEditorState::Selecting { field_index, selected_index, options } => {
            // Use registry to get the field's registered rectangle
            let field_hwnds = [
                crate::constants::HWND_SETTINGS_FIELD_SKETCH_DIR,
                crate::constants::HWND_SETTINGS_FIELD_SKETCH_NAME,
                crate::constants::HWND_SETTINGS_FIELD_ENV,
                crate::constants::HWND_SETTINGS_FIELD_BOARD_MODEL,
                crate::constants::HWND_SETTINGS_FIELD_FQBN,
                crate::constants::HWND_SETTINGS_FIELD_PORT,
                crate::constants::HWND_SETTINGS_FIELD_BAUDRATE,
                crate::constants::HWND_SETTINGS_FIELD_MQTT_HOST,
                crate::constants::HWND_SETTINGS_FIELD_MQTT_PORT,
                crate::constants::HWND_SETTINGS_FIELD_MQTT_USERNAME,
                crate::constants::HWND_SETTINGS_FIELD_MQTT_PASSWORD,
                crate::constants::HWND_SETTINGS_FIELD_MQTT_TOPIC_COMMAND,
                crate::constants::HWND_SETTINGS_FIELD_MQTT_TOPIC_STATE,
                crate::constants::HWND_SETTINGS_FIELD_MQTT_TOPIC_STATUS,
            ];
            
            // Get field label
            let field_label = crate::field_editor::SettingsField::from_index(*field_index)
                .map(|f| f.label())
                .unwrap_or("");
            
            if let Some(hwnd) = field_hwnds.get(*field_index) {
                if let Some(field_box) = get_box_by_name(registry, hwnd) {
                    if let Some(field_rect) = field_box.metrics(registry) {
                        let field_area: Rect = field_rect.into();
                        render_dropdown(f, area, field_area, options, *selected_index, field_label);
                    }
                }
            }
        }
        FieldEditorState::ProfileSelecting { selected_index, options } => {
            if let Some(box_manager) = get_box_by_name(registry, crate::constants::HWND_PROFILE_SELECTOR) {
                if let Some(rect) = box_manager.metrics(registry) {
                    let field_area: Rect = rect.into();
                    render_dropdown(f, area, field_area, options, *selected_index, "Profiles");
                }
            }
        }
        _ => {}
    }
}

/// Helper to render a dropdown list
fn render_dropdown(
    f: &mut Frame,
    area: Rect,
    anchor_area: Rect,
    options: &[String],
    selected_index: usize,
    field_label: &str,
) {
    // Calculate dropdown position - 3 lines up from the bottom of the field
    let dropdown_height = (options.len() + 2).min(10) as u16; // +2 for top and bottom borders
    let dropdown_area = Rect {
        x: anchor_area.x,
        y: (anchor_area.y + anchor_area.height).saturating_sub(3),
        width: anchor_area.width,
        height: dropdown_height,
    };
    
    // Make sure dropdown fits in the frame
    let adjusted_dropdown_area = if dropdown_area.y + dropdown_area.height > area.height {
        // If doesn't fit below, adjust position
        Rect {
            x: dropdown_area.x,
            y: area.height.saturating_sub(dropdown_area.height),
            width: dropdown_area.width,
            height: dropdown_height,
        }
    } else {
        dropdown_area
    };
    
    // Render dropdown
    let mut items = Vec::new();
    for (i, option) in options.iter().enumerate() {
        let style = if i == selected_index {
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
    
    // Create list with all borders and field label as title
    let list = List::new(items)
        .block(Block::default()
            .title(Span::styled(format!(" {} ", field_label), Style::default().fg(Color::Rgb(255, 215, 0))))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 215, 0))));
    f.render_widget(Clear, adjusted_dropdown_area);
    f.render_widget(list, adjusted_dropdown_area);
}

/// Handle cursor positioning for editing fields
pub fn handle_cursor_positioning(
    f: &mut Frame,
    field_editor_state: &FieldEditorState,
    registry: &RectRegistry,
    main_content_tab_bar: &TabBarManager,
    main_content_box_handle_name: &str,
    layout_manager: &mut LayoutManager,
) {
    if let FieldEditorState::Editing { field_index, ref input } = field_editor_state {
        if let Some(active_tab_idx) = registry.get_active_tab(main_content_tab_bar.handle()) {
            if let Some(tab_bar_state) = registry.get_tab_bar_state(main_content_tab_bar.handle()) {
                if let Some(tab_config) = tab_bar_state.tab_configs.get(active_tab_idx) {
                    if tab_config.id == "settings" {
                        if let Some(box_manager) = get_box_by_name(registry, main_content_box_handle_name) {
                            if let Some(content_rect) = box_manager.metrics(registry) {
                                let content_rect: Rect = content_rect.into();
                                
                                // Check if terminal is large enough - don't position cursor if warning is shown
                                let min_width_pixels = crate::constants::MIN_WIDTH_PIXELS;
                                let min_height_pixels = crate::constants::MIN_HEIGHT_PIXELS;
                                
                                // Only position cursor if terminal is large enough
                                if content_rect.width >= min_width_pixels && content_rect.height >= min_height_pixels {
                                    // Get field position from HWND registry
                                    let field_hwnds = [
                                        crate::constants::HWND_SETTINGS_FIELD_SKETCH_DIR,
                                        crate::constants::HWND_SETTINGS_FIELD_SKETCH_NAME,
                                        crate::constants::HWND_SETTINGS_FIELD_ENV,
                                        crate::constants::HWND_SETTINGS_FIELD_BOARD_MODEL,
                                        crate::constants::HWND_SETTINGS_FIELD_FQBN,
                                        crate::constants::HWND_SETTINGS_FIELD_PORT,
                                        crate::constants::HWND_SETTINGS_FIELD_BAUDRATE,
                                        crate::constants::HWND_SETTINGS_FIELD_MQTT_HOST,
                                        
                                        
                                        
                                        crate::constants::HWND_SETTINGS_FIELD_MQTT_PORT,
                                        crate::constants::HWND_SETTINGS_FIELD_MQTT_USERNAME,
                                        crate::constants::HWND_SETTINGS_FIELD_MQTT_PASSWORD,
                                        crate::constants::HWND_SETTINGS_FIELD_MQTT_TOPIC_COMMAND,
                                        crate::constants::HWND_SETTINGS_FIELD_MQTT_TOPIC_STATE,
                                        crate::constants::HWND_SETTINGS_FIELD_MQTT_TOPIC_STATUS,
                                        
                                        
                                        
                                        
                                        
                                        
                                    ];
                                    
                                    if let Some(field_hwnd) = field_hwnds.get(*field_index) {
                                        if let Some(field_box) = get_box_by_name(registry, field_hwnd) {
                                            if let Some(field_rect) = field_box.metrics(registry) {
                                                let field_rect: Rect = field_rect.into();
                                                // Get inner area for text (accounting for borders and padding)
                                                let inner_area = Block::default()
                                                    .borders(Borders::ALL)
                                                    .padding(ratatui::widgets::Padding { left: 1, right: 1, top: 0, bottom: 0 })
                                                    .inner(field_rect);
                                                let text_width = inner_area.width as usize;
                                                
                                                // Calculate scroll offset and visual cursor position
                                                let scroll_offset = input.visual_scroll(text_width);
                                                let visual_cursor = input.visual_cursor();
                                                let cursor_pos_in_view = visual_cursor.saturating_sub(scroll_offset);
                                                
                                                // Calculate cursor position relative to field's inner area
                                                let cursor_x = inner_area.x + cursor_pos_in_view as u16;
                                                let cursor_y = inner_area.y;
                                                
                                                f.set_cursor_position((cursor_x, cursor_y));
                                                return; // Successfully positioned cursor using HWND
                                            }
                                        }
                                    }
                                    
                                    // Fallback to old method if HWND lookup fails
                                    // Use LayoutManager for cached content area calculation
                                    if let Some(content_area) = layout_manager.get_content_area(content_rect) {
                                        // Get inner area width for scroll calculation
                                        let inner_width = if *field_index < 2 {
                                            // Top fields use full width minus borders
                                            (content_area.width.saturating_sub(2)) as usize
                                        } else if *field_index < 5 {
                                            // Device section fields
                                            ((content_area.width / 2).saturating_sub(4)) as usize
                                        } else {
                                            // Connection section fields
                                            ((content_area.width / 2).saturating_sub(4)) as usize
                                        };
                                        
                                        // Calculate scroll offset and visual cursor position
                                        let scroll_offset = input.visual_scroll(inner_width);
                                        let visual_cursor = input.visual_cursor();
                                        
                                        // Use layout_utils to calculate cursor position
                                        if let Some((cursor_x, cursor_y)) = crate::layout_utils::calculate_cursor_position(
                                            content_area,
                                            *field_index,
                                            visual_cursor as usize,
                                            scroll_offset,
                                            inner_width,
                                        ) {
                                            f.set_cursor_position((cursor_x, cursor_y));
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
