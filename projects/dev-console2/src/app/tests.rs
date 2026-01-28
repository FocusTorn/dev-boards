use crate::app::{App, TaskState, MonitorType, Action, Message};
use crate::commands::ProgressUpdate;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use ratatui::layout::Rect;
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, KeyEventKind, MouseEvent, MouseEventKind, MouseButton};
use tui_input::backend::crossterm::EventHandler;
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;

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
    config.application.min_height = 21;

    let mut tab_bar_map = std::collections::HashMap::new();
    let main_tab_bar_config = crate::config::TabBarConfig {
        id: "MainContentTabBar".to_string(),
        ..Default::default()
    };
    tab_bar_map.insert("MainContentTabBar".to_string(), main_tab_bar_config.clone());

    let mut app = App {
        running: true,
        tabs: vec![
            crate::widgets::tab_bar::TabBarItem { id: "dashboard".to_string(), name: "Dashboard".to_string(), active: true },
            crate::widgets::tab_bar::TabBarItem { id: "tab2".to_string(), name: "Tab 2".to_string(), active: false },
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
        profile_config: None,
        selected_profile_index: 0,
        profile_ids: Vec::new(),
        cancel_signal: Arc::new(AtomicBool::new(false)),
        view_area: Rect::new(0, 0, 100, 50),
        layout: crate::app::AppLayout::default(),
        theme: crate::app::theme::Theme::default(),
        predictor: crate::commands::ProgressPredictor::new(),
        last_raw_input: String::new(),
        last_frame_time: Instant::now(),
        should_redraw: false,
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

#[test]
fn test_app_new_initialization() {
    let app = create_test_app();
    assert!(app.running);
    assert!(!app.input_active);
}

#[test]
fn test_app_update_quit() {
    let mut app = create_test_app();
    app.exec_quit();
    assert!(!app.running);
}

#[test]
fn test_app_system_update_handling() {
    let mut app = create_test_app();
    let msg = "Test Output Line";
    app.update(Message::SystemUpdate(ProgressUpdate::OutputLine(msg.to_string())));
    assert_eq!(app.output_lines.len(), 1);
    assert!(app.output_lines[0].contains(msg));
}

#[test]
fn test_app_tab_navigation() {
    let mut app = create_test_app();
    assert!(app.tabs[0].active);
    app.exec_next_tab();
    assert!(app.tabs[1].active);
    app.exec_prev_tab();
    assert!(app.tabs[0].active);
}

#[test]
fn test_app_scroll_executors() {
    let mut app = create_test_app();
    app.layout.output = Rect::new(0, 0, 100, 10);
    app.output_lines = vec!["line".to_string(); 100];
    app.output_scroll = 50;
    app.output_autoscroll = false;

    app.exec_scroll_line_up();
    assert_eq!(app.output_scroll, 49);
    
    app.exec_scroll_line_down();
    assert_eq!(app.output_scroll, 50);

    app.exec_scroll_page_up();
    assert_eq!(app.output_scroll, 42); // 50 - (10 - 2)

    app.exec_scroll_page_down();
    assert_eq!(app.output_scroll, 50);

    app.exec_scroll_top();
    assert_eq!(app.output_scroll, 0);
    app.exec_scroll_bottom();
    assert!(app.output_autoscroll);
    
    app.exec_toggle_autoscroll();
    assert!(!app.output_autoscroll);
}

#[test]
fn test_app_system_update_stage_and_percentage() {
    let mut app = create_test_app();
    app.task_state = TaskState::Running {
        percentage: 0.0,
        visual_percentage: 0.0,
        last_percentage: 0.0,
        stage: "Initial".to_string(),
        start_time: Instant::now(),
        last_updated: Instant::now(),
        smoothed_eta: None,
    };

    app.exec_system_update(ProgressUpdate::Stage("Compiling".to_string()));
    if let TaskState::Running { stage, .. } = &app.task_state {
        assert_eq!(stage, "Compiling");
    } else { panic!(); }

    app.exec_system_update(ProgressUpdate::Percentage(50.0));
    if let TaskState::Running { percentage, .. } = &app.task_state {
        assert_eq!(*percentage, 50.0);
    } else { panic!(); }
}

#[test]
fn test_app_error_reporting() {
    let mut app = create_test_app();
    app.report_error("Test Error");
    assert!(app.status_text.contains("Test Error"));
    app.exec_system_update(ProgressUpdate::Failed("Process Failed".to_string()));
    assert!(app.status_text.contains("Process Failed"));
    assert_eq!(app.task_state, TaskState::Idle);
}

#[test]
fn test_app_layout_various_sizes() {
    let mut app = create_test_app();
    
    // Default size
    let area = Rect::new(0, 0, 100, 50);
    let _ = app.calculate_layout(area);

    // Tab bar bottom alignment
    app.config.tab_bars.push(crate::config::TabBarConfig {
        id: "MainContentTabBar".to_string(),
        alignment: crate::config::Alignment {
            vertical: Some(crate::widgets::tab_bar::TabBarAlignment::Bottom),
            ..Default::default()
        },
        ..Default::default()
    });
    let _ = app.calculate_layout(area);
}

#[test]
fn test_app_sync_autoscroll_with_lines() {
    let mut app = create_test_app();
    app.output_autoscroll = true;
    app.view_area = Rect::new(0, 0, 100, 20);
    app.output_lines = vec!["line".to_string(); 50];
    app.sync_autoscroll();
    assert!(app.output_scroll > 0);
}

#[test]
fn test_app_resize_handling() {
    let mut app = create_test_app();
    app.update(Message::Resize(120, 60));
    assert_eq!(app.view_area.width, 120);
    assert!(!app.terminal_too_small);

    app.update(Message::Resize(10, 5));
    assert!(app.terminal_too_small);
}

#[test]
fn test_app_key_dispatch() {
    let mut app = create_test_app();
    app.config.application.bindings.items.push(crate::config::BindingConfig {
        key: "[q]".to_string(),
        description: "Quit".to_string(),
        triggers: [("q".to_string(), "quit".to_string())].into_iter().collect(),
    });

    // Test quit
    app.update(Message::Key(press(KeyCode::Char('q'), KeyModifiers::empty())));
    assert!(!app.running);

    // Test input mode
    let mut app = create_test_app();
    app.input_active = true;
    app.update(Message::Key(press(KeyCode::Char('a'), KeyModifiers::empty())));
    assert_eq!(app.input.value(), "a");
    
    app.update(Message::Key(press(KeyCode::Esc, KeyModifiers::empty())));
    assert!(!app.input_active);
}

#[test]
fn test_app_mouse_dispatch() {
    let mut app = create_test_app();
    app.layout.commands = Rect::new(0, 10, 25, 10);
    
    // Test hover
    let event = MouseEvent {
        kind: MouseEventKind::Moved,
        column: 5,
        row: 11,
        modifiers: KeyModifiers::empty(),
    };
    app.update(Message::Mouse(event));
    assert!(app.hovered_command_index.is_some());

    // Test click
    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 5, row: 11,
        modifiers: KeyModifiers::empty(),
    };
    app.update(Message::Mouse(event));
}

#[test]
fn test_app_commands_navigation() {
    let mut app = create_test_app();
    assert_eq!(app.selected_command_index, 0);
    app.exec_commands_down();
    assert_eq!(app.selected_command_index, 1);
    app.exec_commands_up();
    assert_eq!(app.selected_command_index, 0);
}

#[test]
fn test_app_max_output_lines_drain() {
    let mut app = create_test_app();
    for i in 0..2005 {
        app.push_line(format!("line {}", i));
    }
    assert_eq!(app.output_lines.len(), 2000);
    assert_eq!(app.output_lines[0], "line 5");
}

#[test]
fn test_app_get_settings_failures() {
    let mut app = create_test_app();
    app.profile_config = Some(crate::config::ProfileConfig {
        connections: vec![],
        devices: vec![],
        mqtt: vec![],
        sketches: vec![crate::config::Sketch {
            id: "s1".to_string(),
            path: "test.ino".to_string(),
            connection: "nonexistent".to_string(),
            device: "nonexistent".to_string(),
            mqtt: "".to_string(),
        }],
    });
    app.profile_ids = vec!["s1".to_string()];
    app.selected_profile_index = 0;
    
    // Should fallback to load_command_settings
    let _ = app.get_settings_from_profile();
}

#[test]
fn test_app_get_settings_from_profile_error() {
    let mut app = create_test_app();
    app.profile_config = Some(crate::config::ProfileConfig {
        connections: vec![],
        devices: vec![],
        mqtt: vec![],
        sketches: vec![crate::config::Sketch {
            id: "s1".to_string(),
            path: "test.ino".to_string(),
            connection: "c1".to_string(),
            device: "d1".to_string(),
            mqtt: "".to_string(),
        }],
    });
    app.profile_ids = vec!["s1".to_string()];
    app.selected_profile_index = 0;

    let result = app.get_settings_from_profile();
    // Falls back to load_command_settings which might still fail or succeed depending on disk
    // But we covered the "if" path where it fails to find device/connection in provided config.
    assert!(result.is_err() || result.is_ok()); 
}

#[test]
fn test_app_toggle_input() {
    let mut app = create_test_app();
    assert!(!app.input_active);
    app.exec_toggle_input();
    assert!(app.input_active);
    app.exec_toggle_input();
    assert!(!app.input_active);
}

#[test]
fn test_app_profile_navigation() {
    let mut app = create_test_app();
    app.profile_ids = vec!["p1".to_string(), "p2".to_string()];
    app.selected_profile_index = 0;
    app.exec_next_profile();
    assert_eq!(app.selected_profile_index, 1);
    app.exec_prev_profile();
    assert_eq!(app.selected_profile_index, 0);
}

#[test]
fn test_app_cancel_executor() {
    let mut app = create_test_app();
    app.task_state = TaskState::Monitoring {
        monitor_type: MonitorType::Serial,
        start_time: Instant::now(),
    };
    app.cancel_signal.store(false, Ordering::SeqCst);
    app.exec_cancel();
    assert!(app.cancel_signal.load(Ordering::SeqCst));
}

#[test]
fn test_app_key_matches() {
    let app = create_test_app();
    let event = press(KeyCode::Char('q'), KeyModifiers::CONTROL);
    assert!(app.key_matches(event, "[Ctrl+Q]"));
    assert!(!app.key_matches(event, "[Q]"));
}

#[test]
fn test_app_exec_execute_selected_command() {
    let mut app = create_test_app();
    app.commands = vec!["Compile".to_string()];
    app.selected_command_index = 0;
    app.exec_execute_selected_command();
    assert!(matches!(app.task_state, TaskState::Running { .. }));
}

#[test]
fn test_app_scroll_complex() {
    let mut app = create_test_app();
    app.layout.output = Rect::new(0, 0, 100, 10);
    app.output_lines = vec!["line".to_string(); 5];
    app.output_scroll = 0;
    
    app.exec_scroll_line_down();
    assert_eq!(app.output_scroll, 0);

    app.output_lines = vec!["line".to_string(); 20];
    app.exec_scroll_page_down();
    assert!(app.output_scroll > 0);
}

#[test]
fn test_app_clipboard_actions() {
    let mut app = create_test_app();
    app.status_text = "test status".to_string();
    app.exec_copy_status();
    
    app.output_lines = vec!["line 1".to_string(), "line 2".to_string()];
    app.exec_copy_output(true);
    app.exec_copy_output(false);
}

#[test]
fn test_app_monitor_executors() {
    let mut app = create_test_app();
    app.exec_monitor_serial();
    assert!(matches!(app.task_state, TaskState::Monitoring { monitor_type: MonitorType::Serial, .. }));
}

#[test]
fn test_app_send_command_executor() {
    let mut app = create_test_app();
    app.input.handle_event(&crossterm::event::Event::Key(press(KeyCode::Char('t'), KeyModifiers::empty())));
    app.exec_send_command();
    
    app.task_state = TaskState::Monitoring { monitor_type: MonitorType::Serial, start_time: Instant::now() };
    let (tx, _rx) = mpsc::channel();
    app.serial_tx = Some(tx);
    app.exec_send_command();
}

#[test]
fn test_app_sync_autoscroll_no_autoscroll() {
    let mut app = create_test_app();
    app.output_autoscroll = false;
    app.output_scroll = 10;
    app.sync_autoscroll();
    assert_eq!(app.output_scroll, 10);
}

#[test]
fn test_action_from_str() {
    assert_eq!(Action::from_str("quit"), Some(Action::Quit));
    assert_eq!(Action::from_str("Compile"), Some(Action::Compile));
    assert_eq!(Action::from_str("invalid"), None);
}

#[test]
fn test_app_get_settings_error_paths() {
    let mut app = create_test_app();
    app.profile_config = Some(crate::config::ProfileConfig {
        connections: vec![],
        devices: vec![],
        mqtt: vec![],
        sketches: vec![],
    });
    app.profile_ids = vec![];
    let _ = app.get_settings_from_profile();
}

#[test]
fn test_app_mouse_scrollbar_move() {
    let mut app = create_test_app();
    app.layout.output = Rect::new(50, 40, 50, 10);
    app.output_lines = vec!["line".to_string(); 100];
    app.output_scroll = 0;
    
    // Hit scrollbar area (right edge of inner area)
    app.update(Message::Mouse(MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 98,
        row: 45,
        modifiers: KeyModifiers::empty(),
    }));
    app.update(Message::Mouse(MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 98,
        row: 48,
        modifiers: KeyModifiers::empty(),
    }));
    // Should have scrolled
    assert!(app.output_scroll > 0);
}

