use crate::app::{App, TaskState, MonitorType, Action, Message, Focus, DispatchMode};
use crate::commands::ProgressUpdate;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use ratatui::layout::Rect;
use ratatui::style::Color;
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, KeyEventKind, MouseEvent, MouseEventKind, MouseButton};
use tui_input::backend::crossterm::EventHandler;
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;

/// Helper to get text content from a buffer for assertion
fn buffer_content(buffer: &Buffer) -> String {
    let mut result = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            result.push_str(buffer[(x, y)].symbol());
        }
        result.push('\n');
    }
    result
}

/// Helper to create a minimal App state for unit testing
fn create_test_app() -> App {
    let (tx, rx) = mpsc::channel();
    let mut config = crate::config::Config::default();
    config.application.min_width = 80;
    config.application.min_height = 27;

    let mut tab_bar_map = std::collections::HashMap::new();
    let mut profiles_tab_bindings = crate::config::BindingsConfig::default();
    profiles_tab_bindings.items = vec![
        crate::config::BindingConfig {
            key: "[Ctrl+N]".to_string(),
            description: "New".to_string(),
            triggers: [("ctrl+n".to_string(), "profile_new".to_string())].into_iter().collect(),
        },
        crate::config::BindingConfig {
            key: "[Ctrl+C]".to_string(),
            description: "Clone".to_string(),
            triggers: [("ctrl+c".to_string(), "profile_clone".to_string())].into_iter().collect(),
        },
        crate::config::BindingConfig {
            key: "[Ctrl+D]".to_string(),
            description: "Delete".to_string(),
            triggers: [("ctrl+d".to_string(), "profile_delete".to_string())].into_iter().collect(),
        },
        crate::config::BindingConfig {
            key: "[Ctrl+S]".to_string(),
            description: "Save".to_string(),
            triggers: [("ctrl+s".to_string(), "profile_save".to_string())].into_iter().collect(),
        },
    ];

    let main_tab_bar_config = crate::config::TabBarConfig {
        id: "MainContentTabBar".to_string(),
        alignment: crate::config::Alignment {
            vertical: Some(crate::widgets::tab_bar::TabBarAlignment::Top),
            horizontal: Some(crate::widgets::tab_bar::TabBarAlignment::Left),
            ..Default::default()
        },
        tab_bindings: [("profiles".to_string(), profiles_tab_bindings)].into_iter().collect(),
        ..Default::default()
    };
    tab_bar_map.insert("MainContentTabBar".to_string(), main_tab_bar_config.clone());

    let mut app = App {
        running: true,
        tabs: vec![
            crate::widgets::tab_bar::TabBarItem { id: "dashboard".to_string(), name: "Dashboard".to_string(), active: true },
            crate::widgets::tab_bar::TabBarItem { id: "profiles".to_string(), name: "Profiles".to_string(), active: false },
        ],
        config: crate::config::Config {
            application: config.application,
            tab_bars: vec![main_tab_bar_config],
            ..Default::default()
        },
        tab_bar_map,
        terminal_too_small: false,
        commands: vec!["Compile".to_string(), "Upload".to_string(), "Monitor-Serial".to_string()],
        selected_command_index: 0,
        hovered_command_index: None,
        command_index_before_hover: None,
        settings_categories: vec!["Device".to_string(), "MQTT".to_string()],
        selected_settings_category_index: 0,
        selected_field_index: 0,
        hovered_field_index: None,
        field_index_before_hover: None,
        icon_focused: false,
        output_lines: Vec::new(),
        output_cached_lines: Vec::new(),
        output_scroll: 0,
        output_scroll_interaction: crate::widgets::smooth_scrollbar::ScrollBarInteraction::default(),
        output_autoscroll: true,
        task_state: TaskState::Idle,
        command_tx: tx,
        command_rx: rx,
        status_text: String::new(),
        toast_manager: crate::widgets::toast::ToastManager::new(crate::widgets::toast::ToastConfig::default()),
        profile_config: Some(crate::config::ProfileConfig {
            connections: vec![crate::config::Connection {
                id: "c1".to_string(),
                compiler: "arduino-cli".to_string(),
                port: "COM1".to_string(),
                baudrate: 115200,
            }],
            devices: vec![crate::config::Device {
                id: "d1".to_string(),
                board_model: "esp32".to_string(),
                fbqn: "esp32:esp32:esp32".to_string(),
            }],
            mqtt: vec![crate::config::Mqtt {
                id: "m1".to_string(),
                host: "localhost".to_string(),
                port: 1883,
                username: "".to_string(),
                password: "".to_string(),
            }],
            sketches: vec![crate::config::Sketch {
                id: "p1".to_string(),
                path: "test.ino".to_string(),
                connection: "c1".to_string(),
                device: "d1".to_string(),
                mqtt: "m1".to_string(),
            }],
        }),
        profile_config_path: "test_config.yaml".to_string(),
        selected_profile_index: 0,
        profile_ids: vec!["p1".to_string()],
        cancel_signal: Arc::new(AtomicBool::new(false)),
        view_area: Rect::new(0, 0, 100, 50),
        layout: crate::app::AppLayout {
            settings: None,
            ..Default::default()
        },
        theme: crate::app::theme::Theme::default(),
        predictor: crate::commands::ProgressPredictor::new(),
        last_raw_input: String::new(),
        last_frame_time: Instant::now(),
        should_redraw: false,
        dispatch_mode: DispatchMode::OnSelect,
        focus: Focus::Sidebar,
        modal: None,
        input: tui_input::Input::default(),
        input_active: false,
        serial_tx: None,
        mqtt_tx: None,
    };
    app.layout = app.calculate_layout(app.view_area);
    app
}

