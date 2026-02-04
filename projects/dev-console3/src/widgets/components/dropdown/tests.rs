use super::state::OverlayDropdown;
use crate::widgets::traits::{Component, WidgetOutcome};
use ratatui::layout::Rect;
use crossterm::event::{MouseEvent, MouseEventKind, MouseButton, KeyModifiers};

#[test]
fn test_dropdown_opens_on_mouse_click() {
    let mut dropdown = OverlayDropdown::new("Test".into(), vec!["Item 1".into()], 5);
    let area = Rect::new(0, 0, 10, 3);
    
    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 5,
        row: 1,
        modifiers: KeyModifiers::empty(),
    };

    let result = dropdown.handle_mouse(event, area).unwrap();
    
    assert_eq!(result, WidgetOutcome::Consumed);
    assert!(dropdown.is_open, "Dropdown should open on mouse click");
}

#[test]
fn test_dropdown_direction_down() {
    let items = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let dropdown = OverlayDropdown::new("Test".into(), items, 5);
    let anchor = Rect::new(0, 0, 10, 3);
    let terminal_height = 20;
    
    let (_, is_down) = dropdown.calculate_layout(anchor, terminal_height);
    assert!(is_down);
}

#[test]
fn test_dropdown_direction_up() {
    let items = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let dropdown = OverlayDropdown::new("Test".into(), items, 5);
    let anchor = Rect::new(0, 15, 10, 3);
    let terminal_height = 20;
    
    let (_, is_down) = dropdown.calculate_layout(anchor, terminal_height);
    assert!(!is_down);
}