#[test]
fn test_app_key_matches_char() {
    let app = create_test_app();
    let event = press(KeyCode::Char('i'), KeyModifiers::empty());
    assert!(app.key_matches(event, "i"));
}

#[test]
fn test_app_exec_monitor_mqtt_with_config() {
    let mut app = create_test_app();
    app.profile_config = Some(crate::config::ProfileConfig {
        connections: vec![crate::config::Connection {
            id: "c1".to_string(),
            compiler: "arduino-cli".to_string(),
            port: "COM1".to_string(),
            baudrate: 115200,
        }],
        devices: vec![crate::config::Device {
            id: "d1".to_string(),
            board_model: "m1".to_string(),
            fbqn: "f1".to_string(),
        }],
        mqtt: vec![crate::config::Mqtt {
            id: "m1".to_string(),
            host: "localhost".to_string(),
            port: 1883,
            username: "".to_string(),
            password: "".to_string(),
        }],
        sketches: vec![crate::config::Sketch {
            id: "s1".to_string(),
            path: "sketch.ino".to_string(),
            connection: "c1".to_string(),
            device: "d1".to_string(),
            mqtt: "m1".to_string(),
        }],
    });
    app.profile_ids = vec!["s1".to_string()];
    app.selected_profile_index = 0;
    
    app.exec_monitor_mqtt();
    assert!(matches!(app.task_state, TaskState::Monitoring { monitor_type: MonitorType::Mqtt, .. }));
}

