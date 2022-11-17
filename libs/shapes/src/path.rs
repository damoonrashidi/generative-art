use palette::color::Color;
use rand::{thread_rng, Rng};

use crate::{point::Point, rectangle::Rectangle, shape::Shape};

#[derive(Debug, Default)]
pub struct Path {
    pub points: Vec<Point>,
    pub style: PathStyle,
}

#[derive(Debug, Default)]
pub struct PathStyle {
    pub stroke_width: Option<f64>,
    pub stroke: Option<Color>,
    pub color: Option<Color>,
}

impl Path {
    pub fn new(points: Vec<Point>, style: PathStyle) -> Path {
        Path { points, style }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn wobble(&mut self) {
        let mut rng = thread_rng();
        let mut new_list: Vec<Point> = vec![];
        let center = self.center();

        for (i, mut point) in self.points.clone().into_iter().enumerate() {
            point.x += rng.gen_range(-3.0..3.0);
            point.y += rng.gen_range(-3.0..3.0);

            new_list.push(point);

            if let Some(next) = self.points.get(i + 1) {
                for p in (1..10).step_by(2) {
                    let mut between = point.between(next, p as f64 / 10.);
                    between.x += between.angle_to(&center).cos() * rng.gen_range(-5.0..5.0);
                    between.y += between.angle_to(&center).sin() * rng.gen_range(-5.0..5.0);
                    new_list.push(between);
                }
            }
        }

        self.points = new_list
    }

    pub fn length(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }

        let mut total = 0.0;
        for i in 1..self.points.len() {
            total += self.points[i - 1].distance(&self.points[i])
        }
        total
    }

    pub fn intersects(l1: (&Point, &Point), l2: (&Point, &Point)) -> bool {
        let a1 = l1.1.y - l1.1.x;
        let b1 = l1.1.x - l1.1.y;
        let c1 = (l1.1.y * l1.1.x) - (l1.1.x * l1.1.y);

        let mut d1 = (a1 * l2.0.x) + (b1 * l2.1.x) + c1;
        let mut d2 = (a1 * l2.0.y) + (b1 * l2.1.y) + c1;

        if (d1 > 0. && d2 > 0.) || (d1 < 0. && d2 < 0.) {
            return false;
        }

        let a2 = l2.1.y - l2.1.x;
        let b2 = l2.0.x - l2.0.y;
        let c2 = (l2.0.y * l2.1.x) - (l2.0.x * l2.1.y);
        d1 = (a2 * l1.1.x) + (b2 * l1.1.x) + c2;
        d2 = (a2 * l1.1.y) + (b2 * l1.1.y) + c2;

        if (d1 > 0. && d2 > 0.) || (d1 < 0. && d2 < 0.) {
            return false;
        }

        true
    }
}

impl Shape for Path {
    fn as_svg(&self) -> String {
        if self.points.is_empty() {
            return String::from("");
        }

        let stroke: String = match &self.style.stroke {
            Some(color) => format!("stroke=\"{}\" ", color),
            None => String::from(""),
        };

        let fill: String = match &self.style.color {
            Some(color) => format!("fill=\"{}\" ", color),
            None => String::from(""),
        };

        let stroke_weight: String = match &self.style.stroke_width {
            Some(stroke) => format!("stroke-width=\"{:.2}\" ", stroke),
            None => String::from(""),
        };

        if let Some(first) = self.points.first() {
            let mut str = self.points.iter().skip(1).enumerate().fold(
                format!(
                    "<path {fill}{stroke}{stroke_weight}d=\"M{:.2},{:.2}",
                    first.x, first.y
                ),
                |mut path, (i, point)| {
                    if let Some(previous) = self.points.get(i) {
                        if previous.x == point.x {
                            path.push_str(&format!(" V{:.2}", point.y));
                        } else if previous.y == point.y {
                            path.push_str(&format!(" H{:.2}", point.x));
                        } else {
                            path.push_str(&format!(" L{:.2},{:.2}", point.x, point.y));
                        }
                    }

                    path
                },
            );

            str.push_str(" \"/>\n");
            return str;
        }
        String::from("")
    }

    fn center(&self) -> Point {
        if let Some(bounding) = self.bounding_box() {
            bounding.center();
        }

        Point { x: 0.0, y: 0.0 }
    }

    fn bounding_box(&self) -> Option<Rectangle> {
        if self.points.is_empty() {
            return None;
        }

        let p = if let Some(p) = self.points.get(0) {
            p
        } else {
            return None;
        };

        let min_x = p.x;
        let min_y = p.y;
        let max_x = min_x;
        let max_y = min_y;

        let bounding = self.points.clone().iter().fold(
            (min_x, min_y, max_x, max_y),
            |(x1, y1, x2, y2), point| {
                (
                    x1.min(point.x),
                    y1.min(point.y),
                    x2.max(point.x),
                    y2.max(point.y),
                )
            },
        );

        Some(Rectangle::new(
            bounding.0,
            bounding.1,
            bounding.2 - bounding.0,
            bounding.3 - bounding.1,
        ))
    }

    fn contains(&self, point: &Point) -> bool {
        /*
         * How this works: it starts by getting the bounding box for the polygon.
         * After which it creates four search rays from the point out in each direction
         * to the bounding box.
         *
         * It then takes two pairs of points (in other words a line) and checks how many
         * times each search ray intersects with each line,
         * if the intersection count is even, then the point is inside the polygon,
         * if the intersection count is uneven, then the point is outside the polygon.
         *
         * Illustrated below with an exaggerated bounding box for legabillity.
         *
         * -----------------------------
         * |           |                |
         * |       ____|______          |
         * |      /    |      |         |
         * |-----|-----*------/---------|
         * |     |__   |     /          |
         * |        |__|____/           |
         * |           |                |
         * -----------------------------
         */

        let bounds = if let Some(bounding) = self.bounding_box() {
            bounding
        } else {
            return false;
        };

        if !bounds.contains(point) {
            return false;
        }

        let search = [
            (
                point,
                &Point {
                    x: point.x,
                    y: bounds.y,
                },
            ),
            (
                point,
                &Point {
                    x: point.x,
                    y: bounds.y + bounds.height,
                },
            ),
            (
                point,
                &Point {
                    x: bounds.x + bounds.width,
                    y: point.y,
                },
            ),
            (
                point,
                &Point {
                    x: bounds.x,
                    y: point.y,
                },
            ),
        ];

        let mut intersections: usize = 0;

        for line in self.points.chunks(2) {
            if line.len() == 1 {
                return intersections % 2 == 0;
            }
            let l1 = (&line[0], &line[1]);
            for l2 in search {
                if Path::intersects(l1, l2) {
                    intersections += 1;
                }
            }
        }

        intersections % 2 == 0
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
            style: Default::default(),
        };

        if let Some(bounding) = path.bounding_box() {
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

    #[test]
    fn does_intersect() {
        let line1 = (&Point { x: 0., y: 0. }, &Point { x: 0., y: 50. });
        let line2 = (&Point { x: -25., y: 25. }, &Point { x: 25., y: 25. });

        assert!(Path::intersects(line1, line2));
    }

    #[test]
    fn point_inside_polygon() {
        let path = Path::new(
            vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 100.0, y: 10.0 },
                Point { x: 100.0, y: 100.0 },
                Point { x: 20.0, y: 80.0 },
                Point { x: 0.0, y: 0.0 },
            ],
            Default::default(),
        );

        assert!(path.contains(&Point { x: 50., y: 50. }));
        assert!(!path.contains(&Point { x: 500., y: 50. }))
    }
}
