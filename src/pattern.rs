use std::hash::{Hash, Hasher};
use std::{fmt::Debug, ops::Index};

use image::Rgb;

use crate::direction::Direction;
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

    pub fn to_slice(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<Rgb<u8>> for Color {
    fn from(value: Rgb<u8>) -> Self {
        Color::new(value[0], value[1], value[2])
    }
}

#[derive(Clone)]
pub struct Pattern<'p> {
    /// The input texture.
    texture: &'p Image,
    /// The pattern ID.
    pub id: usize,
    pub pixels: Vec<Color>,
    pub size: usize,
}

impl<'p> Pattern<'p> {
    pub fn new(id: usize, size: usize, texture: &'p Image) -> Self {
        Pattern {
            id,
            texture,
            pixels: Vec::with_capacity(size),
            size,
        }
    }

    /// Creates a pattern from a position in the texture.
    ///
    /// This means taking a square of pixels from the texture, starting at the
    /// given position, and adding them to the pattern. The starting position
    /// is the top-left corner of the square.
    pub fn from_pos(mut self, pos: (u32, u32)) -> Self {
        for dx in 0..self.size {
            for dy in 0..self.size {
                let mut x = pos.0.saturating_add(dx as u32);
                if x >= self.texture.width() {
                    x = 0;
                }

                let mut y = pos.1.saturating_add(dy as u32);
                if y >= self.texture.height() {
                    y = 0;
                }

                let pixel = self.texture[(x, y)];
                self.pixels.push(pixel.into());
            }
        }

        self
    }

    /// Returns the pixels of the pattern that constitute the side in the given
    /// direction.
    ///
    /// This means that the pixels returned are the ones that are on the side
    /// of the pattern that is facing the given direction. For example, if the
    /// direction is `Up`, then the pixels returned are all the pixels except
    /// the bottom row.
    pub fn get_side(&self, direction: &Direction) -> Vec<Color> {
        let mut pixels = Vec::with_capacity(self.size * (self.size - 1));
        match direction {
            Direction::Up => {
                for x in 0..self.size {
                    for y in 0..self.size - 1 {
                        let pixel = self[(x, y)];
                        pixels.push(pixel.into());
                    }
                }
            }
            Direction::Right => {
                for x in 1..self.size {
                    for y in 0..self.size {
                        let pixel = self[(x, y)];
                        pixels.push(pixel.into());
                    }
                }
            }
            Direction::Down => {
                for x in 0..self.size {
                    for y in 1..self.size {
                        let pixel = self[(x, y)];
                        pixels.push(pixel.into());
                    }
                }
            }
            Direction::Left => {
                for x in 0..self.size - 1 {
                    for y in 0..self.size {
                        let pixel = self[(x, y)];
                        pixels.push(pixel.into());
                    }
                }
            }
        }

        pixels
    }

    /// Checks whether the pattern overlaps with the given pattern in the given
    /// direction.
    ///
    /// This answers whether a given pattern can be put next to another pattern.
    pub fn overlaps(&self, p2: &Pattern, direction: &Direction) -> bool {
        let side1 = self.get_side(direction);
        let side2 = p2.get_side(&direction.opposite());

        side1 == side2
    }
}

impl Hash for Pattern<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pixels.hash(state);
    }
}

impl PartialEq for Pattern<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.pixels == other.pixels
    }
}

impl Eq for Pattern<'_> {}

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
