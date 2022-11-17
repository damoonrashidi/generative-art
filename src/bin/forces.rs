use generative_art::forces_config::ForcesConfig;

use noise::{NoiseFn, OpenSimplex, Seedable};
use palette::{color::Color, weighted_palette::WeightedPalette};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use shapes::{
    blob::Blob,
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
    let bounds = Rectangle::new(0.0, 0.0, config.size, config.size * 1.4);
    let inner_bounds = bounds.scale(0.9);

    let mut svg = SVG::new("Forces", bounds);
    let mut rng = ChaCha20Rng::from_entropy();
    let palette = WeightedPalette::new(vec![
        (Color::HSLa((0, 100., 98., 1.)), 1),
        (Color::HSLa((75, 100., 81., 1.)), 1),
        (Color::HSLa((34, 61., 91., 1.)), 1),
        (Color::HSLa((28, 82., 56., 1.)), 1),
        (Color::HSLa((0, 8., 21., 1.)), 1),
        (Color::HSLa((0, 44., 44., 1.)), 1),
    ]);

    let mut color_bounds: Vec<Blob> = vec![];

    for _ in 0..20 {
        let x = rng.gen_range(bounds.x_range());
        let y = rng.gen_range(bounds.y_range());
        let r = rng.gen_range((bounds.width / 10.0)..(bounds.width / 7.));
        let color = palette.get_random_color();

        let blob = Blob::new(Point { x, y }, r, color);

        color_bounds.push(blob);
    }

    let mut point_map: PointMap<Circle> = PointMap::new(&bounds, 20);
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, config.seed);

    let mut group = Group::new();

    group.set_style(GroupStyle {
        fill: None,
        stroke: Some(Color::Hex("#111")),
        stroke_width: Some(15.0),
    });

    for i in 0..config.line_count {
        let mut x: f64 = rng.gen_range(inner_bounds.x_range());
        let mut y: f64 = rng.gen_range(inner_bounds.y_range());

        let line_color: Option<Color> = match color_bounds
            .iter()
            .find(|region| region.contains(&Point { x, y }))
        {
            Some(region) => region.color,
            _ => Some(Color::HSLa((0, 44., 44., 1.))),
        };

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

            if let Ok(neighbors) = point_map.get_neighbors(&circle, None) {
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

            if rng.gen_bool(0.5) {
                let (first, second) = split_line(line.points);

                let l1 = Path::new(
                    first,
                    PathStyle {
                        stroke_width: Some(r),
                        stroke: line_color,
                        color: None,
                    },
                );

                let l2 = Path::new(
                    second,
                    PathStyle {
                        stroke_width: Some(r),
                        stroke: palette.get_random_color(),
                        color: None,
                    },
                );

                group.add_shape(Box::new(l1));
                group.add_shape(Box::new(l2));
            } else {
                line.style = PathStyle {
                    stroke_width: Some(r),
                    stroke: line_color,
                    color: None,
                };
                group.add_shape(Box::new(line));
            }
        }
    }

    svg.add_group(group);
    svg.save(Some(config.into()));
}

fn split_line(line: Vec<Point>) -> (Vec<Point>, Vec<Point>) {
    let mut rng = thread_rng();
    let split_point: usize = rng.gen_range(1..line.len());

    let first = line[0..split_point + 1].into();
    let second = line[split_point - 1..line.len()].into();

    (first, second)
}
