// Rendering module

pub mod settings;
pub mod dashboard;
pub mod content;
pub mod settings2_standalone;

pub use settings::render_settings;
pub use dashboard::render_dashboard;
pub use content::render_content;
pub use settings2_standalone::render_settings2_standalone;
