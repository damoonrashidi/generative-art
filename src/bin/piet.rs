use generative_art::{palette::Color, rectangle::Rectangle, svg::SVG};
use rand::{thread_rng, Rng};

fn main() {
    let bounds = Rectangle::new(0., 0., 1000., 1000.);

    let mut svg = SVG::new("piet", bounds);
    let root = bounds.scale(0.7);
    let mut rects: Vec<Rectangle> = vec![root];

    let mut rng = thread_rng();

    for split in 0..10 {
        for i in (0..rects.len()).rev() {
            if let Some(rect) = rects.get(i) {
                if rng.gen_bool(0.3) {
                    let (mut a, mut b) = subdivide(rect);
                    rects.remove(i);

                    a.set_color(random_color(split));
                    b.set_color(random_color(split));
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
    let split_horizontally = rng.gen_bool(0.5);

    if split_horizontally {
        let a = Rectangle::new(rect.x, rect.y, rect.width / 2. - padding, rect.height);
        let b = Rectangle::new(
            rect.x + rect.width / 2. + padding,
            rect.y,
            rect.width / 2. - padding,
            rect.height,
        );

        return (a, b);
    }

    let a = Rectangle::new(
        rect.x,
        rect.y,
        rect.width,
        rect.height - rect.height / 2. - padding,
    );
    let b = Rectangle::new(
        rect.x,
        rect.y + rect.height / 2. + padding,
        rect.width,
        rect.height / 2. - padding,
    );

    (a, b)
}

fn random_color(split: u8) -> Color {
    let mut rng = thread_rng();
    let hue: u16 = rng.gen_range(0..70);
    let saturation = 0.1 * split as f64;
    Color::HSLa((hue, saturation, 50., 1.))
}
