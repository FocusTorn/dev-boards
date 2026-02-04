use std::collections::HashMap;
use crate::widgets::{ComponentRegistry, WidgetOutcome};
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use color_eyre::Result;

pub struct ComponentManager {
    components: HashMap<String, ComponentRegistry>,
    focus_stack: Vec<String>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            focus_stack: Vec::new(),
        }
    }

    pub fn register(&mut self, id: &str, component: ComponentRegistry) {
        self.components.insert(id.to_string(), component);
    }

    pub fn focus(&mut self, id: &str) {
        // Remove if already exists in stack to move to top
        if let Some(pos) = self.focus_stack.iter().position(|x| x == id) {
            self.focus_stack.remove(pos);
        }
        self.focus_stack.push(id.to_string());
    }

    pub fn blur(&mut self) {
        self.focus_stack.pop();
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Result<WidgetOutcome<String>> {
        if let Some(active_id) = self.focus_stack.last() {
            if let Some(component) = self.components.get_mut(active_id) {
                return component.handle_key(key);
            }
        }
        Ok(WidgetOutcome::None)
    }

    pub fn handle_mouse(&mut self, mouse: MouseEvent, area: Rect) -> Result<WidgetOutcome<String>> {
        if let Some(active_id) = self.focus_stack.last() {
            if let Some(component) = self.components.get_mut(active_id) {
                return component.handle_mouse(mouse, area);
            }
        }
        Ok(WidgetOutcome::None)
    }

    pub fn on_tick(&mut self) -> Result<()> {
        for component in self.components.values_mut() {
            component.on_tick()?;
        }
        Ok(())
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut ComponentRegistry> {
        self.components.get_mut(id)
    }
}
