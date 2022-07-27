pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

impl Circle {
    pub fn distance(&self, other: &Circle) -> f64 {
        ((self.x - other.x).powf(2.0) * (self.y - other.y).powf(2.0)).sqrt()
    }

    pub fn intersects(&self, other: &Circle) -> bool {
        self.distance(&other) < (self.r + other.r)
    }
}

impl super::Shape for Circle {
    fn as_svg(&self) -> String {
        format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" />",
            self.x, self.y, self.r
        )
    }

    fn contains(&self, point: &super::point::Point) -> bool {
        self.distance(&Circle {
            x: point.x,
            y: point.y,
            r: 0.0,
        }) < self.r
    }
}
