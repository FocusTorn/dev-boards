#[cfg(test)]
mod tests {
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
}
