pub mod blob;
pub mod circle;
pub mod group;
pub mod helpers;
pub mod palette;
pub mod path;
pub mod point;
pub mod pointmap;
pub mod rectangle;
pub mod svg;

use crate::point::Point;

pub trait Shape {
    fn as_svg(&self) -> String;
    fn contains(&self, point: Point) -> bool;
}
