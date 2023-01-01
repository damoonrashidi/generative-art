use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::{
    palette::{color::Color, palettes::Palettes, regional_palette::RegionalPalette},
    shapes::{blob::Blob, point::Point, pointmap::PointMap, rectangle::Rectangle, shape::Shape},
    svg::document::Document,
};

pub fn wildlands() -> Document<'static> {
    let bounds = Rectangle {
        position: Point(0., 0.),
        width: 1400.,
        height: 1400. * 1.4,
        color: Some(Palettes::orange_autumn().0),
    };

    let palette: RegionalPalette<5> =
        RegionalPalette::from_region(bounds, &Palettes::orange_autumn().1);

    let inner_bounds = bounds.scale(0.9);
    let long_bounds = bounds.scale(0.94);
    let mut document = Document::new("Wildlands", bounds);
    document.add_shape(Box::new(bounds));

    let r: f64 = 3.5;
    let step_size: f64 = r * 2.5;
    let mut rng = ChaCha20Rng::from_entropy();
    let mut point_map: PointMap<'_, Blob> = PointMap::new(&bounds, 20);
    let noise = OpenSimplex::new().set_seed(rng.gen_range(0..2000));

    let mut color_bounds: Vec<Blob> = vec![];

    for _ in 0..10 {
        let x = rng.gen_range(bounds.x_range());
        let y = rng.gen_range(bounds.y_range());
        let r = rng.gen_range((bounds.width / 10.0)..(bounds.width / 7.));
        let color = palette.get_color(&Point(x, y));

        let blob = Blob::new(Point(x, y), r, color);

        color_bounds.push(blob);
    }

    for _ in 0..15_000 {
        let is_long = rng.gen_bool(0.03);
        let mut point = Point(
            rng.gen_range(bounds.x_range()),
            rng.gen_range(bounds.y_range()),
        );

        let mut line: Vec<Blob> = vec![];
        let line_color: Option<Color> =
            match color_bounds.iter().find(|region| region.contains(&point)) {
                Some(region) => region.color,
                _ => Some(Color::HSLa((0, 44., 44., 1.))),
            };

        while (is_long && long_bounds.contains(&point))
            || inner_bounds.contains(&point) && line.len() < 150
        {
            let n = noise.get([point.0 / 500., point.1 / 500.]);
            point.0 += (4.0 * n).cos() * step_size;
            point.1 += (4.0 * n).sin() * step_size;
            let blob = Blob::new(point, r, line_color);

            if let Ok(neighbors) = point_map.get_neighbors(&blob, None) {
                if neighbors
                    .iter()
                    .any(|neighbor| neighbor.distance(&blob) < blob.radius)
                {
                    break;
                }
            } else {
                break;
            }

            line.push(blob);
        }

        if line.len() > 3 {
            for blob in line {
                let clone = Blob::new(blob.center(), blob.radius, blob.color);
                let _ = point_map.insert(blob);
                document.add_shape(Box::new(clone));
            }
        }
    }

    document
}
