use self::color::Color;

pub trait Palette {
    /// Get a random color from the given palette
    fn get_random_color(&self) -> Option<Color>;
}

pub mod color;
pub mod palettes;
pub mod simple_palette;
pub mod weighted_palette;
