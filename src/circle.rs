use std::fmt::Display;

use crate::{palette::Color, point::Point, rectangle::Rectangle, shape::Shape};

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    color: Option<Color>,
}

impl Circle {
    pub fn new(center: Point, r: f64) -> Circle {
        Circle {
            x: center.x,
            y: center.y,
            r,
            color: None,
        }
    }

    pub fn distance(&self, other: &Circle) -> f64 {
        let d_x = self.x - other.x;
        let d_y = self.y - other.y;
        (d_x.powi(2) + d_y.powi(2)).sqrt() - (self.r * 2.0 + other.r * 2.0)
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
            x: self.x,
            y: self.y,
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
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />",
            self.x, self.y, self.r, fill
        )
    }

    fn center(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    fn bounding_box(&self) -> Rectangle {
        Rectangle {
            x: self.x - self.r,
            y: self.y - self.r,
            width: self.x + self.r,
            height: self.y + self.r,
            color: None,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        self.distance(&Circle::new(*point, 0.0)) < self.r
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.r == other.r
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{} y:{} r:{}", self.x, self.y, self.r)
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle {
            color: None,
            x: 0.0,
            y: 0.0,
            r: 0.0,
        }
    }
}
