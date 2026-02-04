use crate::config::{Alignment, TabConfig, TabBarColors, TabBarAlignment, BindingsConfig};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
pub struct TabItem {
    pub id: String,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TabbedBarConfig {
    pub id: String,
    pub color: Option<String>,
    pub colors: Option<TabBarColors>,
    pub alignment: Alignment,
    pub tabs: Vec<TabConfig>,
    pub min_tab_width: u16,
    #[serde(default)]
    pub tab_bindings: HashMap<String, BindingsConfig>,
}

#[derive(Debug)]
pub struct TabbedBar {
    pub config: TabbedBarConfig,
    pub items: Vec<TabItem>,
}

impl TabbedBar {
    pub fn from_config(id: &str) -> color_eyre::Result<Self> {
        let config_path = std::path::PathBuf::from("src/widgets/components/tabbed_bar/config.yaml");
        let content = fs::read_to_string(&config_path)?;
        let configs: Vec<TabbedBarConfig> = serde_saphyr::from_str(&content)?;
        
        let config = configs.into_iter().find(|c| c.id == id)
            .ok_or_else(|| color_eyre::eyre::eyre!("Config not found"))?;

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

    pub fn get_item_width(&self, item: &TabItem) -> u16 {
        let base_width = if item.active { item.name.len() as u16 + 4 } else { item.name.len() as u16 + 2 };
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

    pub fn get_aligned_area(&self, area: ratatui::layout::Rect) -> ratatui::layout::Rect {
        use ratatui::layout::Rect;
        let width = self.estimate_width();
        let height = 2; // Always 2 for Tab style
        
        let horizontal = self.config.alignment.horizontal.unwrap_or(TabBarAlignment::Center);
        let vertical = self.config.alignment.vertical.unwrap_or(TabBarAlignment::Top);

        let x = match horizontal {
            TabBarAlignment::Left => area.x + 1,
            TabBarAlignment::Center => area.x + (area.width.saturating_sub(width)) / 2,
            TabBarAlignment::Right => area.x + area.width.saturating_sub(width).saturating_sub(1),
            _ => area.x + 1,
        };

        let y = match vertical {
            TabBarAlignment::Top => area.y,
            TabBarAlignment::Bottom => area.y + area.height.saturating_sub(height),
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
