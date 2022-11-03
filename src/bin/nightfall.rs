use generative_art::nightfall_config::NightfallConfig;
use palette::palette::Color;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
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
    let mut pointmap: PointMap<Point> = PointMap::new::<Point>(&bounds, 50);

    let mut rng = ChaCha20Rng::from_entropy();

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

    for _ in 0..config.points {
        let x = rng.gen_range(scaled_bounds.x_range());
        let y = gen_weighted(scaled_bounds.y_range(), &mut rng);

        let point = Point { x, y };
        let _ = pointmap.insert(point);
    }

    let sphere = Circle::new(bounds.center(), config.size / 3.);

    pointmap
        .get_neighbors(sphere.center(), Some(sphere.r))
        .unwrap()
        .into_iter()
        .for_each(|mut point| {
            let center = &sphere.center();
            let distance = point.distance(center);
            let angle = point.angle_to(center);
            let force = -(sphere.r - distance) / sphere.r;
            point.x += map(angle.cos() * force, 0.0..1.0, 0.0..sphere.r);
            point.y += map(angle.sin() * force, 0.0..1.0, 0.0..sphere.r);
        });

    pointmap.get_items().iter().for_each(|point| {
        match pointmap.get_neighbors(**point, Some(config.distance)) {
            Err(_) => {}
            Ok(neighbors) => neighbors.iter().for_each(|neighbor| {
                let path = Path::new(vec![**point, *neighbor], Default::default());

                g.add_shape(Box::new(path));
            }),
        }
    });
    svg.add_shape(Box::new(bounds));
    svg.add_group(g);
    svg.save(None);
}
