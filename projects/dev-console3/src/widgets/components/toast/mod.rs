pub mod state;
pub mod view;

pub use state::{ToastManager, ToastLevel, ToastPosition, ToastConfig};
use crate::widgets::traits::Component;
use ratatui::prelude::*;
use color_eyre::Result;
use std::time::Duration;

impl Component for ToastManager {
    type Outcome = ();

    fn on_tick(&mut self) -> Result<()> {
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
        Ok(())
    }

    fn view(&mut self, f: &mut Frame, area: Rect) {
        view::render(self, f, area);
    }
}