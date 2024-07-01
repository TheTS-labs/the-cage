use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::Widget;

use crate::maze::Maze;

#[derive(Debug)]
pub struct DebugWidget {
    show:              bool,
    elapsed_per_frame: Duration,
    inner:             Rc<RefCell<Maze>>,
}

impl DebugWidget {
    pub fn new(inner: Rc<RefCell<Maze>>) -> DebugWidget {
        Self {
            inner,
            show: true,
            elapsed_per_frame: Duration::ZERO,
        }
    }

    pub fn toggle_show(&mut self) { self.show = !self.show; }

    pub fn set_elapsed_per_frame(&mut self, elapsed_per_frame: Duration) { self.elapsed_per_frame = elapsed_per_frame; }
}

impl Widget for &DebugWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.show {
            return;
        }

        let inner = self.inner.borrow();
        Text::from(vec![
            Line::from(vec![
                "Character pos (".bold(),
                "X".red().bold(),
                " ".into(),
                "Y".green().bold(),
                "): ".bold(),
                inner.character.character_pos[0].to_string().red().bold(),
                " ".into(),
                inner.character.character_pos[1].to_string().green().bold(),
            ]),
            Line::from(vec![
                "Area (".bold(),
                "X".red().bold(),
                "/".into(),
                "Y".green().bold(),
                "/".into(),
                "Width".yellow().bold(),
                "/".into(),
                "Height".magenta().bold(),
                "): ".bold(),
                inner.area.x.to_string().red().bold(),
                " / ".into(),
                inner.area.y.to_string().green().bold(),
                " / ".into(),
                inner.area.width.to_string().yellow().bold(),
                " / ".into(),
                inner.area.height.to_string().magenta().bold(),
            ]),
            Line::from(vec![
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
            ]),
        ])
        .render(area, buf);
    }
}
