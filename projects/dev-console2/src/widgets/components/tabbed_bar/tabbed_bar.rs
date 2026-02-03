use ratatui::{
    buffer::Buffer,
    layout::{Rect, Layout, Constraint, Spacing, Position},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Widget, Block},
};
use crossterm::event::{MouseEvent, MouseEventKind, MouseButton};
use serde::Deserialize;
use crate::widgets::{InteractiveWidget, WidgetOutcome};
use std::fs;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum TabStyle {
    #[strum(serialize = "tab")]
    Tab,
    #[strum(serialize = "text")]
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Display, Deserialize)]
pub enum TabAlignment {
    Left,
    Center,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct TabItem {
    pub id: String,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TabbedBarConfig {
    pub id: String,
    pub style: TabStyle,
    pub color: Option<String>,
    pub colors: Option<crate::config::TabBarColors>,
    pub alignment: crate::config::Alignment,
    pub tabs: Vec<crate::config::TabConfig>,
    pub min_tab_width: u16,
}

#[derive(Debug)]
pub struct TabbedBar {
    pub config: TabbedBarConfig,
    pub items: Vec<TabItem>,
}

impl TabbedBar {
    pub fn new(id: &str) -> color_eyre::Result<Self> {
        let curr = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        
        let config_path = if curr.ends_with("dev-console2") {
            curr.join("src/widgets/components/tabbed_bar/config.yaml")
        } else {
            let root = std::env::var("WORKSPACE_ROOT").map(std::path::PathBuf::from).unwrap_or_else(|_| curr);
            root.join("projects/dev-console2/src/widgets/components/tabbed_bar/config.yaml")
        };

        let config_content = fs::read_to_string(&config_path)
            .map_err(|e| color_eyre::eyre::eyre!("Failed to read {:?}: {}", config_path, e))?;
        let configs: Vec<TabbedBarConfig> = serde_saphyr::from_str(&config_content)?;
        
        let config = configs.into_iter().find(|c| c.id == id)
            .ok_or_else(|| color_eyre::eyre::eyre!("No config found with id '{}' in tabbed_bar/config.yaml", id))?;
        
        let items = config.tabs.iter().map(|t| TabItem {
            id: t.id.clone(),
            name: t.name.clone(),
            active: t.default.as_deref() == Some("active"),
        }).collect();

        Ok(Self {
            config,
            items,
        })
    }

    pub fn set_active(&mut self, id: &str) {
        for item in &mut self.items {
            item.active = item.id == id;
        }
    }

    pub fn get_active_id(&self) -> Option<String> {
        self.items.iter().find(|i| i.active).map(|i| i.id.clone())
    }

    pub fn next_tab(&mut self) {
        if self.items.is_empty() { return; }
        let current = self.items.iter().position(|t| t.active).unwrap_or(0);
        let next = (current + 1) % self.items.len();
        for (i, tab) in self.items.iter_mut().enumerate() {
            tab.active = i == next;
        }
    }

    pub fn prev_tab(&mut self) {
        if self.items.is_empty() { return; }
        let current = self.items.iter().position(|t| t.active).unwrap_or(0);
        let prev = if current > 0 { current - 1 } else { self.items.len() - 1 };
        for (i, tab) in self.items.iter_mut().enumerate() {
            tab.active = i == prev;
        }
    }

    fn get_item_width(&self, item: &TabItem) -> u16 {
        let base_width = match self.config.style {
            TabStyle::Tab => {
                if item.active { item.name.len() as u16 + 4 } else { item.name.len() as u16 + 2 }
            }
            _ => item.name.len() as u16 + 2,
        };
        base_width.max(self.config.min_tab_width)
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

    pub fn get_content_area(&self, area: Rect) -> Rect {
        let consumed = if self.config.style == TabStyle::Tab { 1 } else { 0 };
        let mut body_area = area;
        
        use crate::config::TabBarAlignment as ConfigAlign;
        
        let vertical = self.config.alignment.vertical.map(|a| match a {
            ConfigAlign::Top => TabAlignment::Top,
            ConfigAlign::Bottom => TabAlignment::Bottom,
            _ => TabAlignment::Top,
        }).unwrap_or(TabAlignment::Top);

        if vertical == TabAlignment::Top {
            body_area.y += consumed;
            body_area.height = body_area.height.saturating_sub(consumed);
        } else {
            body_area.height = body_area.height.saturating_sub(consumed);
        }
        Block::bordered().inner(body_area)
    }

    fn get_aligned_area(&self, area: Rect) -> Rect {
        let width = self.estimate_width();
        let height = if self.config.style == TabStyle::Tab { 2 } else { 1 };
        
        use crate::config::TabBarAlignment as ConfigAlign;
        
        let horizontal = self.config.alignment.horizontal.map(|a| match a {
            ConfigAlign::Left => TabAlignment::Left,
            ConfigAlign::Center => TabAlignment::Center,
            ConfigAlign::Right => TabAlignment::Right,
            _ => TabAlignment::Center,
        }).unwrap_or(TabAlignment::Center);

        let vertical = self.config.alignment.vertical.map(|a| match a {
            ConfigAlign::Top => TabAlignment::Top,
            ConfigAlign::Bottom => TabAlignment::Bottom,
            _ => TabAlignment::Top,
        }).unwrap_or(TabAlignment::Top);

        let x = match horizontal {
            TabAlignment::Left => area.x + 1,
            TabAlignment::Center => area.x + (area.width.saturating_sub(width)) / 2,
            TabAlignment::Right => area.x + area.width.saturating_sub(width).saturating_sub(1),
            _ => area.x + 1,
        };

        let y = match vertical {
            TabAlignment::Top => area.y,
            TabAlignment::Bottom => area.y + area.height.saturating_sub(height),
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

    pub fn render_integrated(&self, area: Rect, buf: &mut Buffer, block: Block) -> Rect {
        let consumed = if self.config.style == TabStyle::Tab { 1 } else { 0 };
        
        use crate::config::TabBarAlignment as ConfigAlign;
        
        let vertical = self.config.alignment.vertical.map(|a| match a {
            ConfigAlign::Top => TabAlignment::Top,
            ConfigAlign::Bottom => TabAlignment::Bottom,
            _ => TabAlignment::Top,
        }).unwrap_or(TabAlignment::Top);

        let (tab_target_area, body_area) = if vertical == TabAlignment::Top {
            let [header, body] = Layout::vertical([
                Constraint::Length(consumed + 1),
                Constraint::Min(0),
            ]).spacing(Spacing::Overlap(1)).areas(area);
            (header, body)
        } else {
            let [body, footer] = Layout::vertical([
                Constraint::Min(0),
                Constraint::Length(consumed + 1),
            ]).spacing(Spacing::Overlap(1)).areas(area);
            (footer, body)
        };

        block.render(body_area, buf);
        self.render_into(tab_target_area, buf);
        
        Block::bordered().inner(body_area)
    }

    fn render_into(&self, area: Rect, buf: &mut Buffer) {
        let aligned_area = self.get_aligned_area(area);
        if aligned_area.height == 0 || aligned_area.width == 0 { return; }

        let active_text_color = self.config.colors.as_ref()
            .and_then(|c| c.active.as_deref())
            .map(parse_color)
            .or_else(|| self.config.color.as_deref().map(parse_color))
            .unwrap_or(Color::Cyan);
        
        let active_style = Style::default().fg(active_text_color).add_modifier(Modifier::BOLD);
        let inactive_style = Style::default().fg(Color::White);

        if self.config.style == TabStyle::Tab && aligned_area.height >= 2 {
            // Build Top Line
            if let Some(active_idx) = self.items.iter().position(|i| i.active) {
                let mut pre_width = 0;
                for i in 0..active_idx {
                    pre_width += self.get_item_width(&self.items[i]) + 1;
                }
                let item_width = self.get_item_width(&self.items[active_idx]);
                let active_item = &self.items[active_idx];
                let mut spans = Vec::new();
                if pre_width > 0 { spans.push(Span::raw(" ".repeat(pre_width as usize))); }
                
                let content_len = active_item.name.len() as u16 + 4;
                let total_pad = item_width.saturating_sub(content_len);
                let left_pad = total_pad / 2;
                
                spans.push(Span::raw(" ".repeat(left_pad as usize)));
                spans.push(Span::styled("╭", inactive_style));
                spans.push(Span::styled("─".repeat(active_item.name.len() + 2), inactive_style));
                spans.push(Span::styled("╮", inactive_style));
                
                Line::from(spans).render(aligned_area, buf);
            }

            // Build Tab Line
            let mut spans = Vec::new();
            for (idx, item) in self.items.iter().enumerate() {
                if idx > 0 { spans.push(Span::styled("─", inactive_style)); }
                let item_width = self.get_item_width(item);
                if item.active {
                    let content = format!(" {} ", item.name);
                    let content_len = content.chars().count() as u16 + 2;
                    let total_pad = item_width.saturating_sub(content_len);
                    let left_pad = total_pad / 2;
                    let right_pad = total_pad - left_pad;
                    if left_pad > 0 { spans.push(Span::raw(" ".repeat(left_pad as usize))); }
                    spans.push(Span::styled("╯", inactive_style));
                    spans.push(Span::styled(content, active_style));
                    spans.push(Span::styled("╰", inactive_style));
                    if right_pad > 0 { spans.push(Span::raw(" ".repeat(right_pad as usize))); }
                } else {
                    let content = format!(" {} ", item.name);
                    let total_pad = item_width.saturating_sub(content.len() as u16);
                    let left_pad = total_pad / 2;
                    let right_pad = total_pad - left_pad;
                    if left_pad > 0 { spans.push(Span::raw(" ".repeat(left_pad as usize))); }
                    spans.push(Span::styled(content, inactive_style));
                    if right_pad > 0 { spans.push(Span::raw(" ".repeat(right_pad as usize))); }
                }
            }
            let tab_line_area = Rect { y: aligned_area.y + 1, height: 1, ..aligned_area };
            Line::from(spans).render(tab_line_area, buf);
        } else {
            // Fallback to single line
            let mut spans = Vec::new();
            for (idx, item) in self.items.iter().enumerate() {
                if idx > 0 { spans.push(Span::styled("─", inactive_style)); }
                let content = format!(" {} ", item.name);
                let style = if item.active { active_style } else { inactive_style };
                spans.push(Span::styled(content, style));
            }
            Line::from(spans).render(aligned_area, buf);
        }
    }
}

impl InteractiveWidget for TabbedBar {
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

        // For mouse hits, we only care about the line where labels are (y=0 or y=1 depending on style)
        let label_row = if self.config.style == TabStyle::Tab && aligned_area.height >= 2 {
            aligned_area.y + 1
        } else {
            aligned_area.y
        };

        if mouse.row != label_row { return WidgetOutcome::None; }

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
