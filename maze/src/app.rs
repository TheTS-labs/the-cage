use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::io::{self, Stdout};
use std::rc::Rc;

use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::Canvas;
use ratatui::{Frame, Terminal};

use crate::objects::debug_screen::DebugScreen;
use crate::objects::maze::Maze;
use crate::traits::object::Object;

pub type ForbiddenPoints = HashSet<(u16, u16)>;

#[derive(Debug)]
pub struct App {
    debug:            bool,
    exit:             bool,
    objects:          RefCell<Vec<Box<dyn Object>>>,
    forbidden_points: Rc<RefCell<ForbiddenPoints>>,
}

impl App {
    pub fn new(maze_size: usize, maze_seed: u64) -> App {
        Self {
            debug:            false,
            exit:             false,
            objects:          RefCell::new(vec![Box::new(Maze::new(maze_size, maze_seed))]),
            forbidden_points: Rc::new(RefCell::new(HashSet::new())),
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
        let mut debug_screen = DebugScreen::new(self.debug);
        let area = frame.size();

        debug_screen.measure(|| {
            for object in &mut *self.objects.borrow_mut() {
                object.render(area, frame.buffer_mut(), Rc::clone(&self.forbidden_points));
            }

            frame.render_widget(
                Canvas::default()
                    .marker(Marker::HalfBlock)
                    .paint(|ctx| {
                        for object in &mut *self.objects.borrow_mut() {
                            object.draw(ctx, Rc::clone(&self.forbidden_points));
                        }
                    })
                    .x_bounds([area.x as f64, (area.x + area.width) as f64])
                    .y_bounds([area.y as f64, (area.y + area.height) as f64]),
                area,
            );
        });

        let binding = self.objects.borrow();
        for object in binding.iter() {
            debug_screen.add_lines(object.debug_lines());
        }

        debug_screen.render(area, frame.buffer_mut(), Rc::clone(&self.forbidden_points));
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self.on_key_pressed(key_event.code),
                _ => todo!("Handle other events"),
            };
        }

        Ok(())
    }

    pub fn on_key_pressed(&mut self, key_code: KeyCode) -> Option<KeyCode> {
        for object in &mut *self.objects.borrow_mut() {
            object.on_key_pressed(key_code)?;
        }

        match key_code {
            KeyCode::Esc => self.exit = true,
            KeyCode::Tab => self.debug = !self.debug,
            _ => (),
        };

        Some(key_code)
    }
}
