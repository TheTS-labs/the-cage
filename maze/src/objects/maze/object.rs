use std::cell::RefCell;
use std::rc::Rc;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Context, Line};

use super::{Maze, MAZE_NODE_SIZE};
use crate::app::ForbiddenPoints;
use crate::objects::maze_node::MazeNode;
use crate::traits::object::Object;
use crate::traits::pixel_coords::PixelCoords;

impl Object for Maze {
    fn render(&mut self, area: Rect, _buf: &mut Buffer, _forbidden_points: Rc<RefCell<ForbiddenPoints>>) {
        self.area = area;
        self.character.area = area;

        let maze_size = self.maze.size as u16 * MAZE_NODE_SIZE;
        let x_offset = (area.width - maze_size) / 2;
        let y_offset = ((area.height * 2) - maze_size) / 2;

        if self.character.character_pos == [0, 0] {
            self.character.character_pos = [
                (self.maze.start.0 as u16 * MAZE_NODE_SIZE) + 2 + x_offset,
                (self.maze.size as u16 - 1) * MAZE_NODE_SIZE - (self.maze.start.1 as u16 * MAZE_NODE_SIZE) +
                    y_offset +
                    MAZE_NODE_SIZE -
                    1,
            ];
        }
    }

    fn draw(&mut self, ctx: &mut Context, forbidden_points: Rc<RefCell<ForbiddenPoints>>) {
        let maze_size = self.maze.size as u16 * MAZE_NODE_SIZE;
        let x_offset = (self.area.width - maze_size) / 2;
        let y_offset = ((self.area.height * 2) - maze_size) / 2;

        for (i, row) in self.maze.maze.iter().rev().enumerate() {
            for (j, node) in row.iter().enumerate() {
                MazeNode {
                    walls:            *node,
                    x:                (j as u16 * MAZE_NODE_SIZE) + 1 + x_offset,
                    y:                (i as u16 * MAZE_NODE_SIZE) + 1 + y_offset,
                    size:             MAZE_NODE_SIZE,
                    forbidden_points: Rc::clone(&forbidden_points),
                    color:            Color::White,
                }
                .draw(ctx, Rc::clone(&forbidden_points));
            }
        }

        ctx.draw(&Line::new(
            self.transform_pixel_x((self.maze.exit.1 as u16 * MAZE_NODE_SIZE) + 1 + x_offset + MAZE_NODE_SIZE + 2),
            self.transform_pixel_y(
                (self.maze.size as u16 - 1) * MAZE_NODE_SIZE - (self.maze.exit.0 as u16 * MAZE_NODE_SIZE) + y_offset,
            ),
            self.transform_pixel_x((self.maze.exit.1 as u16 * MAZE_NODE_SIZE) + 1 + x_offset + MAZE_NODE_SIZE + 2),
            self.transform_pixel_y(
                (self.maze.size as u16 - 1) * MAZE_NODE_SIZE - (self.maze.exit.0 as u16 * MAZE_NODE_SIZE) +
                    y_offset +
                    1 +
                    MAZE_NODE_SIZE,
            ),
            Color::Green,
        ));

        self.character.draw(ctx, forbidden_points);
    }
}
