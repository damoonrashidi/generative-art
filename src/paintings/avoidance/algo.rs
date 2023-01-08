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
    let (bg, palette) = Palettes::wild();
    bounds.set_color(bg);

    let mut pointmap: PointMap<'_, Circle> = PointMap::new(&bounds, 50);
    let mut svg = Document::new("Avoidance", bounds);
    let _noise = SuperSimplex::new().set_seed(500);

    svg.add_shape(Box::new(bounds));

    let mut rng = thread_rng();

    for _ in 0..5000 {
        let trail = Trail::new(
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

            let _neighbors = pointmap
                .get_neighbors(&circle, Some(trail.radius / 2.))
                .unwrap();

            break;
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
