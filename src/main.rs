use rand::prelude::*;
use std::{fs::File, io::Write};
mod shape;

fn main() {
    const WIDTH: i32 = 4500;
    const HEIGHT: i32 = (4500.0 * 1.4) as i32;
    const PADDING: i32 = WIDTH / 10;

    let mut x: i32 = PADDING;

    let mut rng = rand::thread_rng();
    let mut document = format!(
        "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
        WIDTH, HEIGHT
    );

    while x < WIDTH - PADDING {
        let min_width: i32 = (WIDTH as f64 * 0.003).round() as i32;
        let max_width: i32 = (WIDTH as f64 * 0.04).round() as i32;
        let block_width: f64 = rng.gen_range(min_width..max_width) as f64;
        let mut y: i32 = 0;
        while y < HEIGHT - PADDING {
            let is_tall = rng.gen_bool(0.2);

            let block_height: f64 = if is_tall {
                HEIGHT as f64 * rng.gen_range(0.03..0.045)
            } else {
                HEIGHT as f64 * rng.gen_range(0.002..0.01)
            };

            let area = block_width * block_height;
            let dot_count: i32 = get_dot_count(y, area, HEIGHT);
            let diff = (area / 2.0).round();

            if dot_count < diff as i32 {
                document.push_str(&format!(
                    "<rect fill='#111' x='{}' y='{}' width='{}' height='{}'/>",
                    x, y, block_width, block_height
                ));
                document.push_str("<g fill='#fff' stroke='transparent'>");
            }

            for _ in 0..dot_count - diff as i32 {
                let cy: f64 = rng.gen_range(y..(y + block_height as i32)) as f64;
                let cx: f64 = rng.gen_range(x..(x + block_width as i32)) as f64;
                let r: f64 = 0.5;

                let circle = shape::circle::Circle { x: cx, y: cy, r };
                document.push_str(&format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"{}\"/>",
                    circle.x, circle.y, circle.r
                ));
            }

            if dot_count < (area / 2.0).round() as i32 {
                document.push_str("</g>");
            }

            y += block_height as i32;
        }
        x += block_width as i32;
    }

    document.push_str("</svg>");

    let time: String = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let mut f = File::create(format!("./output/image{}.svg", time))
        .expect("could not open file for writing");

    let _result = f
        .write_all(document.as_bytes())
        .expect("Could not write to file");
    println!("Wrote image to ./output/image{}.svg", time);
}

fn get_dot_count(y: i32, area: f64, render_height: i32) -> i32 {
    let area_str = format!("{}", area as i32);

    let max_str_len = std::cmp::min(area_str.len(), 4);

    let normalized_area = area_str
        .get(0..max_str_len)
        .unwrap()
        .parse::<i32>()
        .expect("Panic! Could not parse int");

    let mut rng = rand::thread_rng();

    let count = (render_height - y) * rng.gen_range(2..4) + normalized_area;

    return std::cmp::min(count, 1_500);
    // return std::cmp::min(count, 3_200);
}
