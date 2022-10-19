use std::cmp::min;

use rand::Rng;
use rust_gen_art::{circle::Circle, point::Point, rectangle::Rectangle, svg::SVG};

fn main() {
    let bounds = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 1000.0,
        height: 1000.0 * 1.4,
        color: None,
    };

    let padding = bounds.width / 10.0;
    let mut document = SVG::new("Grid", bounds);

    let mut x: f64 = padding;
    let mut rng = rand::thread_rng();

    while x < bounds.width - padding {
        let block_width = rng.gen_range(bounds.width * 0.003..bounds.width * 0.04);
        let mut y: f64 = padding;

        while y < bounds.height - padding {
            let block_height = if rng.gen_bool(0.2) {
                bounds.height * rng.gen_range(0.03..0.045)
            } else {
                bounds.height * rng.gen_range(0.002..0.01)
            };

            let area = block_width * block_height;
            let dot_count = get_dot_count(y, area, bounds.height);

            for _ in 0..dot_count {
                let cy: f64 = rng.gen_range(y..(y + block_height));
                let cx: f64 = rng.gen_range(x..(x + block_width));
                let r: f64 = 1.0;

                document.add_shape(Box::new(Circle::new(Point { x: cx, y: cy }, r)));
            }

            y += block_height;
        }

        x += block_width;
    }

    document.save();
}

fn get_dot_count(y: f64, area: f64, render_height: f64) -> i32 {
    let area_str = format!("{}", area);

    let max_str_len = std::cmp::min(area_str.len(), 4);

    let normalized_area = area_str
        .get(0..max_str_len)
        .unwrap_or("0.0")
        .parse::<f64>()
        .unwrap_or(0.);

    let mut rng = rand::thread_rng();
    let count = (render_height - y) * rng.gen_range(2.0..4.0) + normalized_area;

    return min(count as i32, 100);
}
