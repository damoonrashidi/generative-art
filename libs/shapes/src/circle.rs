use std::fmt::Display;

use palette::color::Color;

use crate::{point::Point, rectangle::Rectangle, shape::Shape};

/// A Circle
#[derive(Clone, Copy, Debug)]
pub struct Circle {
    /// Center point, or origo, for the Circle
    pub center: Point,

    /// Radius
    pub radius: f64,

    /// Fill color for a circle
    color: Option<Color>,
}

impl Circle {
    /// Create a new Circle at given [`point`] with given [`radius`]
    ///
    /// ```
    /// let circle = Circle::new(Point{x: 0.0, y: 0.0}, 10.0);
    /// ```
    pub fn new(center: Point, radius: f64) -> Circle {
        Circle {
            center,
            radius,
            color: None,
        }
    }

    /// Calculate the distance between this circle and another circle.
    /// The distance will be calculated based on the edges of the circles,
    /// not the center.
    ///
    /// ```
    /// let circle = Circle::new(Point{x: 0.0, y: 0.0}, 10.0);
    /// let other = Circle::new(Point{x: 20.0, y: 0.0}, 10.0);
    /// let distance = circle.distance(&other); // -> 10.0
    /// ```
    pub fn distance(&self, other: &Circle) -> f64 {
        let d_x = self.center.x - other.center.x;
        let d_y = self.center.y - other.center.y;
        (d_x.powi(2) + d_y.powi(2)).sqrt() - self.radius - other.radius
    }

    /// True if a given intersects another circle, otherwise false.
    pub fn intersects(&self, other: &Circle) -> bool {
        self.distance(other) < self.radius + other.radius
    }

    /// True if a given circle intersects any other circle in the, otherwise false
    pub fn instersects_any(&self, others: Vec<Circle>) -> bool {
        others.iter().any(|circle| self.intersects(circle))
    }

    /// Set the fill color of the circle
    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    /// Scale a circle by a factor of [`scale`]. The radius remains unchanged.
    pub fn scale(&self, scale: f64) -> Circle {
        Circle {
            radius: self.radius * scale,
            center: self.center,
            color: self.color,
        }
    }
}

impl Shape for Circle {
    fn as_svg(&self) -> String {
        let fill: String = match self.color {
            Some(color) => format!("{}", color),
            _ => String::from("transparent"),
        };

        format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" fill=\"{}\" />",
            self.center.x, self.center.y, self.radius, fill
        )
    }

    fn center(&self) -> Point {
        Point {
            x: self.center.x,
            y: self.center.y,
        }
    }

    fn bounding_box(&self) -> Option<Rectangle> {
        Some(Rectangle {
            position: Point {
                x: self.center.x - self.radius,
                y: self.center.y - self.radius,
            },
            width: self.center.x + self.radius,
            height: self.center.y + self.radius,
            color: None,
        })
    }

    fn contains(&self, point: &Point) -> bool {
        self.center().distance(point) < self.radius
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.center.x == other.center.x
            && self.center.y == other.center.y
            && self.radius == other.radius
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x:{} y:{} r:{}",
            self.center.x, self.center.y, self.radius
        )
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle {
            center: Point { x: 0.0, y: 0.0 },
            radius: 0.0,
            color: None,
        }
    }
}
