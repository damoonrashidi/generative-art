use generative_art::forces_config::ForcesConfig;

use noise::{NoiseFn, OpenSimplex, Seedable};
use palette::palette::{Color, WeightedPalette};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use shapes::{
    circle::Circle,
    path::{Path, PathStyle},
    point::Point,
    pointmap::PointMap,
    rectangle::Rectangle,
    shape::Shape,
};
use svg::{
    group::{Group, GroupStyle},
    svg::SVG,
};

fn main() {
    const MIN_LINE_LENGHT: f64 = 80.0;

    let config = ForcesConfig::new();
    let mut bounds = Rectangle::new(0.0, 0.0, config.size, config.size * 1.4);
    let inner_bounds = bounds.scale(0.9);

    bounds.set_color(Color::Hex("#fff"));

    let mut svg = SVG::new("Forces", bounds);
    let mut rng = ChaCha20Rng::from_entropy();
    let palette = WeightedPalette::new(vec![
        (Color::Hex("#F9F2ED"), 1),
        (Color::Hex("#3AB0FF"), 3),
        (Color::Hex("#FFB562"), 3),
        (Color::Hex("#F87474"), 3),
    ]);

    let mut point_map: PointMap<Circle> = PointMap::new(&bounds, 20);
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, config.seed);

    svg.add_shape(Box::new(bounds));

    let mut group = Group::new();

    group.set_style(GroupStyle {
        fill: None,
        stroke: Some(Color::Hex("#111")),
        stroke_width: Some(15.0),
    });

    for i in 0..config.line_count {
        let mut x: f64 = rng.gen_range(inner_bounds.x_range());
        let mut y: f64 = rng.gen_range(inner_bounds.y_range());
        let mut r = 65.0;
        let mut step_size = 50.0;

        if rng.gen_bool(0.7) && i < 5 {
            r = 200.;
            step_size = 250.;
        } else if rng.gen_bool(0.1) {
            r = 40.;
            step_size = 30.;
        }

        let mut line = Path {
            points: vec![],
            style: PathStyle {
                stroke_width: Some(r),
                ..Default::default()
            },
        };

        while inner_bounds.contains(&Point { x, y }) {
            let n = noise.get([x / config.smoothness, y / config.smoothness]);
            x += (config.chaos * n).cos() * step_size;
            y += (config.chaos * n).sin() * step_size;
            let circle = Circle::new(Point { x, y }, r);

            if let Ok(neighbors) = point_map.get_neighbors(circle, None) {
                if neighbors
                    .iter()
                    .any(|neighbor| neighbor.distance(&circle) < 0.)
                {
                    break;
                }
            } else {
                break;
            }

            line.add_point(Point { x, y });
        }

        if line.length() > MIN_LINE_LENGHT {
            for point in line.points.iter() {
                let circle = Circle::new(*point, r);
                let _ = point_map.insert(circle);
            }

            line.style = PathStyle {
                stroke_width: Some(r),
                stroke: palette.get_random_color(),
                color: None,
            };

            group.add_shape(Box::new(line));
        }
    }

    svg.add_group(group);
    svg.save(Some(config.into()));
}
