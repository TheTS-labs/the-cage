mod app;
mod tui;
mod maze;
mod shapes;
mod traits;
mod types;

use std::io;

use app::App;
use tui::Tui;

fn main() -> io::Result<()> {
    Tui::init_panic_hook();
    let mut terminal = Tui::init()?.terminal;
    
    App::new(30, 535030530).run(&mut terminal)?;

    Tui::restore()?;
    Ok(())
}
