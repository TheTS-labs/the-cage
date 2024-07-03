pub mod debug_lines;
pub mod object;
pub mod on_event;

use maze_gen::MazeGen;
use ratatui::layout::Rect;
use ratatui::symbols::Marker;

use crate::objects::character::Character;
use crate::traits::pixel_coords::PixelCoords;

pub const MAZE_NODE_SIZE: u16 = 3;

#[derive(Clone, Debug)]
pub struct Maze {
    pub marker:            Marker,
    pub character:         Character,
    pub area:              Rect,
    pub maze:              MazeGen,
}

impl PixelCoords for Maze {}

impl Maze {
    pub fn new(maze_size: usize, maze_seed: u64) -> Self {
        let mut maze = MazeGen::new(maze_seed, maze_size);
        maze.generate();

        Self {
            marker: Marker::HalfBlock,
            character: Character::new([0, 0], Rect::default()),
            area: Rect::default(),
            maze,
        }
    }
}
