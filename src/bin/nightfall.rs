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

    for _ in 0..2000 {
        let x = rng.gen_range(scaled_bounds.x_range());
        let y = gen_weighted(scaled_bounds.y_range());

        let point = Point { x, y };
        points.push(point);
    }

    points.iter().for_each(|point| {
        let neighbors = get_neighbors(&points, point, 80.);

        neighbors.iter().for_each(|neighbor| {
            g.add_shape(Box::new(Path::new(
                vec![point.to_owned(), neighbor.to_owned()],
                PathStyle {
                    stroke_width: Some(0.5),
                    stroke: Some(Color::Hex("#111")),
                    ..Default::default()
                },
            )))
        })
    });

    svg.add_group(Box::new(g));
    svg.save();
}

fn gen_weighted<T: Float + SampleUniform>(range: Range<T>) -> T {
    let mut rng = ChaCha20Rng::from_entropy();

    let a = rng.gen_range(range.start..range.end);
    let b = rng.gen_range(range.start..range.end);

    return (b - a).max(range.start);
}

fn get_neighbors(points: &Vec<Point>, point: &Point, proximity: f64) -> Vec<Point> {
    points
        .iter()
        .filter(|neighbor| neighbor.distance(point) < proximity)
        .map(|p| p.to_owned())
        .take(60)
        .collect::<Vec<Point>>()
}
