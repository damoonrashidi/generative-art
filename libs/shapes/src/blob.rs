use std::f64::consts::PI;

use palette::color::Color;
use rand::Rng;

use crate::{point::Point, rectangle::Rectangle, shape::Shape};

pub struct Blob {
    pub position: Point,
    pub radius: f64,
    pub color: Option<Color>,
    points: Vec<Point>,
}

impl Blob {
    pub fn new(position: Point, radius: f64, color: Option<Color>) -> Blob {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(7..24);

        let mut points = vec![];

        for i in 0..count {
            let angle = (i as f64 / count as f64) * PI * 2.0;
            points.push(Point {
                x: position.x + angle.cos() * radius * rng.gen_range(0.8..1.2),
                y: position.y + angle.sin() * radius * rng.gen_range(0.8..1.2),
            });
        }

        Blob {
            position,
            radius,
            points,
            color,
        }
    }
}

impl Shape for Blob {
    fn as_svg(&self) -> String {
        if self.points.is_empty() {
            return String::from("");
        }

        let fill: String = match &self.color {
            Some(color) => format!("fill=\"{}\" ", color),
            _ => String::from(""),
        };

        let mut str = format!("<path {}d=\"M ", fill);

        for point in &self.points {
            str.push_str(&format!("{:.2} {:.2}, ", point.x, point.y));
        }

        str.push_str("\"/>\n");
        str
    }

    fn contains(&self, _point: &Point) -> bool {
        todo!();
    }

    fn center(&self) -> Point {
        Point {
            x: self.position.x,
            y: self.position.y,
        }
    }

    fn bounding_box(&self) -> Option<Rectangle> {
        todo!()
    }
}

impl Default for Blob {
    fn default() -> Self {
        Blob {
            position: Point { x: 0.0, y: 0.0 },
            radius: 0.0,
            color: None,
            points: vec![],
        }
    }
}
