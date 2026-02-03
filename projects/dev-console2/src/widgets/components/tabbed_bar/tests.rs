use super::tabbed_bar::*;
use ratatui::{layout::Rect, widgets::{Block, Borders}, buffer::Buffer};
use crate::widgets::{InteractiveWidget, WidgetOutcome};
use crossterm::event::{MouseEvent, MouseEventKind, MouseButton, KeyModifiers};
use crate::config::TabBarAlignment;

#[test]
fn test_tabbed_bar_inner_area_calculation() {
    let items = vec![];
    let config = TabbedBarConfig {
        id: "test".to_string(),
        style: TabStyle::Tab,
        color: None,
        colors: None,
        alignment: crate::config::Alignment::default(),
        tabs: vec![],
        min_tab_width: 0,
    };
    
    let bar = TabbedBar {
        config,
        items,
    };
    
    let area = Rect::new(0, 0, 50, 20);
    let inner = bar.get_content_area(area);
    
    assert_eq!(inner.height, 17);
}

#[test]
fn test_tabbed_bar_text_style_inner_area() {
    let items = vec![];
    let config = TabbedBarConfig {
        id: "test".to_string(),
        style: TabStyle::Text,
        color: None,
        colors: None,
        alignment: crate::config::Alignment::default(),
        tabs: vec![],
        min_tab_width: 0,
    };
    
    let bar = TabbedBar {
        config,
        items,
    };
    
    let area = Rect::new(0, 0, 50, 20);
    let inner = bar.get_content_area(area);
    
    assert_eq!(inner.height, 18);
}

#[test]
fn test_tabbed_bar_mouse_hit() {
    let items = vec![
        TabItem { id: "dash".to_string(), name: "Dashboard".to_string(), active: true },
        TabItem { id: "prof".to_string(), name: "Profiles".to_string(), active: false },
    ];
    let config = TabbedBarConfig {
        id: "test".to_string(),
        style: TabStyle::Text,
        color: None,
        colors: None,
        alignment: crate::config::Alignment {
            horizontal: Some(TabBarAlignment::Left),
            vertical: Some(TabBarAlignment::Top),
            ..Default::default()
        },
        tabs: vec![],
        min_tab_width: 0,
    };
    
    let mut bar = TabbedBar { config, items };
    let area = Rect::new(0, 0, 50, 20);
    
    let hit_event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 2,
        row: 0,
        modifiers: KeyModifiers::empty(),
    };
    assert_eq!(bar.handle_mouse(hit_event, area), WidgetOutcome::Confirmed("dash".to_string()));
}

#[test]
fn test_tabbed_bar_navigation() {
    let items = vec![
        TabItem { id: "t1".to_string(), name: "T1".to_string(), active: true },
        TabItem { id: "t2".to_string(), name: "T2".to_string(), active: false },
    ];
    let config = TabbedBarConfig {
        id: "test".to_string(),
        style: TabStyle::Text,
        color: None,
        colors: None,
        alignment: crate::config::Alignment::default(),
        tabs: vec![],
        min_tab_width: 0,
    };
    
    let mut bar = TabbedBar { config, items };
    
    bar.next_tab();
    assert_eq!(bar.get_active_id().unwrap(), "t2");
    
    bar.next_tab();
    assert_eq!(bar.get_active_id().unwrap(), "t1");
}

#[test]
fn test_tabbed_bar_load_real_config() {
    let bar = TabbedBar::new("MainContentTabBar");
    assert!(bar.is_ok());
    let bar = bar.unwrap();
    assert_eq!(bar.config.id, "MainContentTabBar");
}
