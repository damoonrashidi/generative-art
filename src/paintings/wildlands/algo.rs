use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::{
    palette::{color::Color, palettes::Palettes, regional_palette::RegionalPalette},
    shapes::{blob::Blob, point::Point, pointmap::PointMap, rectangle::Rectangle, shape::Shape},
    svg::document::Document,
};

use super::config::WildlandsConfig;

pub fn wildlands(config: &WildlandsConfig) -> Document<'static> {
    let bounds = Rectangle {
        position: Point(0., 0.),
        width: config.size,
        height: config.size * 1.4,
        color: Some(Palettes::orange_autumn().0),
    };

    let palette = RegionalPalette::new([
        Rectangle {
            position: Point(0., 0.),
            width: bounds.width,
            height: bounds.height / 3.,
            color: Some(Color::Hex("#f00")),
        },
        Rectangle {
            position: Point(0., bounds.height / 3.),
            width: bounds.width,
            height: bounds.height / 3.,
            color: Some(Color::Hex("#0f0")),
        },
        Rectangle {
            position: Point(0., 2.0 * bounds.height / 3.),
            width: bounds.width,
            height: bounds.height / 3.,
            color: Some(Color::Hex("#00f")),
        },
    ]);

    let inner_bounds = bounds.scale(0.9);
    let long_bounds = bounds.scale(0.94);
    let mut document = Document::new("Wildlands", bounds);
    document.add_shape(Box::new(bounds));

    let r: f64 = 3.5;
    let step_size: f64 = r * 2.5;
    let mut rng = ChaCha20Rng::from_entropy();
    let mut point_map: PointMap<'_, Blob> = PointMap::new(&bounds, 20);
    let noise = OpenSimplex::new().set_seed(rng.gen_range(0..2000));

    for _ in 0..config.line_count {
        let is_long = rng.gen_bool(0.03);
        let mut point = Point(
            rng.gen_range(bounds.x_range()),
            rng.gen_range(bounds.y_range()),
        );

        let mut line: Vec<Blob> = vec![];
        let line_color: Option<Color> = palette.get_color(&point);

        while (is_long && long_bounds.contains(&point))
            || inner_bounds.contains(&point) && line.len() < config.max_line_length
        {
            let n = noise.get([point.0 / config.smoothness, point.1 / config.smoothness]);
            point.0 += (config.chaos * n).cos() * step_size;
            point.1 += (config.chaos * n).sin() * step_size;
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