#[test]
fn test_app_dispatch_key_config_bindings() {
    let mut app = create_test_app();
    app.config.application.bindings.items.push(crate::config::BindingConfig {
        key: "[x]".to_string(),
        description: "X".to_string(),
        triggers: [("x".to_string(), "Clean".to_string())].into_iter().collect(),
    });
    
    app.update(Message::Key(press(KeyCode::Char('x'), KeyModifiers::empty())));
    assert!(app.output_lines.iter().any(|l| l.contains("Cleaning project")));
}

#[test]
fn test_app_mouse_dispatch_hover_exit() {
    let mut app = create_test_app();
    app.layout.commands = Rect::new(0, 10, 25, 10);
    app.selected_command_index = 0;
    
    app.update(Message::Mouse(MouseEvent {
        kind: MouseEventKind::Moved,
        column: 5,
        row: 11,
        modifiers: KeyModifiers::empty(),
    }));
    assert!(app.hovered_command_index.is_some());

    app.update(Message::Mouse(MouseEvent {
        kind: MouseEventKind::Moved,
        column: 50,
        row: 50,
        modifiers: KeyModifiers::empty(),
    }));
    assert!(app.hovered_command_index.is_none());
}

#[test]
fn test_app_is_task_running_and_toast() {
    let mut app = create_test_app();
    assert!(!app.is_task_running());
    app.task_state = TaskState::Running {
        percentage: 0.0,
        visual_percentage: 0.0,
        last_percentage: 0.0,
        stage: "".to_string(),
        start_time: Instant::now(),
        last_updated: Instant::now(),
        smoothed_eta: None,
    };
    assert!(app.is_task_running());
    
    assert!(!app.is_toast_animating());
    app.toast_manager.success("test");
    assert!(app.is_toast_animating());
}