fn press(code: KeyCode, mods: KeyModifiers) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: mods,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::empty(),
    }
}

/// --------------------------------------------------------------------------- 
/// MODULE: Dashboard
/// Tests for the main monitoring and command execution dashboard.
/// --------------------------------------------------------------------------- 
mod dashboard {
    use super::*;

    #[test]
    fn test_initialization() {
        let app = create_test_app();
        assert!(app.running);
        assert!(app.tabs[0].active);
    }

    #[test]
    fn test_command_navigation() {
        let mut app = create_test_app();
        assert_eq!(app.selected_command_index, 0);
        app.exec_commands_down();
        assert_eq!(app.selected_command_index, 1);
        app.exec_commands_up();
        assert_eq!(app.selected_command_index, 0);
    }

    #[test]
    fn test_output_scrolling() {
        let mut app = create_test_app();
        app.layout.output = Rect::new(0, 0, 100, 10);
        app.output_lines = vec!["line".to_string(); 100];
        app.output_scroll = 50;
        app.output_autoscroll = false;

        app.exec_scroll_line_up();
        assert_eq!(app.output_scroll, 49);
        app.exec_scroll_line_down();
        assert_eq!(app.output_scroll, 50);
    }

    #[test]
    fn test_system_update_handling() {
        let mut app = create_test_app();
        let msg = "Test Output";
        app.update(Message::SystemUpdate(ProgressUpdate::OutputLine(msg.to_string())));
        assert!(app.output_lines.iter().any(|l| l.contains(msg)));
    }
}

/// --------------------------------------------------------------------------- 
/// MODULE: Profiles (Settings)
/// Comprehensive tests for profile management, field editing, and navigation.
/// --------------------------------------------------------------------------- 
mod profiles {
    use super::*;

    fn setup_profiles_tab() -> App {
        let mut app = create_test_app();
        for tab in &mut app.tabs {
            tab.active = tab.id == "profiles";
        }
        app.layout = app.calculate_layout(app.view_area);
        app
    }

    #[test]
    fn test_category_navigation() {
        let mut app = setup_profiles_tab();
        app.focus = Focus::Sidebar;
        
        let initial_cat = app.selected_settings_category_index;
        app.update(Message::Key(press(KeyCode::Down, KeyModifiers::empty())));
        assert_ne!(app.selected_settings_category_index, initial_cat);
        
        app.update(Message::Key(press(KeyCode::Up, KeyModifiers::empty())));
        assert_eq!(app.selected_settings_category_index, initial_cat);
    }

    #[test]
    fn test_field_navigation_and_looping() {
        let mut app = setup_profiles_tab();
        app.focus = Focus::Content;
        
        // Navigate through all 4 fields in 'Device' category
        app.selected_field_index = 0;
        for i in 1..4 {
            app.update(Message::Key(press(KeyCode::Down, KeyModifiers::empty())));
            assert_eq!(app.selected_field_index, i);
        }
        
        // Loop back to top
        app.update(Message::Key(press(KeyCode::Down, KeyModifiers::empty())));
        assert_eq!(app.selected_field_index, 0);
    }

    #[test]
    fn test_tab_cycling_with_icons() {
        let mut app = setup_profiles_tab();
        app.focus = Focus::Content;
        
        // Field 0: Input
        app.selected_field_index = 0;
        app.icon_focused = false;
        
        // Tab -> Field 1 Input
        app.update(Message::Key(press(KeyCode::Tab, KeyModifiers::empty())));
        assert_eq!(app.selected_field_index, 1);
        assert!(!app.icon_focused);
        
        // Tab -> Field 1 Icon (Since Sketch Path has a folder icon)
        app.update(Message::Key(press(KeyCode::Tab, KeyModifiers::empty())));
        assert_eq!(app.selected_field_index, 1);
        assert!(app.icon_focused);
        
        // Tab -> Field 2 Input
        app.update(Message::Key(press(KeyCode::Tab, KeyModifiers::empty())));
        assert_eq!(app.selected_field_index, 2);
        assert!(!app.icon_focused);
    }

    #[test]
    fn test_inline_editing_persistence() {
        let mut app = setup_profiles_tab();
        app.focus = Focus::Content;
        app.selected_field_index = 0; // Profile ID
        
        // 1. Enter edit mode
        app.update(Message::Key(press(KeyCode::Enter, KeyModifiers::empty())));
        assert!(app.input_active);
        
        // 2. Type new value
        app.input = tui_input::Input::new("modified_id".to_string());
        
        // 3. Confirm edit (Enter triggers finish_edit)
        app.update(Message::Key(press(KeyCode::Enter, KeyModifiers::empty())));
        assert!(!app.input_active);
        
        // 4. Verify persistence in profile config
        let profile_id = app.get_current_sketch_id().unwrap();
        assert_eq!(profile_id, "modified_id");
    }

