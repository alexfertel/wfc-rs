#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
        ]
    }

    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

impl From<(i32, i32)> for Direction {
    fn from(value: (i32, i32)) -> Self {
        match value {
            (0, -1) => Direction::Up,
            (1, 0) => Direction::Right,
            (0, 1) => Direction::Down,
            (-1, 0) => Direction::Left,
            _ => panic!("Invalid direction"),
        }
    }
}

impl From<(i8, i8)> for Direction {
    fn from(value: (i8, i8)) -> Self {
        match value {
            (0, -1) => Direction::Up,
            (1, 0) => Direction::Right,
            (0, 1) => Direction::Down,
            (-1, 0) => Direction::Left,
            _ => panic!("Invalid direction"),
        }
    }
}
