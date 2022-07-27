use noise::{NoiseFn, OpenSimplex};
use rand::Rng;
use rust_gen_art::{circle::Circle, line::Line, point::Point, rectangle::Rectangle, Shape, SVG};

fn main() {
    const WIDTH: f64 = 500.0;
    const HEIGHT: f64 = 850.0;
    const PADDING: f64 = WIDTH / 10.0;
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
        fill: "#111",
    };

    let perlin = OpenSimplex::new();
    let distort = rng.gen_range(2.0..4.2);

    for _ in 0..1000 {
        let mut x: f64 = rng.gen_range(0.0..WIDTH);
        let mut y: f64 = rng.gen_range(0.0..HEIGHT);

        let r = 3.0;

        let mut line = Line {
            points: vec![],
            stroke: "#000",
            stroke_width: r,
        };

        while bounds.contains(&Point { x, y }) {
            let n = perlin.get([x / 800.0, y / 800.0, 0.0]);
            x += (distort * n).sin() * 5.0;
            y += (distort * n).cos() * 5.0;

            let circle = Circle { x, y, r };

            println!("checking for collissions against {} points", dots.len());

            if line.points.iter().any(|dot| {
                circle.distance(&Circle {
                    x: dot.x,
                    y: dot.y,
                    r,
                }) < r * 2.0 + 5.0
            }) {
                break;
            }

            line.add_point(Point { x, y });
        }

        line.points.iter().for_each(|point| {
            dots.push(Circle {
                x: point.x,
                y: point.y,
                r,
            });
        });

        document.add(Box::new(line));
    }

    document.save();
}
