use super::state::Dashboard;
use crate::widgets::traits::{Component, WidgetOutcome};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, KeyEventKind, KeyEventState};

#[test]
fn test_dashboard_opens_dropdown_on_enter() {
    let mut dashboard = Dashboard::new();
    assert!(!dashboard.profile_dropdown.is_open, "Dropdown should be closed initially");

    // Simulate Enter key press
    let event = KeyEvent {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };

    let result = dashboard.handle_key(event).unwrap();
    
    assert_eq!(result, WidgetOutcome::Consumed);
    assert!(dashboard.profile_dropdown.is_open, "Dropdown should be open after Enter");
}
