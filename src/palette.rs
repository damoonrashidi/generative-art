use std::fmt::Display;

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    HSLa(u16, f64, f64, f64),
    Hex(&'static str),
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Hex(x) => write!(f, "{}", x),
            Color::HSLa(h, s, l, a) => write!(f, "hsla({}deg, {:.2}%, {:.2}%, {:.2})", h, s, l, a),
        }
    }
}

pub struct Palette {
    colors: Vec<Color>,
}

impl Palette {
    pub fn new(&mut self, colors: Vec<Color>) -> &Self {
        self.colors = colors;
        return self;
    }

    pub fn get_random_color(&self) -> Option<Color> {
        let mut rng = rand::thread_rng();
        match self.colors.len() {
            0 => None,
            i => Some(self.colors[rng.gen_range(0..i - 1)]),
        }
    }
}
