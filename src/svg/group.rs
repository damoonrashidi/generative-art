use std::fmt::{Debug, Display};

use crate::{palette::color::Color, shapes::shape::Shape};

/// A group style defined the fill, stroke width and stroke color
/// for all shapes contained in the group, unless the styles are
/// defined on the shape level, in which case they override the
/// group styles.
#[derive(Debug, Default, Copy, Clone)]
pub struct GroupStyle {
    /// Fill color
    pub fill: Option<Color>,

    /// Stroke outline color
    pub stroke: Option<Color>,

    /// Stroke outline width
    pub stroke_width: Option<f64>,
}

impl Display for GroupStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stroke_width: String = match &self.stroke_width {
            Some(width) => format!(" stroke-width=\"{width}\""),
            None => String::from(""),
        };

        let stroke: String = match &self.stroke {
            Some(color) => format!(" stroke=\"{color}\""),
            None => String::from(""),
        };

        let fill: String = match &self.fill {
            Some(color) => format!(" fill=\"{color}\""),
            None => String::from(" fill=\"none\""),
        };

        write!(f, "{fill}{stroke}{stroke_width}")
    }
}

/**
A Group (https://developer.mozilla.org/en-US/docs/Web/SVG/Element/g) can contain other shapes but
ensures that they have the same styles (fill, stroke, stroke_width) applied to them. This can be
advantageous if you have a lot of shapes with the same styles, since the output SVG will be smaller
if the styles are hoisted to the group instead of applied at the shape level.

Example:

```
let mut g = Group::new();
g.set_style(GroupStyle {
    fill: Color::Hex("#111"),
    ..Default.default()
});

let rect1 = Rectangle::new(Point{x: 0.0, y:0.0}, 100.0, 100.0);
let rect2 = Rectangle::new(Point{x: 0.0, y:0.0}, 100.0, 100.0);

g.add_shape(rect1);
g.add_shape(rect2);
```
*/
#[derive(Default)]
pub struct Group {
    svg: String,
}

impl Debug for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Group")
    }
}

impl Group {
    /**
    Create a new group to hold shapes. The shapes will inherit the styles from the group,
    unless explicitly overriden in the shape itself.

    ```
    let bounds = Rectangle::new(Point{x: 0., y: 0.}, 500., 500.);
    let mut document = SVG::new("art", &bounds);
    let mut g = Group::new();

    g.set_style(GroupStyle{
      fill: Color::Hex("#111");
      ..Default::default()
    });

    let square = Rectangle::new(Point{x: 100., y: 100., }, 100., 100);
    g.add_shape(square);

    document.add_group(g);
    document.save();

    ```
    */
    pub fn new(style: Option<GroupStyle>) -> Group {
        let svg = match style {
            None => String::from("<g>"),
            Some(style) => format!("<g {style}>"),
        };

        Group { svg }
    }

    /// Add a new shape to the group
    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.svg = format!("{}{}", self.svg, shape.as_svg());
    }

    /// Get the entire SVG string of the group
    pub fn as_svg(&self) -> String {
        format!("{}</g>", self.svg)
    }
}

#[cfg(test)]
mod test {

    use crate::{
        palette::color::Color,
        shapes::{point::Point, rectangle::Rectangle},
    };

    use super::Group;

    #[test]
    fn render() {
        let rect = Rectangle::new(Point { x: 0.0, y: 0.0 }, 10.0, 10.0);
        let mut g = Group::new(Some(super::GroupStyle {
            fill: Some(Color::Hex("#111")),
            stroke: None,
            stroke_width: None,
        }));

        g.add_shape(Box::new(rect));

        assert_eq!(
            g.as_svg(),
            String::from(
                "<g fill=\"#111\"><rect x=\"0.00\" y=\"0.00\" width=\"10.00\" height=\"10.00\"/></g>"
            )
        );
    }
}
