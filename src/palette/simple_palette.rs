use rand::Rng;

use super::{color::Color, Palette};
/// A set of colors where a color can be chosen randomly
#[derive(Debug)]
pub struct SimplePalette {
    colors: Vec<Color>,
}

impl SimplePalette {
    /**
     Create a new color palette

     Example

     ```
     let palette = SimplePalette::new(vec![
     Color::Hex("#f00"),
     Color::Hex("#0f0"),
     Color::Hex("#00f")
     ]);

     if let Some(random_color) = palette.get_random_color() {
     // do something with random_color
     }
     ```

    */
    pub fn new(colors: Vec<Color>) -> SimplePalette {
        SimplePalette { colors }
    }
}

impl Palette for SimplePalette {
    fn get_random_color(&self) -> Option<Color> {
        let mut rng = rand::thread_rng();
        match self.colors.len() {
            0 => None,
            i => Some(self.colors[rng.gen_range(0..i - 1)]),
        }
    }
}
