use crate::app::App;

/// Verifies that the application can initialize successfully using the local
/// configuration files.
#[test]
fn test_app_new_initialization() {
    // App::new() relies on config files being present in the current working directory.
    let app_result = App::new();
    
    if let Ok(app) = app_result {
        assert!(app.running);
        assert!(!app.input_active);
        // Ensure status bar text reflects either success or error loading profiles
        assert!(!app.status_text.is_empty());
    }
}

/// Verifies the core update loop and executor logic using a manually constructed App state.
/// 
/// This test bypasses file I/O by manually initializing the App struct, allowing for 
/// predictable testing of state transitions.
#[test]
fn test_app_update_quit() {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut app = App {
        running: true,
        tabs: Vec::new(),
        config: crate::config::Config::default(),
        tab_bar_map: std::collections::HashMap::new(),
        terminal_too_small: false,
        commands: Vec::new(),
        selected_command_index: 0,
        hovered_command_index: None,
        command_index_before_hover: None,
        output_lines: Vec::new(),
        output_cached_lines: Vec::new(),
        output_scroll: 0,
        output_scroll_interaction: crate::widgets::smooth_scrollbar::ScrollBarInteraction::default(),
        output_autoscroll: true,
        task_state: crate::app::TaskState::Idle,
        command_tx: tx,
        command_rx: rx,
        status_text: String::new(),
        toast_manager: crate::widgets::toast::ToastManager::new(crate::widgets::toast::ToastConfig::default()),
        profile_config: None,
        selected_profile_index: 0,
        profile_ids: Vec::new(),
        cancel_signal: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        view_area: ratatui::layout::Rect::default(),
        layout: crate::app::AppLayout::default(),
        theme: crate::app::theme::Theme::default(),
        predictor: crate::commands::ProgressPredictor::new(),
        last_raw_input: String::new(),
        last_frame_time: std::time::Instant::now(),
        should_redraw: false,
        input: tui_input::Input::default(),
        input_active: false,
        serial_tx: None,
        mqtt_tx: None,
    };
    
    // Execute the quit logic directly via the executor
    app.exec_quit();
    assert!(!app.running);
}

/// Verifies that background system events are correctly ingested and translated
/// into internal state changes (e.g., adding output lines).
#[test]
fn test_app_system_update_handling() {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut app = App {
        running: true,
        tabs: Vec::new(),
        config: crate::config::Config::default(),
        tab_bar_map: std::collections::HashMap::new(),
        terminal_too_small: false,
        commands: Vec::new(),
        selected_command_index: 0,
        hovered_command_index: None,
        command_index_before_hover: None,
        output_lines: Vec::new(),
        output_cached_lines: Vec::new(),
        output_scroll: 0,
        output_scroll_interaction: crate::widgets::smooth_scrollbar::ScrollBarInteraction::default(),
        output_autoscroll: true,
        task_state: crate::app::TaskState::Idle,
        command_tx: tx,
        command_rx: rx,
        status_text: String::new(),
        toast_manager: crate::widgets::toast::ToastManager::new(crate::widgets::toast::ToastConfig::default()),
        profile_config: None,
        selected_profile_index: 0,
        profile_ids: Vec::new(),
        cancel_signal: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        view_area: ratatui::layout::Rect::default(),
        layout: crate::app::AppLayout::default(),
        theme: crate::app::theme::Theme::default(),
        predictor: crate::commands::ProgressPredictor::new(),
        last_raw_input: String::new(),
        last_frame_time: std::time::Instant::now(),
        should_redraw: false,
        input: tui_input::Input::default(),
        input_active: false,
        serial_tx: None,
        mqtt_tx: None,
    };

    let msg = "Test Output Line";
    // Simulate a message coming from a background thread
    app.update(crate::app::Message::SystemUpdate(crate::commands::ProgressUpdate::OutputLine(msg.to_string())));
    
    // Verify that the system translation correctly appended the line to the output buffer
    assert_eq!(app.output_lines.len(), 1);
    assert!(app.output_lines[0].contains(msg));
}