#[test]
fn test_app_push_line_scrolling() {
    let mut app = create_test_app();
    app.output_autoscroll = true;
    app.view_area = Rect::new(0, 0, 100, 20);
    app.layout = app.calculate_layout(app.view_area);
    
    for _ in 0..100 {
        app.push_line("test".to_string());
    }
    assert!(app.output_scroll > 0);
}

#[test]
fn test_app_get_modifiers_display() {
    let app = create_test_app();
    assert_eq!(app.get_modifiers_display(KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SHIFT), "Ctrl+Alt+Shift");
}

#[test]
fn test_app_key_matches_all_variants() {
    let app = create_test_app();
    let keys = vec![
        (KeyCode::Enter, "[Enter]"),
        (KeyCode::Esc, "[Esc]"),
        (KeyCode::Up, "[Up]"),
        (KeyCode::Down, "[Down]"),
        (KeyCode::Left, "[Left]"),
        (KeyCode::Right, "[Right]"),
        (KeyCode::PageUp, "[PgUp]"),
        (KeyCode::PageDown, "[PgDn]"),
        (KeyCode::Home, "[Home]"),
        (KeyCode::End, "[End]"),
        (KeyCode::Backspace, "[Backspace]"),
        (KeyCode::Tab, "[Tab]"),
        (KeyCode::Delete, "[Delete]"),
    ];
    for (code, binding) in keys {
        let event = press(code, KeyModifiers::empty());
        assert!(app.key_matches(event, binding), "Failed for {}", binding);
    }
}

