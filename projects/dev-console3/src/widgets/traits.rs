use ratatui::layout::Rect;
use ratatui::Frame;
use crossterm::event::{KeyEvent, MouseEvent};
use color_eyre::Result;

/// Generic outcome for interactive components.
/// Used to communicate state changes or actions back to the orchestrator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WidgetOutcome<T> {
    /// No change or event occurred.
    None,
    /// The input was consumed but no significant state change occurred.
    Consumed,
    /// The component's internal state has changed.
    Changed(T),
    /// A final action was confirmed (e.g., Enter pressed).
    Confirmed(T),
    /// The interaction was canceled (e.g., Esc pressed).
    Canceled,
}

/// The core trait for all UI components in dev-console3.
/// Encapsulates logic, state updates, and rendering.
pub trait Component {
    /// Associated type for the outcome returned by interactions.
    type Outcome;

    /// Update logic called on every terminal tick.
    fn on_tick(&mut self) -> Result<()> {
        Ok(())
    }

    /// Handle keyboard input.
    fn handle_key(&mut self, key: KeyEvent) -> Result<WidgetOutcome<Self::Outcome>> {
        let _ = key;
        Ok(WidgetOutcome::None)
    }

    /// Handle mouse input within a specific area.
    fn handle_mouse(&mut self, mouse: MouseEvent, area: Rect) -> Result<WidgetOutcome<Self::Outcome>> {
        let _ = mouse;
        let _ = area;
        Ok(WidgetOutcome::None)
    }

    /// Render the component into the provided frame and area.
    fn view(&mut self, f: &mut Frame, area: Rect);
}
