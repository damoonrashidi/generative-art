use std::fmt::Display;

use rand::{distributions::WeightedIndex, prelude::Distribution, Rng};

#[derive(Clone, Copy, Debug)]
pub enum Color {
    HSLa((u16, f64, f64, f64)),
    Hex(&'static str),
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Hex(x) => write!(f, "{}", x),
            Color::HSLa((h, s, l, a)) => {
                write!(f, "hsla({}deg, {:.2}%, {:.2}%, {:.2})", h, s, l, a)
            }
        }
    }
}

pub struct Palette {
    colors: Vec<Color>,
}

impl Palette {
    pub fn new(colors: Vec<Color>) -> Palette {
        Palette { colors }
    }

    pub fn get_random_color(&self) -> Option<Color> {
        let mut rng = rand::thread_rng();
        match self.colors.len() {
            0 => None,
            i => Some(self.colors[rng.gen_range(0..i - 1)]),
        }
    }
}

pub struct WeightedPalette {
    colors: Vec<(Color, usize)>,
}

impl WeightedPalette {
    pub fn new(colors: Vec<(Color, usize)>) -> WeightedPalette {
        WeightedPalette { colors }
    }

    pub fn get_random_color(&self) -> Option<Color> {
        let mut rng = rand::thread_rng();
        let weights = self
            .colors
            .clone()
            .into_iter()
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
