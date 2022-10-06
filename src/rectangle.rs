use std::{fmt::Display, ops::Range};

use super::Shape;
use crate::{palette::Color, point::Point};

#[derive(Debug)]
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

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn x_range(&self) -> Range<f64> {
        return self.x..(self.x + self.width);
    }

    pub fn y_range(&self) -> Range<f64> {
        return self.y..(self.y + self.height);
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
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

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x
            && self.y == other.y
            && self.width == other.width
            && self.height == other.height;
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.as_svg());
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
