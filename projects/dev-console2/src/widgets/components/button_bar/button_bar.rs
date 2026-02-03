use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};
use crossterm::event::{MouseEvent, MouseEventKind, MouseButton};
use serde::Deserialize;
use crate::widgets::{InteractiveWidget, WidgetOutcome};
use std::fs;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum ButtonBarStyle {
    #[strum(serialize = "boxed")]
    Boxed,
    #[strum(serialize = "text")]
    Text,
    #[strum(serialize = "box_static")]
    BoxStatic,
    #[strum(serialize = "text_static")]
    TextStatic,
}

#[derive(Debug, Clone, Copy, PartialEq, Display, Deserialize)]
pub enum ButtonAlignment {
    Left,
    Center,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct ButtonItem {
    pub id: String,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ButtonBarConfig {
    pub id: String,
    pub style: ButtonBarStyle,
    pub colors: Option<crate::config::TabBarColors>,
    pub alignment: crate::config::Alignment,
    pub tabs: Vec<crate::config::TabConfig>,
}

#[derive(Debug)]
pub struct ButtonBar {
    pub config: ButtonBarConfig,
    pub items: Vec<ButtonItem>,
}

impl ButtonBar {
    pub fn new(id: &str) -> color_eyre::Result<Self> {
        let curr = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        
        let config_path = if curr.ends_with("dev-console2") {
            curr.join("src/widgets/components/button_bar/config.yaml")
        } else {
            let root = std::env::var("WORKSPACE_ROOT").map(std::path::PathBuf::from).unwrap_or_else(|_| curr);
            root.join("projects/dev-console2/src/widgets/components/button_bar/config.yaml")
        };

        let config_content = fs::read_to_string(&config_path)
            .map_err(|e| color_eyre::eyre::eyre!("Failed to read {:?}: {}", config_path, e))?;
        let configs: Vec<ButtonBarConfig> = serde_saphyr::from_str(&config_content)?;
        
        let config = configs.into_iter().find(|c| c.id == id)
            .ok_or_else(|| color_eyre::eyre::eyre!("No config found with id '{}' in button_bar/config.yaml", id))?;
        
        let items = config.tabs.iter().map(|t| ButtonItem {
            id: t.id.clone(),
            name: t.name.clone(),
            active: t.default.as_deref() == Some("active"),
        }).collect();

        Ok(Self {
            config,
            items,
        })
    }

    pub fn set_active(&mut self, id: &str, active: bool) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.active = active;
        }
    }

    pub fn is_active(&self, id: &str) -> bool {
        self.items.iter().any(|i| i.id == id && i.active)
    }

    fn get_item_width(&self, item: &ButtonItem) -> u16 {
        match self.config.style {
            ButtonBarStyle::Boxed | ButtonBarStyle::BoxStatic => {
                if item.active || self.config.style == ButtonBarStyle::BoxStatic {
                    item.name.len() as u16 + 4
                } else {
                    item.name.len() as u16 + 2
                }
            }
            _ => item.name.len() as u16 + 2,
        }
    }

    fn estimate_width(&self) -> u16 {
        if self.items.is_empty() { return 0; }
        let mut width = 0;
        for (idx, item) in self.items.iter().enumerate() {
            if idx > 0 { width += 1; }
            width += self.get_item_width(item);
        }
        width
    }