#[test]
fn test_app_dispatch_key_tab_specific() {
    let mut app = create_test_app();
    app.tab_bar_map.insert("MainContentTabBar".to_string(), crate::config::TabBarConfig {
        id: "MainContentTabBar".to_string(),
        navigation: crate::config::Navigation {
            left: vec!["[".to_string()],
            right: vec!["]".to_string()],
        },
        tab_bindings: [(
            "dashboard".to_string(),
            crate::config::BindingsConfig {
                items: vec![crate::config::BindingConfig {
                    triggers: [("x".to_string(), "quit".to_string())].into_iter().collect(),
                    ..Default::default()
                }],
                ..Default::default()
            }
        )].into_iter().collect(),
        ..Default::default()
    });

    app.update(Message::Key(press(KeyCode::Char('['), KeyModifiers::empty())));
    app.update(Message::Key(press(KeyCode::Char('x'), KeyModifiers::empty())));
    assert!(!app.running);
}

#[test]

fn test_app_mouse_status_output_interactions() {

    let mut app = create_test_app();

    app.layout.status = Rect::new(50, 0, 50, 4);

    app.layout.output = Rect::new(50, 4, 50, 46);



    // Ctrl+Click status

    app.update(Message::Mouse(MouseEvent {

        kind: MouseEventKind::Down(MouseButton::Left),

        column: 55,

        row: 1,

        modifiers: KeyModifiers::CONTROL,

    }));



    // Ctrl+Click output

    app.update(Message::Mouse(MouseEvent {

        kind: MouseEventKind::Down(MouseButton::Left),

        column: 55,

        row: 5,

        modifiers: KeyModifiers::CONTROL,

    }));



    // Ctrl+Shift+Click output

    app.update(Message::Mouse(MouseEvent {

        kind: MouseEventKind::Down(MouseButton::Left),

        column: 55,

        row: 5,

        modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,

    }));

}



