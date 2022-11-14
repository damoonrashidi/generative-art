use palette::palette::Color;
use shapes::shape::Shape;

#[derive(Default)]
pub struct GroupStyle {
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: Option<f64>,
}

#[derive(Default)]
pub struct Group {
    pub shapes: Vec<Box<dyn Shape>>,
    style: GroupStyle,
}

impl Group {
    pub fn new() -> Group {
        Group {
            shapes: vec![],
            style: GroupStyle::default(),
        }
    }

    pub fn set_style(&mut self, style: GroupStyle) {
        self.style.fill = style.fill;
        self.style.stroke = style.stroke;
        self.style.stroke_width = style.stroke_width;
    }

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
