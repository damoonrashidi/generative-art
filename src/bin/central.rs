use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use rust_gen_art::{
    circle::Circle, helpers::map, palette::Color, path::Path, point::Point, pointmap::PointMap,
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

    let mut pointmap = PointMap::new(&bounds, 5);

    let mut rng = ChaCha20Rng::from_entropy();
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, rng.gen_range(0..500));

    for _ in 0..3000 {
        let mut x = rng.gen_range(bounds.x_range());
        let mut y = rng.gen_range(bounds.y_range());
        let distance_to_center = bounds.center().distance(Point { x, y });

        let r = 200.0 - map(distance_to_center, 0.0..bounds.height / 2., 50.0..250.0);

        let step: f64 = if r > 80. { r } else { 20. };

        let mut points: Vec<Point> = vec![];

        while bounds.contains(Point { x, y }) {
            let n = noise.get([x / 500.0, y / 500.0]);

            let point = Point { x, y };
            points.push(point);

            x += (n * 3.).cos() * step;
            y += (n * 3.).sin() * step;

            if let Ok(neighbors) = pointmap.get_neighbors(Circle::new(x, y, r)) {
                if Circle::new(x, y, r).instersects_any(neighbors) {
                    break;
                }
            }
        }

        let line = Path::new(points, r, Some(Color::Hex("#111")));

        if line.length() > 50.0 {
            line.points.iter().for_each(|point| {
                let _ = pointmap.insert(Circle::new(point.x, point.y, r));
            });
            svg.add_shape(Box::new(line));
        }
    }

    svg.save();
}
