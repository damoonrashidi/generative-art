use std::ops::{Add, Div, Mul, Range, Sub};

use num_traits::FromPrimitive;

/**
Map a value from one range to a corresponding value in another range

Example

```
use generative_art::transforms::map::map;
let value = map(10.0, 0.0..20.0, 0.0..40.0); // value = 20.0;
```
*/
pub fn map<
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy + FromPrimitive,
>(
    value: T,
    from: Range<T>,
    to: Range<T>,
) -> T {
    ((value - from.start) * (to.end - to.start)) / (from.end - from.start) + to.start
}
