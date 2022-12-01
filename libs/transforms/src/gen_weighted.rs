use std::ops::Range;

use rand::{rngs::ThreadRng, Rng};

/**
Genarate a value inside a range, weighted towards the beginning of the range

Basic example
```
let random_value = gen_weighted(0.0..1.0) // -> will most often be closer to 0.0 than 1.0
```

Another example
```
let bounds = Rectangle(Point{x: 0.0, y: 0.0}, 1000.0, 1000.0);

let point_in_rectangle = Point {
  x: 500.0,
  y: gen_weighted(bounds.y_range());
}
```
*/
pub fn gen_weighted(range: Range<f64>, rng: &mut ThreadRng) -> f64 {
    let a = rng.gen_range(0.0..1.0) as f64;
    let b = rng.gen_range(0.0..1.0);

    ((b - a).abs() * (1.0 + range.end - range.start) + range.start).floor()
}
