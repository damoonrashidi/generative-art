use generative_art::{
    configs::piet_config::PietConfig,
    palette::palettes::Palettes,
    shapes::{path::PathStyle, point::Point, rectangle::Rectangle},
    svg::{document::Document, group::Group},
};
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let (background, palette) = Palettes::orange_autumn();
    let config = PietConfig::new();
    let mut bounds = Rectangle::new(Point(0., 0.), config.size, config.size * 1.4);
    let root = bounds.scale(0.95);

    let mut svg = Document::new("piet", bounds);
    let mut group = Group::new(None);
    let mut rects = vec![root];

    for _ in 0..config.rounds {
        for i in (0..rects.len()).rev() {
            if let Some(rect) = rects.get(i) {
                if rng.gen_bool(config.split_chance) && rect.area() > bounds.area() * 0.01 {
                    let (mut a, mut b) = subdivide(rect);
                    rects.remove(i);

                    if let Some(a_color) = palette.get_random_color() {
                        a.set_color(a_color);
                    }

                    if let Some(b_color) = palette.get_random_color() {
                        b.set_color(b_color);
                    }

                    rects.push(a);
                    rects.push(b);
                }
            }
        }
    }

    bounds.set_color(background);
    svg.add_shape(Box::new(bounds));

    rects
        .iter()
        .map(|rect| {
            let mut path = rect.to_path(PathStyle {
                color: rect.color,
                stroke_weight: None,
                stroke: None,
            });
            path.wobble();
            path
        })
        .for_each(|path| {
            group.add_shape(Box::new(path));
        });

    svg.add_group(group);
    svg.save();
}

fn subdivide(rect: &Rectangle) -> (Rectangle, Rectangle) {
    let scaled = rect.scale(0.9);
    let mut rng = thread_rng();

    let split_point = Point(
        rng.gen_range(scaled.x_range()),
        rng.gen_range(scaled.y_range()),
    );

    if rng.gen_bool(0.5) {
        return split_horizontally(rect, 16., &split_point);
    }

    split_vertically(rect, 16., &split_point)
}

fn split_horizontally(
    rect: &Rectangle,
    padding: f64,
    split_point: &Point,
) -> (Rectangle, Rectangle) {
    (
        Rectangle::new(
            rect.position,
            split_point.0 - padding - rect.position.0,
            rect.height,
        ),
        Rectangle::new(
            Point(split_point.0 + padding, rect.position.1),
            rect.position.0 + rect.width - split_point.0 - padding,
            rect.height,
        ),
    )
}

fn split_vertically(rect: &Rectangle, padding: f64, split_point: &Point) -> (Rectangle, Rectangle) {
    (
        Rectangle::new(
            rect.position,
            rect.width,
            split_point.1 - padding - rect.position.1,
        ),
        Rectangle::new(
            Point(rect.position.0, split_point.1 + padding),
            rect.width,
            rect.position.1 + rect.height - split_point.1 - padding,
        ),
    )
}
