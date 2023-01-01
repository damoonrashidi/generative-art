use std::fmt::Debug;

use rand::{thread_rng, Rng};

use crate::palette::Palette;
use crate::shapes::point::Point;
use crate::shapes::rectangle::{Rectangle, SplitDirection};
use crate::shapes::shape::Shape;

use super::color::Color;

pub type ColoredRegion = (Rectangle, Color);

pub struct RegionalPalette<const N: usize> {
    bounds: [ColoredRegion; N],
}

impl<const N: usize> Debug for RegionalPalette<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = String::from("");

        write!(f, "{}", output)
    }
}

impl<'a, const N: usize> RegionalPalette<N> {
    pub fn new(bounds: [ColoredRegion; N]) -> Self {
        RegionalPalette { bounds }
    }

    pub fn from_region(bounds: Rectangle, palette: &dyn Palette) -> Self {
        let mut rects = vec![bounds];

        let mut rng = thread_rng();

        for _ in 0..5 {
            for i in (0..rects.len()).rev() {
                if let Some(rect) = rects.get(i) {
                    let split_direction = if rng.gen_bool(0.5) {
                        SplitDirection::Horizontally
                    } else {
                        SplitDirection::Vertically
                    };

                    let split_point =
                        Point(rng.gen_range(rect.x_range()), rng.gen_range(rect.y_range()));

                    let (a, b) = rect.subdivide(&split_point, split_direction);

                    rects.remove(i);

                    rects.push(a);
                    rects.push(b);
                }
            }
        }

        RegionalPalette::new(
            rects
                .into_iter()
                .map(|rect| (rect, palette.get_random_color().unwrap()))
                .collect::<Vec<(Rectangle, Color)>>()
                .try_into()
                .unwrap(),
        )
    }

    pub fn get_color(&self, point: &Point) -> Option<Color> {
        match self
            .bounds
            .into_iter()
            .find(|bound| bound.0.contains(point))
        {
            Some((_rect, color)) => Some(color),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn create_new_regional_palette() {}
}
