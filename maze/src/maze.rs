use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use maze_gen::MazeGen;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::{Canvas, Line};
use ratatui::widgets::Widget;

use crate::shapes::character::Character;
use crate::shapes::maze_node::MazeNode;
use crate::traits::pixel_coords::PixelCoords;

pub const MAZE_NODE_SIZE: u16 = 3;

#[derive(Clone, Debug)]
pub struct Maze {
    pub marker:          Marker,
    pub character:       Character,
    pub area:            Rect,
    pub maze:            MazeGen,
    pub maze_map_points: Rc<RefCell<HashSet<(u16, u16)>>>,
}

impl Maze {
    pub fn new(maze_size: usize, maze_seed: u64) -> Self {
        let drawn_points = Rc::new(RefCell::new(HashSet::new()));

        Self {
            marker:          Marker::HalfBlock,
            character:       Character::new([0, 0], Rect::default(), Rc::clone(&drawn_points)),
            area:            Rect::default(),
            maze:            MazeGen::new(maze_seed, maze_size),
            maze_map_points: Rc::clone(&drawn_points),
        }
    }

    pub fn on_character_move(&mut self, key_code: KeyCode) {
        self.character.move_character(
            key_code
                .try_into()
                .unwrap_or_else(|_| panic!("Invalid character move KeyCode: {:?}", key_code)),
            1,
        );
    }
}

impl PixelCoords for Maze {}

impl Widget for &mut Maze {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.area = area;
        self.character.area = area;

        let maze_size = self.maze.size as u16 * MAZE_NODE_SIZE;
        let x_offset = (area.width - maze_size) / 2;
        let y_offset = ((area.height * 2) - maze_size) / 2;

        if self.maze.stack_len() != 0 {
            let t: &RefCell<HashSet<(u16, u16)>> = self.maze_map_points.borrow();
            t.borrow_mut().clear();
            self.maze.run_step();
        }

        if self.character.character_pos == [0, 0] {
            self.character.character_pos = [
                (self.maze.start.0 as u16 * MAZE_NODE_SIZE) + 2 + x_offset,
                (self.maze.size as u16 - 1) * MAZE_NODE_SIZE - (self.maze.start.1 as u16 * MAZE_NODE_SIZE) +
                    y_offset +
                    MAZE_NODE_SIZE -
                    1,
            ];
        }

        Canvas::default()
            .marker(self.marker)
            .paint(|ctx| {
                for (i, row) in self.maze.maze.iter().rev().enumerate() {
                    for (j, node) in row.iter().enumerate() {
                        ctx.draw(&MazeNode {
                            walls:        *node,
                            x:            (j as u16 * MAZE_NODE_SIZE) + 1 + x_offset,
                            y:            (i as u16 * MAZE_NODE_SIZE) + 1 + y_offset,
                            size:         MAZE_NODE_SIZE,
                            drawn_points: Rc::clone(&self.maze_map_points),
                            color:        Color::White,
                        });
                    }
                }

                // ctx.draw(&MazeNode {
                //     walls:        Walls::Bottom as u8 | Walls::Top as u8 | Walls::Left as u8
                // | Walls::Right as u8,     x:            (self.maze.exit.1 as
                // u16 * MAZE_NODE_SIZE) + 1 + x_offset,     y:
                // (self.maze.size as u16 - 1) * MAZE_NODE_SIZE - (self.maze.exit.0 as u16 *
                // MAZE_NODE_SIZE) + y_offset + 1,     size:
                // MAZE_NODE_SIZE,     drawn_points:
                // Rc::clone(&self.maze_map_points),     color:
                // Color::Red, });

                ctx.draw(&Line::new(
                    self.transform_pixel_x(
                        (self.maze.exit.1 as u16 * MAZE_NODE_SIZE) + 1 + x_offset + MAZE_NODE_SIZE + 2,
                    ),
                    self.transform_pixel_y(
                        (self.maze.size as u16 - 1) * MAZE_NODE_SIZE - (self.maze.exit.0 as u16 * MAZE_NODE_SIZE) +
                            y_offset +
                            1,
                    ),
                    self.transform_pixel_x(
                        (self.maze.exit.1 as u16 * MAZE_NODE_SIZE) + 1 + x_offset + MAZE_NODE_SIZE + 2,
                    ),
                    self.transform_pixel_y(
                        (self.maze.size as u16 - 1) * MAZE_NODE_SIZE - (self.maze.exit.0 as u16 * MAZE_NODE_SIZE) +
                            y_offset +
                            1 +
                            MAZE_NODE_SIZE,
                    ),
                    Color::Green,
                ));

                ctx.draw(&self.character);
            })
            .x_bounds([area.x as f64, (area.x + area.width) as f64])
            .y_bounds([area.y as f64, (area.y + area.height) as f64])
            .render(area, buf);
    }
}
