use crate::{palette::Color, point::Point, rectangle::Rectangle, shape::Shape};

#[derive(Debug)]
pub struct Path {
    pub points: Vec<Point>,
    pub style: PathStyle,
}

#[derive(Debug)]
pub struct PathStyle {
    pub stroke_width: Option<f64>,
    pub stroke: Option<Color>,
    pub color: Option<Color>,
}

impl Default for PathStyle {
    fn default() -> Self {
        PathStyle {
            stroke_width: None,
            stroke: None,
            color: None,
        }
    }
}

impl Path {
    pub fn new(points: Vec<Point>, style: PathStyle) -> Path {
        Path { points, style }
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
            total += self.points[i - 1].distance(&self.points[i])
        }
        return total;
    }
}

impl Shape for Path {
    fn as_svg(&self) -> String {
        if self.points.len() == 0 {
            return String::from("");
        }

        let stroke: String = match &self.style.stroke {
            Some(color) => format!("stroke=\"{}\" ", color),
            _ => String::from(""),
        };

        let fill: String = match &self.style.stroke {
            Some(color) => format!("fill=\"{}\" ", color),
            _ => String::from(""),
        };

        let stroke_weight: String = match &self.style.stroke_width {
            Some(stroke) => format!("stroke-width=\"{:.2}\" ", stroke),
            None => String::from(""),
        };

        let mut str = format!("<path {}{}{}d=\"M ", fill, stroke, stroke_weight);

        for point in &self.points {
            str.push_str(&format!("{:.2} {:.2}, ", point.x, point.y));
        }

        str.push_str("\"/>\n");
        return str;
    }

    fn center(&self) -> Point {
        todo!()
    }

    fn bounding_box(&self) -> Rectangle {
        if self.points.len() == 0 {
            panic!()
        }

        let default_point = Point { x: 0., y: 0. };

        let mut min_x = default_point.x;
        let mut min_y = default_point.y;

        if let Some(p) = self.points.get(0) {
            min_x = p.x;
            min_y = p.y;
        }

        let max_x = min_x;
        let max_y = min_y;

        let bounding =
            self.points
                .iter()
                .fold((min_x, min_y, max_x, max_y), |(x1, y1, x2, y2), point| {
                    (
                        x1.min(point.x),
                        y1.min(point.y),
                        x2.max(point.x),
                        y2.max(point.y),
                    )
                });

        Rectangle {
            x: bounding.0,
            y: bounding.1,
            width: bounding.2 - bounding.0,
            height: bounding.3 - bounding.1,
            color: None,
        }
    }

    fn contains(&self, _point: Point) -> bool {
        todo!("Not yet implemented")
    }
}

#[cfg(test)]
mod test {
    use crate::{point::Point, rectangle::Rectangle, shape::Shape};

    use super::Path;

    #[test]
    fn get_bounding_box() {
        let path = Path {
            points: vec![
                Point { x: 0., y: 0. },
                Point { x: 5., y: 5. },
                Point { x: -5., y: 10. },
            ],
            style: super::PathStyle::default(),
        };

        let bounding = path.bounding_box();

        assert_eq!(
            bounding,
            Rectangle {
                x: -5.,
                y: 0.,
                width: 10.,
                height: 10.,
                color: None
            }
        )
    }
}
