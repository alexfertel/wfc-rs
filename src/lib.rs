mod direction;
mod pattern;
mod table;
mod test_utils;
mod wfc;

pub use self::wfc::Wfc;
pub use pattern::get_patterns;

type Image = image::ImageBuffer<image::Rgb<u8>, Vec<u8>>;

pub struct Config {
    pub pattern_size: usize,
    pub width: usize,
    pub height: usize,
}

pub fn generate(image: Image, cfg: Config) -> Image {
    let patterns = pattern::get_patterns(&image, cfg.pattern_size);
    let patterns = patterns.iter().collect();
    let solver = wfc::Wfc::new(patterns);
    solver.generate(cfg.width as u32, cfg.height as u32)
}
