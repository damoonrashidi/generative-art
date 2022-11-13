use rand::Rng;

use crate::color::Color;

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
