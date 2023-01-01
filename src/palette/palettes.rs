use super::{
    color::Color, simple_palette::SimplePalette, weighted_palette::WeightedPalette, Palette,
};

/// A list of predefined color palettes
#[derive(Debug)]
pub struct Palettes;

impl Palettes {
    /// Vibrant Orange, red, off-white against a dark background
    pub fn orange_autumn() -> (Color, Box<dyn Palette>) {
        let background = Color::Hex("#181D31");
        let colors = WeightedPalette::new([
            (Color::Hex("#E1B31E"), 3),
            (Color::Hex("#678983"), 1),
            (Color::Hex("#FB5252"), 1),
            (Color::Hex("#F0E9D2"), 2),
            (Color::Hex("#E6DDC4"), 2),
        ]);

        (background, Box::new(colors))
    }

    /// Pastelly pinks, orange, red
    pub fn peaches_and_cream() -> (Color, Box<dyn Palette>) {
        let background = Color::Hex("#EAA984");
        let colors = SimplePalette::new([
            Color::Hex("#CBCBE5"),
            Color::Hex("#EAD5C9"),
            Color::Hex("#C4594A"),
            Color::Hex("#8786BF"),
        ]);

        (background, Box::new(colors))
    }

    /// Blue, white, yellow
    pub fn spring_break() -> (Color, Box<dyn Palette>) {
        let background = Color::Hex("#F9F9F9");
        let colors = SimplePalette::new([
            Color::Hex("#ABD2EB"),
            Color::Hex("#5AA9E6"),
            Color::Hex("#DFC232"),
            Color::Hex("#BE2C58"),
        ]);

        (background, Box::new(colors))
    }

    /// black and white against red
    pub fn red_white_black() -> (Color, Box<dyn Palette>) {
        let background = Color::Hex("#EC0000");
        let colors = WeightedPalette::new([
            (Color::Hex("#ffffff"), 2),
            (Color::Hex("#000231"), 1),
            (Color::Hex("#002214"), 1),
        ]);

        (background, Box::new(colors))
    }
}
