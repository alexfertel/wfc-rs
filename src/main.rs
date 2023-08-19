use std::path::PathBuf;

use clap::Parser;
use image::ImageResult;

use wfc::{generate, Config};

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    /// Path to the texture to process.
    input_texture: PathBuf,
    /// Path to the output texture.
    #[arg(short = 'o', long = "output")]
    output_texture: Option<PathBuf>,
    /// The pattern (kernel) size.
    #[arg(short = 's', long = "size", default_value = "2")]
    size: usize,
    /// The width of the output image.
    #[arg(long = "width", default_value = "10")]
    width: usize,
    /// The height of the output image.
    #[arg(long = "height", default_value = "10")]
    height: usize,
}

fn main() -> ImageResult<()> {
    let args = Cli::parse();
    let image = image::open(&args.input_texture)?.to_rgb8();

    let output = generate(
        image,
        Config {
            pattern_size: args.size,
            width: args.width,
            height: args.height,
        },
    );

    if let Some(path) = args.output_texture {
        output.save(path)?;
    }

    Ok(())
}
