use crate::{palette::Color, point::Point, Shape};

#[derive(Debug)]
pub struct Line {
    pub points: Vec<Point>,
    pub stroke_width: f64,
    pub stroke: Option<Color>,
}

impl Line {
    pub fn new(&mut self, points: Vec<Point>) {
        self.points = points;
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn length(&self) -> f64 {
        if self.points.len() == 0 {
            return 0.0;
        }

        let mut total = 0.0;
        for i in 1..self.points.len() {
            total += self.points[i - 1].distance(self.points[i])
        }
        return total;
    }
}

impl Shape for Line {
    fn as_svg(&self) -> String {
        if self.points.len() == 0 {
            return String::from("");
        }

        let stroke: String = match &self.stroke {
            Some(color) => format!("stroke=\"{}\" ", color),
            _ => String::from(""),
        };

        let stroke_weight: String = if &self.stroke_width == &0.0 {
            String::from("")
        } else {
            format!("stroke-width=\"{:.2}\" ", &self.stroke_width)
        };

        let mut str = format!("<path {}{}d=\"M ", stroke, stroke_weight,);

        for point in &self.points {
            str.push_str(&format!("{:.2} {:.2}, ", point.x, point.y));
        }

        str.push_str("\"/>\n");
        return str;
    }

    fn contains(&self, _point: Point) -> bool {
        false
    }
}
