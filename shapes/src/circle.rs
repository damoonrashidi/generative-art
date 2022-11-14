use std::fmt::Display;

use palette::Color;

use crate::{point::Point, rectangle::Rectangle, shape::Shape};

#[derive(Clone, Copy, Debug, Default)]
pub struct Circle {
    pub center: Point,
    pub r: f64,
    color: Option<Color>,
}

impl Circle {
    pub fn new(center: Point, r: f64) -> Circle {
        Circle {
            center,
            r,
            color: None,
        }
    }

    pub fn distance(&self, other: &Circle) -> f64 {
        let d_x = self.center.x - other.center.x;
        let d_y = self.center.y - other.center.y;
        (d_x.powi(2) + d_y.powi(2)).sqrt() - self.r - other.r
    }

    pub fn intersects(&self, other: &Circle) -> bool {
        self.distance(other) < self.r + other.r
    }

    pub fn instersects_any(&self, others: Vec<Circle>) -> bool {
        others.iter().any(|circle| self.intersects(circle))
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn scale(&self, scale: f64) -> Circle {
        Circle {
            r: self.r * scale,
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
            self.center.x, self.center.y, self.r, fill
        )
    }

    fn center(&self) -> Point {
        self.center
    }

    fn bounding_box(&self) -> Option<Rectangle> {
        let corner = self.center - self.r;
        Some(Rectangle {
            x: corner.x,
            y: corner.y,
            width: 2.0 * self.r,
            height: 2.0 * self.r,
            color: None,
        })
    }

    fn contains(&self, point: &Point) -> bool {
        self.center().distance(point) < self.r
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.r == other.r
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{} y:{} r:{}", self.center.x, self.center.y, self.r)
    }
}
