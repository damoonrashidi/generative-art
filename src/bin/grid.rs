use std::cmp::min;

use rand::Rng;
use rust_gen_art::{circle::Circle, SVG};

fn main() {
    const WIDTH: f64 = 500.0;
    const HEIGHT: f64 = 850.0;
    const PADDING: f64 = WIDTH / 10.0;
    let mut document = SVG {
        name: "Grid",
        width: WIDTH,
        height: HEIGHT,
        document: String::from(""),
    };

    document.create_document();

    let mut x: f64 = PADDING;
    let mut rng = rand::thread_rng();
    let mut total_dot_count: i32 = 0;

    while x < WIDTH - PADDING {
        let block_width = rng.gen_range(WIDTH * 0.003..WIDTH * 0.04);
        let mut y: f64 = PADDING;

        while y < HEIGHT - PADDING {
            let block_height = if rng.gen_bool(0.2) {
                HEIGHT * rng.gen_range(0.03..0.045)
            } else {
                HEIGHT * rng.gen_range(0.002..0.01)
            };

            let area = block_width * block_height;
            let dot_count = get_dot_count(y, area, HEIGHT);
            total_dot_count += dot_count;

            for _ in 0..dot_count {
                let cy: f64 = rng.gen_range(y..(y + block_height));
                let cx: f64 = rng.gen_range(x..(x + block_width));
                let r: f64 = 1.0;

                document.add(Box::new(Circle { x: cx, y: cy, r }));
            }

            y += block_height;
        }

        x += block_width;
    }

    println!("wrote {} dots", total_dot_count);

    document.save();
}

fn get_dot_count(y: f64, area: f64, render_height: f64) -> i32 {
    let area_str = format!("{}", area);

    let max_str_len = std::cmp::min(area_str.len(), 4);

    let normalized_area = area_str
        .get(0..max_str_len)
        .unwrap()
        .parse::<f64>()
        .expect("Panic! Could not parse int");

    let mut rng = rand::thread_rng();
    let count = (render_height - y) * rng.gen_range(2.0..4.0) + normalized_area;

    return min(count as i32, 100);
}
