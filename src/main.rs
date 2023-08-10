use std::{collections::HashSet, path::PathBuf};

use clap::Parser;
use image::ImageResult;
use itertools::iproduct;

mod ctable;
mod direction;
mod pattern;
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
    let constraints = build_constraints(pattern_set.iter().collect());

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

fn build_constraints<'p>(patterns: Vec<&'p pattern::Pattern>) -> ctable::ConstraintsTable<'p> {
    let directions = direction::Direction::all();
    let mut table = Vec::with_capacity(patterns.len() * patterns.len() * directions.len());
    for (p1, p2, d) in iproduct!(patterns.iter(), patterns.iter(), directions) {
        let allowed = p1.check_overlap(&p2, &d);
        table.push(allowed);
    }

    let ctable = ctable::ConstraintsTable::new(table, patterns);
    ctable
}
