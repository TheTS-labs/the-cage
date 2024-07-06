use ratatui::crossterm::event::KeyCode;

use super::Maze;
use crate::traits::on_event::OnEvent;

impl OnEvent for Maze {
    fn on_key_pressed(&mut self, key_code: KeyCode) -> Option<KeyCode> { self.character.on_key_pressed(key_code) }
}
