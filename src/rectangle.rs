use super::Shape;
use crate::palette::Color;

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,

    pub color: Option<Color>,
}

impl Rectangle {}

impl Shape for Rectangle {
    fn as_svg(&self) -> String {
        let fill = match self.color {
            Some(color) => format!("{}", color),
            _ => String::from("transparent"),
        };

        format!(
            "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"{}\"/>",
            self.x, self.y, self.width, self.height, fill
        )
    }

    fn contains(&self, point: &super::point::Point) -> bool {
        (self.x..(self.x + self.width)).contains(&point.x)
            && (self.y..(self.y + self.height)).contains(&point.y)
    }
}
