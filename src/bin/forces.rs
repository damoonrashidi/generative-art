use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use rust_gen_art::{
    circle::Circle, line::Line, palette::Color, point::Point, pointmap::PointMap,
    rectangle::Rectangle, Shape, SVG,
};

fn main() {
    const WIDTH: f64 = 3000.0;
    const HEIGHT: f64 = WIDTH * 1.4;
    const PADDING: f64 = WIDTH / 10.0;
    const MAX_LINE_LENGTH: f64 = 2000.0;

    let mut svg = SVG::new("Forces", WIDTH, HEIGHT);
    let mut rng = ChaCha20Rng::from_entropy();

    let mut dots: PointMap<Circle> = PointMap::new(WIDTH, HEIGHT);

    let bounds = Rectangle {
        x: PADDING,
        y: PADDING,
        width: WIDTH - (PADDING * 2.0),
        height: HEIGHT - (PADDING * 2.0),
        color: None,
    };

    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, rng.gen_range(1..100_000));

    let distort = rng.gen_range(1.5..4.2);
    let zoom = rng.gen_range(1_200.0..4_000.0);

    for _ in 0..10_000 {
        let mut x: f64 = rng.gen_range(PADDING..WIDTH - PADDING);
        let mut y: f64 = rng.gen_range(PADDING..HEIGHT - PADDING);
        let mut r = 15.0;
        let mut step_size = 30.0;
        let (h, s, l, a) = (rng.gen_range(350..360), 50.0, 50.0, 1.0);

        if rng.gen_bool(0.2) {
            r *= 5.0;
            step_size = 120.0;
        }

        let mut line = Line {
            points: vec![],
            stroke: Color::HSLa(h, s, l, a),
            stroke_width: r,
        };

        while bounds.contains(Point { x, y }) && line.length() < MAX_LINE_LENGTH {
            let n = noise.get([x / zoom, y / zoom]);
            x += (distort * n).cos() * step_size;
            y += (distort * n).sin() * step_size;

            let circle = Circle::new(x, y, r);

            if let Some(neighbors) = dots.get_neighbors(circle) {
                if neighbors.iter().any(|dot| circle.intersects(dot)) {
                    break;
                }
            }

            line.add_point(Point { x, y });
        }

        if line.length() > 200.0 {
            line.points
                .iter()
                .for_each(|point| dots.insert(Circle::new(point.x, point.y, r)));

            svg.add(Box::new(line));
        }
    }

    svg.save();
}
