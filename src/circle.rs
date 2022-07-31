use std::fmt::Display;

use crate::palette::Color;

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    color: Option<Color>,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Circle {
        Circle {
            x,
            y,
            r,
            color: None,
        }
    }

    pub fn distance(&self, other: &Circle) -> f64 {
        let d_x = self.x - other.x;
        let d_y = self.y - other.y;
        return (d_x.powi(2) + d_y.powi(2)).sqrt();
    }

    pub fn intersects(&self, other: &Circle) -> bool {
        self.distance(&other) < (self.r + other.r)
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn copy(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            r: self.r,
            color: None,
        }
    }
}

impl super::Shape for Circle {
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

    fn contains(&self, point: super::point::Point) -> bool {
        self.distance(&Circle::new(point.x, point.y, 0.0)) < self.r
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.r == other.r;
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
