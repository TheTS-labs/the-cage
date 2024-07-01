pub enum Walls {
    Left   = 1 << 0,
    Right  = 1 << 1,
    Top    = 1 << 2,
    Bottom = 1 << 3,
}

impl Walls {
    pub fn is_left(value: u8) -> bool { value & Walls::Left as u8 > 0 }

    pub fn is_right(value: u8) -> bool { value & Walls::Right as u8 > 0 }

    pub fn is_top(value: u8) -> bool { value & Walls::Top as u8 > 0 }

    pub fn is_bottom(value: u8) -> bool { value & Walls::Bottom as u8 > 0 }
}

pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn as_wall(&self) -> Walls {
        match self {
            Direction::Up => Walls::Top,
            Direction::Down => Walls::Bottom,
            Direction::Left => Walls::Left,
            Direction::Right => Walls::Right,
        }
    }

    pub fn as_opposite_wall(&self) -> Walls {
        match self {
            Direction::Up => Walls::Bottom,
            Direction::Down => Walls::Top,
            Direction::Left => Walls::Right,
            Direction::Right => Walls::Left,
        }
    }
}
