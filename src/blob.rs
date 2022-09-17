use std::f64::consts::PI;

use rand::Rng;

use crate::{palette::Color, point::Point, Shape};

pub struct Blob {
    pub position: Point,
    pub radius: f64,
    pub color: Option<Color>,
    points: Vec<Point>,
}

impl Blob {
    pub fn new(position: Point, radius: f64, color: Option<Color>) -> Blob {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(5..12);

        let mut points = vec![];

        for i in 0..count {
            let angle = (i as f64 / count as f64) * PI * 2.0;
            points.push(Point {
                x: position.x + angle.cos() * radius * rng.gen_range(0.8..1.1),
                y: position.y + angle.sin() * radius * rng.gen_range(0.8..1.1),
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
        if self.points.len() == 0 {
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
        return str;
    }

    fn contains(&self, _point: Point) -> bool {
        todo!();
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
