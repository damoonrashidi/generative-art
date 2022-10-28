use std::ops::Range;

use generative_art::{
    group::{Group, GroupStyle},
    palette::Color,
    path::{Path, PathStyle},
    point::Point,
    pointmap::PointMap,
    rectangle::Rectangle,
    svg::SVG,
};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

fn main() {
    let bounds = Rectangle {
        x: 0.,
        y: 0.,
        width: 1000.,
        height: 1000.,
        color: None,
    };
    let scaled_bounds = bounds.scale(0.9);

    let mut svg = SVG::new("Nightfall", bounds);
    let mut pointmap: PointMap<Point> = PointMap::new::<Point>(&bounds, 30);

    let mut rng = ChaCha20Rng::from_entropy();

    let mut g: Group = Group::new();
    g.set_style(GroupStyle {
        stroke: Some(Color::Hex("#111")),
        stroke_width: Some(0.5),
        ..Default::default()
    });

    for _ in 0..100 {
        let x = rng.gen_range(scaled_bounds.x_range());
        let y = gen_weighted(
            scaled_bounds.y..(scaled_bounds.y + scaled_bounds.y * 0.05),
            &mut rng,
        );

        let point = Point { x, y };
        let _ = pointmap.insert(point);
    }

    for _ in 0..2000 {
        let x = rng.gen_range(scaled_bounds.x_range());
        let y = gen_weighted(scaled_bounds.y_range(), &mut rng);

        let point = Point { x, y };
        let _ = pointmap.insert(point);
    }

    pointmap.get_items().iter().for_each(|point| {
        match pointmap.get_neighbors(**point, Some(50.)) {
            Err(_) => {}
            Ok(neighbors) => neighbors.iter().for_each(|neighbor| {
                let path = Path::new(
                    vec![**point, *neighbor],
                    PathStyle {
                        stroke_width: Some(1.),
                        stroke: Some(Color::Hex("#111")),
                        ..Default::default()
                    },
                );

                g.add_shape(Box::new(path));
            }),
        }
    });

    svg.add_group(g);
    svg.save();
}

fn gen_weighted(range: Range<f64>, rng: &mut ChaCha20Rng) -> f64 {
    let a = rng.gen_range(0.0..1.0) as f64;
    let b = rng.gen_range(0.0..1.0);

    ((b - a).abs() * (1.0 + range.end - range.start) + range.start).floor()
}
