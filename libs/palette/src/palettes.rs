use crate::{
    color::Color, simple_palette::SimplePalette, weighted_palette::WeightedPalette, Palette,
};

pub struct Palettes;

impl Palettes {
    pub fn orange_autumn() -> (Color, Box<dyn Palette>) {
        let background = Color::Hex("#181D31");
        let colors = WeightedPalette::new(vec![
            (Color::Hex("#E1B31E"), 3),
            (Color::Hex("#678983"), 1),
            (Color::Hex("#FB5252"), 1),
            (Color::Hex("#F0E9D2"), 2),
            (Color::Hex("#E6DDC4"), 2),
        ]);

        (background, Box::new(colors))
    }

    pub fn peaches_and_cream() -> (Color, Box<dyn Palette>) {
        let background = Color::Hex("#EAA984");
        let colors = SimplePalette::new(vec![
            Color::Hex("#CBCBE5"),
            Color::Hex("#EAD5C9"),
            Color::Hex("#C4594A"),
            Color::Hex("#8786BF"),
        ]);

        (background, Box::new(colors))
    }
}
