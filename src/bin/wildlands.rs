use noise::{NoiseFn, OpenSimplex, Seedable};
use palette::{color::Color, weighted_palette::WeightedPalette};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use shapes::{
    circle::Circle, point::Point, pointmap::PointMap, rectangle::Rectangle, shape::Shape,
};
use svg::svg::SVG;

fn main() {
    let bounds = Rectangle::new(0., 0., 1000., 1000. * 1.4);
    let mut document = SVG::new("Wildlands", bounds);

    let r: f64 = 2.5;
    let step_size: f64 = r.powf(2.0);
    let mut rng = ChaCha20Rng::from_entropy();
    let mut point_map: PointMap<Circle> = PointMap::new(&bounds, 20);
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, rng.gen_range(0..2000));

    let palette = WeightedPalette::new(vec![
        (Color::Hex("#F9F2ED"), 1),
        (Color::Hex("#3AB0FF"), 3),
        (Color::Hex("#FFB562"), 3),
        (Color::Hex("#F87474"), 3),
    ]);

    for _ in 0..10_000 {
        let mut x = rng.gen_range(bounds.x_range());
        let mut y = rng.gen_range(bounds.y_range());

        let mut line: Vec<Circle> = vec![];
        let line_color = palette.get_random_color().unwrap();

        while bounds.contains(&Point { x, y }) && line.len() < 20 {
            let n = noise.get([x / 350., y / 350.]);
            x += (4.2 * n).cos() * step_size;
            y += (4.2 * n).sin() * step_size;
            let mut circle = Circle::new(Point { x, y }, r);
            circle.set_color(line_color);

            if let Ok(neighbors) = point_map.get_neighbors(circle, None) {
                if neighbors
                    .iter()
                    .any(|neighbor| neighbor.distance(&circle) < circle.r)
                {
                    break;
                }
            } else {
                break;
            }

            line.push(circle);
        }

        if line.len() > 3 {
            for circle in line {
                let _ = point_map.insert(circle);
                document.add_shape(Box::new(circle));
            }
        }
    }

    document.save(None);
}
