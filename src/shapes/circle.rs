use std::fmt::Display;

use crate::palette::color::Color;

use super::{point::Point, rectangle::Rectangle, shape::Shape};

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
    /// use generative_art::shapes::{circle::Circle, point::Point};
    /// let circle = Circle::new(Point(0.0, 0.0), 10.0);
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
    /// use generative_art::shapes::{circle::Circle, point::Point};
    /// let circle = Circle::new(Point(0.0, 0.0), 10.0);
    /// let other = Circle::new(Point(20.0, 0.0), 10.0);
    /// let distance = circle.distance(&other); // -> 10.0
    /// ```
    pub fn distance(&self, other: &Circle) -> f64 {
        let d_x = self.center.0 - other.center.0;
        let d_y = self.center.1 - other.center.1;
        (d_x.powi(2) + d_y.powi(2)).sqrt() - self.radius / 2. - other.radius / 2.
    }

    /// True if a given intersects another circle, otherwise false.
    pub fn intersects(&self, other: &Circle) -> bool {
        self.distance(other) < self.radius / 2. + other.radius / 2.
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
            self.center.0, self.center.1, self.radius, fill
        )
    }

    fn center(&self) -> Point {
        Point(self.center.0, self.center.1)
    }

    fn bounding_box(&self) -> Option<Rectangle> {
        Some(Rectangle {
            position: Point(self.center.0 - self.radius, self.center.1 - self.radius),
            width: self.center.0 + self.radius,
            height: self.center.1 + self.radius,
            color: None,
        })
    }

    fn contains(&self, point: &Point) -> bool {
        self.center().distance_to(point) < self.radius
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.center.0 == other.center.0
            && self.center.1 == other.center.1
            && self.radius == other.radius
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x:{} y:{} r:{}",
            self.center.0, self.center.1, self.radius
        )
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle {
            center: Point(0.0, 0.0),
            radius: 0.0,
            color: None,
        }
    }
}