    pub fn get_aligned_area(&self, area: Rect) -> Rect {
        let width = self.estimate_width();
        let height = 1;
        
        use crate::config::TabBarAlignment as ConfigAlign;
        
        let horizontal = self.config.alignment.horizontal.map(|a| match a {
            ConfigAlign::Left => ButtonAlignment::Left,
            ConfigAlign::Center => ButtonAlignment::Center,
            ConfigAlign::Right => ButtonAlignment::Right,
            _ => ButtonAlignment::Center,
        }).unwrap_or(ButtonAlignment::Center);

        let vertical = self.config.alignment.vertical.map(|a| match a {
            ConfigAlign::Top => ButtonAlignment::Top,
            ConfigAlign::Bottom => ButtonAlignment::Bottom,
            _ => ButtonAlignment::Top,
        }).unwrap_or(ButtonAlignment::Top);

        let x = match horizontal {
            ButtonAlignment::Left => area.x + 1,
            ButtonAlignment::Center => area.x + (area.width.saturating_sub(width)) / 2,
            ButtonAlignment::Right => area.x + area.width.saturating_sub(width).saturating_sub(1),
            _ => area.x + 1,
        };

        let y = match vertical {
            ButtonAlignment::Top => area.y,
            ButtonAlignment::Bottom => area.y + area.height.saturating_sub(height),
            _ => area.y,
        };

        let off_x = self.config.alignment.offset_x;
        let off_y = self.config.alignment.offset_y;

        let final_x = if off_x >= 0 { x.saturating_add(off_x as u16) } else { x.saturating_sub(off_x.abs() as u16) };
        let final_y = if off_y >= 0 { y.saturating_add(off_y as u16) } else { y.saturating_sub(off_y.abs() as u16) };

        Rect {
            x: final_x.max(area.x).min(area.right().saturating_sub(1)),
            y: final_y.max(area.y).min(area.bottom().saturating_sub(1)),
            width: width.min(area.right().saturating_sub(final_x)),
            height: height.min(area.bottom().saturating_sub(final_y)),
        }
    }
}

impl InteractiveWidget for ButtonBar {
    type Outcome = String;

    fn handle_key(&mut self, _key: crossterm::event::KeyEvent) -> WidgetOutcome<Self::Outcome> {
        WidgetOutcome::None
    }

    fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent, area: Rect) -> WidgetOutcome<Self::Outcome> {
        if !matches!(mouse.kind, MouseEventKind::Down(MouseButton::Left)) {
            return WidgetOutcome::None;
        }

        let aligned_area = self.get_aligned_area(area);
        if !aligned_area.contains(Position::new(mouse.column, mouse.row)) {
            return WidgetOutcome::None;
        }

        let rel_x = mouse.column.saturating_sub(aligned_area.x);
        let mut current_x = 0;
        for item in &self.items {
            let item_width = self.get_item_width(item);
            if rel_x >= current_x && rel_x < current_x + item_width {
                return WidgetOutcome::Confirmed(item.id.clone());
            }
            current_x += item_width + 1;
        }

        WidgetOutcome::None
    }
}

impl Widget for &ButtonBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let aligned_area = self.get_aligned_area(area);
        if aligned_area.height == 0 || aligned_area.width == 0 { return; }

        let mut spans = Vec::new();
        let active_color = self.config.colors.as_ref().and_then(|c| c.active.as_deref()).map(parse_color).unwrap_or(Color::Cyan);
        let negate_color = self.config.colors.as_ref().and_then(|c| c.negate.as_deref()).map(parse_color).unwrap_or(Color::White);
        let inactive_style = Style::default().fg(Color::White);
        let active_style = Style::default().fg(active_color).add_modifier(Modifier::BOLD);
        let negate_style = Style::default().fg(negate_color);

        for (idx, item) in self.items.iter().enumerate() {
            if idx > 0 {
                spans.push(Span::styled("─", inactive_style));
            }

            if item.active {
                match self.config.style {
                    ButtonBarStyle::Boxed | ButtonBarStyle::BoxStatic => {
                        spans.push(Span::styled("[", inactive_style));
                        spans.push(Span::styled(format!(" {} ", item.name), active_style));
                        spans.push(Span::styled("]", inactive_style));
                    }
                    _ => {
                        spans.push(Span::styled(format!(" {} ", item.name), active_style));
                    }
                }
            } else {
                let item_style = if self.config.style == ButtonBarStyle::BoxStatic || self.config.style == ButtonBarStyle::TextStatic {
                    negate_style
                } else {
                    inactive_style
                };
                let content = if self.config.style == ButtonBarStyle::BoxStatic {
                    format!("[ {} ]", item.name)
                } else {
                    format!(" {} ", item.name)
                };
                spans.push(Span::styled(content, item_style));
            }
        }

        Line::from(spans).render(aligned_area, buf);
    }
}

fn parse_color(c: &str) -> Color {
    match c.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" | "grey" => Color::Gray,
        "darkgray" | "darkgrey" => Color::DarkGray,
        "lightred" => Color::LightRed,
        "lightgreen" => Color::LightGreen,
        "lightyellow" => Color::LightYellow,
        "lightblue" => Color::LightBlue,
        "lightmagenta" => Color::LightMagenta,
        "lightcyan" => Color::LightCyan,
        "white" => Color::White,
        _ => Color::White,
    }
}