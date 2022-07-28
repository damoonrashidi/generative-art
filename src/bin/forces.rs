use noise::{NoiseFn, Seedable, SuperSimplex};
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

    let perlin = SuperSimplex::new();
    Seedable::set_seed(perlin, rng.gen_range(0..100));

    let distort = rng.gen_range(1.5..3.0);

    for _ in 0..5000 {
        let mut x: f64 = rng.gen_range(PADDING..WIDTH - PADDING);
        let mut y: f64 = rng.gen_range(PADDING..HEIGHT - PADDING);
        let r = rng.gen_range(35.0..40.0);

        let (h, s, l, a) = (rng.gen_range(200..240), 50.0, 50.0, 1.0);

        let mut line = Line {
            points: vec![],
            stroke: Color::HSLa(h, s, l, a),
            stroke_width: r,
        };

        while bounds.contains(&Point { x, y }) && line.length() < MAX_LINE_LENGTH {
            let n = perlin.get([x / 1200.0, y / 1200.0, 0.0]);
            x += (distort * n).cos() * 30.0;
            y += (distort * n).sin() * 30.0;

            let current_point = Circle { x, y, r };

            if dots.iter().any(|dot| {
                let distance = current_point.distance(&Circle {
                    x: dot.x,
                    y: dot.y,
                    r,
                });
                return distance < r * 2.0;
            }) {
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
