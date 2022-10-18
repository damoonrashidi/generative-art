use crate::point::Point;

pub trait Shape {
    fn as_svg(&self) -> String;
    fn center(&self) -> Point;
    fn contains(&self, point: Point) -> bool;
}
