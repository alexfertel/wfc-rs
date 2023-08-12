use std::{collections::HashSet, path::PathBuf};

use clap::Parser;
use image::ImageResult;
use itertools::iproduct;

mod direction;
mod pattern;
mod table;
mod wfc;

type Image = image::ImageBuffer<image::Rgb<u8>, Vec<u8>>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    /// The texture to process.
    texture: PathBuf,
    /// The pattern (kernel) size.
    size: usize,
}

fn main() -> ImageResult<()> {
    let args = Cli::parse();
    let image = image::open(&args.texture)?.to_rgb8();
    let pattern_set = get_patterns(&image, args.size);
    let patterns = pattern_set.iter().collect();
    let ctable = build_constraints(&patterns);
    let solver = wfc::Wfc::new(patterns);

    solver.generate(ctable, 10, 10);

    Ok(())
}

fn get_patterns(image: &Image, size: usize) -> HashSet<pattern::Pattern> {
    let mut patterns = HashSet::with_capacity(size * size);

    for x in 0..image.width() {
        for y in 0..image.width() {
            let id = patterns.len();
            let pattern = pattern::Pattern::new(id, size, image).from_pos((x, y));
            patterns.insert(pattern);
        }
    }

    patterns
}

fn build_constraints<'p>(patterns: &Vec<&'p pattern::Pattern>) -> table::Table<[bool; 4]> {
    let directions = direction::Direction::all();
    let mut ctable = Vec::with_capacity(patterns.len() * patterns.len() * directions.len());
    for (p1, p2) in iproduct!(patterns.iter(), patterns.iter()) {
        let mut row = [false; 4];
        for (i, d) in directions.iter().enumerate() {
            row[i] = p1.overlaps(p2, d);
        }
        ctable.push(row);
    }

    table::Table::new(ctable, patterns.len())
}
