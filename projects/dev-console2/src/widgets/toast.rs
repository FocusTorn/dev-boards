use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Clear, Paragraph, Widget},
};
use std::time::{Duration, Instant};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastLevel {
    #[allow(dead_code)]
    Info,
    Success,
    #[allow(dead_code)]
    Warning,
    Error,
}

impl ToastLevel {
    fn color(&self) -> Color {
        match self {
            ToastLevel::Info => Color::Cyan,
            ToastLevel::Success => Color::Green,
            ToastLevel::Warning => Color::Yellow,
            ToastLevel::Error => Color::Red,
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            ToastLevel::Info => "ℹ",
            ToastLevel::Success => "✓",
            ToastLevel::Warning => "⚠",
            ToastLevel::Error => "✗",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    #[default]
    BottomCenter,
    BottomRight,
    Center,
}

fn default_duration_seconds() -> f32 { 1.5 }
fn default_fade_out_seconds() -> f32 { 0.5 }
fn bottom_center() -> ToastPosition { ToastPosition::BottomCenter }

#[derive(Debug, Clone, Deserialize)]
pub struct ToastConfig {
    #[serde(default = "bottom_center")]
    pub position: ToastPosition,
    #[serde(default = "default_duration_seconds")]
    pub duration_seconds: f32,
    #[serde(default = "default_fade_out_seconds")]
    pub fade_out_seconds: f32,
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub level: ToastLevel,
    pub shown_at: Instant,
    pub duration: Duration,
    pub opacity: f64, // 0.0 to 1.0
}

impl Toast {
    pub fn new(message: String, level: ToastLevel, duration: Duration) -> Self {
        Self {
            message,
            level,
            shown_at: Instant::now(),
            duration,
            opacity: 1.0,
        }
    }
}

#[derive(Debug)]
pub struct ToastManager {
    pub toasts: Vec<Toast>,
    pub config: ToastConfig,
}

impl ToastManager {
    pub fn new(config: ToastConfig) -> Self {
        Self {
            toasts: Vec::new(),
            config,
        }
    }

    pub fn add(&mut self, message: String, level: ToastLevel) {
        let total_duration = Duration::from_secs_f32(self.config.duration_seconds + self.config.fade_out_seconds);
        self.toasts.push(Toast::new(message, level, total_duration));
    }

    #[allow(dead_code)]
    pub fn info(&mut self, message: &str) {
        self.add(message.to_string(), ToastLevel::Info);
    }

    pub fn success(&mut self, message: &str) {
        self.add(message.to_string(), ToastLevel::Success);
    }

    #[allow(dead_code)]
    pub fn warning(&mut self, message: &str) {
        self.add(message.to_string(), ToastLevel::Warning);
    }

    pub fn error(&mut self, message: &str) {
        self.add(message.to_string(), ToastLevel::Error);
    }

    pub fn update(&mut self) {
        let fade_start_offset = Duration::from_secs_f32(self.config.duration_seconds);
        let fade_duration = Duration::from_secs_f32(self.config.fade_out_seconds);

        self.toasts.retain_mut(|t| {
            let elapsed = t.shown_at.elapsed();
            if elapsed >= t.duration {
                return false;
            }

            if elapsed > fade_start_offset {
                let fade_elapsed = elapsed.saturating_sub(fade_start_offset);
                let fade_pct = fade_elapsed.as_secs_f64() / fade_duration.as_secs_f64();
                t.opacity = (1.0 - fade_pct).max(0.0);
            } else {
                t.opacity = 1.0;
            }
            true
        });
    }
}

pub struct ToastWidget<'a> {
    manager: &'a mut ToastManager,
}

impl<'a> ToastWidget<'a> {
    pub fn new(manager: &'a mut ToastManager) -> Self {
        Self { manager }
    }
}

impl<'a> Widget for ToastWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Encapsulate expiration logic within the render pass
        self.manager.update();

        if self.manager.toasts.is_empty() {
            return;
        }

        let toasts = &self.manager.toasts;
        let position = &self.manager.config.position;

        // Calculate the maximum width of all toasts
        let mut max_width = 0usize;
        let mut toast_data: Vec<(String, Color, f64)> = Vec::new();

        for toast in toasts {
            let icon = toast.level.icon();
            let fg_color = toast.level.color();
            let opacity = toast.opacity;

            let content = format!("{} {}", icon, toast.message);
            max_width = max_width.max(content.len());
            toast_data.push((content, fg_color, opacity));
        }

        // Add padding
        max_width += 3;
        let toast_height = 1u16;
        let max_width_u16 = max_width as u16;

        let mut y_offset = 0u16;

        // Determine render order and base position
        // If stacking up (Bottom*), we render in reverse order (newest at bottom)
        // If stacking down (Top*), we render in normal order (newest at top) or reverse?
        // Usually newest is "closest to the edge".
        
        let stack_up = matches!(
            position,
            ToastPosition::BottomLeft | ToastPosition::BottomCenter | ToastPosition::BottomRight
        );

        let iter: Box<dyn Iterator<Item = &(String, Color, f64)>> = if stack_up {
            Box::new(toast_data.iter().rev())
        } else {
            Box::new(toast_data.iter().rev()) 
        };

        for (content, fg_color, opacity) in iter {
            // Left-pad content to match max width
            let content_len = content.len();
            let left_padding = max_width.saturating_sub(content_len).saturating_sub(1).max(2);

            let mut padded_text = format!("{}{} ", " ".repeat(left_padding), content);
            while padded_text.len() < max_width {
                padded_text.push(' ');
            }
            if padded_text.len() > max_width {
                padded_text.truncate(max_width);
            }

            let (toast_x, toast_y) = match position {
                ToastPosition::TopLeft => (area.x + 1, area.y + 1 + y_offset),
                ToastPosition::TopRight => (area.x + area.width.saturating_sub(max_width_u16).saturating_sub(1), area.y + 1 + y_offset),
                ToastPosition::TopCenter => (area.x + (area.width.saturating_sub(max_width_u16)) / 2, area.y + 1 + y_offset),
                ToastPosition::BottomLeft => (area.x + 1, area.y + area.height.saturating_sub(1 + toast_height + y_offset)),
                ToastPosition::BottomRight => (area.x + area.width.saturating_sub(max_width_u16).saturating_sub(1), area.y + area.height.saturating_sub(1 + toast_height + y_offset)),
                ToastPosition::BottomCenter => (area.x + (area.width.saturating_sub(max_width_u16)) / 2, area.y + area.height.saturating_sub(1 + toast_height + y_offset)),
                ToastPosition::Center => (
                    area.x + (area.width.saturating_sub(max_width_u16)) / 2, 
                    area.y + (area.height.saturating_sub(toast_height * toasts.len() as u16)) / 2 + y_offset // Simple center stacking
                ),
            };

            // Check bounds
            if toast_y >= area.y + area.height || toast_x >= area.x + area.width {
                continue; 
            }

            let toast_area = Rect {
                x: toast_x,
                y: toast_y,
                width: max_width_u16,
                height: toast_height,
            };

            Clear.render(toast_area, buf);

            // Apply fade effect: Interpolate color toward grey/black based on opacity
            let current_fg = if *opacity < 1.0 {
                Color::Indexed(240 + (*opacity * 15.0) as u8) // Fade to dark grey
            } else {
                *fg_color
            };

            let toast_widget = Paragraph::new(padded_text)
                .style(Style::default()
                    .fg(current_fg)
                    .bg(Color::Rgb(10, 10, 10))
                    .add_modifier(Modifier::BOLD));

            toast_widget.render(toast_area, buf);

            y_offset += toast_height;
        }
    }
}