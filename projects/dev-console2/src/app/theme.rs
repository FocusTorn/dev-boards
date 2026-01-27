use ratatui::style::{Color, Modifier, Style};
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ThemeConfig {
    #[serde(default)]
    pub styles: HashMap<String, String>,
    #[serde(default)]
    pub message_types: HashMap<String, MessageTypeConfig>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MessageTypeConfig {
    pub icon: String,
    pub icon_style: String,
    pub text_style: String,
}

#[derive(Debug)]
pub struct Theme {
    resolved: HashMap<String, Style>,
    message_templates: HashMap<String, MessageTypeConfig>,
}

impl Theme {
    pub fn new(config: &ThemeConfig) -> Self {
        let mut resolved = HashMap::new();
        for (name, style_str) in &config.styles {
            resolved.insert(name.clone(), parse_style(style_str));
        }
        Self { 
            resolved,
            message_templates: config.message_types.clone(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new(&ThemeConfig::default())
    }
}

impl Theme {
    /// Formats a semantic message into an ANSI string for the output panel
    pub fn format_message(&self, kind: &str, message: &str) -> String {
        let template = self.message_templates.get(kind).cloned().unwrap_or_else(|| {
            // Provide sensible defaults if the template is missing from YAML
            match kind {
                "system" => MessageTypeConfig { icon: "⬒".to_string(), icon_style: "#808080".to_string(), text_style: "white".to_string() },
                "action" => MessageTypeConfig { icon: "⮻".to_string(), icon_style: "#466473".to_string(), text_style: "white".to_string() },
                "serial" => MessageTypeConfig { icon: "⇄".to_string(), icon_style: "#466473".to_string(), text_style: "white".to_string() },
                "error" => MessageTypeConfig { icon: "✗".to_string(), icon_style: "red".to_string(), text_style: "red".to_string() },
                "warn" => MessageTypeConfig { icon: "⚠".to_string(), icon_style: "yellow".to_string(), text_style: "yellow".to_string() },
                "info" => MessageTypeConfig { icon: "ｉ".to_string(), icon_style: "bold white".to_string(), text_style: "white".to_string() },
                _ => MessageTypeConfig { icon: "".to_string(), icon_style: "white".to_string(), text_style: "white".to_string() },
            }
        });

        let icon_ansi = style_to_ansi(&parse_style(&template.icon_style));
        let text_ansi = style_to_ansi(&parse_style(&template.text_style));
        let reset = "\x1b[0m";

        // Skip space if no icon, or if it's the specific 'i' icon requested to be tight
        let space = if template.icon.is_empty() || template.icon == "ｉ" { "" } else { " " };

        format!("{}{}{}{}{}{}{}", 
            icon_ansi, template.icon, reset,
            space,
            text_ansi, message, reset
        )
    }



    pub fn style(&self, name: &str) -> Style {
        self.resolved.get(name).cloned().unwrap_or_else(|| {
            // Provide sensible defaults if the key is missing from YAML
            match name {
                "commands_highlight" => Style::default().fg(Color::Cyan).bg(Color::Rgb(0, 40, 40)).add_modifier(Modifier::BOLD),
                "output_title" | "commands_title" | "progress_title" | "input_title" => Style::default().add_modifier(Modifier::BOLD),
                "input_border" => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            }
        })
    }
}

/// Parses strings like "bold cyan on black" or "red" or "#ff0000 on #000000"
fn parse_style(s: &str) -> Style {
    let mut style = Style::default();
    let parts: Vec<&str> = s.split_whitespace().collect();
    let mut is_bg = false;

    for part in parts {
        let p = part.to_lowercase();
        match p.as_str() {
            "on" => { is_bg = true; continue; }
            "bold" => style = style.add_modifier(Modifier::BOLD),
            "dim" => style = style.add_modifier(Modifier::DIM),
            "italic" => style = style.add_modifier(Modifier::ITALIC),
            "underline" => style = style.add_modifier(Modifier::UNDERLINED),
            _ => {
                if let Some(color) = parse_color(&p) {
                    if is_bg {
                        style = style.bg(color);
                    } else {
                        style = style.fg(color);
                    }
                }
            }
        }
    }
    style
}

fn parse_color(c: &str) -> Option<Color> {
    if c.starts_with('#') && c.len() == 7 {
        let r = u8::from_str_radix(&c[1..3], 16).ok()?;
        let g = u8::from_str_radix(&c[3..5], 16).ok()?;
        let b = u8::from_str_radix(&c[5..7], 16).ok()?;
        return Some(Color::Rgb(r, g, b));
    }

    if c.starts_with("rgb(") && c.ends_with(')') {
        let inner = &c[4..c.len()-1];
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        if parts.len() == 3 {
            let r = parts[0].parse::<u8>().ok()?;
            let g = parts[1].parse::<u8>().ok()?;
            let b = parts[2].parse::<u8>().ok()?;
            return Some(Color::Rgb(r, g, b));
        }
    }

    match c {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "gray" | "grey" => Some(Color::Gray),
        "darkgray" | "darkgrey" => Some(Color::DarkGray),
        "white" => Some(Color::White),
        _ => None,
    }
}

/// Helper to convert a Ratatui Style into an ANSI escape sequence
fn style_to_ansi(style: &Style) -> String {
    let mut parts = Vec::new();
    
    if let Some(fg) = style.fg {
        match fg {
            Color::Rgb(r, g, b) => parts.push(format!("38;2;{};{};{}", r, g, b)),
            Color::Indexed(i) => parts.push(format!("38;5;{}", i)),
            _ => {
                let code = match fg {
                    Color::Black => 30, Color::Red => 31, Color::Green => 32,
                    Color::Yellow => 33, Color::Blue => 34, Color::Magenta => 35,
                    Color::Cyan => 36, Color::White => 37, _ => 39,
                };
                parts.push(code.to_string());
            }
        }
    }

    if let Some(bg) = style.bg {
        match bg {
            Color::Rgb(r, g, b) => parts.push(format!("48;2;{};{};{}", r, g, b)),
            _ => {
                let code = match bg {
                    Color::Black => 40, Color::Red => 41, Color::Green => 42,
                    Color::Yellow => 43, Color::Blue => 44, Color::Magenta => 45,
                    Color::Cyan => 46, Color::White => 47, _ => 49,
                };
                parts.push(code.to_string());
            }
        }
    }

    if style.add_modifier.contains(Modifier::BOLD) { parts.push("1".to_string()); }
    if style.add_modifier.contains(Modifier::DIM) { parts.push("2".to_string()); }
    if style.add_modifier.contains(Modifier::ITALIC) { parts.push("3".to_string()); }
    if style.add_modifier.contains(Modifier::UNDERLINED) { parts.push("4".to_string()); }

    if parts.is_empty() {
        "".to_string()
    } else {
        format!("\x1b[{}m", parts.join(";"))
    }
}
