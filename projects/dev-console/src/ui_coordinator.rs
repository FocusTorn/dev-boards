// UI rendering coordination module
// Handles UI rendering logic and layout management

use crate::render::{render_content, render_settings, render_dashboard};
use crate::field_editor::{FieldEditorState, SettingsFields};
use crate::dashboard::DashboardState;
use crate::layout_manager::LayoutManager;

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
    dashboard_arc: &Arc<Mutex<DashboardState>>,
    popup: &Option<Popup>,
    toasts: &Vec<Toast>,
    current_tab_bar: &mut Option<(TabBar, RectHandle)>,
) {
    // Render base layout
    let base_layout = BaseLayout::new(
        config,
        None,
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
                    render_settings(f, nested_area, &settings, settings_fields, field_editor_state);
                } else if tab_config.id == "dashboard" {
                    // Render dashboard directly from Arc to avoid cloning
                    if let Ok(mut state) = dashboard_arc.lock() {
                        render_dashboard(f, nested_area, &mut *state);
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
    }
    
    // Render dropdown overlay if selecting
    render_dropdown_overlay(
        f,
        area,
        field_editor_state,
        registry,
        main_content_box_handle_name,
        layout_manager,
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
    main_content_box_handle_name: &str,
    layout_manager: &mut LayoutManager,
) {
    if let FieldEditorState::Selecting { field_index, selected_index, options } = field_editor_state {
        if let Some(box_manager) = get_box_by_name(registry, main_content_box_handle_name) {
            if let Some(content_rect) = box_manager.metrics(registry) {
                let content_rect: Rect = content_rect.into();
                
                // Use LayoutManager for cached content area calculation
                if let Some(content_area) = layout_manager.get_content_area(content_rect) {
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
