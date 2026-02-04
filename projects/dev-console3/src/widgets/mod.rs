#[macro_use]
pub mod macros;
pub mod traits;
pub mod elements;
pub mod components;
pub mod manager;

pub use traits::{Component, WidgetOutcome};

// Register all "Smart" components and "Views" here
tui_component! {
    #[derive(Debug)]
    pub enum ComponentRegistry {
        Dashboard(crate::views::dashboard::Dashboard),
        Profiles(crate::views::profiles::Profiles),
        Dropdown(components::dropdown::OverlayDropdown),
        Toast(components::toast::ToastManager),
        TabbedBar(components::tabbed_bar::TabbedBar),
        ButtonBar(components::button_bar::ButtonBar),
    }
}