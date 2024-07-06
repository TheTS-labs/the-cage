use ratatui::prelude::*;

use super::Maze;
use crate::traits::debug_lines::DebugLines;

impl DebugLines for Maze {
    fn debug_lines(&self) -> Vec<Line> {
        let mut lines = vec![
            Line::from(vec![
                "Area (".bold(),
                "Width".yellow().bold(),
                "/".into(),
                "Height".magenta().bold(),
                "): ".bold(),
                self.area.width.to_string().yellow().bold(),
                " / ".into(),
                self.area.height.to_string().magenta().bold(),
            ]),
            Line::from(vec![
                "Maze (".bold(),
                "Size".cyan().bold(),
                "/".into(),
                "Seed".blue().bold(),
                "/".into(),
                "Won".green().bold(),
                "): ".bold(),
                self.maze.size.to_string().cyan().bold(),
                " / ".into(),
                self.maze.seed.to_string().blue().bold(),
                " / ".into(),
                self.is_won.to_string().green().bold(),
            ]),
        ];

        lines.extend_from_slice(&self.character.debug_lines()[..]);

        lines
    }
}
