use itertools::Itertools;
use palette::Color;
use shapes::shape::Shape;

#[derive(Debug, Default)]
pub struct GroupStyle {
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: Option<f64>,
}

#[allow(missing_debug_implementations)]
#[derive(Default)]
pub struct Group {
    pub shapes: Vec<Box<dyn Shape>>,
    style: GroupStyle,
}

impl Group {
    pub fn new() -> Group {
        Group::default()
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
        let stroke_width = self
            .style
            .stroke_width
            .map(|sw| format!(r#" stroke-width="{sw}""#))
            .unwrap_or_default();

        let stroke = self
            .style
            .stroke
            .map(|s| format!(r#" stroke="{s}""#))
            .unwrap_or_default();

        let fill = self
            .style
            .fill
            .map(|f| format!(r#" fill="{f}""#))
            .unwrap_or_else(|| String::from(r#" fill="none""#));

        let g = self.shapes.iter().map(|s| s.as_svg()).join("");

        format!("<g{fill}{stroke}{stroke_width}>{g}</g>")
    }
}

#[cfg(test)]
mod test {
    use palette::Color;
    use shapes::rectangle::Rectangle;

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
                r##"<g fill="#111"><rect x="0.00" y="0.00" width="10.00" height="10.00"/></g>"##
            )
        );
    }
}
