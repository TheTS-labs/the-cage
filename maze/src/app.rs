use std::borrow::Borrow;
use std::cell::RefCell;
use std::io::{self, Stdout};
use std::rc::Rc;
use std::time::Instant;

use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{Frame, Terminal};

use crate::maze::Maze;
use crate::shapes::debug::DebugWidget;

#[derive(Debug)]
pub struct App {
    exit:  bool,
    debug: DebugWidget,
    maze:  Rc<RefCell<Maze>>,
}

impl App {
    pub fn new(maze_size: usize, maze_seed: u64) -> Self {
        let maze = Rc::new(RefCell::new(Maze::new(maze_size, maze_seed)));

        Self {
            exit: false,
            debug: DebugWidget::new(Rc::clone(&maze)),
            maze,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let frame_start = Instant::now();

        {
            let maze: &RefCell<Maze> = self.maze.borrow();
            let mut maze = maze.borrow_mut();

            frame.render_widget(&mut *maze, frame.size());
        }

        self.debug.set_elapsed_per_frame(frame_start.elapsed());
        frame.render_widget(&self.debug, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self.on_key_pressed(key_event.code),
                _ => todo!("Handle other events"),
            }
        }

        Ok(())
    }

    pub fn on_key_pressed(&mut self, key_code: KeyCode) {
        let maze: &RefCell<Maze> = self.maze.borrow();
        let mut maze = maze.borrow_mut();
        let maze = &mut *maze;

        match key_code {
            KeyCode::Char('w' | 'a' | 's' | 'd') => maze.on_character_move(key_code),
            KeyCode::Char('p') => self.debug.toggle_show(),
            KeyCode::Char('q') => self.exit = true,
            _ => (),
        };
    }
}
