use crate::point::Point;

pub trait Shape {
    fn as_svg(&self) -> String;
    fn contains(&self, point: Point) -> bool;
}
