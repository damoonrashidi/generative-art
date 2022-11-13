use std::ops::Range;

use rand::{rngs::ThreadRng, Rng};

pub fn gen_weighted(range: Range<f64>, rng: &mut ThreadRng) -> f64 {
    let a = rng.gen_range(0.0..1.0) as f64;
    let b = rng.gen_range(0.0..1.0);

    ((b - a).abs() * (1.0 + range.end - range.start) + range.start).floor()
}