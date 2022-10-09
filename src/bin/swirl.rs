use std::f64::consts::PI;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rust_gen_art::{
    blob::Blob, circle::Circle, group::Group, palette::Color, point::Point, pointmap::PointMap,
    rectangle::Rectangle, Shape, SVG,
};

fn main() {
    let bounds = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 1000.0,
        height: 1000.0,
        color: None,
    };
    let mut svg = SVG::new("Swirl", bounds);
    let mut rng = ChaCha20Rng::from_entropy();
    let mut point_map = PointMap::new(&bounds, 10);

    for _ in 0..5000 {
        let mut g = Group::new();
        g.set_style(rust_gen_art::group::GroupStyle {
            fill: None,
            stroke: Some(Color::Hex("#111")),
            stroke_width: Some(5.0),
        });
        let mut circles: Vec<Circle> = vec![];
        let mut x: f64 = rng.gen_range(0.0..bounds.width);
        let mut y: f64 = rng.gen_range(0.0..bounds.height);
        let step_size = 5.0;
        let mut count = 0;

        while count < 500 && bounds.contains(Point { x, y }) {
            let n = swirl(x / 2.0, y / 2.0, &bounds);
            x += (n * 2.5).sin() * step_size;
            y += (n * 2.5).cos() * step_size;
            count += 1;
            let circle = Circle::new(x, y, 2.5);

            if let Some(neighbors) = point_map.get_neighbors(circle) {
                let collides_with_any = neighbors
                    .iter()
                    .any(|neighbor| circle.distance(neighbor) < (circle.r + neighbor.r).powf(0.5));

                if collides_with_any {
                    break;
                }
            }

            circles.push(circle);
        }

        if circles.len() > 5 {
            circles.iter().for_each(|circle| {
                let _ = point_map.insert(Circle::new(circle.x, circle.y, circle.r));
                g.add_shape(Box::new(Blob::new(
                    circle.center(),
                    circle.r,
                    Some(Color::HSLa((0, 0.5, 0.5, 1.0))),
                )));
            });

            svg.add_group(Box::new(g));
        }
    }

    svg.save();
}

#[allow(unused)]
fn swirl(x: f64, y: f64, bounds: &Rectangle) -> f64 {
    let center = Point {
        x: bounds.width / 2.0,
        y: bounds.height / 2.0,
    };

    let point = Point { x, y };

    return point.distance(center).sqrt();
}

#[allow(unused)]
fn to_center(x: f64, y: f64, bounds: &Rectangle) -> f64 {
    let angle = (x - bounds.center().x).atan2((y - bounds.center().y).abs());

    return angle * PI;
}
