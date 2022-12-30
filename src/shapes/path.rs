use rand::{thread_rng, Rng};

use crate::palette::color::Color;

use super::{point::Point, rectangle::Rectangle, shape::Shape};

/// An SVG path
#[derive(Debug, Default)]
pub struct Path {
    /// List of points that make up the path.
    pub points: Vec<Point>,

    /// Stroke width, stroke color and fill color.
    pub style: PathStyle,
}

/// A style for a given [`Path`], it can specify fill, stroke color and stroke width
#[derive(Debug, Default)]
pub struct PathStyle {
    /// The width of the stroke around this path
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-width
    pub stroke_weight: Option<f64>,

    /// The color of the stroke around this path
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke
    pub stroke: Option<Color>,

    /// The fill color of this path
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill
    pub color: Option<Color>,
}

impl Path {
    /// Create new [`Path`] with the given [`Point`]s and [`PathStyle`]
    pub fn new(points: Vec<Point>, style: PathStyle) -> Path {
        Path { points, style }
    }

    /// Adds another [`Point`] to the end of this path. This is good if
    /// You want to make a line longer.
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    /// Take a shape and rough it up a bit but adding a bunch of points between each,
    /// already existing, point and move those injected points around a bit.
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

    /// The total distance between each point in this shape, i.e, the true
    /// length of the shape.
    pub fn length(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }

        let mut total = 0.0;
        for i in 1..self.points.len() {
            total += self.points[i - 1].distance_to(&self.points[i])
        }
        total
    }

    /// Check if two lines intersect at any point.
    fn intersects(a: (&Point, &Point), b: (&Point, &Point)) -> bool {
        let dx0 = a.1.x - a.0.x;
        let dx1 = b.1.x - b.0.x;
        let dy0 = a.1.y - a.0.y;
        let dy1 = b.1.y - b.0.y;
        let p0 = dy1 * (b.1.x - a.0.x) - dx1 * (b.1.y - a.0.y);
        let p1 = dy1 * (b.1.x - a.1.x) - dx1 * (b.1.y - a.1.y);
        let p2 = dy0 * (a.1.x - b.0.x) - dx0 * (a.1.y - b.0.y);
        let p3 = dy0 * (a.1.x - b.1.x) - dx0 * (a.1.y - b.1.y);
        return (p0 * p1 <= 0.0) & (p2 * p3 <= 0.0);
    }
}

impl Shape for Path {
    fn as_svg(&self) -> String {
        if self.points.is_empty() {
            return String::from("");
        }

        let stroke: String = match self.style.stroke {
            Some(color) => format!("stroke=\"{color}\" "),
            None => String::from(""),
        };

        let fill: String = match self.style.color {
            Some(color) => format!("fill=\"{color}\" "),
            None => String::from("fill=\"none\" "),
        };

        let stroke_weight: String = match self.style.stroke_weight {
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

            str.push_str("\"/>\n");
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
            Point {
                x: bounding.0,
                y: bounding.1,
            },
            bounding.2 - bounding.0,
            bounding.3 - bounding.1,
        ))
    }

    /**
    How this works: it starts by getting the bounding box for the polygon.
    After which it creates four search rays from the point out in each direction
    to the bounding box.

    It then takes two pairs of points (in other words a line) and checks how many
    times each search ray intersects with each line,
    if the intersection count is even, then the point is inside the polygon,
    if the intersection count is uneven, then the point is outside the polygon.
    **Note:** this is for each search ray, so if any ray has uneven hits
    the line is outside

    Illustrated below with an exaggerated bounding box for legabillity.
    ```
    -----------------------------
    |           |                |
    |       ____|______          |
    |      /    |      |         |
    |-----|-----*------/---------|
    |     |__   |     /          |
    |        |__|____/           |
    |           |                |
    -----------------------------
    ```
    */
    fn contains(&self, point: &Point) -> bool {
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
                    y: bounds.position.y,
                },
            ),
            (
                point,
                &Point {
                    x: point.x,
                    y: bounds.position.y + bounds.height,
                },
            ),
            (
                point,
                &Point {
                    x: bounds.position.x + bounds.width,
                    y: point.y,
                },
            ),
            (
                point,
                &Point {
                    x: bounds.position.x,
                    y: point.y,
                },
            ),
        ];

        for ray in search {
            let mut intersections = 0;

            for i in 0..self.points.len() {
                match self.points.get(i + 1) {
                    None => break,
                    Some(_) => {
                        let line = (&self.points[i], &self.points[i + 1]);
                        if Path::intersects(line, ray) {
                            intersections += 1;
                        }
                    }
                }
            }

            if intersections % 2 == 0 {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use crate::shapes::{point::Point, rectangle::Rectangle};

    use super::{Path, Shape};

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
                    position: Point { x: -5., y: 0. },
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
