#[macro_use]
mod widgets;
mod app;
mod terminal;
mod config;
mod views;

use app::App;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    terminal::init_panic_hook();

    let mut app = App::new()?;
    app.run()?;

    Ok(())
}
