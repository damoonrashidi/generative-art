use noise::{Seedable, SuperSimplex};
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
    let (bg, palette) = Palettes::orange_autumn();
    bounds.set_color(bg);

    let mut pointmap: PointMap<'_, Circle> = PointMap::new(&bounds, 50);
    let mut svg = Document::new("Avoidance", bounds);
    let _noise = SuperSimplex::new().set_seed(500);

    svg.add_shape(Box::new(bounds));

    let mut rng = thread_rng();

    for _ in 0..2 {
        let mut trail = Trail::new(
            60.,
            Point(
                rng.gen_range(bounds.x_range()),
                rng.gen_range(bounds.y_range()),
            ),
            rng.gen_range(-1.0..1.0),
        );

        let mut points: Vec<Point> = vec![];
        while bounds.contains(&trail.position) && points.len() < 100 {
            points.push(trail.position);
            let circle = Circle::new(trail.position, trail.radius);

            let neighbors = pointmap
                .get_neighbors(&circle, Some(trail.radius / 2.))
                .unwrap();

            if neighbors.iter().any(|c| c.intersects(&circle)) {
                println!("breaking due to collision");
                break;
            }

            // move the trail forward in the direction of its velocity

            trail.position.0 += trail.direction.cos();
            trail.position.1 += trail.direction.sin();

            break;
        }

        let path = Path::new(
            &mut points,
            PathStyle {
                stroke_weight: Some(config.size / 300.),
                stroke: palette.get_random_color(),
                ..Default::default()
            },
        );

        for point in &points {
            let _ = pointmap.insert(Circle::new(*point, 40.));
        }

        svg.add_shape(Box::new(path));
    }

    svg.generate()
}
