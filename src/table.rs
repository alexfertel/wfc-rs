use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use crate::direction::Direction;

#[derive(Debug)]
pub struct Table<T> {
    collection: Vec<T>,
    width: usize,
}

impl<T> Table<T> {
    pub fn new(collection: Vec<T>, width: usize) -> Self {
        Table { collection, width }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.collection.len() / self.width
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.collection.iter()
    }

    pub fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx / self.height(), idx % self.width())
    }

    pub fn get_neighbors(&self, (x, y): (usize, usize)) -> Vec<(&T, Direction)> {
        let mut neighbors = Vec::with_capacity(4);

        for d in Direction::all() {
            let (dx, dy) = d.add_pos((x as i32, y as i32));
            if dx < 0 || dy < 0 || dx >= self.width() as i32 || dy >= self.height() as i32 {
                continue;
            }

            neighbors.push((&self[(dx as usize, dy as usize)], d));
        }

        neighbors
    }

    pub fn len(&self) -> usize {
        self.collection.len()
    }
}

impl<T> Index<usize> for Table<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.collection.index(index)
    }
}

impl<T> IndexMut<usize> for Table<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.collection.index_mut(index)
    }
}

impl<T> Index<(usize, usize)> for Table<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.collection.index(x * self.width() + y)
    }
}

impl<T> IndexMut<(usize, usize)> for Table<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.collection.index_mut(x * self.width() + y)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::direction::Direction;

    use super::Table;

    #[test]
    fn basic_properties() {
        // [0, 3, 6]
        // [1, 4, 7]
        // [2, 5, 8]
        let table = Table::new((0..9).collect_vec(), 3);
        assert_eq!(table.width(), 3);
        assert_eq!(table.height(), 3);
        assert_eq!(table.len(), 9);
        assert_eq!(table.idx_to_pos(0), (0, 0));
        assert_eq!(table.idx_to_pos(1), (0, 1));
        assert_eq!(table.idx_to_pos(2), (0, 2));
        assert_eq!(table.idx_to_pos(3), (1, 0));
        assert_eq!(table.idx_to_pos(4), (1, 1));
    }

    #[test]
    fn indexing() {
        // [0, 3, 6]
        // [1, 4, 7]
        // [2, 5, 8]
        let table = Table::new((0..9).collect_vec(), 3);
        assert_eq!(table[0], 0);
        assert_eq!(table[4], 4);
        assert_eq!(table[(0, 0)], 0);
        assert_eq!(table[(0, 1)], 1);
        assert_eq!(table[(1, 1)], 4);
    }

    #[test]
    fn get_neighbors() {
        // [0, 3, 6]
        // [1, 4, 7]
        // [2, 5, 8]
        let table = Table::new((0..9).collect_vec(), 3);
        let neighbors = table.get_neighbors((0, 0));
        assert!(neighbors.contains(&(&3, Direction::Right)));
        assert!(neighbors.contains(&(&1, Direction::Down)));

        let neighbors = table.get_neighbors((1, 1));
        dbg!(&neighbors);
        assert!(neighbors.contains(&(&1, Direction::Left)));
        assert!(neighbors.contains(&(&7, Direction::Right)));
        assert!(neighbors.contains(&(&3, Direction::Up)));
        assert!(neighbors.contains(&(&5, Direction::Down)));

        let neighbors = table.get_neighbors((2, 2));
        assert!(neighbors.contains(&(&5, Direction::Left)));
        assert!(neighbors.contains(&(&7, Direction::Up)));

        let neighbors = table.get_neighbors((2, 1));
        assert!(neighbors.contains(&(&4, Direction::Left)));
        assert!(neighbors.contains(&(&6, Direction::Up)));
        assert!(neighbors.contains(&(&8, Direction::Down)));
    }
}
