use std::path::PathBuf;

use clap::Parser;
use image::ImageResult;
use pattern::get_patterns;
use wfc::build_constraints;

mod direction;
mod pattern;
mod table;
mod test_utils;
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
