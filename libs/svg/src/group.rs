use std::fmt::Debug;

use palette::color::Color;
use shapes::shape::Shape;

#[derive(Debug, Default)]
pub struct GroupStyle {
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: Option<f64>,
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
    pub shapes: Vec<Box<dyn Shape>>,
    style: GroupStyle,
}

impl Debug for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Group<{}>", self.shapes.len())
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
    pub fn new() -> Group {
        Group {
            shapes: vec![],
            style: GroupStyle::default(),
        }
    }

    /// Set the style for the entire group, these can be overriden
    /// on the shape level as well by just applying shape styles.
    pub fn set_style(&mut self, style: GroupStyle) {
        self.style.fill = style.fill;
        self.style.stroke = style.stroke;
        self.style.stroke_width = style.stroke_width;
    }

    /// Add a new shape to the group
    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    pub fn as_svg(&self) -> String {
        let stroke_width: String = match &self.style.stroke_width {
            Some(width) => format!(" stroke-width=\"{}\"", width),
            None => String::from(""),
        };

        let stroke: String = match &self.style.stroke {
            Some(color) => format!(" stroke=\"{}\"", color),
            None => String::from(""),
        };

        let fill: String = match &self.style.fill {
            Some(color) => format!(" fill=\"{}\"", color),
            None => String::from(" fill=\"none\""),
        };

        let g = self
            .shapes
            .iter()
            .fold(format!("<g {fill}{stroke}{stroke_width}>"), |r, shape| {
                format!("{}{}", r, shape.as_svg().trim())
            });

        format!("{}</g>", g)
    }
}

#[cfg(test)]
mod test {
    use crate::{palette::Color, rectangle::Rectangle};

    use super::Group;

    #[test]
    fn render() {
        let rect = Rectangle::new(0.0, 0.0, 10.0, 10.0);
        let mut g = Group::new();

        g.set_style(super::GroupStyle {
            fill: Some(Color::Hex("#111")),
            stroke: None,
            stroke_width: None,
        });

        g.add_shape(Box::new(rect));

        assert_eq!(
            g.as_svg(),
            String::from(
                "<g fill=\"#111\"><rect x=\"0.00\" y=\"0.00\" width=\"10.00\" height=\"10.00\"/></g>"
            )
        );
    }
}
