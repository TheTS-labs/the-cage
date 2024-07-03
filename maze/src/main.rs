mod app;
mod objects;
mod traits;
mod tui;

use std::io;

use app::App;
use objects::maze::MAZE_NODE_SIZE;
use tui::Tui;

fn main() -> io::Result<()> {
    Tui::init_panic_hook();
    let mut terminal = Tui::init()?.terminal;

    let area = terminal.size()?;

    App::new((((area.height * 2) / MAZE_NODE_SIZE) - 1) as usize, 535030530).run(&mut terminal)?;

    Tui::restore()?;
    Ok(())
}
