use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use rust_gen_art::{
    circle::Circle,
    helpers::map,
    palette::Color,
    path::{Path, PathStyle},
    point::Point,
    pointmap::PointMap,
    rectangle::Rectangle,
    shape::Shape,
    svg::SVG,
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

    let inner_bounds = bounds.scale(0.9);
    let mut pointmap: PointMap<Circle> = PointMap::new::<Circle>(&inner_bounds, 50);

    let mut rng = ChaCha20Rng::from_entropy();
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, rng.gen_range(0..500));

    for _ in 0..3000 {
        let mut x = rng.gen_range(inner_bounds.x_range());
        let mut y = rng.gen_range(inner_bounds.y_range());
        let distance_to_center = inner_bounds.center().distance(&Point { x, y });

        let r = map(
            distance_to_center,
            0.0..inner_bounds.height / 1.5,
            40.0..2.0,
        );

        let step: f64 = if r > 50. { r * 2. } else { 20. };

        let mut line = Path::new(
            vec![],
            PathStyle {
                stroke_width: Some(r),
                ..Default::default()
            },
        );

        while bounds.contains(Point { x, y }) {
            let n = noise.get([x / 500.0, y / 500.0]);

            let point = Point { x, y };
            line.add_point(point);

            if line.length() > 100. {
                break;
            }

            x += (n * 3.).cos() * step;
            y += (n * 3.).sin() * step;

            if let Ok(neighbors) = pointmap.get_neighbors(Circle::new(Point { x, y }, r)) {
                if Circle::new(Point { x, y }, r).instersects_any(neighbors) {
                    break;
                }
            }
        }

        if line.length() > 50.0 {
            line.points.iter().for_each(|point| {
                let _ = pointmap.insert(Circle::new(point.clone(), r));
            });
            svg.add_shape(Box::new(line));
        }
    }

    svg.save();
}
