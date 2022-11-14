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
            Color::Hex(x) => write!(f, "{x}"),
            Color::HSLa((h, s, l, a)) => {
                write!(f, "hsla({h}deg, {s:.2}%, {l:.2}%, {a:.2})")
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
        self.colors
            .get(rand::thread_rng().gen_range(0..self.colors.len()))
            .copied()
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
        if self.colors.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let weights: Vec<_> = self.colors.iter().map(|color| color.1).collect();

        let dist = WeightedIndex::new(&weights).ok()?;
        let i = dist.sample(&mut rng);
        Some(self.colors[i].0)
    }
}
