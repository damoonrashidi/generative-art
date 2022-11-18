use crate::{color::Color, Palette};
use rand::{distributions::WeightedIndex, prelude::Distribution};

pub struct WeightedPalette {
    colors: Vec<(Color, usize)>,
}

impl WeightedPalette {
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
