use ratatui::crossterm::event::KeyCode;

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryInto<Direction> for KeyCode {
    type Error = ();

    fn try_into(self) -> Result<Direction, Self::Error> {
        match self {
            KeyCode::Char('w') => Ok(Direction::Up),
            KeyCode::Char('a') => Ok(Direction::Left),
            KeyCode::Char('s') => Ok(Direction::Down),
            KeyCode::Char('d') => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}