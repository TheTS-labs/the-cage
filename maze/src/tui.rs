use std::io::{self, stdout, Stdout};
use std::panic::{set_hook, take_hook};

use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::crossterm::ExecutableCommand;
use ratatui::Terminal;

pub struct Tui {
    pub terminal: Terminal<CrosstermBackend<Stdout>>
}

impl Tui {
    pub fn init() -> io::Result<Tui> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        Ok(Tui { terminal })
    }

    pub fn restore() -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;

        Ok(())
    }

    pub fn init_panic_hook() {
        let original_hook = take_hook();

        set_hook(Box::new(move |panic_info| {
            let _ = Tui::restore();

            original_hook(panic_info); 
        }));
    }
}