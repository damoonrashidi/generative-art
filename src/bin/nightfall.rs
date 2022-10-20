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
use noise::{OpenSimplex, Seedable};
use num_traits::Float;
use rand::{distributions::uniform::SampleUniform, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

fn main() {
    let bounds = Rectangle {
        x: 0.,
        y: 0.,
        width: 1500.,
        height: 1500. * 1.4,
        color: None,
    };
    let scaled_bounds = bounds.scale(0.9);

    let mut svg = SVG::new("Nightfall", bounds);

    let mut map: PointMap<Point> = PointMap::new::<Point>(&bounds, 10);
    let mut rng = ChaCha20Rng::from_entropy();
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, rng.gen_range(0..500));

    let mut g: Group = Group::new();
    g.set_style(GroupStyle {
        stroke: Some(Color::Hex("#111")),
        stroke_width: Some(0.5),
        ..Default::default()
    });

    for _ in 0..3000 {
        let x = rng.gen_range(scaled_bounds.x_range());
        let y = gen_weighted(scaled_bounds.y_range());

        let point = Point { x, y };
        let _ = map.insert(point);
    }

    let points = map.get_items();

    for point in points {
        match map.get_neighbors(point) {
            Ok(list) => list
                .into_iter()
                .filter(|neighbor| point.distance(neighbor) < 150.)
                .take(10)
                .for_each(|neighbor| {
                    g.add_shape(Box::new(Path::new(
                        vec![point, neighbor],
                        PathStyle {
                            stroke_width: Some(0.5),
                            stroke: Some(Color::Hex("#111")),
                            ..Default::default()
                        },
                    )))
                }),
            Err(_) => break,
        };
    }

    svg.add_group(Box::new(g));
    svg.save();
}

fn gen_weighted<T: Float + SampleUniform>(range: Range<T>) -> T {
    let mut rng = ChaCha20Rng::from_entropy();

    let a = rng.gen_range(range.start..range.end);
    let b = rng.gen_range(range.start..range.end);

    return (b - a).max(range.start);
}
