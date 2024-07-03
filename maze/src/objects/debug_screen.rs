use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use ratatui::buffer::Buffer;
use ratatui::crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::Widget;

use crate::app::ForbiddenPoints;
use crate::traits::debug_lines::DebugLines;
use crate::traits::object::Object;
use crate::traits::on_event::OnEvent;

#[derive(Debug, Default)]
pub struct DebugScreen<'a> {
    pub show:          bool,
    elapsed_per_frame: Duration,
    lines:             Vec<Line<'a>>,
}

impl<'a> DebugScreen<'a> {
    pub fn new(show: bool) -> DebugScreen<'a> {
        DebugScreen {
            show,
            ..Default::default()
        }
    }

    pub fn set_elapsed_per_frame(&mut self, elapsed_per_frame: Duration) { self.elapsed_per_frame = elapsed_per_frame; }

    pub fn add_lines(&mut self, lines: Vec<Line<'a>>) { self.lines.extend(lines); }

    pub fn measure<T: FnMut() -> R, R>(&mut self, mut func: T) -> R {
        let start = Instant::now();

        let result = func();

        self.set_elapsed_per_frame(start.elapsed());

        result
    }
}

impl DebugLines for DebugScreen<'_> {
    fn debug_lines(&self) -> Vec<Line> { vec![] }
}

impl OnEvent for DebugScreen<'_> {
    fn on_key_pressed(&mut self, key_code: KeyCode) -> Option<KeyCode> { Some(key_code) }
}

impl Object for DebugScreen<'_> {
    fn render(&mut self, area: Rect, buf: &mut Buffer, _forbidden_points: Rc<RefCell<ForbiddenPoints>>) {
        let mut text = Text::from_iter(self.lines.drain(..));
        if !self.show {
            return;
        }

        text.push_line(Line::from(vec![
            "Performance (".bold(),
            "Elapsed per frame".bold().cyan(),
            "/".into(),
            "FPS".bold().blue(),
            "): ".bold(),
            format!("{}ms", self.elapsed_per_frame.as_millis()).bold().cyan(),
            " / ".into(),
            format!("{:.0}", 1.0 / self.elapsed_per_frame.as_secs_f64())
                .bold()
                .blue(),
        ]));

        text.render(area, buf);
    }
}
