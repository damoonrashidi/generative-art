use std::rc::Rc;

use crate::{
    palette::palettes::Palettes,
    shapes::{
        path::PathStyle,
        point::Point,
        rectangle::{Rectangle, SplitDirection},
    },
    svg::{document::Document, group::Group},
};
use rand::{thread_rng, Rng};

use super::config::PietConfig;

pub fn piet(config: Rc<&PietConfig>) -> Document<'static> {
    let mut rng = thread_rng();
    let (background, palette) = Palettes::orange_autumn();
    let mut bounds = Rectangle::new(Point(0., 0.), config.size, config.size * 1.4);
    let root = bounds.scale(0.95);

    let mut svg = Document::new("piet", bounds);
    let mut group = Group::new(None);
    let mut rects = vec![root];

    for _ in 0..config.rounds {
        for i in (0..rects.len()).rev() {
            if let Some(rect) = rects.get(i) {
                if rng.gen_bool(config.split_chance) && rect.area() > bounds.area() * 0.01 {
                    let scaled = rect.scale(0.9);

                    let split_point = Point(
                        rng.gen_range(scaled.x_range()),
                        rng.gen_range(scaled.y_range()),
                    );

                    let split_direction = if rng.gen_bool(0.5) {
                        SplitDirection::Horizontally
                    } else {
                        SplitDirection::Vertically
                    };

                    let (mut a, mut b) =
                        rect.subdivide(&split_point, split_direction, Some(config.padding));
                    rects.remove(i);

                    if let Some(a_color) = palette.get_random_color() {
                        a.set_color(a_color);
                    }

                    if let Some(b_color) = palette.get_random_color() {
                        b.set_color(b_color);
                    }

                    rects.push(a);
                    rects.push(b);
                }
            }
        }
    }

    bounds.set_color(background);
    svg.add_shape(Box::new(bounds));

    rects
        .iter()
        .map(|rect| {
            rect.to_path(PathStyle {
                color: rect.color,
                stroke_weight: None,
                stroke: None,
            })
        })
        .for_each(|path| {
            group.add_shape(Box::new(path));
        });

    svg.add_group(group);

    svg
}
