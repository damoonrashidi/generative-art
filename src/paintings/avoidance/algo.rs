use rand::{thread_rng, Rng};

use crate::{
    palette::palettes::Palettes,
    shapes::{
        circle::Circle,
        path::{Path, PathStyle},
        point::Point,
        pointmap::PointMap,
        rectangle::Rectangle,
        shape::Shape,
    },
    svg::document::Document,
};

use super::config::{AvoidanceConfig, Trail};

pub fn avoidance(config: &AvoidanceConfig) -> String {
    let mut bounds = Rectangle::new(Point(0.0, 0.0), config.size, config.size * 1.4);
    let (bg, palette) = Palettes::wild();
    bounds.set_color(bg);

    let mut pointmap: PointMap<'_, Circle> = PointMap::new(&bounds, 50);

    let mut svg = Document::new("Avoidance", bounds);

    svg.add_shape(Box::new(bounds));

    let mut rng = thread_rng();

    for _ in 0..200 {
        let mut point = Trail::new(
            40.,
            Point(
                rng.gen_range(bounds.x_range()),
                rng.gen_range(bounds.y_range()),
            ),
            rng.gen_range(-0.1..1.0),
        );

        let mut points: Vec<Point> = vec![];
        while bounds.contains(&point.position) && points.len() < 20 {
            points.push(point.position);
            let circle = Circle::new(point.position, point.radius);

            let neighbors = pointmap
                .get_neighbors(&circle, Some(point.radius / 2.))
                .unwrap();

            let next_options: Vec<(Point, f64)> = point
                .move_candidates(&config.scan_distance, &config.scan_angle)
                .into_iter()
                .filter_map(|candidate| {
                    let c = Circle::new(candidate.0, point.radius);
                    if !c.instersects_any(neighbors.clone()) {
                        Some(candidate)
                    } else {
                        None
                    }
                })
                .collect();

            if next_options.len() > 0 {
                let pick = rng.gen_range(0..next_options.len());
                let (position, direction) = next_options[pick];
                point.position = position;
                point.direction = direction;
            } else {
                break;
            }
        }

        let path = Path::new(
            points.clone(),
            PathStyle {
                stroke_weight: Some(config.size / 300.),
                stroke: palette.get_random_color(),
                ..Default::default()
            },
        );

        for point in points {
            let _ = pointmap.insert(Circle::new(point, 40.));
        }

        svg.add_shape(Box::new(path));
    }

    svg.generate()
}
