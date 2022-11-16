use std::fmt::Display;

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
