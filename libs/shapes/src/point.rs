use std::fmt::Display;

use crate::{rectangle::Rectangle, shape::Shape};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        let d_x = (self.x - other.x).abs();
        let d_y = (self.y - other.y).abs();

        (d_x.powi(2) + d_y.powi(2)).sqrt()
    }

    pub fn move_self(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    pub fn offset(&self, x: f64, y: f64) -> Point {
        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn angle_to(&self, other: &Point) -> f64 {
        (other.y - self.y).atan2(other.x - self.x)
    }

    pub fn between(&self, other: &Point, percent: f64) -> Point {
        Point {
            x: self.x + (other.x - self.x) * percent,
            y: self.y + (other.y - self.y) * percent,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}

impl Shape for Point {
    fn as_svg(&self) -> String {
        String::from("")
    }

    fn center(&self) -> Point {
        *self
    }

    fn bounding_box(&self) -> Option<Rectangle> {
        Some(Rectangle {
            position: Point {
                x: self.x,
                y: self.y,
            },
            width: 1.,
            height: 1.,
            color: None,
        })
    }

    fn contains(&self, point: &Point) -> bool {
        self.eq(point)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn distance() {
        let a = Point { x: -10.0, y: 0.0 };
        let b = Point { x: 10.0, y: 10.0 };
        let distance = a.distance(&b);
        assert_eq!(distance.round(), 22.0);
    }

    #[test]
    fn non_equality() {
        let a = Point { x: 10.0, y: 10.0 };
        let b = Point { x: 5.0, y: 99.0 };
        assert_ne!(a, b);
    }

    #[test]
    fn equality() {
        let a = Point {
            x: 5.0,
            y: 99.41231,
        };
        let b = Point {
            x: 5.0,
            y: 99.41231,
        };
        assert_eq!(a, b)
    }

    #[test]
    fn offset() {
        let a = Point { x: 5.0, y: -100.0 };

        let b = a.offset(10.0, 0.0);
        assert_eq!(b, Point { x: 15.0, y: -100.0 });
    }
}
