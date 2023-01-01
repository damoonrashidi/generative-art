use std::{fmt::Display, ops::Range};

use crate::palette::color::Color;

use super::{
    path::{Path, PathStyle},
    point::Point,
    shape::Shape,
};

#[derive(Debug)]
pub enum SplitDirection {
    Horizontally,
    Vertically,
}

/**
A Rectangle

Example
```

use generative_art::{shapes::{rectangle::Rectangle, point::Point}, svg::document::Document, palette::color::Color};

let mut rect = Rectangle::new(Point(0.0, 0.0), 100.0, 100.0);
rect.set_color(Color::Hex("#f00"));

let mut svg = Document::new("my_doc", rect);

svg.add_shape(Box::new(rect));
svg.save();
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
        let x = self.position.0 - (width - self.width) / 2.0;
        let y = self.position.1 - (height - self.height) / 2.0;

        Rectangle {
            position: Point(x, y),
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
        self.position.0..(self.position.0 + self.width)
    }

    /// Returns a range that starts at teh y position of the rectangle
    /// and ends at the bottom of the rectangle.
    pub fn y_range(&self) -> Range<f64> {
        self.position.1..(self.position.1 + self.height)
    }

    /// Converts this rectangle to a [`Path`]. Useful for path wobbling.
    pub fn to_path(&self, style: PathStyle) -> Path {
        let points = vec![
            (self.position.0, self.position.1),
            (self.position.0 + self.width, self.position.1),
            (self.position.0 + self.width, self.position.1 + self.height),
            (self.position.0, self.position.1 + self.height),
            (self.position.0, self.position.1),
        ]
        .iter()
        .map(|(x, y)| Point(*x, *y))
        .collect();

        Path::new(points, style)
    }

    pub fn subdivide(
        &self,
        split_point: &Point,
        split_direction: SplitDirection,
    ) -> (Rectangle, Rectangle) {
        match split_direction {
            SplitDirection::Horizontally => Rectangle::split_horizontally(&self, &split_point, 16.),
            SplitDirection::Vertically => Rectangle::split_vertically(&self, &split_point, 16.),
        }
    }

    fn split_horizontally(
        rect: &Rectangle,
        split_point: &Point,
        padding: f64,
    ) -> (Rectangle, Rectangle) {
        (
            Rectangle::new(
                rect.position,
                split_point.0 - padding - rect.position.0,
                rect.height,
            ),
            Rectangle::new(
                Point(split_point.0 + padding, rect.position.1),
                rect.position.0 + rect.width - split_point.0 - padding,
                rect.height,
            ),
        )
    }

    fn split_vertically(
        rect: &Rectangle,
        split_point: &Point,
        padding: f64,
    ) -> (Rectangle, Rectangle) {
        (
            Rectangle::new(
                rect.position,
                rect.width,
                split_point.1 - padding - rect.position.1,
            ),
            Rectangle::new(
                Point(rect.position.0, split_point.1 + padding),
                rect.width,
                rect.position.1 + rect.height - split_point.1 - padding,
            ),
        )
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
            self.position.0, self.position.1, self.width, self.height, fill
        )
    }

    fn contains(&self, point: &Point) -> bool {
        self.x_range().contains(&point.0) && self.y_range().contains(&point.1)
    }

    fn center(&self) -> Point {
        Point(
            (self.position.0 + self.width) / 2.0,
            (self.position.1 + self.height) / 2.0,
        )
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
            position: Point(0., 0.),
            width: 0.0,
            height: 0.0,
            color: None,
        }
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.position.0 == other.position.0
            && self.position.1 == other.position.1
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

    use crate::shapes::{point::Point, shape::Shape};

    use super::Rectangle;

    #[test]
    fn does_not_contain() {
        let rect = Rectangle {
            position: Point(0., 0.),
            width: 20.0,
            height: 20.0,
            color: None,
        };

        let point = Point(10.0, 30.0);

        assert!(!rect.contains(&point));
    }

    #[test]
    fn scale_rect_up() {
        let rect = Rectangle {
            position: Point(0., 0.),
            width: 100.0,
            height: 100.0,
            color: None,
        };

        let scaled = rect.scale(1.1);
        assert_eq!(
            Rectangle {
                position: Point(scaled.position.0.round(), scaled.position.1.round()),
                width: scaled.width.round(),
                height: scaled.height.round(),
                ..scaled
            },
            Rectangle {
                position: Point(-5., -5.),
                width: 110.0,
                height: 110.0,
                color: None
            }
        );
    }

    #[test]
    fn scale_rect_down() {
        let rect = Rectangle {
            position: Point(0., 0.),
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
                position: Point(5., 5.),
                width: 90.0,
                height: 90.0,
                color: None
            }
        );
    }

    #[test]
    fn test_center_0_0() {
        let rect = Rectangle::new(Point(0.0, 0.0), 100.0, 100.0);

        assert_eq!(rect.center(), Point(50.0, 50.0));
    }

    #[test]
    fn test_center_other() {
        let rect = Rectangle::new(Point(50., 0.), 100.0, 100.0);

        assert_eq!(rect.center(), Point(75.0, 50.0));
    }
}
