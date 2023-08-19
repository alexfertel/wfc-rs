use std::path::PathBuf;

use clap::Parser;
use image::ImageResult;

use wfc::{generate, Config};

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    /// The texture to process.
    texture: PathBuf,
    /// The pattern (kernel) size.
    size: usize,
    /// The width of the output image.
    #[arg(short = 'w', long = "width", default_value = "10")]
    width: usize,
    /// The height of the output image.
    #[arg(short = 'h', long = "height", default_value = "10")]
    height: usize,
}

fn main() -> ImageResult<()> {
    let args = Cli::parse();
    let image = image::open(&args.texture)?.to_rgb8();

    generate(
        image,
        Config {
            pattern_size: args.size,
            width: args.width,
            height: args.height,
        },
    );

    Ok(())
}
