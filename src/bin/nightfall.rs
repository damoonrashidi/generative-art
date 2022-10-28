use std::ops::Range;

use generative_art::{
    group::{Group, GroupStyle},
    palette::Color,
    path::{Path, PathStyle},
    point::Point,
    rectangle::Rectangle,
    svg::SVG,
};
use noise::{OpenSimplex, Seedable};

use rand::{Rng, SeedableRng};
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

    let mut rng = ChaCha20Rng::from_entropy();
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, rng.gen_range(0..500));

    let mut g: Group = Group::new();
    g.set_style(GroupStyle {
        stroke: Some(Color::Hex("#111")),
        stroke_width: Some(0.5),
        ..Default::default()
    });

    let mut points: Vec<Point> = vec![];

    for _ in 0..5000 {
        let x = rng.gen_range(scaled_bounds.x_range());
        let y = gen_weighted(scaled_bounds.y_range(), &mut rng);

        let point = Point { x, y };
        points.push(point);
    }

    points.iter().for_each(|point| {
        let neighbors = get_neighbors(&points, point, 50.);

        neighbors.iter().for_each(|neighbor| {
            let path = Path::new(
                vec![point.to_owned(), neighbor.to_owned()],
                PathStyle {
                    stroke_width: Some(1.),
                    stroke: Some(Color::Hex("#111")),
                    ..Default::default()
                },
            );

            g.add_shape(Box::new(path));
        })
    });

    svg.add_group(g);
    svg.save();
}

fn gen_weighted(range: Range<f64>, rng: &mut ChaCha20Rng) -> f64 {
    let a = rng.gen_range(0.0..1.0) as f64;
    let b = rng.gen_range(0.0..1.0);

    ((b - a).abs() * (1.0 + range.end - range.start) + range.start).floor()
}

fn get_neighbors(points: &[Point], point: &Point, proximity: f64) -> Vec<Point> {
    points
        .iter()
        .filter(|neighbor| neighbor.distance(point) < proximity)
        .take(10)
        .copied()
        .collect::<Vec<Point>>()
}
