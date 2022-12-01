use std::fmt::Display;

/// Color representation
#[derive(Clone, Copy, Debug)]
pub enum Color {
    /**
    HSLa representation of a color. Good for when you want to do small variations to a given color.

    Example:

    ```
    let bright_red = HSLa((0, 50.0, 65.0, 1.0));
    ```
    */
    HSLa((u16, f64, f64, f64)),

    /**
    Hex representation for a color.

    Example:

    ```
    let bright_red = Hex("#f00");
    ```
    */
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
