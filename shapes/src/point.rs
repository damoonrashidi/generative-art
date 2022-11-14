use std::{fmt::Display, ops::{Add, Mul, Sub}};

use crate::{rectangle::Rectangle, shape::Shape};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
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

    pub fn move_self(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
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

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<f64> for Point {
    type Output = Point;
    fn add(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<f64> for Point {
    type Output = Point;
    fn sub(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: rhs * self.x,
            y: rhs * self.y,
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
            x: self.x,
            y: self.y,
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

        let b = a + Point{ x: 10.0, y: 0.0};
        assert_eq!(b, Point { x: 15.0, y: -100.0 });
    }
}
