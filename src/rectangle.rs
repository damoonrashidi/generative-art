use super::Shape;

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,

    pub fill: &'static str,
}

impl Rectangle {}

impl Shape for Rectangle {
    fn as_svg(&self) -> String {
        format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" />",
            self.x, self.y, self.width, self.height,
        )
    }

    fn contains(&self, point: super::point::Point) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}
