use generative_art::{
    circle::Circle,
    group::{Group, GroupStyle},
    palette::Color,
    path::{Path, PathStyle},
    point::Point,
    pointmap::PointMap,
    rectangle::Rectangle,
    shape::Shape,
    svg::SVG,
};
use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

fn main() {
    const WIDTH: f64 = 3000.0;
    const HEIGHT: f64 = WIDTH * 1.4;
    const PADDING: f64 = WIDTH / 10.0;
    const MAX_LINE_LENGTH: f64 = 800.0;
    const MIN_LINE_LENGHT: f64 = 150.0;

    let mut svg = SVG::new(
        "Forces",
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: WIDTH,
            height: HEIGHT,
            color: None,
        },
    );
    let mut rng = ChaCha20Rng::from_entropy();

    let bounds = Rectangle {
        x: PADDING,
        y: PADDING,
        width: WIDTH - (PADDING * 2.0),
        height: HEIGHT - (PADDING * 2.0),
        color: Rectangle::default().color,
    };

    let mut point_map: PointMap<Circle> = PointMap::new::<Circle>(&bounds, 30);
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, rng.gen_range(1..100_000));

    let distort = rng.gen_range(1.5..3.2);
    let zoom = rng.gen_range(500.0..900.0);

    let mut group = Group::new();

    group.set_style(GroupStyle {
        fill: None,
        stroke: Some(Color::Hex("#111")),
        stroke_width: Some(15.0),
    });

    for _ in 0..1000 {
        let mut x: f64 = rng.gen_range(PADDING..WIDTH - PADDING);
        let mut y: f64 = rng.gen_range(PADDING..HEIGHT - PADDING);
        let r = 90.0;
        let step_size = 25.0;
        let mut line = Path {
            points: vec![],
            style: PathStyle {
                stroke_width: Some(r),
                ..Default::default()
            },
        };

        while bounds.contains(&Point { x, y }) && line.length() < MAX_LINE_LENGTH {
            let n = noise.get([x / zoom, y / zoom]);
            x += (distort * n).cos() * step_size;
            y += (distort * n).sin() * step_size;
            let circle = Circle::new(Point { x, y }, r);

            if let Ok(neighbors) = point_map.get_neighbors(circle) {
                if circle.instersects_any(neighbors) {
                    break;
                }
            }

            line.add_point(Point { x, y });
        }

        if line.length() > MIN_LINE_LENGHT {
            for point in line.points.iter() {
                let circle = Circle::new(*point, r);
                let _ = point_map.insert(circle);
            }

            group.add_shape(Box::new(line));
        }
    }

    svg.add_group(group);

    svg.save();
}
