use crate::{point::Point, rectangle::Rectangle};

pub trait Shape {
    fn as_svg(&self) -> String;
    fn center(&self) -> Point;
    fn bounding_box(&self) -> Rectangle;
    fn contains(&self, point: Point) -> bool;
}
