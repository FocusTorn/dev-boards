use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Clear, Paragraph, Widget},
};
use serde::Deserialize;
use std::time::{Duration, Instant};

/// Severity level for a toast notification.
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
    /// Returns the thematic color for the level.
    fn color(&self) -> Color {
        match self {
            ToastLevel::Info => Color::Cyan,
            ToastLevel::Success => Color::Green,
            ToastLevel::Warning => Color::Yellow,
            ToastLevel::Error => Color::Red,
        }
    }

    /// Returns the semantic icon for the level.
    fn icon(&self) -> &'static str {
        match self {
            ToastLevel::Info => "ℹ",
            ToastLevel::Success => "✓",
            ToastLevel::Warning => "⚠",
            ToastLevel::Error => "✗",
        }
    }
}

/// Logical positioning for toast overlays.
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

fn default_duration_seconds() -> f32 {
    1.5
}
fn default_fade_out_seconds() -> f32 {
    0.5
}
fn bottom_center() -> ToastPosition {
    ToastPosition::BottomCenter
}

/// Deserializable configuration for toast behavior.
#[derive(Debug, Clone, Deserialize)]
pub struct ToastConfig {
    #[serde(default = "bottom_center")]
    pub position: ToastPosition,
    #[serde(default = "default_duration_seconds")]
    pub duration_seconds: f32,
    #[serde(default = "default_fade_out_seconds")]
    pub fade_out_seconds: f32,
}

impl Default for ToastConfig {
    fn default() -> Self {
        Self {
            position: bottom_center(),
            duration_seconds: default_duration_seconds(),
            fade_out_seconds: default_fade_out_seconds(),
        }
    }
}

/// A single active notification entry.
#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub level: ToastLevel,
    pub shown_at: Instant,
    pub duration: Duration,
    pub opacity: f64, // 0.0 to 1.0
}

impl Toast {
    /// Creates a new notification with the current timestamp.
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

/// Lifecycle coordinator for multiple concurrent notifications.
#[derive(Debug)]
pub struct ToastManager {
    pub toasts: Vec<Toast>,
    pub config: ToastConfig,
}

impl ToastManager {
    /// Initializes a manager with specified timing and position rules.
    pub fn new(config: ToastConfig) -> Self {
        Self {
            toasts: Vec::new(),
            config,
        }
    }

    /// Appends a new message to the notification queue.
    pub fn add(&mut self, message: String, level: ToastLevel) {
        let total_duration =
            Duration::from_secs_f32(self.config.duration_seconds + self.config.fade_out_seconds);
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

    /// Processes aging and fade-out math for all active notifications.
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

/// Overlay widget for rendering active notifications.
pub struct ToastWidget<'a> {
    manager: &'a mut ToastManager,
}

impl<'a> ToastWidget<'a> {
    /// Creates a widget bound to the provided lifecycle manager.
    pub fn new(manager: &'a mut ToastManager) -> Self {
        Self { manager }
    }
}

impl<'a> Widget for ToastWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.manager.update();

        if self.manager.toasts.is_empty() {
            return;
        }

        let toasts = &self.manager.toasts;
        let position = &self.manager.config.position;

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

        max_width += 3;
        let toast_height = 1u16;
        let max_width_u16 = max_width as u16;

        let mut y_offset = 0u16;

        for (content, fg_color, opacity) in toast_data.iter().rev() {
            let content_len = content.len();
            let left_padding = max_width
                .saturating_sub(content_len)
                .saturating_sub(1)
                .max(2);

            let mut padded_text = format!("{}{} ", " ".repeat(left_padding), content);
            while padded_text.len() < max_width {
                padded_text.push(' ');
            }
            if padded_text.len() > max_width {
                padded_text.truncate(max_width);
            }

            let (toast_x, toast_y) = match position {
                ToastPosition::TopLeft => (area.x + 1, area.y + 1 + y_offset),
                ToastPosition::TopRight => (
                    area.x + area.width.saturating_sub(max_width_u16).saturating_sub(1),
                    area.y + 1 + y_offset,
                ),
                ToastPosition::TopCenter => (
                    area.x + (area.width.saturating_sub(max_width_u16)) / 2,
                    area.y + 1 + y_offset,
                ),
                ToastPosition::BottomLeft => (
                    area.x + 1,
                    area.y + area.height.saturating_sub(1 + toast_height + y_offset),
                ),
                ToastPosition::BottomRight => (
                    area.x + area.width.saturating_sub(max_width_u16).saturating_sub(1),
                    area.y + area.height.saturating_sub(1 + toast_height + y_offset),
                ),
                ToastPosition::BottomCenter => (
                    area.x + (area.width.saturating_sub(max_width_u16)) / 2,
                    area.y + area.height.saturating_sub(1 + toast_height + y_offset),
                ),
                ToastPosition::Center => (
                    area.x + (area.width.saturating_sub(max_width_u16)) / 2,
                    area.y
                        + (area
                            .height
                            .saturating_sub(toast_height * toasts.len() as u16))
                            / 2
                        + y_offset,
                ),
            };

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

            let current_fg = if *opacity < 1.0 {
                Color::Indexed(240 + (*opacity * 15.0) as u8)
            } else {
                *fg_color
            };

            let toast_widget = Paragraph::new(padded_text).style(
                Style::default()
                    .fg(current_fg)
                    .bg(Color::Rgb(10, 10, 10))
                    .add_modifier(Modifier::BOLD),
            );

            toast_widget.render(toast_area, buf);

            y_offset += toast_height;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn buffer_content(buffer: &Buffer) -> String {
        let mut result = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                result.push_str(buffer[(x, y)].symbol());
            }
            result.push('\n');
        }
        result
    }

    #[test]
    fn test_toast_manager_lifecycle() {
        let mut manager = ToastManager::new(ToastConfig {
            duration_seconds: 0.1,
            fade_out_seconds: 0.1,
            ..Default::default()
        });

        manager.success("test");
        assert_eq!(manager.toasts.len(), 1);

        std::thread::sleep(Duration::from_millis(250));
        manager.update();
        assert_eq!(manager.toasts.len(), 0);
    }

    #[test]
    fn test_toast_rendering() {
        let mut manager = ToastManager::new(ToastConfig {
            position: ToastPosition::TopRight,
            ..Default::default()
        });
        manager.success("Task Complete");

        let backend = TestBackend::new(50, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let widget = ToastWidget::new(&mut manager);
                f.render_widget(widget, f.area());
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        let s = buffer_content(buffer);

        assert!(s.contains("✓ Task Complete"));

        // Success should be Green
        // We find the '✓' character
        let mut found = false;
        for y in 0..10 {
            for x in 0..50 {
                if buffer[(x, y)].symbol() == "✓" {
                    assert_eq!(buffer[(x, y)].fg, Color::Green);
                    found = true;
                }
            }
        }
        assert!(found);
    }

    #[test]
    fn test_toast_multiple_levels() {
        let mut manager = ToastManager::new(ToastConfig::default());
        manager.error("Failure");
        manager.info("Note");

        let backend = TestBackend::new(50, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let widget = ToastWidget::new(&mut manager);
                f.render_widget(widget, f.area());
            })
            .unwrap();

        let s = buffer_content(terminal.backend().buffer());
        assert!(s.contains("✗ Failure"));
        assert!(s.contains("ℹ Note"));
    }
}
