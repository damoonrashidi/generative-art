use generative_art::nightfall_config::{ForceMethod, NightfallConfig};
use palette::palette::Color;
use rand::{thread_rng, Rng};

use shapes::{
    circle::Circle, path::Path, point::Point, pointmap::PointMap, rectangle::Rectangle,
    shape::Shape,
};
use svg::{
    group::{Group, GroupStyle},
    svg::SVG,
};
use transforms::{gen_weighted::gen_weighted, map::map};

fn main() {
    let config = NightfallConfig::new();

    let bounds = Rectangle {
        x: 0.,
        y: 0.,
        width: config.size,
        height: config.size * 1.4,
        color: Some(Color::Hex("#111")),
    };
    let scaled_bounds = bounds.scale(0.9);

    let mut svg = SVG::new("Nightfall", bounds);
    let mut pointmap: PointMap<Point> =
        PointMap::new(&bounds, (config.distance / config.size * 1000.0) as usize);

    let mut rng = thread_rng();

    let mut g: Group = Group::new();
    g.set_style(GroupStyle {
        stroke: Some(Color::HSLa((30, 40., 95., 0.7))),
        stroke_width: Some(0.5),
        ..Default::default()
    });

    for _ in 0..config.points / 10 {
        let x = rng.gen_range(scaled_bounds.x_range());
        let y = gen_weighted(
            scaled_bounds.y..(scaled_bounds.y + scaled_bounds.y * 0.05),
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
            let distance = point.distance(&center);
            let angle = point.angle_to(&center);

            let force = match config.force {
                ForceMethod::Distort => -distance / sphere.r,
                ForceMethod::Push => -(sphere.r - distance) / sphere.r,
                ForceMethod::Pull => sphere.r / distance,
            };

            let new_x = point.x + map(angle.cos() * force, 0.0..1.0, 1.0..sphere.r);
            let new_y = point.y + map(angle.sin() * force, 0.0..1.0, 1.0..sphere.r);

            let _ = pointmap.insert(Point {
                x: new_x.min(bounds.width).max(bounds.x),
                y: new_y.min(bounds.height).max(bounds.y),
            });
        } else {
            let _ = pointmap.insert(point);
        }
    }

    pointmap.get_items().into_iter().for_each(|point| {
        match pointmap.get_neighbors(*point, Some(config.distance)) {
            Err(_) => {}
            Ok(neighbors) => {
                let max_count = map(
                    point.y,
                    scaled_bounds.y..bounds.height - scaled_bounds.y,
                    105.0..10.0,
                ) as usize;

                neighbors.iter().take(max_count).for_each(|neighbor| {
                    let path = Path::new(vec![*point, *neighbor], Default::default());

                    g.add_shape(Box::new(path));
                })
            }
        }
    });

    svg.add_shape(Box::new(bounds));
    svg.add_group(g);
    svg.save(Some(config.into()));
}
