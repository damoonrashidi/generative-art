use super::{color::Color, Palette};
use rand::{distributions::WeightedIndex, prelude::Distribution};

/// A set of colors where one can be chosen randomly but biased by a given weight
#[derive(Debug)]
pub struct WeightedPalette<const N: usize> {
    colors: [(Color, usize); N],
}

impl<const N: usize> WeightedPalette<N> {
    /**
     Create a new weighted color palette

     Example

     ```

     use generative_art::{palette::{Palette, color::Color, weighted_palette::WeightedPalette}};

     let palette = WeightedPalette::new([
        (Color::Hex("#f00"), 1),
        (Color::Hex("#0f0"), 5),
        (Color::Hex("#00f"), 1)
     ]);

     if let Some(random_color) = palette.get_random_color() {
     // random_color has 5 times as high of a chance to be picked as
     // either red or blue.
     }
     ```

    */
    pub fn new(colors: [(Color, usize); N]) -> Self {
        WeightedPalette { colors }
    }
}

impl<const N: usize> Palette for WeightedPalette<N> {
    fn get_random_color(&self) -> Option<Color> {
        let mut rng = rand::thread_rng();
        let weights = self
            .colors
            .iter()
            .map(|color| color.1)
            .collect::<Vec<usize>>();

        let dist = if let Ok(dist) = WeightedIndex::new(&weights) {
            dist
        } else {
            return None;
        };

        let i = dist.sample(&mut rng);

        match self.colors.len() {
            0 => None,
            _ => Some(self.colors[i].0),
        }
    }
}
