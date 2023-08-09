use std::{fmt::Debug, ops::Index};

use image::Rgb;

use crate::Image;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

impl From<Rgb<u8>> for Color {
    fn from(value: Rgb<u8>) -> Self {
        Color::new(value[0], value[1], value[2])
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Pattern<'p> {
    image: &'p Image,

    pub pixels: Vec<Color>,
    pub size: usize,
}

impl Debug for Pattern<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, pixel) in self.pixels.iter().enumerate() {
            write!(f, "{:?}", pixel)?;

            if i % self.size == 3 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

impl<'p> Pattern<'p> {
    pub fn new(image: &'p Image, size: usize) -> Self {
        Pattern {
            image,
            pixels: Vec::with_capacity(size),
            size,
        }
    }

    pub fn from_pos(mut self, pos: (u32, u32)) -> Self {
        for dx in 0..self.size {
            for dy in 0..self.size {
                let mut x = pos.0.saturating_add(dx as u32);
                if x >= self.image.width() {
                    x = 0;
                }

                let mut y = pos.1.saturating_add(dy as u32);
                if y >= self.image.height() {
                    y = 0;
                }

                let pixel = self.image[(x, y)];
                self.pixels.push(pixel.into());
            }
        }

        self
    }

    pub fn get_side(&self, direction: &Direction) -> Vec<Color> {
        let mut pixels = Vec::with_capacity(self.size * (self.size - 1));
        match direction {
            Direction::Up => {
                for x in 0..self.size {
                    for y in 0..self.size - 1 {
                        let pixel = self[(x as u32, y as u32)];
                        pixels.push(pixel.into());
                    }
                }
            }
            Direction::Right => {
                for x in 1..self.size {
                    for y in 0..self.size {
                        let pixel = self[(x as u32, y as u32)];
                        pixels.push(pixel.into());
                    }
                }
            }
            Direction::Down => {
                for x in 0..self.size {
                    for y in 1..self.size {
                        let pixel = self[(x as u32, y as u32)];
                        pixels.push(pixel.into());
                    }
                }
            }
            Direction::Left => {
                for x in 0..self.size - 1 {
                    for y in 0..self.size {
                        let pixel = self[(x as u32, y as u32)];
                        pixels.push(pixel.into());
                    }
                }
            }
        }

        pixels
    }
}

impl Index<(u32, u32)> for Pattern<'_> {
    type Output = Color;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        let i = index.0 as usize * self.size + index.1 as usize;
        &self.pixels[i]
    }
}

impl Index<(usize, usize)> for Pattern<'_> {
    type Output = Color;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let i = index.0 * self.size + index.1;
        &self.pixels[i]
    }
}

impl Index<(u8, u8)> for Pattern<'_> {
    type Output = Color;

    fn index(&self, index: (u8, u8)) -> &Self::Output {
        let i = index.0 as usize * self.size + index.1 as usize;
        &self.pixels[i]
    }
}

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
