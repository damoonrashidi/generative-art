use generative_art::{palette::Color, point::Point, rectangle::Rectangle, svg::SVG};
use rand::{thread_rng, Rng};

fn main() {
    let bounds = Rectangle::new(0., 0., 1000., 1000.);

    let mut svg = SVG::new("piet", bounds);
    let root = bounds.scale(0.7);
    let mut rects: Vec<Rectangle> = vec![root];

    let mut rng = thread_rng();

    for _ in 0..10 {
        for i in (0..rects.len()).rev() {
            if let Some(rect) = rects.get(i) {
                if rng.gen_bool(0.3) && rect.width > 0. {
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
        svg.add_shape(Box::new(rect));
    }

    svg.save();
}

fn subdivide(rect: &Rectangle) -> (Rectangle, Rectangle) {
    let padding = 8.;

    let mut rng = thread_rng();

    let split_point = Point {
        x: rng.gen_range(rect.x_range()),
        y: rng.gen_range(rect.y_range()),
    };

    if rng.gen_bool(0.5) {
        return split_horizontally(rect, padding, &split_point);
    }
    split_vertically(rect, padding, &split_point)
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
    let a = Rectangle::new(
        rect.x,
        rect.y,
        split_point.x - padding - rect.x,
        rect.height,
    );
    let b = Rectangle::new(
        split_point.x + padding,
        rect.y,
        rect.x + rect.width - split_point.x - padding,
        rect.height,
    );

    (a, b)
}

fn split_vertically(rect: &Rectangle, padding: f64, split_point: &Point) -> (Rectangle, Rectangle) {
    let a = Rectangle::new(rect.x, rect.y, rect.width, split_point.y - padding - rect.y);
    let b = Rectangle::new(
        rect.x,
        split_point.y + padding,
        rect.width,
        rect.y + rect.height - split_point.y - padding,
    );

    (a, b)
}
