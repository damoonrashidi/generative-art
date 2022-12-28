use generative_art::nightfall_config::{ForceMethod, NightfallConfig};
use palette::color::Color;
use rand::{thread_rng, Rng};

use shapes::{
    circle::Circle,
    path::{Path, PathStyle},
    point::Point,
    pointmap::PointMap,
    rectangle::Rectangle,
    shape::Shape,
};
use svg::svg::SVG;
use transforms::{gen_weighted::gen_weighted, map::map};

fn main() {
    let config = NightfallConfig::new();

    let bounds = Rectangle {
        position: Point { x: 0.0, y: 0.0 },
        width: config.size,
        height: config.size * 1.4,
        color: Some(Color::Hex("#111")),
    };
    let scaled_bounds = bounds.scale(0.9);

    let mut svg = SVG::new("Nightfall", bounds);
    svg.add_shape(Box::new(bounds));

    let mut pointmap: PointMap<'_, Point> =
        PointMap::new(&bounds, (config.distance / config.size * 1000.0) as usize);

    let mut rng = thread_rng();

    for _ in 0..config.points / 10 {
        let x = rng.gen_range(scaled_bounds.x_range());
        let y = gen_weighted(
            scaled_bounds.position.y..(scaled_bounds.position.y + scaled_bounds.position.y * 0.05),
            &mut rng,
        );

        let point = Point { x, y };
        let _ = pointmap.insert(point);
    }

    let sphere = Circle::new(
        Point {
            x: rng.gen_range(scaled_bounds.x_range()),
            y: rng.gen_range(scaled_bounds.y_range()),
        },
        config.size / rng.gen_range(4..8) as f64,
    );
    let center = sphere.center();

    for _ in 0..config.points {
        let x = rng.gen_range(scaled_bounds.x_range());
        let y = gen_weighted(scaled_bounds.y_range(), &mut rng);

        let point = Point { x, y };

        if sphere.contains(&point) {
            let distance = point.distance_to(&center);
            let angle = point.angle_to(&center);

            let force = match config.force {
                ForceMethod::Distort => -distance / sphere.radius,
                ForceMethod::Push => -(sphere.radius - distance) / sphere.radius,
                ForceMethod::Pull => sphere.radius / distance,
            };

            let new_x = point.x + map(angle.cos() * force, 0.0..1.0, 1.0..sphere.radius);
            let new_y = point.y + map(angle.sin() * force, 0.0..1.0, 1.0..sphere.radius);

            let _ = pointmap.insert(Point {
                x: new_x.min(bounds.width).max(bounds.position.x),
                y: new_y.min(bounds.height).max(bounds.position.y),
            });
        } else {
            let _ = pointmap.insert(point);
        }
    }

    let points = pointmap.get_items();

    for point in points {
        let max_count = map(
            point.y,
            scaled_bounds.position.y..bounds.height - scaled_bounds.position.y,
            105.0..10.0,
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
                            stroke_weight: Some(0.5),
                            stroke: Some(Color::Hex("#eee")),
                            color: None,
                        },
                    );
                    svg.add_shape(Box::new(path));
                });
        }
    }

    svg.save(Some(config.into()));
}
