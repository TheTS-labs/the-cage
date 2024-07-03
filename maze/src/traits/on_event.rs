use ratatui::crossterm::event::KeyCode;

pub trait OnEvent {
    fn on_key_pressed(&mut self, key_code: KeyCode) -> Option<KeyCode> { Some(key_code) }
}
