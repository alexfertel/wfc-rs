use std::{collections::HashSet, path::PathBuf};

use clap::Parser;
use image::ImageResult;
use itertools::iproduct;

mod direction;
mod pattern;

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
    let patterns = get_patterns(&image, args.size);
    let constraints = build_constraints(&patterns);

    Ok(())
}

fn get_patterns(image: &Image, size: usize) -> HashSet<pattern::Pattern> {
    let mut patterns = HashSet::with_capacity(size * size);

    for x in 0..image.width() {
        for y in 0..image.width() {
            let pattern = pattern::Pattern::new(image, size).from_pos((x, y));
            patterns.insert(pattern);
        }
    }

    patterns
}

fn build_constraints(patterns: &HashSet<pattern::Pattern>) -> Vec<bool> {
    let mut constraints = Vec::with_capacity(patterns.len() * patterns.len() * 4);
    for (p1, p2, d) in iproduct!(
        patterns.iter(),
        patterns.iter(),
        direction::Direction::all()
    ) {
        let allowed = p1.check_overlap(p2, &d);
        constraints.push(allowed);
    }

    constraints
}
