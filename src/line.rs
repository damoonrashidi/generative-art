use crate::{point::Point, Shape};

pub struct Line {
    points: Vec<Point>,
}

impl Line {
    pub fn new(&mut self, points: Vec<Point>) {
        self.points = points;
    }
}

impl Shape for Line {
    fn as_svg(&self) -> String {
        let mut str = String::from("<path d=\"");

        for point in &self.points {
            str.push_str(&format!("L{} {}", point.x, point.y));
        }

        str.push_str("\" />");
        return str;
    }

    fn contains(&self, _point: Point) -> bool {
        false
    }
}
