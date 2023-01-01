use rand::Rng;

use super::{color::Color, Palette};
/// A set of colors where a color can be chosen randomly
#[derive(Debug)]
pub struct SimplePalette<const N: usize> {
    colors: [Color; N],
}

impl<const N: usize> SimplePalette<N> {
    /**
     Create a new color palette

     Example

     ```
     use generative_art::palette::{Palette, color::Color, simple_palette::SimplePalette};
     let palette = SimplePalette::new([
     Color::Hex("#f00"),
     Color::Hex("#0f0"),
     Color::Hex("#00f")
     ]);

     if let Some(random_color) = palette.get_random_color() {
        // do something with random_color
     }
     ```

    */
    pub fn new(colors: [Color; N]) -> Self {
        SimplePalette { colors }
    }
}

impl<const N: usize> Palette for SimplePalette<N> {
    fn get_random_color(&self) -> Option<Color> {
        let mut rng = rand::thread_rng();
        match self.colors.len() {
            0 => None,
            i => Some(self.colors[rng.gen_range(0..i - 1)]),
        }
    }
}
