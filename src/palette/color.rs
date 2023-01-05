use std::fmt::Display;

/// Color representation
#[derive(Clone, Copy, Debug)]
pub enum Color {
    /**
    HSLa representation of a color. Good for when you want to do small variations to a given color.

    Example:

    ```
    use generative_art::palette::color::Color::HSLa;
    let bright_red = HSLa(0, 50.0, 65.0, 1.0);
    ```
    */
    HSLa(u16, f64, f64, f64),

    /**
    Hex representation for a color.

    Example:

    ```
    use generative_art::palette::color::Color::Hex;
    let bright_red = Hex("#f00");
    ```
    */
    Hex(&'static str),
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Hex(color) => write!(f, "{color}"),
            Color::HSLa(h, s, l, a) => {
                write!(f, "hsla({h}, {s:.1}%, {l:.1}%, {a:.2})")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Color;

    #[test]
    fn render_hsla() {
        let color = Color::HSLa(360, 0.0, 0.0, 1.0);

        assert_eq!("hsla(360, 0.0%, 0.0%, 1.00)", color.to_string());
    }

    #[test]
    fn render_hex() {
        let color = Color::Hex("#111");
        assert_eq!("#111", color.to_string());
    }
}
