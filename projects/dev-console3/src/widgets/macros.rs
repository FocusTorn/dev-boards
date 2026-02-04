#[macro_export]
macro_rules! tui_component {
    (
        #[derive($($derive:meta),*)]
        pub enum $name:ident {
            $($variant:ident($inner:ty)),* $(,)?
        }
    ) => {
        #[derive($($derive),*)]
        pub enum $name {
            $($variant($inner)),*
        }

        impl $name {
            pub fn on_tick(&mut self) -> color_eyre::Result<()> {
                match self {
                    $(Self::$variant(inner) => inner.on_tick()),*
                }
            }

            pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> color_eyre::Result<$crate::widgets::WidgetOutcome<String>> {
                match self {
                    $(Self::$variant(inner) => {
                        let outcome = inner.handle_key(key)?;
                        match outcome {
                            $crate::widgets::WidgetOutcome::None => Ok($crate::widgets::WidgetOutcome::None),
                            $crate::widgets::WidgetOutcome::Consumed => Ok($crate::widgets::WidgetOutcome::Consumed),
                            $crate::widgets::WidgetOutcome::Changed(_) => Ok($crate::widgets::WidgetOutcome::Consumed),
                            $crate::widgets::WidgetOutcome::Confirmed(_) => Ok($crate::widgets::WidgetOutcome::Confirmed("Action".into())),
                            $crate::widgets::WidgetOutcome::Canceled => Ok($crate::widgets::WidgetOutcome::Canceled),
                        }
                    }),*
                }
            }

            pub fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent, area: ratatui::layout::Rect) -> color_eyre::Result<$crate::widgets::WidgetOutcome<String>> {
                match self {
                    $(Self::$variant(inner) => {
                        let outcome = inner.handle_mouse(mouse, area)?;
                        match outcome {
                            $crate::widgets::WidgetOutcome::None => Ok($crate::widgets::WidgetOutcome::None),
                            $crate::widgets::WidgetOutcome::Consumed => Ok($crate::widgets::WidgetOutcome::Consumed),
                            $crate::widgets::WidgetOutcome::Changed(_) => Ok($crate::widgets::WidgetOutcome::Consumed),
                            $crate::widgets::WidgetOutcome::Confirmed(_) => Ok($crate::widgets::WidgetOutcome::Confirmed("Action".into())),
                            $crate::widgets::WidgetOutcome::Canceled => Ok($crate::widgets::WidgetOutcome::Canceled),
                        }
                    }),*
                }
            }

            pub fn view(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
                match self {
                    $(Self::$variant(inner) => inner.view(f, area)),*
                }
            }
        }
    };
}
