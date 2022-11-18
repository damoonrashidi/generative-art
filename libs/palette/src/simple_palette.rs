use rand::Rng;

use crate::{color::Color, Palette};

pub struct SimplePalette {
    colors: Vec<Color>,
}

impl SimplePalette {
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
