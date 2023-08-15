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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite() {
        assert_eq!(Direction::Up.opposite(), Direction::Down);
        assert_eq!(Direction::Down.opposite(), Direction::Up);
        assert_eq!(Direction::Right.opposite(), Direction::Left);
        assert_eq!(Direction::Left.opposite(), Direction::Right);
    }

    #[test]
    fn add_to_pos() {
        assert_eq!(Direction::Up.add_pos((0, 0)), (0, -1));
        assert_eq!(Direction::Right.add_pos((0, 0)), (1, 0));
        assert_eq!(Direction::Down.add_pos((0, 0)), (0, 1));
        assert_eq!(Direction::Left.add_pos((0, 0)), (-1, 0));
    }

    #[test]
    fn conversions() {
        assert_eq!(Direction::from((0, -1)), Direction::Up);
        assert_eq!(Direction::from((1, 0)), Direction::Right);
        assert_eq!(Direction::from((0, 1)), Direction::Down);
        assert_eq!(Direction::from((-1, 0)), Direction::Left);

        assert_eq!(Direction::from(0), Direction::Up);
        assert_eq!(Direction::from(1), Direction::Right);
        assert_eq!(Direction::from(2), Direction::Down);
        assert_eq!(Direction::from(3), Direction::Left);

        assert_eq!(usize::from(Direction::Up), 0);
        assert_eq!(usize::from(Direction::Right), 1);
        assert_eq!(usize::from(Direction::Down), 2);
        assert_eq!(usize::from(Direction::Left), 3);
    }

    #[test]
    fn all() {
        assert_eq!(
            Direction::all(),
            [
                Direction::Up,
                Direction::Down,
                Direction::Right,
                Direction::Left
            ]
        );
    }
}