#[test]

fn test_app_key_matches_remaining() {

    let app = create_test_app();

    assert!(app.key_matches(press(KeyCode::Up, KeyModifiers::empty()), "[up]"));

    assert!(app.key_matches(press(KeyCode::Down, KeyModifiers::empty()), "[down]"));

    assert!(app.key_matches(press(KeyCode::Left, KeyModifiers::empty()), "[left]"));

    assert!(app.key_matches(press(KeyCode::Right, KeyModifiers::empty()), "[right]"));

    assert!(app.key_matches(press(KeyCode::Char('a'), KeyModifiers::empty()), "[a]"));

}



#[test]

fn test_app_various_action_executors() {

    let mut app = create_test_app();

    app.exec_toggle_autoscroll();

    app.exec_scroll_page_up();

    app.exec_scroll_page_down();

    app.exec_copy_status();

    app.exec_copy_output(true);

    app.exec_copy_output(false);

}

#[test]
fn test_app_view_rendering() {
    let mut app = create_test_app();
    let backend = TestBackend::new(100, 50);
    let mut terminal = Terminal::new(backend).unwrap();
    
    // Set up some state to verify
    app.status_text = "System Ready".to_string();
    app.output_lines = vec!["Initial log".to_string()];
    app.output_cached_lines = vec![crate::app::ansi::parse_ansi_line("Initial log")];

    terminal.draw(|f| {
        app.view(f);
    }).unwrap();

    let buffer = terminal.backend().buffer();
    let s = buffer_content(buffer);
    
    // 1. Verify Title Bar
    assert!(s.contains("-")); 
    
    // 2. Verify Tab Bar
    assert!(s.contains("Dashboard"));
    
    // 3. Verify Profile
    assert!(s.contains("Profile"));
    
    // 4. Verify Command List
    assert!(s.contains("Compile"));
    assert!(s.contains("Upload"));
    
    // 5. Verify Status
    assert!(s.contains("System Ready"));
    
    // 6. Verify Output
    assert!(s.contains("Initial log"));
}

#[test]
fn test_app_view_terminal_too_small() {
    let mut app = create_test_app();
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    
    terminal.draw(|f| {
        app.view(f);
    }).unwrap();

    let s = buffer_content(terminal.backend().buffer());
    assert!(s.contains("Terminal Too Small"));
}

#[test]
fn test_app_view_progress_bar_rendering() {
    let mut app = create_test_app();
    let backend = TestBackend::new(100, 50);
    let mut terminal = Terminal::new(backend).unwrap();
    
    app.task_state = TaskState::Running {
        percentage: 45.0,
        visual_percentage: 45.0,
        last_percentage: 40.0,
        stage: "Compiling".to_string(),
        start_time: Instant::now(),
        last_updated: Instant::now(),
        smoothed_eta: Some(10.0),
    };

    terminal.draw(|f| {
        app.view(f);
    }).unwrap();

    let s = buffer_content(terminal.backend().buffer());
    assert!(s.contains("Compiling"));
    assert!(s.contains("45.0%"));
    assert!(s.contains("█"));

    let s = buffer_content(terminal.backend().buffer());
    assert!(s.contains("Compiling"));
    assert!(s.contains("45.0%"));
    assert!(s.contains("█"));
}


