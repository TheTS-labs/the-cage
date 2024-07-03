use std::cell::RefCell;
use std::rc::Rc;

use ratatui::crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::canvas::{Context, Points};

use crate::app::ForbiddenPoints;
use crate::traits::debug_lines::DebugLines;
use crate::traits::object::Object;
use crate::traits::on_event::OnEvent;
use crate::traits::pixel_coords::PixelCoords;

pub const CHARACTER_STEP: u16 = 1;

#[derive(Clone, Debug)]
pub struct Character {
    pub character_pos: [u16; 2],
    pub area:          Rect,
    forbidden_points:  Option<Rc<RefCell<ForbiddenPoints>>>,
}

impl Character {
    pub fn intersects_with_drawn(&self, x: u16, y: u16) -> bool {
        let forbidden = Rc::clone(self.forbidden_points.as_ref().unwrap());
        let forbidden = forbidden.borrow();

        forbidden.contains(&(x, y)) ||
                forbidden.contains(&(x + 1, y + 1)) ||
                forbidden.contains(&(x + 1, y)) ||
                forbidden.contains(&(x, y + 1))
    }

    pub fn new(character_pos: [u16; 2], area: Rect) -> Self {
        Self {
            character_pos,
            area,
            forbidden_points: None,
        }
    }
}

impl Object for Character {
    fn draw(&mut self, ctx: &mut Context, forbidden_points: Rc<RefCell<ForbiddenPoints>>) {
        self.forbidden_points = Some(Rc::clone(&forbidden_points));

        ctx.draw(&Points {
            coords: &[
                self.calculate_coords(self.character_pos[0], self.character_pos[1])
                    .into(),
                self.calculate_coords(self.character_pos[0] + 1, self.character_pos[1] + 1)
                    .into(),
                self.calculate_coords(self.character_pos[0] + 1, self.character_pos[1])
                    .into(),
                self.calculate_coords(self.character_pos[0], self.character_pos[1] + 1)
                    .into(),
            ],
            color:  Color::Yellow,
        })
    }
}

impl PixelCoords for Character {}
impl OnEvent for Character {
    fn on_key_pressed(&mut self, key_code: KeyCode) -> Option<KeyCode> {
        if key_code == KeyCode::Char('w') &&
            self.character_pos[1] + CHARACTER_STEP <= self.area.height * 2 - 2 &&
            !self.intersects_with_drawn(self.character_pos[0], self.character_pos[1] + CHARACTER_STEP)
        {
            self.character_pos[1] += CHARACTER_STEP
        } else if key_code == KeyCode::Char('s') &&
            self.character_pos[1] - CHARACTER_STEP >= 1 &&
            !self.intersects_with_drawn(self.character_pos[0], self.character_pos[1] - CHARACTER_STEP)
        {
            self.character_pos[1] -= CHARACTER_STEP
        } else if key_code == KeyCode::Char('a') &&
            self.character_pos[0] - CHARACTER_STEP >= 1 &&
            !self.intersects_with_drawn(self.character_pos[0] - CHARACTER_STEP, self.character_pos[1])
        {
            self.character_pos[0] -= CHARACTER_STEP
        } else if key_code == KeyCode::Char('d') &&
            self.character_pos[0] + CHARACTER_STEP <= self.area.width - 1 &&
            !self.intersects_with_drawn(self.character_pos[0] + CHARACTER_STEP, self.character_pos[1])
        {
            self.character_pos[0] += CHARACTER_STEP
        }

        Some(key_code)
    }
}
impl DebugLines for Character {
    fn debug_lines(&self) -> Vec<Line> {
        vec![Line::from(vec![
            "Character pos (".bold(),
            "X".red().bold(),
            " ".into(),
            "Y".green().bold(),
            "): ".bold(),
            self.character_pos[0].to_string().red().bold(),
            " ".into(),
            self.character_pos[1].to_string().green().bold(),
        ])]
    }
}
