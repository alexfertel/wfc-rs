use std::collections::HashSet;
use std::fmt::Display;
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
    pub fn new(id: usize, size: usize, texture: &'p Image, pos: (u32, u32)) -> Self {
        let pixels = Vec::with_capacity(size * size);

        Pattern {
            id,
            texture,
            pixels,
            size,
        }
        .from_pos(pos)
    }

    /// Creates a pattern from a position in the texture.
    ///
    /// This means taking a square of pixels from the texture, starting at the
    /// given position, and adding them to the pattern. The starting position
    /// is the top-left corner of the square.
    fn from_pos(mut self, pos: (u32, u32)) -> Self {
        for dx in 0..self.size {
            for dy in 0..self.size {
                let x = pos.0.wrapping_add(dx as u32) % self.texture.height();
                let y = pos.1.wrapping_add(dy as u32) % self.texture.width();

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
                for x in 0..self.size - 1 {
                    for y in 0..self.size {
                        let pixel = self[(x, y)];
                        pixels.push(pixel.into());
                    }
                }
            }
            Direction::Right => {
                for x in 0..self.size {
                    for y in 1..self.size {
                        let pixel = self[(x, y)];
                        pixels.push(pixel.into());
                    }
                }
            }
            Direction::Down => {
                for x in 1..self.size {
                    for y in 0..self.size {
                        let pixel = self[(x, y)];
                        pixels.push(pixel.into());
                    }
                }
            }
            Direction::Left => {
                for x in 0..self.size {
                    for y in 0..self.size - 1 {
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
        write!(f, "[")?;
        for i in 0..self.pixels.len() {
            if i % self.size == 0 && i != 0 {
                write!(f, "\n")?;
            }

            let idx = (i % self.size) * self.size + i / self.size;
            let pixel = self.pixels[idx];
            write!(f, "{:?}", pixel)?;
        }
        write!(f, "]")?;

        Ok(())
    }
}

impl Display for Pattern<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.pixels.len() {
            if i % self.size == 0 && i != 0 {
                write!(f, "\n")?;
            }

            let idx = (i % self.size) * self.size + i / self.size;
            let pixel = self.pixels[idx];
            write!(f, "{:?}", pixel)?;
        }
        write!(f, "]")?;

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

pub fn get_patterns(image: &Image, size: usize) -> HashSet<Pattern> {
    let mut patterns = HashSet::with_capacity(size * size);

    for x in 0..image.height() {
        for y in 0..image.width() {
            let id = patterns.len();
            let pattern = Pattern::new(id, size, image, (x, y));
            patterns.insert(pattern);
        }
    }

    patterns
}

#[cfg(test)]
mod tests {
    use image::{Rgb, RgbImage};
    use pretty_assertions::assert_eq;

    use crate::{
        direction::Direction,
        test_utils::{c, img, p},
    };

    #[test]
    fn new() {
        // [0, 1]
        // [2, 3]
        let texture = img(2);
        let pattern = p(0, 2, &texture, (0, 0));
        assert_eq!(pattern.pixels.len(), 4);
        assert_eq!(pattern.pixels, vec![c(0), c(1), c(2), c(3)]);

        let pattern = p(1, 2, &texture, (0, 1));
        assert_eq!(pattern.pixels.len(), 4);
        assert_eq!(pattern.pixels, vec![c(1), c(0), c(3), c(2)]);

        // [0, 1, 2, 3]
        // [4, 5, 6, 7]
        // [8, 9, 10, 11]
        // [12, 13, 14, 15]
        let texture = img(4);
        let pattern = p(0, 2, &texture, (0, 0));
        assert_eq!(pattern.pixels.len(), 4);
        assert_eq!(pattern.pixels, vec![c(0), c(1), c(4), c(5)]);

        let pattern = p(1, 2, &texture, (0, 1));
        assert_eq!(pattern.pixels.len(), 4);
        assert_eq!(pattern.pixels, vec![c(1), c(2), c(5), c(6)]);

        let pattern = p(2, 3, &texture, (3, 3));
        assert_eq!(pattern.pixels.len(), 9);
        assert_eq!(
            pattern.pixels,
            vec![c(15), c(12), c(13), c(3), c(0), c(1), c(7), c(4), c(5)]
        );

        let pattern = p(3, 1, &texture, (0, 0));
        assert_eq!(pattern.pixels.len(), 1);
        assert_eq!(pattern.pixels, vec![c(0)]);

        let pattern = p(4, 2, &texture, (3, 1));
        assert_eq!(pattern.pixels.len(), 4);
        assert_eq!(pattern.pixels, vec![c(13), c(14), c(1), c(2)]);
    }

    #[test]
    fn get_side() {
        // [0, 1, 2, 3]
        // [4, 5, 6, 7]
        // [8, 9, 10, 11]
        // [12, 13, 14, 15]
        let texture = img(4);
        let pattern = p(0, 2, &texture, (0, 0));
        assert_eq!(pattern.pixels, vec![c(0), c(1), c(4), c(5)]);
        assert_eq!(pattern.get_side(&Direction::Up), vec![c(0), c(1)]);
        assert_eq!(pattern.get_side(&Direction::Right), vec![c(1), c(5)]);
        assert_eq!(pattern.get_side(&Direction::Down), vec![c(4), c(5)]);
        assert_eq!(pattern.get_side(&Direction::Left), vec![c(0), c(4)]);

        let pattern = p(1, 3, &texture, (3, 3));
        assert_eq!(
            pattern.pixels,
            vec![c(15), c(12), c(13), c(3), c(0), c(1), c(7), c(4), c(5)]
        );
        assert_eq!(
            pattern.get_side(&Direction::Up),
            vec![c(15), c(12), c(13), c(3), c(0), c(1)]
        );
        assert_eq!(
            pattern.get_side(&Direction::Right),
            vec![c(12), c(13), c(0), c(1), c(4), c(5)]
        );
        assert_eq!(
            pattern.get_side(&Direction::Down),
            vec![c(3), c(0), c(1), c(7), c(4), c(5)]
        );
        assert_eq!(
            pattern.get_side(&Direction::Left),
            vec![c(15), c(12), c(3), c(0), c(7), c(4)]
        );
    }

    #[test]
    fn overlaps() {
        // [0, 1, 2, 3]
        // [1, 2, 3, 4]
        // [2, 3, 4, 5]
        // [3, 4, 5, 6]
        let mut texture = RgbImage::new(4, 4);
        for x in 0..4 {
            for y in 0..4 {
                texture.put_pixel(x, y, Rgb([(x + y) as u8, 0, 0]));
            }
        }

        let p1 = p(0, 2, &texture, (0, 0));
        let p2 = p(1, 2, &texture, (1, 0));
        assert!(p1.overlaps(&p2, &Direction::Right));
        assert!(p1.overlaps(&p2, &Direction::Down));
        assert!(!p1.overlaps(&p2, &Direction::Left));
        assert!(!p1.overlaps(&p2, &Direction::Up));

        let p1 = p(2, 2, &texture, (0, 1));
        let p2 = p(3, 2, &texture, (2, 0));
        assert!(p1.overlaps(&p2, &Direction::Right));
        assert!(p1.overlaps(&p2, &Direction::Down));
        assert!(!p1.overlaps(&p2, &Direction::Left));
        assert!(!p1.overlaps(&p2, &Direction::Up));

        let p1 = p(4, 2, &texture, (1, 2));
        let p2 = p(5, 2, &texture, (2, 0));
        assert!(p1.overlaps(&p2, &Direction::Up));
        assert!(p1.overlaps(&p2, &Direction::Left));
        assert!(!p1.overlaps(&p2, &Direction::Down));
        assert!(!p1.overlaps(&p2, &Direction::Right));
    }

    #[test]
    fn get_patterns() {
        // [0, 3, 6]
        // [1, 4, 7]
        // [2, 5, 8]
        let texture = img(3);
        let patterns = super::get_patterns(&texture, 2);
        assert_eq!(patterns.len(), 9);
        let expected = vec![
            p(0, 2, &texture, (0, 0)),
            p(1, 2, &texture, (1, 0)),
            p(2, 2, &texture, (2, 0)),
            p(3, 2, &texture, (0, 1)),
            p(4, 2, &texture, (1, 1)),
            p(5, 2, &texture, (2, 1)),
            p(6, 2, &texture, (0, 2)),
            p(7, 2, &texture, (1, 2)),
            p(8, 2, &texture, (2, 2)),
        ];
        for pattern in expected {
            assert!(patterns.contains(&pattern));
        }

        // [0, 0, 0]
        // [0, 0, 0]
        // [0, 0, 0]
        let mut texture = RgbImage::new(3, 3);
        for x in 0..3 {
            for y in 0..3 {
                texture.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }
        let patterns = super::get_patterns(&texture, 2);
        assert_eq!(patterns.len(), 1);
        assert!(patterns.contains(&p(0, 2, &texture, (0, 0))));
    }
}
