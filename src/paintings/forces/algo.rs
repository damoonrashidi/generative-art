use std::rc::Rc;

use crate::{
    paintings::forces::config::ForcesConfig,
    palette::{palettes::Palettes, regional_palette::RegionalPalette, Palette},
    shapes::{
        circle::Circle,
        path::{Path, PathStyle},
        point::Point,
        pointmap::PointMap,
        rectangle::Rectangle,
        shape::Shape,
    },
    svg::document::Document,
    transforms::gen_weighted::WeightedChoice,
};
use noise::{NoiseFn, Seedable, SuperSimplex};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub fn forces(config: Rc<&ForcesConfig>) -> Document<'static> {
    let mut bounds = Rectangle::new(Point(0.0, 0.0), config.size, config.size);
    let (background, colors) = Palettes::orange_autumn();
    let palette = RegionalPalette::from_region(bounds, 5, colors);

    bounds.set_color(background);
    let inner_bounds = bounds.scale(0.9);

    let mut svg = Document::new("Forces", bounds);
    svg.add_shape(Box::new(bounds));
    let mut rng = ChaCha20Rng::from_entropy();

    let mut point_map: PointMap<'_, Circle> = PointMap::new(&bounds, 20);
    let noise = SuperSimplex::new().set_seed(config.seed);

    let center = &bounds.center();

    for _ in 0..config.line_count {
        let mut x: f64 = rng.gen_range(inner_bounds.x_range());
        let mut y: f64 = rng.gen_range(inner_bounds.y_range());

        let line_color = palette.get_color(&Point(x, y));

        let radii = WeightedChoice {
            choices: [(40.0, 10), (100.0, 4), (150.0, 2), (250., 2)],
        };

        let r = radii.get_random_choice().unwrap();

        let step_size = if (0.0..=150.).contains(&r) {
            20.0
        } else {
            180.0
        };

        let mut line = Path {
            points: &mut vec![],
            style: PathStyle {
                stroke_weight: Some(r),
                ..Default::default()
            },
        };

        while inner_bounds.contains(&Point(x, y)) && line.length() < config.max_line_length {
            let smoothness = if (400.0..600.0).contains(&r) {
                config.smoothness * 3.0
            } else {
                config.smoothness
            };

            let n = if false {
                noise.get([x / smoothness, y / smoothness])
            } else {
                get_next(&Point(x, y), center, 0.01)
            };

            x += (config.chaos * n).cos() * step_size;
            y += (config.chaos * n).sin() * step_size;
            let circle = Circle::new(Point(x, y), r);

            if let Ok(neighbors) = point_map.get_neighbors(&circle, None) {
                if neighbors
                    .iter()
                    .any(|neighbor| neighbor.distance(&circle) < r / 2.)
                {
                    break;
                }
            } else {
                break;
            }

            line.add_point(Point(x, y));
        }

        if line.length() > config.min_line_length {
            for point in line.points.iter() {
                let circle = Circle::new(*point, r);
                let _ = point_map.insert(circle);
            }

            if config.split_line_chance > 0.0 && rng.gen_bool(config.split_line_chance) {
                split_line(*line.points, config.split_with_gap)
                    .into_iter()
                    .map(|mut points| {
                        Path::new(
                            &mut points,
                            PathStyle {
                                stroke_weight: Some(r),
                                stroke: palette.get_random_color(),
                                color: None,
                            },
                        )
                    })
                    .for_each(|path| svg.add_shape(Box::new(path)))
            } else {
                line.style = PathStyle {
                    stroke_weight: Some(r),
                    stroke: line_color,
                    color: None,
                };
                svg.add_shape(Box::new(line));
            }
        }
    }

    svg
}

fn split_line(line: Vec<Point>, use_gap: bool) -> Vec<Vec<Point>> {
    let mut rng = thread_rng();
    let mut lines: Vec<Vec<Point>> = vec![];
    let mut last_split = 0;
    for i in 0..line.len() {
        if rng.gen_bool(0.2) {
            if use_gap {
                lines.push(line[last_split..i].into());
            } else {
                lines.push(line[last_split..i + 1].into());
            }
            last_split = i;
        }
    }

    lines.into_iter().filter(|line| line.len() > 1).collect()
}

fn get_next(point: &Point, center: &Point, step: f64) -> f64 {
    let angle = center.angle_to(point);

    angle + step
}
