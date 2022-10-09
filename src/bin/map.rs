use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rust_gen_art::{helpers::map, palette::Color, point::Point, rectangle::Rectangle, SVG};

fn main() {
    let bounds: Rectangle = Rectangle::new(0.0, 0.0, 1000.0, 1000.0);

    let mut svg = SVG::new("map", bounds);
    let mut rng = ChaCha20Rng::from_entropy();
    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, 1);

    for _ in 0..2000 {
        let mut point = Point {
            x: rng.gen_range(bounds.x_range()),
            y: rng.gen_range(bounds.y_range()),
        };

        let n = noise.get([point.x / 800.0, point.y / 800.0]);

        let size = point.distance(bounds.center()).sqrt();
        let mut rect = snap_to_cell(point, size);
        rect.set_width(size);
        rect.set_height(size);
        let hue = map::<f64>(n, -1.0..1.0, 200.0..260.0) as u16;
        rect.set_color(Color::HSLa((hue, 50.0, 50.0, 1.0)));
        svg.add_shape(Box::new(rect));

        point.x += n.cos() * 25.0;
        point.y += n.sin() * 25.0;
    }

    svg.save();
}

fn snap_to_cell(point: Point, cell_size: f64) -> Rectangle {
    let x = (point.x / cell_size).floor() * cell_size;
    let y = (point.y / cell_size).floor() * cell_size;

    Rectangle {
        x,
        y,
        width: cell_size,
        height: cell_size,
        color: None,
    }
}

#[cfg(test)]
mod test {
    use rust_gen_art::{point::Point, rectangle::Rectangle};

    use crate::snap_to_cell;

    #[test]
    fn test_snap_to_cell() {
        assert_eq!(
            snap_to_cell(Point { x: 22.0, y: 1.0 }, 5.0),
            Rectangle {
                x: 20.0,
                y: 0.0,
                width: 5.0,
                height: 5.0,
                color: None
            }
        );

        assert_eq!(
            snap_to_cell(Point { x: 23.0, y: 8.0 }, 7.0),
            Rectangle {
                x: 21.0,
                y: 7.0,
                width: 7.0,
                height: 7.0,
                color: None
            }
        );
    }
}
