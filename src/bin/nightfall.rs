use noise::{OpenSimplex, Seedable};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rust_gen_art::{point::Point, pointmap::PointMap, rectangle::Rectangle, svg::SVG};

fn main() {
    let bounds = Rectangle {
        x: 0.,
        y: 0.,
        width: 1500.,
        height: 1500.,
        color: None,
    };

    let mut svg = SVG::new("Nightfall", bounds);
    let mut map: PointMap<Point> = PointMap::new::<Point>(&bounds, 5);

    let mut rng = ChaCha20Rng::from_entropy();
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, rng.gen_range(0..500));

    for _ in 0..500 {
        let x = rng.gen_range(bounds.x_range());
        let y = rng.gen_range(bounds.y_range());

        let point = Point { x, y };

        match map.insert(point) {
            Ok(_) => (),
            Err(_) => break,
        }
    }

    svg.save();
}
