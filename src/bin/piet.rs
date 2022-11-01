use generative_art::{
    group::Group, palette::Color, path::PathStyle, point::Point, rectangle::Rectangle, svg::SVG,
};
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let mut bounds = Rectangle::new(0., 0., 2000., 2000.0);
    let root = bounds.scale(0.95);

    let mut svg = SVG::new("piet", bounds);
    let mut group = Group::new();
    let mut rects: Vec<Rectangle> = vec![root];

    for _ in 0..5 {
        for i in (0..rects.len()).rev() {
            if let Some(rect) = rects.get(i) {
                if rng.gen_bool(0.7) && rect.area() > bounds.area() * 0.01 {
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

    bounds.set_color(Color::HSLa((30, 85., 95., 1.)));
    svg.add_shape(Box::new(bounds));

    for rect in rects {
        let mut path = rect.to_path(PathStyle {
            color: rect.color,
            stroke_width: None,
            stroke: None,
        });

        path.wobble();
        group.add_shape(Box::new(path));
    }

    svg.add_group(group);
    svg.save();
}

fn subdivide(rect: &Rectangle) -> (Rectangle, Rectangle) {
    let scaled = rect.scale(0.9);
    let mut rng = thread_rng();

    let split_point = Point {
        x: rng.gen_range(scaled.x_range()),
        y: rng.gen_range(scaled.y_range()),
    };

    if rng.gen_bool(0.5) {
        return split_horizontally(rect, 16., &split_point);
    }

    split_vertically(rect, 16., &split_point)
}

fn random_color() -> Color {
    let mut rng = thread_rng();
    let hue: u16 = rng.gen_range(0..359);
    let s: f64 = rng.gen_range(40.0..80.0);
    let l: f64 = rng.gen_range(40.0..100.);
    Color::HSLa((hue, s, l, 1.))
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
