use std::{fmt::Display, ops::Range};

use palette::color::Color;

use super::shape::Shape;
use crate::{
    path::{Path, PathStyle},
    point::Point,
};

/**
A Rectangle

Example
```
let mut rect = Rectangle::new(Point{x: 0.0, y: 0.0}, 100.0, 100.0);
rect.set_color(Color::Hex("#f00"));
svg.add_shape(Box::new(rect));
rect.save();
```
*/
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    /// Upper left corner of the rectangle
    pub position: Point,

    /// Width of the rectangle
    pub width: f64,

    /// Height of the rectangle
    pub height: f64,

    /// Fill color of the rectangle.
    pub color: Option<Color>,
}

impl Rectangle {
    /// Create a new Rectangle where the top-left corner will be located at
    /// the given position.
    pub fn new(position: Point, width: f64, height: f64) -> Rectangle {
        Rectangle {
            position,
            width,
            height,
            ..Default::default()
        }
    }

    /// Set the color of the rectangle.
    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    /// Scale a rectangle by a factor of the scale parameter.
    /// The rectangle will scale from the origo, meaning
    /// new rectangle that will be returned will be moved.
    pub fn scale(&self, scale: f64) -> Rectangle {
        let width = self.width * scale;
        let height = self.height * scale;
        let x = self.position.x - (width - self.width) / 2.0;
        let y = self.position.y - (height - self.height) / 2.0;

        Rectangle {
            position: Point { x, y },
            width,
            height,
            color: self.color,
        }
    }

    /// Surface area of the rectangle.
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// Returns a range that starts at the x position of the rectangle
    /// and ends on the right side of the rectangle.
    pub fn x_range(&self) -> Range<f64> {
        self.position.x..(self.position.x + self.width)
    }

    /// Returns a range that starts at teh y position of the rectangle
    /// and ends at the bottom of the rectangle.
    pub fn y_range(&self) -> Range<f64> {
        self.position.y..(self.position.y + self.height)
    }

    /// Converts this rectangle to a [`Path`]. Useful for path wobbling.
    pub fn to_path(&self, style: PathStyle) -> Path {
        let points = vec![
            (self.position.x, self.position.y),
            (self.position.x + self.width, self.position.y),
            (self.position.x + self.width, self.position.y + self.height),
            (self.position.x, self.position.y + self.height),
            (self.position.x, self.position.y),
        ]
        .iter()
        .map(|(x, y)| Point { x: *x, y: *y })
        .collect();

        Path::new(points, style)
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
            self.position.x, self.position.y, self.width, self.height, fill
        )
    }

    fn contains(&self, point: &Point) -> bool {
        self.x_range().contains(&point.x) && self.y_range().contains(&point.y)
    }

    fn center(&self) -> Point {
        Point {
            x: (self.position.x + self.width) / 2.0,
            y: (self.position.y + self.height) / 2.0,
        }
    }

    fn bounding_box(&self) -> Option<Rectangle> {
        Some(Rectangle {
            position: self.position,
            width: self.width,
            height: self.height,
            color: None,
        })
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            position: Point { x: 0., y: 0. },
            width: 0.0,
            height: 0.0,
            color: None,
        }
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.position.x == other.position.x
            && self.position.y == other.position.y
            && self.width == other.width
            && self.height == other.height
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.as_svg())
    }
}

#[cfg(test)]
mod test {
    use crate::{point::Point, shape::Shape};

    use super::Rectangle;

    #[test]
    fn does_not_contain() {
        let rect = Rectangle {
            position: Point { x: 0., y: 0. },
            width: 20.0,
            height: 20.0,
            color: None,
        };

        let point = Point { x: 10.0, y: 30.0 };

        assert!(!rect.contains(&point));
    }

    #[test]
    fn scale_rect_up() {
        let rect = Rectangle {
            position: Point { x: 0., y: 0. },
            width: 100.0,
            height: 100.0,
            color: None,
        };

        let scaled = rect.scale(1.1);
        assert_eq!(
            Rectangle {
                position: scaled.position,
                width: scaled.width.round(),
                height: scaled.height.round(),
                ..scaled
            },
            Rectangle {
                position: Point { x: -5., y: -5. },
                width: 110.0,
                height: 110.0,
                color: None
            }
        );
    }

    #[test]
    fn scale_rect_down() {
        let rect = Rectangle {
            position: Point { x: 0., y: 0. },
            width: 100.0,
            height: 100.0,
            color: None,
        };

        let scaled = rect.scale(0.9);
        assert_eq!(
            Rectangle {
                position: scaled.position,
                width: scaled.width.round(),
                height: scaled.height.round(),
                ..scaled
            },
            Rectangle {
                position: Point { x: 5., y: 5. },
                width: 90.0,
                height: 90.0,
                color: None
            }
        );
    }

    #[test]
    fn test_center_0_0() {
        let rect = Rectangle::new(Point { x: 0.0, y: 0.0 }, 100.0, 100.0);

        assert_eq!(rect.center(), Point { x: 50.0, y: 50.0 });
    }

    #[test]
    fn test_center_other() {
        let rect = Rectangle::new(Point { x: 50., y: 0. }, 100.0, 100.0);

        assert_eq!(rect.center(), Point { x: 75.0, y: 50.0 });
    }
}
