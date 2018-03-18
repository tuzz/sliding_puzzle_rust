#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn x(&self) -> isize {
        match *self {
            Direction::Left => 1,
            Direction::Right => -1,
            _ => 0,
        }
    }

    pub fn y(&self) -> isize {
        match *self {
            Direction::Up => 1,
            Direction::Down => -1,
            _ => 0,
        }
    }

    pub fn opposite(&self) -> Self {
        match *self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[cfg(test)]
mod test;
