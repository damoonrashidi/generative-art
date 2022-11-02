/**
* Parameters
* -----------------------------
* size: f64
* color_scheme: WeightedPalette
* line_weights: Vec<{radius: f64, step_size: f64, probability: f64}>
* density: f64
* distort: f64
* zoom: f64
*/
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

    let bounds = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 2000.,
        height: 2000. * 1.4,
        color: Some(Color::Hex("#181D31")),
    };
    let mut svg = SVG::new("Forces", bounds);
    let mut rng = ChaCha20Rng::from_entropy();
    let palette = WeightedPalette::new(vec![
        (Color::Hex("#678983"), 1),
        (Color::Hex("#E6DDC4"), 3),
        (Color::Hex("#F0E9D2"), 5),
    ]);

    let mut point_map: PointMap<Circle> = PointMap::new::<Circle>(&bounds, 20);
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, 5);

    let distort = 1.5;
    let zoom = 800.;
    svg.add_shape(Box::new(bounds));

    let mut group = Group::new();

    group.set_style(GroupStyle {
        fill: None,
        stroke: Some(Color::Hex("#111")),
        stroke_width: Some(15.0),
    });

    for i in 0..5000 {
        let mut x: f64 = rng.gen_range(bounds.x_range());
        let mut y: f64 = rng.gen_range(bounds.y_range());
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

        while bounds.contains(&Point { x, y }) {
            let n = noise.get([x / zoom, y / zoom]);
            x += (distort * n).cos() * step_size;
            y += (distort * n).sin() * step_size;
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
    svg.save();
}
