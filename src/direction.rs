#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn add_pos(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match *self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
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

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Invalid direction"),
        }
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}
