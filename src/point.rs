use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        let d_x = self.x - other.x;
        let d_y = self.y - other.y;
        return (d_x.powi(2) + d_y.powi(2)).sqrt();
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn distance() {
        let a = Point { x: -10.0, y: 0.0 };
        let b = Point { x: 10.0, y: 10.0 };
        let distance = a.distance(b);
        assert_eq!(distance.round(), 22.0);
    }
}
