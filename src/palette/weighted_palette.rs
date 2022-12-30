use super::{color::Color, Palette};
use rand::{distributions::WeightedIndex, prelude::Distribution};

/// A set of colors where one can be chosen randomly but biased by a given weight
#[derive(Debug)]
pub struct WeightedPalette {
    colors: Vec<(Color, usize)>,
}

impl WeightedPalette {
    /**
     Create a new weighted color palette

     Example

     ```
     let palette = WeightedPalette::new(vec![
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
    pub fn new(colors: Vec<(Color, usize)>) -> WeightedPalette {
        WeightedPalette { colors }
    }
}

impl Palette for WeightedPalette {
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
