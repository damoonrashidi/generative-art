use crate::{point::Point, rectangle::Rectangle};

pub trait Shape {
    /// SVG representation of this shape
    fn as_svg(&self) -> String;

    /// Center Point of this shape
    fn center(&self) -> Point;

    /**
    A tight bounding box around a given shape, this will create a Rectangle around the shape
    where the left-most Point in the shape yields the left line of the resulting Rectangle,
    the top-most point yields the top line, etc, until the shape is bound by a box.

    Good for debugging purposes.
    */
    fn bounding_box(&self) -> Option<Rectangle>;

    /// True if the given shape contains {point}, otherwise false.
    fn contains(&self, point: &Point) -> bool;
}
