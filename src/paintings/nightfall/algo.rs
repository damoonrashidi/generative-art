use std::rc::Rc;

use crate::{
    palette::color::Color,
    shapes::{
        circle::Circle,
        path::{Path, PathStyle},
        point::Point,
        pointmap::PointMap,
        rectangle::Rectangle,
        shape::Shape,
    },
    svg::document::Document,
    transforms::{gen_weighted::gen_weighted, map},
};
use rand::{thread_rng, Rng};

use super::config::{ForceMethod, NightfallConfig};

pub fn generate_nightfall(config: Rc<&NightfallConfig>) -> Document<'static> {
    let bounds = Rectangle {
        position: Point(0.0, 0.0),
        width: config.size,
        height: config.size * 1.4,
        color: Some(Color::Hex("#111")),
    };
    let scaled_bounds = bounds.scale(0.9);

    let mut svg = Document::new("Nightfall", bounds);
    svg.add_shape(Box::new(bounds));

    let mut pointmap: PointMap<'_, Point> =
        PointMap::new(&bounds, (config.distance / config.size * 1000.0) as usize);

    let mut rng = thread_rng();

    for _ in 0..config.points / 10 {
        let x = rng.gen_range(scaled_bounds.x_range());
        let y = gen_weighted(
            scaled_bounds.position.1..(scaled_bounds.position.1 + scaled_bounds.position.1 * 0.05),
            &mut rng,
        );

        let point = Point(x, y);
        let _ = pointmap.insert(point);
    }

    let spheres = [0; 3]
        .iter()
        .map(|_| {
            Circle::new(
                Point(
                    rng.gen_range(scaled_bounds.x_range()),
                    rng.gen_range(scaled_bounds.y_range()),
                ),
                config.size / rng.gen_range(4..8) as f64,
            )
        })
        .collect::<Vec<Circle>>();

    for _ in 0..config.points {
        let x = rng.gen_range(scaled_bounds.x_range());
        let y = gen_weighted(scaled_bounds.y_range(), &mut rng);

        let mut point = Point(x, y);

        spheres.iter().for_each(|sphere| {
            if sphere.contains(&point) {
                let center = sphere.center();
                let distance = point.distance_to(&center);
                let angle = point.angle_to(&center);

                let force = match config.force {
                    ForceMethod::Distort => -distance / sphere.radius,
                    ForceMethod::Push => -(sphere.radius - distance) / sphere.radius,
                    ForceMethod::Pull => sphere.radius / distance,
                };

                let new_x = point.0 + map::map(angle.cos() * force, 0.0..1.0, 1.0..sphere.radius);
                let new_y = point.1 + map::map(angle.sin() * force, 0.0..1.0, 1.0..sphere.radius);

                point = Point(new_x, new_y);
            }
        });
        let _ = pointmap.insert(point);
    }

    let clone = pointmap.clone();
    let points = clone.get_items();

    for point in points {
        let max_count = map::map(
            point.1,
            scaled_bounds.position.1..bounds.height - scaled_bounds.position.1,
            70.0..5.0,
        ) as usize;

        if let Ok(neighbors) = pointmap.get_neighbors(point, Some(50.)) {
            neighbors
                .iter()
                .filter(|n| n.distance_to(point) > 10.)
                .take(max_count)
                .for_each(|n| {
                    let path = Path::new(
                        vec![*point, *n],
                        PathStyle {
                            stroke_weight: Some(0.2),
                            stroke: Some(Color::Hex("#eee")),
                            color: None,
                        },
                    );
                    svg.add_shape(Box::new(path));
                });
        }

        pointmap.remove(*point);
    }

    svg
}
