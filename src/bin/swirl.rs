use std::{f64::consts::PI, ops::Range};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rust_gen_art::{
    circle::Circle, group::Group, palette::Color, point::Point, pointmap::PointMap,
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
    let mut svg = SVG::new("Swirl", bounds.width, bounds.height);
    let mut rng = ChaCha20Rng::from_entropy();
    let mut point_map = PointMap::new(bounds.width, bounds.height, 20);

    for _ in 0..500 {
        let mut g = Group::new();
        g.set_style(rust_gen_art::group::GroupStyle {
            fill: None,
            stroke: Some(Color::Hex("#111")),
            stroke_width: Some(5.0),
        });
        let mut circles: Vec<Circle> = vec![];
        let mut x: f64 = rng.gen_range(0.0..bounds.width);
        let mut y: f64 = rng.gen_range(0.0..bounds.height);
        let mut r = 0.2;
        let step_size = rng.gen_range(0.1..0.8);
        let mut count = 0;

        while count < 500 && bounds.contains(Point { x, y }) {
            let n = swirl(x, y, &bounds);
            x += (n * 10.0).sin() * step_size;
            y += (n * 10.0).cos() * step_size;
            count += 1;
            let circle = Circle::new(x, y, r);

            if let Some(neighbors) = point_map.get_neighbors(circle) {
                let collides_with_any = neighbors
                    .iter()
                    .any(|neighbor| circle.distance(neighbor) < (circle.r + neighbor.r).powf(2.0));

                if collides_with_any {
                    break;
                }
            }

            circles.push(circle);
            r += 0.09;
        }

        if circles.len() > 5 {
            circles.iter().for_each(|circle| {
                let _ = point_map.insert(Circle::new(circle.x, circle.y, circle.r));
                g.add_shape(Box::new(Circle::new(circle.x, circle.y, circle.r)));
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

#[allow(unused)]
fn map(value: f64, from: Range<f64>, to: Range<f64>) -> f64 {
    return ((value - from.start) * (to.end - to.start)) / (from.end - from.start) + to.start;
}
