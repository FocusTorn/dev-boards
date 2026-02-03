pub mod command_list;
pub mod components;
pub mod dimmer;
pub mod file_browser;
pub mod output_box;
pub mod popup;
pub mod progress_bar;
pub mod selection_list;
pub mod smooth_scrollbar;
pub mod status_box;
// pub mod tab_bar;
pub mod toast;

/// Generic outcome for interactive widgets.
/// Used to communicate state changes from encapsulated widgets to the parent view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WidgetOutcome<T> {
    /// No change or event occurred.
    None,
    /// The input was consumed but no significant state change occurred.
    Consumed,
    /// The widget's internal state (selection, text) has changed.
    Changed(T),
    /// A final action was confirmed (e.g., Enter pressed).
    Confirmed(T),
    /// The action was canceled (e.g., Esc pressed).
    Canceled,
}

/// Trait for widgets that can handle keyboard and mouse input and return an outcome.
pub trait InteractiveWidget {
    type Outcome;
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> WidgetOutcome<Self::Outcome>;
    fn handle_mouse(
        &mut self,
        mouse: crossterm::event::MouseEvent,
        area: ratatui::layout::Rect,
    ) -> WidgetOutcome<Self::Outcome>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_outcome_equality() {
        assert_eq!(WidgetOutcome::<()>::None, WidgetOutcome::None);
        assert_eq!(
            WidgetOutcome::<i32>::Changed(10),
            WidgetOutcome::Changed(10)
        );
        assert_ne!(
            WidgetOutcome::<i32>::Changed(10),
            WidgetOutcome::Changed(20)
        );
        assert_eq!(
            WidgetOutcome::<String>::Confirmed("test".to_string()),
            WidgetOutcome::Confirmed("test".to_string())
        );
        assert_eq!(WidgetOutcome::<()>::Canceled, WidgetOutcome::Canceled);
    }
}
