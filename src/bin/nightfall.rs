use std::ops::Range;

use noise::{OpenSimplex, Seedable};
use num_traits::Float;
use rand::{distributions::uniform::SampleUniform, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rust_gen_art::{
    circle::Circle, palette::Color, path::Path, point::Point, rectangle::Rectangle, svg::SVG,
};

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

    let mut points: Vec<Point> = vec![];

    for _ in 0..10000 {
        let x = rng.gen_range(scaled_bounds.x_range());
        let y = gen_weighted(scaled_bounds.y_range());

        let point = Point { x, y };

        points.push(point);

        let mut circle = Circle::new(point, 1.);
        circle.set_color(Color::Hex("#111"));

        svg.add_shape(Box::new(circle));
    }

    points.iter().for_each(|point| {
        let neighbors = points
            .iter()
            .filter(|n| n.distance(point) < 50.)
            .collect::<Vec<&Point>>();

        let max = neighbors.len().min(50);
        let sliced = &neighbors[0..max];

        sliced.iter().for_each(|n| {
            svg.add_shape(Box::new(Path::new(
                vec![n.clone().to_owned(), point.clone().to_owned()],
                0.5,
                Some(Color::Hex("#111")),
            )));
        });
    });

    svg.save();
}

fn gen_weighted<T: Float + SampleUniform>(range: Range<T>) -> T {
    let mut rng = ChaCha20Rng::from_entropy();

    let a = rng.gen_range(range.start..range.end);
    let b = rng.gen_range(range.start..range.end);

    return (b - a).max(range.start);
}
