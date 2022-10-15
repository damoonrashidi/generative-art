use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use rust_gen_art::{
    circle::Circle, palette::Color, path::Path, point::Point, pointmap::PointMap,
    rectangle::Rectangle, Shape, SVG,
};

fn main() {
    let bounds = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 2000.0,
        height: 2000.0 * 1.4,
        color: Some(Color::Hex("#f00")),
    };

    let mut svg = SVG::new("Central", bounds);

    let mut pointmap = PointMap::new(&bounds, 50);

    let mut rng = ChaCha20Rng::from_entropy();
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, 1);

    let mut r = 150.0;

    for _ in 0..90 {
        let mut x = rng.gen_range(bounds.x_range());
        let mut y = rng.gen_range(bounds.y_range());

        let mut points: Vec<Point> = vec![];
        let mut steps = 50;

        while bounds.contains(Point { x, y }) && steps >= 0 {
            let n = noise.get([x / 250.0, y / 250.0]);

            let point = Point { x, y };
            points.push(point);

            x += (n * 2.5).cos() * (r / 5.0);
            y += (n * 2.5).sin() * (r / 5.0);

            if let Ok(neighbors) = pointmap.get_neighbors(Circle::new(x, y, r)) {
                if Circle::new(x, y, 150.0).instersects_any(neighbors) {
                    break;
                }
            }

            steps -= 1;
        }

        let line = Path::new(points, r, Some(Color::Hex("#111")));

        line.points.iter().for_each(|point| {
            let _ = pointmap.insert(Circle::new(point.x, point.y, r));
        });
        r /= 1.05;

        svg.add_shape(Box::new(line));
    }

    svg.save();
}
