use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Painter, Shape};

use crate::traits::pixel_coords::PixelCoords;
use crate::types::direction::Direction;

#[derive(Clone, Debug)]
pub struct Character {
    pub character_pos: [u16; 2],
    pub area:          Rect,
    pub drawn_points:  Rc<RefCell<HashSet<(u16, u16)>>>,
}

impl Character {
    pub fn move_character(&mut self, direction: Direction, step: u16) {
        if direction == Direction::Up &&
            self.character_pos[1] + step <= self.area.height * 2 - 2 &&
            !self.intersects_with_drawn(self.character_pos[0], self.character_pos[1] + step)
        {
            self.character_pos[1] += step
        } else if direction == Direction::Down &&
            self.character_pos[1] - step >= 1 &&
            !self.intersects_with_drawn(self.character_pos[0], self.character_pos[1] - step)
        {
            self.character_pos[1] -= step
        } else if direction == Direction::Left &&
            self.character_pos[0] - step >= 1 &&
            !self.intersects_with_drawn(self.character_pos[0] - step, self.character_pos[1])
        {
            self.character_pos[0] -= step
        } else if direction == Direction::Right &&
            self.character_pos[0] + step <= self.area.width - 1 &&
            !self.intersects_with_drawn(self.character_pos[0] + step, self.character_pos[1])
        {
            self.character_pos[0] += step
        }
    }

    pub fn intersects_with_drawn(&self, x: u16, y: u16) -> bool {
        let drawn = self.drawn_points.borrow();
        drawn.contains(&(x, y)) ||
            drawn.contains(&(x + 1, y + 1)) ||
            drawn.contains(&(x + 1, y)) ||
            drawn.contains(&(x, y + 1))
    }

    pub fn new(character_pos: [u16; 2], area: Rect, drawn_points: Rc<RefCell<HashSet<(u16, u16)>>>) -> Self {
        Self {
            character_pos,
            area,
            drawn_points,
        }
    }
}

impl PixelCoords for Character {}
impl Shape for Character {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        let points = [
            self.calculate_coords(self.character_pos[0], self.character_pos[1]),
            self.calculate_coords(self.character_pos[0] + 1, self.character_pos[1] + 1),
            self.calculate_coords(self.character_pos[0] + 1, self.character_pos[1]),
            self.calculate_coords(self.character_pos[0], self.character_pos[1] + 1),
        ];

        for [x, y] in points {
            let Some((x, y)) = painter.get_point(x, y) else {
                continue;
            };
            painter.paint(x, y, Color::Yellow);
        }
    }
}
