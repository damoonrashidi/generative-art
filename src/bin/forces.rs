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
    const MIN_LINE_LENGHT: f64 = 150.0;

    let bounds = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 2000.,
        height: 2000. * 1.4,
        color: None,
    };
    let mut svg = SVG::new("Forces", bounds);
    let mut rng = ChaCha20Rng::from_entropy();

    let mut point_map: PointMap<Circle> = PointMap::new::<Circle>(&bounds, 90);
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
        let mut x: f64 = rng.gen_range(bounds.x_range());
        let mut y: f64 = rng.gen_range(bounds.y_range()o);
        let r = 90.0;
        let step_size = 25.0;
        let mut line = Path {
            points: vec![],
            style: PathStyle {
                stroke_width: Some(r),
                ..Default::default()
            },
        };

        while bounds.contains(&Point { x, y }) {
            let n = noise.get([x / zoom, y / zoom]);
            x += (distort * n).cos() * step_size;
            y += (distort * n).sin() * step_size;
            let circle = Circle::new(Point { x, y }, r);

            if let Ok(neighbors) = point_map.get_neighbors(circle, None) {
                if neighbors
                    .iter()
                    .any(|neighbor| neighbor.distance(&circle) < 150.)
                {
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
