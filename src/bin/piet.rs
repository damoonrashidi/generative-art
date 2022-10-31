use generative_art::{
    palette::Color, path::PathStyle, point::Point, rectangle::Rectangle, svg::SVG,
};
use rand::{thread_rng, Rng};

fn main() {
    let bounds = Rectangle::new(0., 0., 1000., 1000.0 * 1.4);

    let mut svg = SVG::new("piet", bounds);
    let root = bounds.scale(0.7);
    let mut rects: Vec<Rectangle> = vec![root];

    let mut rng = thread_rng();

    for _ in 0..20 {
        for i in (0..rects.len()).rev() {
            if let Some(rect) = rects.get(i) {
                if rng.gen_bool(0.3) && rect.area() > bounds.area() * 0.1 {
                    let (mut a, mut b) = subdivide(rect);
                    rects.remove(i);

                    a.set_color(random_color());
                    b.set_color(random_color());
                    rects.push(a);
                    rects.push(b);
                }
            }
        }
    }

    for rect in rects {
        let path = rect.to_path(PathStyle {
            color: rect.color,
            stroke_width: None,
            stroke: None,
        });
        svg.add_shape(Box::new(path));
    }

    svg.save();
}

fn subdivide(rect: &Rectangle) -> (Rectangle, Rectangle) {
    let scaled = rect.scale(0.8);
    let mut rng = thread_rng();
    const PADDING: f64 = 4.;

    let split_point = Point {
        x: rng.gen_range(scaled.x_range()),
        y: rng.gen_range(scaled.y_range()),
    };

    if rng.gen_bool(0.5) {
        return split_horizontally(rect, PADDING, &split_point);
    }

    split_vertically(rect, PADDING, &split_point)
}

fn random_color() -> Color {
    let mut rng = thread_rng();
    let hue: u16 = rng.gen_range(0..70);
    Color::HSLa((hue, 50., 50., 1.))
}

fn split_horizontally(
    rect: &Rectangle,
    padding: f64,
    split_point: &Point,
) -> (Rectangle, Rectangle) {
    (
        Rectangle::new(
            rect.x,
            rect.y,
            split_point.x - padding - rect.x,
            rect.height,
        ),
        Rectangle::new(
            split_point.x + padding,
            rect.y,
            rect.x + rect.width - split_point.x - padding,
            rect.height,
        ),
    )
}

fn split_vertically(rect: &Rectangle, padding: f64, split_point: &Point) -> (Rectangle, Rectangle) {
    (
        Rectangle::new(rect.x, rect.y, rect.width, split_point.y - padding - rect.y),
        Rectangle::new(
            rect.x,
            split_point.y + padding,
            rect.width,
            rect.y + rect.height - split_point.y - padding,
        ),
    )
}