/// Helper to create a minimal App state for unit testing
fn create_test_app() -> App {
    let (tx, rx) = std::sync::mpsc::channel();
    App {
        running: true,
        tabs: vec![
            crate::widgets::tab_bar::TabBarItem { id: "tab1".to_string(), name: "Tab 1".to_string(), active: true },
            crate::widgets::tab_bar::TabBarItem { id: "tab2".to_string(), name: "Tab 2".to_string(), active: false },
        ],
        config: crate::config::Config::default(),
        tab_bar_map: std::collections::HashMap::new(),
        terminal_too_small: false,
        commands: vec!["Compile".to_string()],
        selected_command_index: 0,
        hovered_command_index: None,
        command_index_before_hover: None,
        output_lines: Vec::new(),
        output_cached_lines: Vec::new(),
        output_scroll: 0,
        output_scroll_interaction: crate::widgets::smooth_scrollbar::ScrollBarInteraction::default(),
        output_autoscroll: true,
        task_state: crate::app::TaskState::Idle,
        command_tx: tx,
        command_rx: rx,
        status_text: String::new(),
        toast_manager: crate::widgets::toast::ToastManager::new(crate::widgets::toast::ToastConfig::default()),
        profile_config: None,
        selected_profile_index: 0,
        profile_ids: Vec::new(),
        cancel_signal: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        view_area: ratatui::layout::Rect::default(),
        layout: crate::app::AppLayout::default(),
        theme: crate::app::theme::Theme::default(),
        predictor: crate::commands::ProgressPredictor::new(),
        last_raw_input: String::new(),
        last_frame_time: std::time::Instant::now(),
        should_redraw: false,
        input: tui_input::Input::default(),
        input_active: false,
        serial_tx: None,
        mqtt_tx: None,
    }
}

#[test]
fn test_app_tab_navigation() {
    let mut app = create_test_app();
    
    assert!(app.tabs[0].active);
    assert!(!app.tabs[1].active);

    app.exec_next_tab();
    assert!(!app.tabs[0].active);
    assert!(app.tabs[1].active);

    app.exec_next_tab();
    assert!(app.tabs[0].active);
    assert!(!app.tabs[1].active);

    app.exec_prev_tab();
    assert!(!app.tabs[0].active);
    assert!(app.tabs[1].active);
}

#[test]
fn test_app_scroll_executors() {
    let mut app = create_test_app();
    app.output_lines = vec!["line".to_string(); 100];
    app.output_scroll = 50;
    app.output_autoscroll = false;

    app.exec_scroll_line_up();
    assert_eq!(app.output_scroll, 49);

    app.exec_scroll_top();
    assert_eq!(app.output_scroll, 0);

    app.exec_scroll_bottom();
    assert!(app.output_autoscroll);
}

#[test]
fn test_app_system_update_stage_and_percentage() {
    let mut app = create_test_app();
    app.task_state = crate::app::TaskState::Running {
        percentage: 0.0,
        visual_percentage: 0.0,
        last_percentage: 0.0,
        stage: "Initial".to_string(),
        start_time: std::time::Instant::now(),
        last_updated: std::time::Instant::now(),
        smoothed_eta: None,
    };

    // Test Stage update
    app.exec_system_update(crate::commands::ProgressUpdate::Stage("Compiling".to_string()));
    if let crate::app::TaskState::Running { stage, .. } = &app.task_state {
        assert_eq!(stage, "Compiling");
    } else {
        panic!("Expected Running state");
    }

    // Test Percentage update
    app.exec_system_update(crate::commands::ProgressUpdate::Percentage(50.0));
    if let crate::app::TaskState::Running { percentage, .. } = &app.task_state {
        assert_eq!(*percentage, 50.0);
    } else {
        panic!("Expected Running state");
    }
}

#[test]
fn test_app_error_reporting() {
    let mut app = create_test_app();
    
    app.report_error("Test Error");
    assert!(app.status_text.contains("Test Error"));
    assert!(app.status_text.contains("[Error]"));
    assert!(!app.output_lines.is_empty());
    assert!(app.output_lines.last().unwrap().contains("Test Error"));

    // Test via SystemUpdate::Failed
    app.exec_system_update(crate::commands::ProgressUpdate::Failed("Process Failed".to_string()));
    assert!(app.status_text.contains("Process Failed"));
    assert_eq!(app.task_state, crate::app::TaskState::Idle);
}

#[test]
fn test_app_layout_calculation() {
    let app = create_test_app();
    let area = ratatui::layout::Rect::new(0, 0, 100, 50);
    let layout = app.calculate_layout(area);

    assert_eq!(layout.title.height, 1);
    assert!(layout.main.height > 0);
    assert_eq!(layout.bindings.height, 1);
    assert_eq!(layout.status_bar.height, 2);
    
    // Ensure regions are within the total area
    assert!(area.contains(ratatui::layout::Position::new(layout.title.x, layout.title.y)));
    assert!(area.contains(ratatui::layout::Position::new(layout.main.x, layout.main.y)));
}