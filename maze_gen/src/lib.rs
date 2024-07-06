use std::collections::VecDeque;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use types::{Direction, Walls};

pub mod types;

#[derive(Clone, Debug)]
pub struct MazeGen {
    pub maze:  Vec<Vec<u8>>,
    pub size:  usize,
    pub seed:  u64,
    stack:     VecDeque<(usize, usize)>,
    visited:   Vec<Vec<bool>>,
    pub rng:   StdRng,
    pub exit:  (usize, usize),
    pub start: (usize, usize),
}

impl MazeGen {
    pub fn new(seed: u64, size: usize) -> Self {
        let init_walls = Walls::Left as u8 | Walls::Right as u8 | Walls::Top as u8 | Walls::Bottom as u8;

        let mut _self = Self {
            maze: vec![vec![init_walls; size]; size],
            size,
            seed,
            stack: VecDeque::new(),
            visited: vec![vec![false; size]; size],
            rng: StdRng::seed_from_u64(seed),
            exit: (0, 0),
            start: (0, 0),
        };

        _self.start = (
            _self.rng.gen_range(0.._self.size / 3),
            _self.rng.gen_range(0.._self.size),
        );
        _self.stack.push_front(_self.start);
        _self.visited[_self.stack[0].1][_self.stack[0].0] = true;

        _self.exit = (_self.size - _self.stack[0].1, _self.size - 1);

        _self.maze[_self.exit.0][_self.exit.1] ^= Direction::Right.as_wall() as u8;

        _self
    }

    pub fn generate(&mut self) -> Option<()> {
        while !self.stack.is_empty() {
            self.run_step()?;
        }

        Some(())
    }

    pub fn run_step(&mut self) -> Option<()> {
        let current_node = self.stack[0];
        let available_adjacent_nodes = self.available_adjacent_nodes(current_node.0, current_node.1);

        if available_adjacent_nodes.is_empty() {
            return self.backtrack();
        }

        let (x, y, direction) = available_adjacent_nodes.choose(&mut self.rng)?;
        self.stack.push_front((*x, *y));
        self.visited[*y][*x] = true;

        self.maze[current_node.1][current_node.0] ^= direction.as_wall() as u8;
        self.maze[*y][*x] ^= direction.as_opposite_wall() as u8;

        Some(())
    }

    fn adjacent_nodes(&self, x: usize, y: usize) -> [Option<(usize, usize, Direction)>; 4] {
        [
            if x > 0 { Some((x - 1, y, Direction::Left)) } else { None },
            if y > 0 { Some((x, y - 1, Direction::Up)) } else { None },
            if x + 1 < self.size {
                Some((x + 1, y, Direction::Right))
            } else {
                None
            },
            if y + 1 < self.size {
                Some((x, y + 1, Direction::Down))
            } else {
                None
            },
        ]
    }

    fn available_adjacent_nodes(&self, x: usize, y: usize) -> Vec<(usize, usize, Direction)> {
        self.adjacent_nodes(x, y)
            .into_iter()
            .flatten()
            .filter(|(x, y, _dir)| !self.visited[*y][*x])
            .collect()
    }

    fn backtrack(&mut self) -> Option<()> {
        while let Some(prev_node) = self.stack.pop_front() {
            let available_adjacent_nodes = self.available_adjacent_nodes(prev_node.0, prev_node.1);

            if !available_adjacent_nodes.is_empty() {
                self.stack.push_front(prev_node);
                return Some(());
            }
        }

        Some(())
    }

    pub fn stack_len(&self) -> usize { self.stack.len() }
}
