use super::button_bar::*;
use ratatui::{layout::Rect, buffer::Buffer};
use crate::widgets::{InteractiveWidget, WidgetOutcome};
use crossterm::event::{MouseEvent, MouseEventKind, MouseButton, KeyModifiers};
use crate::config::TabBarAlignment;

#[test]
fn test_button_bar_initialization() {
    let items = vec![
        ButtonItem { id: "t1".to_string(), name: "T1".to_string(), active: true },
    ];
    let config = ButtonBarConfig {
        id: "test".to_string(),
        style: ButtonBarStyle::BoxStatic,
        colors: None,
        alignment: crate::config::Alignment::default(),
        tabs: vec![],
    };
    
    let bar = ButtonBar { config, items };
    assert!(bar.is_active("t1"));
    assert!(!bar.is_active("nonexistent"));
}

#[test]
fn test_button_bar_set_active() {
    let items = vec![
        ButtonItem { id: "t1".to_string(), name: "T1".to_string(), active: true },
        ButtonItem { id: "t2".to_string(), name: "T2".to_string(), active: false },
    ];
    let config = ButtonBarConfig {
        id: "test".to_string(),
        style: ButtonBarStyle::BoxStatic,
        colors: None,
        alignment: crate::config::Alignment::default(),
        tabs: vec![],
    };
    
    let mut bar = ButtonBar { config, items };
    
    // Positive
    bar.set_active("t2", true);
    assert!(bar.is_active("t2"));
    
    // Negative - setting active to false
    bar.set_active("t1", false);
    assert!(!bar.is_active("t1"));
}

#[test]
fn test_button_bar_mouse_hit() {
    let items = vec![
        ButtonItem { id: "t1".to_string(), name: "T1".to_string(), active: true },
    ];
    let config = ButtonBarConfig {
        id: "test".to_string(),
        style: ButtonBarStyle::Text,
        colors: None,
        alignment: crate::config::Alignment {
            horizontal: Some(TabBarAlignment::Left),
            vertical: Some(TabBarAlignment::Top),
            ..Default::default()
        },
        tabs: vec![],
    };
    
    let mut bar = ButtonBar { config, items };
    let area = Rect::new(0, 0, 20, 1);
    
    // Positive hit
    let hit_event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 1, // " T1 " usually starts at 1 if left aligned
        row: 0,
        modifiers: KeyModifiers::empty(),
    };
    let outcome = bar.handle_mouse(hit_event, area);
    assert_eq!(outcome, WidgetOutcome::Confirmed("t1".to_string()));
}

#[test]
fn test_button_bar_load_real_config() {
    let bar = ButtonBar::new("OutputPanelStaticOptions");
    assert!(bar.is_ok(), "Failed to load ButtonBar config: {:?}", bar.err());
    let bar = bar.unwrap();
    assert_eq!(bar.config.id, "OutputPanelStaticOptions");
    assert!(!bar.items.is_empty());
    assert_eq!(bar.items[0].id, "autoscroll");
}

#[test]
fn test_button_bar_alignment_and_offsets() {
    let items = vec![
        ButtonItem { id: "t1".to_string(), name: "TEST".to_string(), active: true },
    ];
    // " TEST " is 6 chars
    
    let mut config = ButtonBarConfig {
        id: "test".to_string(),
        style: ButtonBarStyle::Text,
        colors: None,
        alignment: crate::config::Alignment {
            horizontal: Some(TabBarAlignment::Center),
            vertical: Some(TabBarAlignment::Bottom),
            offset_x: 2,
            offset_y: -1,
        },
        tabs: vec![],
    };
    
    let bar = ButtonBar { config: config.clone(), items: items.clone() };
    let area = Rect::new(0, 0, 100, 10);
    let aligned = bar.get_aligned_area(area);
    
    // Width 6. 
    // Center of 100 is x=47 ( (100-6)/2 ).
    // Offset +2 makes it x=49.
    assert_eq!(aligned.x, 49);
    
    // Bottom of 10 is y=9 ( 10-1 ).
    // Offset -1 makes it y=8.
    assert_eq!(aligned.y, 8);
    
    // Test clipping
    config.alignment.offset_x = 200; // Way outside
    let bar_clipped = ButtonBar { config, items };
    let aligned_clipped = bar_clipped.get_aligned_area(area);
    assert!(aligned_clipped.right() <= area.right());
}
