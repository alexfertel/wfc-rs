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
        (idx % self.width, idx / self.width)
    }

    pub fn get_neighbors(&mut self, (x, y): (usize, usize)) -> Vec<(&mut T, Direction)> {
        let mut neighbors = Vec::with_capacity(4);

        if x > 0 {
            neighbors.push((&mut self[(x - 1, y)], Direction::Left));
        } else if y > 0 {
            neighbors.push((&mut self[(x, y - 1)], Direction::Up));
        } else if x < self.width - 1 {
            neighbors.push((&mut self[(x + 1, y)], Direction::Right));
        } else if y < self.height() - 1 {
            neighbors.push((&mut self[(x, y + 1)], Direction::Down));
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
        self.collection.index(x + y * self.width)
    }
}

impl<T> IndexMut<(usize, usize)> for Table<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.collection.index_mut(x + y * self.width)
    }
}
