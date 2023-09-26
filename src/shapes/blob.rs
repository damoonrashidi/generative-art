/**
A splotch of a shape, like a circle but not symmetrical
Not a binary large object
*/
use std::f64::consts::PI;

use rand::Rng;

use crate::palette::color::Color;

use super::{path::Path, point::Point, rectangle::Rectangle, shape::Shape};

/**
* A Circle like shape, but slightly distorted to give a more natural look
*/
#[derive(Debug, Clone)]
pub struct Blob {
    /// Center point of the shape.
    pub position: Point,

    /// Approximetly how large the radius should, this cannot be guaranteed
    /// due to the uneven nature of the shape, in this case, this is desired
    /// behavior.
    pub radius: f64,

    /// The color to use for the blob
    pub color: Option<Color>,

    /// Each point in the shape of the blob
    points: Vec<Point>,
}

impl PartialEq for Blob {
    /**
    A blob is considered equal to another if the shapes are the same size
    and positioned at the same point.
    */
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.radius == other.radius
    }
}

impl Blob {
    /// Create a new blob at a given {position} with a given @radius
    pub fn new(position: Point, radius: f64, color: Option<Color>) -> Blob {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(7..15);

        let mut points = vec![];

        for i in 0..count {
            let angle = (i as f64 / count as f64) * PI * 2.0;
            points.push(Point(
                position.0 + angle.cos() * radius * rng.gen_range(0.8..1.2),
                position.1 + angle.sin() * radius * rng.gen_range(0.8..1.2),
            ));
        }

        Blob {
            position,
            radius,
            points,
            color,
        }
    }

    /// Calculates the distance between this blob and another given blob
    pub fn distance(&self, other: &Blob) -> f64 {
        self.center().distance_to(&other.center()) - self.radius - other.radius
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
            str.push_str(&format!("{:.2} {:.2},", point.0, point.1));
        }
        str.pop();

        str.push_str(" Z\"/>\n");
        str
    }

    fn contains(&self, point: &Point) -> bool {
        let path = Path::new(&mut self.points, Default::default());
        path.contains(point)
    }

    fn center(&self) -> Point {
        Point(self.position.0, self.position.1)
    }

    fn bounding_box(&self) -> Option<Rectangle> {
        let path = Path::new(&mut self.points, Default::default());
        path.bounding_box()
    }
}

impl Default for Blob {
    fn default() -> Self {
        Blob {
            position: Point(0.0, 0.0),
            radius: 0.0,
            color: None,
            points: vec![],
        }
    }
}
