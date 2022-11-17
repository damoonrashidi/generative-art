use noise::{NoiseFn, OpenSimplex, Seedable};
use palette::{color::Color, palette::Palette};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use shapes::{
    blob::Blob, circle::Circle, point::Point, pointmap::PointMap, rectangle::Rectangle,
    shape::Shape,
};
use svg::svg::SVG;

fn main() {
    let bounds = Rectangle {
        x: 0.,
        y: 0.,
        width: 1000.,
        height: 1000. * 1.4,
        color: Some(Color::Hex("#111")),
    };
    let mut document = SVG::new("Wildlands", bounds);
    document.add_shape(Box::new(bounds));

    let r: f64 = 2.5;
    let step_size: f64 = r.powf(2.0);
    let mut rng = ChaCha20Rng::from_entropy();
    let mut point_map: PointMap<Circle> = PointMap::new(&bounds, 20);
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, rng.gen_range(0..2000));
    let palette = Palette::new(vec![
        Color::HSLa((0, 100., 98., 1.)),
        Color::HSLa((75, 100., 81., 1.)),
        Color::HSLa((34, 61., 91., 1.)),
        Color::HSLa((28, 82., 56., 1.)),
        Color::HSLa((0, 8., 21., 1.)),
        Color::HSLa((0, 44., 44., 1.)),
    ]);

    let mut color_bounds: Vec<Blob> = vec![];

    for _ in 2..rng.gen_range(5..25) {
        let x = rng.gen_range(bounds.x_range());
        let y = rng.gen_range(bounds.y_range());
        let r = rng.gen_range((bounds.width / 12.0)..(bounds.width / 10.));
        let color = palette.get_random_color();

        let blob = Blob::new(Point { x, y }, r, color);

        color_bounds.push(blob);
    }

    for _ in 0..20_000 {
        let mut x = rng.gen_range(bounds.x_range());
        let mut y = rng.gen_range(bounds.y_range());

        let mut line: Vec<Circle> = vec![];
        let line_color: Option<Color> = match color_bounds
            .iter()
            .find(|region| region.contains(&Point { x, y }))
        {
            Some(region) => region.color,
            _ => palette.get_random_color(),
        };

        while bounds.contains(&Point { x, y }) && line.len() < 20 {
            let n = noise.get([x / 350., y / 350.]);
            x += (4.2 * n).cos() * step_size;
            y += (4.2 * n).sin() * step_size;
            let mut circle = Circle::new(Point { x, y }, r);
            circle.set_color(line_color.unwrap());

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
