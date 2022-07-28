use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::Rng;
use rust_gen_art::{
    circle::Circle, line::Line, palette::Color, point::Point, rectangle::Rectangle, Shape, SVG,
};

fn main() {
    const WIDTH: f64 = 3000.0;
    const HEIGHT: f64 = WIDTH * 1.4;
    const PADDING: f64 = WIDTH / 10.0;
    const MAX_LINE_LENGTH: f64 = 2000.0;

    let mut document = SVG {
        name: "Forces",
        width: WIDTH,
        height: HEIGHT,
        document: String::from(""),
    };

    document.create_document();
    let mut rng = rand::thread_rng();

    let mut dots: Vec<Circle> = vec![];

    let bounds = Rectangle {
        x: PADDING,
        y: PADDING,
        width: WIDTH - (PADDING * 2.0),
        height: HEIGHT - (PADDING * 2.0),
        color: None,
    };

    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, rng.gen_range(0..100_000));

    let distort = rng.gen_range(1.5..4.2);
    let zoom = rng.gen_range(800.0..2_000.0);

    for _ in 0..10_000 {
        let mut x: f64 = rng.gen_range(PADDING..WIDTH - PADDING);
        let mut y: f64 = rng.gen_range(PADDING..HEIGHT - PADDING);
        let mut r = 15.0;
        let mut step_size = 30.0;
        let (h, s, l, a) = (rng.gen_range(300..320), 50.0, 50.0, 1.0);

        if rng.gen_bool(0.2) {
            r *= 5.0;
            step_size = 120.0;
        }

        let mut line = Line {
            points: vec![],
            stroke: Color::HSLa(h, s, l, a),
            stroke_width: r,
        };

        while bounds.contains(&Point { x, y }) && line.length() < MAX_LINE_LENGTH {
            let n = noise.get([x / zoom, y / zoom]);
            x += (distort * n).cos() * step_size;
            y += (distort * n).sin() * step_size;

            let current_point = Circle { x, y, r };

            if dots.iter().any(|dot| current_point.intersects(dot)) {
                break;
            }

            line.add_point(Point { x, y });
        }

        if line.length() > 200.0 {
            line.points.iter().for_each(|point| {
                dots.push(Circle {
                    x: point.x,
                    y: point.y,
                    r,
                });
            });
            document.add(Box::new(line));
        }
    }

    document.save();
}
