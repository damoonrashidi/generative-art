use super::Shape;
use crate::{palette::Color, point::Point};

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,

    pub color: Option<Color>,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
            color: Rectangle::default().color,
        }
    }

    pub fn center(&self) -> Point {
        return Point {
            x: self.x,
            y: self.y,
        };
    }
}

impl Shape for Rectangle {
    fn as_svg(&self) -> String {
        let fill = match self.color {
            Some(color) => format!(" fill=\"{}\"", color),
            _ => String::from(""),
        };

        format!(
            "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\"{}/>",
            self.x, self.y, self.width, self.height, fill
        )
    }

    fn contains(&self, point: super::point::Point) -> bool {
        (self.x..(self.x + self.width)).contains(&point.x)
            && (self.y..(self.y + self.height)).contains(&point.y)
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            color: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{point::Point, Shape};

    use super::Rectangle;

    #[test]
    fn does_not_contain() {
        let rect = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 20.0,
            height: 20.0,
            color: None,
        };

        let point = Point { x: 10.0, y: 30.0 };

        assert_eq!(rect.contains(point), false);
    }
}
