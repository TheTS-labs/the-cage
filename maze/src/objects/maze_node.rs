use std::cell::RefCell;
use std::rc::Rc;

use maze_gen::types::Walls;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Context, Painter, Shape};

use crate::app::ForbiddenPoints;
use crate::traits::debug_lines::DebugLines;
use crate::traits::object::Object;
use crate::traits::on_event::OnEvent;
use crate::traits::pixel_coords::PixelCoords;

#[derive(Clone, Debug)]
pub struct MazeNode {
    pub walls:            u8,
    pub x:                u16,
    pub y:                u16,
    pub size:             u16,
    pub forbidden_points: Rc<RefCell<ForbiddenPoints>>,
    pub color:            Color,
}

impl Object for MazeNode {
    fn draw(&mut self, ctx: &mut Context, _forbidden_points: Rc<RefCell<ForbiddenPoints>>) { ctx.draw(self) }
}
impl DebugLines for MazeNode {}
impl OnEvent for MazeNode {}
impl PixelCoords for MazeNode {}

impl Shape for MazeNode {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        if Walls::is_bottom(self.walls) {
            for x in self.x..(self.x + self.size) {
                {
                    self.forbidden_points.borrow_mut().insert((x, self.y));
                }
                let Some((x, y)) = painter.get_point(self.transform_pixel_x(x), self.transform_pixel_y(self.y)) else {
                    continue;
                };
                painter.paint(x, y, self.color);
            }
        }

        if Walls::is_left(self.walls) {
            for y in self.y..(self.y + self.size) {
                {
                    self.forbidden_points.borrow_mut().insert((self.x, y));
                }
                let Some((x, y)) = painter.get_point(self.transform_pixel_x(self.x), self.transform_pixel_y(y)) else {
                    continue;
                };
                painter.paint(x, y, self.color);
            }
        }

        if Walls::is_right(self.walls) {
            for y in self.y..(self.y + self.size) {
                {
                    self.forbidden_points.borrow_mut().insert((self.x + self.size, y));
                }
                let Some((x, y)) =
                    painter.get_point(self.transform_pixel_x(self.x + self.size), self.transform_pixel_y(y))
                else {
                    continue;
                };
                painter.paint(x, y, self.color);
            }
        }

        if Walls::is_top(self.walls) {
            for x in self.x..(self.x + self.size + 1) {
                {
                    self.forbidden_points.borrow_mut().insert((x, self.y + self.size));
                }
                let Some((x, y)) =
                    painter.get_point(self.transform_pixel_x(x), self.transform_pixel_y(self.y + self.size))
                else {
                    continue;
                };
                painter.paint(x, y, self.color);
            }
        }
    }
}
