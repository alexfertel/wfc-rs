use image::{Rgb, RgbImage};

use crate::{
    pattern::{Color, Pattern},
    Image,
};

pub fn c(id: u8) -> Color {
    Color::new(id, 0, 0)
}

pub fn p<'p>(id: usize, size: usize, texture: &'p Image, pos: (u32, u32)) -> Pattern<'p> {
    Pattern::new(id, size, texture, pos)
}

pub fn img(size: u32) -> RgbImage {
    let mut texture = RgbImage::new(size, size);
    let mut count = 0;
    for x in 0..size {
        for y in 0..size {
            texture.put_pixel(x, y, Rgb([count, 0, 0]));
            count += 1;
        }
    }

    texture
}
