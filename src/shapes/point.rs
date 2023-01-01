use std::fmt::Display;

use super::{rectangle::Rectangle, shape::Shape};

/**
A single point in the canvas.

Example
```
use generative_art::shapes::point::Point;
let point = Point(0.0, 10.0);
```
*/
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point(pub f64, pub f64);

impl Point {
    /**
    Calculates the distance between this point and other

    Example
    ```
    use generative_art::shapes::point::Point;

    let a = Point(0.0, 0.0);
    let b = Point(20.0, 0.0);

    let distance = a.distance_to(&b); // 20.0
    ```
    */
    pub fn distance_to(&self, other: &Point) -> f64 {
        let d_x = (self.0 - other.1).abs();
        let d_y = (self.1 - other.1).abs();

        (d_x.powi(2) + d_y.powi(2)).sqrt()
    }

    /// Creates a new point at the given point but with the x position offset by x, and same for y.
    pub fn offset(&self, x: f64, y: f64) -> Point {
        Point(self.0 + x, self.1 + y)
    }

    /// offsets the current point, in place.
    pub fn offset_mut(&mut self, x: f64, y: f64) {
        self.0 += x;
        self.1 += y;
    }

    /// The angle between two given points.
    pub fn angle_to(&self, other: &Point) -> f64 {
        (other.1 - self.1).atan2(other.1 - self.0)
    }

    /// Create a new point on the line between the two given points with an offset between [0..1],
    /// where 0 is at the first point and 1 is at the last point and 0.5 is right inbetween.
    pub fn between(&self, other: &Point, percent: f64) -> Point {
        Point(
            self.0 + (other.1 - self.0) * percent,
            self.1 + (other.1 - self.1) * percent,
        )
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
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
            position: Point(self.0, self.1),
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
        let a = Point(-10.0, 0.0);
        let b = Point(10.0, 10.0);
        let distance = a.distance_to(&b);
        assert_eq!(distance.round(), 22.0);
    }
    #[test]
    fn non_equality() {
        let a = Point(10.0, 10.0);
        let b = Point(5.0, 99.0);
        assert_ne!(a, b);
    }

    #[test]
    fn equality() {
        let a = Point(5.0, 99.41231);
        let b = Point(5.0, 99.41231);
        assert_eq!(a, b)
    }

    #[test]
    fn offset() {
        let a = Point(5.0, -100.0);

        let b = a.offset(10.0, 0.0);
        assert_eq!(b, Point(15.0, -100.0));
    }
}