    #[test]
    fn test_numeric_field_validation() {
        let mut app = setup_profiles_tab();
        app.focus = Focus::Content;
        app.selected_field_index = 3; // Baud Rate
        
        let initial_baud = app.profile_config.as_ref().unwrap().connections[0].baudrate;
        
        // 1. Enter edit mode
        app.update(Message::Key(press(KeyCode::Enter, KeyModifiers::empty())));
        
        // 2. Type INVALID numeric value
        app.input = tui_input::Input::new("not_a_number".to_string());
        
        // 3. Confirm edit
        app.update(Message::Key(press(KeyCode::Enter, KeyModifiers::empty())));
        
        // 4. Verify value was NOT changed (fallback or ignored)
        let current_baud = app.profile_config.as_ref().unwrap().connections[0].baudrate;
        assert_eq!(current_baud, initial_baud);
    }

    #[test]
    fn test_mouse_hit_detection_accuracy() {
        let mut app = setup_profiles_tab();
        let settings_layout = app.layout.settings.unwrap();
        
        // Click on Field 1 Input area
        let area = settings_layout.field_areas[1];
        let event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: area.x + 1,
            row: area.y + 1,
            modifiers: KeyModifiers::empty(),
        };
        app.dispatch_mouse(event);
        
        assert_eq!(app.selected_field_index, 1);
        assert!(!app.icon_focused);
        assert!(app.input_active); // Should trigger edit on click
    }
}

/// --------------------------------------------------------------------------- 
/// MODULE: Modals (Negative Testing)
/// Tests to ensure modals correctly isolate and block background input.
/// --------------------------------------------------------------------------- 
mod modals {
    use super::*;
    use crate::widgets::file_browser::FileBrowser;
    use crate::widgets::popup::Popup;
    use std::path::PathBuf;

    fn setup_with_modal(app: &mut App) {
        let browser = FileBrowser::new(PathBuf::from("."));
        app.modal = Some(Popup::new(browser, "MODAL".to_string()));
    }

    #[test]
    fn test_modal_blocks_tab_switching() {
        let mut app = create_test_app();
        setup_with_modal(&mut app);
        
        let initial_tab = app.tabs.iter().position(|t| t.active).unwrap();
        
        // Simulate ']' which normally triggers NextTab
        app.tab_bar_map.get_mut("MainContentTabBar").unwrap().navigation.right = vec![ "]".to_string()];
        app.update(Message::Key(press(KeyCode::Char(']'), KeyModifiers::empty())));
        
        // Tab should NOT have changed
        let current_tab = app.tabs.iter().position(|t| t.active).unwrap();
        assert_eq!(initial_tab, current_tab);
    }

    #[test]
    fn test_modal_blocks_mouse_leakage() {
        let mut app = create_test_app();
        setup_with_modal(&mut app);
        
        // Attempt to click on a tab (Row 1)
        let event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 15,
            row: 1,
            modifiers: KeyModifiers::empty(),
        };
        app.dispatch_mouse(event);
        
        // Tab should NOT have changed
        assert!(app.tabs[0].active);
    }

    #[test]
    fn test_outside_click_closes_modal() {
        let mut app = create_test_app();
        setup_with_modal(&mut app);
        
        // Click at coordinates far outside the modal area (which is centered 60x40)
        let event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 1,
            row: 1,
            modifiers: KeyModifiers::empty(),
        };
        app.dispatch_mouse(event);
        
        // Modal should be gone
        assert!(app.modal.is_none());
    }
}

/// --------------------------------------------------------------------------- 
/// MODULE: Navigation (Focus & Transitions)
/// Tests for moving focus between semantic regions (Sidebar <-> Content).
/// --------------------------------------------------------------------------- 
mod navigation {
    use super::*;

    #[test]
    fn test_sidebar_content_transition_via_arrows() {
        let mut app = create_test_app();
        for tab in &mut app.tabs { tab.active = tab.id == "profiles"; }
        app.focus = Focus::Sidebar;
        
        // Right arrow from Sidebar should move to Content
        app.update(Message::Key(press(KeyCode::Right, KeyModifiers::empty())));
        assert_eq!(app.focus, Focus::Content);
        
        // Left arrow from Content should move back to Sidebar
        app.update(Message::Key(press(KeyCode::Left, KeyModifiers::empty())));
        assert_eq!(app.focus, Focus::Sidebar);
    }

    #[test]
    fn test_sidebar_content_transition_via_tab() {
        let mut app = create_test_app();
        for tab in &mut app.tabs { tab.active = tab.id == "profiles"; }
        app.focus = Focus::Sidebar;
        
        // Tab should switch focus
        app.update(Message::Key(press(KeyCode::Tab, KeyModifiers::empty())));
        assert_eq!(app.focus, Focus::Content);
    }
}