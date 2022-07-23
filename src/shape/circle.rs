pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

impl Circle {
    pub fn distance(self: Circle, other: Circle) -> f64 {
        ((self.x - other.x).abs() * (self.y - other.y).abs()).sqrt()
    }

    pub fn intersects(self: Circle, other: Circle) -> bool {
        self.distance(other) < (self.r + other.r)
    }

    pub fn inside(self: Circle, other: Circle) -> bool {
        false
    }
}
