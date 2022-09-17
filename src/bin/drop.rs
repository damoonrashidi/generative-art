use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::{thread_rng, Rng};
use rust_gen_art::{
    blob::Blob, circle::Circle, group::Group, palette::Color, point::Point, pointmap::PointMap,
    rectangle::Rectangle, Shape, SVG,
};

fn main() {
    const WIDTH: f64 = 1000.0;
    const HEIGHT: f64 = 1000.0 * 1.4;
    let mut svg = SVG::new("drop", WIDTH, HEIGHT);
    let mut point_map: PointMap<Circle> = PointMap::new(WIDTH, HEIGHT, 75);

    let bounds = Rectangle {
        x: 0.0,
        y: 0.0,
        width: WIDTH,
        height: HEIGHT,
        color: Rectangle::default().color,
    };

    let mut rng = thread_rng();
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, rng.gen_range(1..50));

    for _ in 0..10_000 {
        let mut x: f64 = rng.gen_range(0.0..bounds.width);
        let mut y: f64 = rng.gen_range(0.0..bounds.height);
        let mut r: f64 = 3.0;
        let mut step = 8.0;

        (r, step) = if rng.gen_bool(0.005) {
            (r * 5.0, step * 5.0)
        } else {
            (r, step)
        };

        let mut circles: Vec<Circle> = vec![];
        let mut g = Group::new();
        let color: Color = Color::HSLa((0, rng.gen_range(50.0..100.0), 50.0, 1.0));

        while bounds.contains(Point { x, y }) {
            let n = noise.get([x / 150.0, y / 150.0]);
            let circle = Circle::new(x, y, r);

            if let Some(neighbors) = point_map.get_neighbors(circle) {
                let collides_with_any = neighbors
                    .iter()
                    .any(|neighbor| circle.distance(neighbor) < (circle.r + neighbor.r + 10.0));

                if collides_with_any {
                    break;
                }
            }

            circles.push(circle);
            x += (n * 1.3).cos() * step;
            y += (n * 1.3).sin() * step;
        }

        if circles.len() > 3 {
            circles.iter().for_each(|circle| {
                let _ = point_map.insert(Circle::new(circle.x, circle.y, circle.r));
                let blob = Blob::new(
                    Point {
                        x: circle.x,
                        y: circle.y,
                    },
                    circle.r,
                    Some(color),
                );
                g.add_shape(Box::new(blob));
            });
        }

        svg.add_group(Box::new(g));
    }

    svg.save();
}
