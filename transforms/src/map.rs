use std::ops::{Add, Div, Mul, Range, Sub};

use num_traits::FromPrimitive;

pub fn map<T>(value: T, from: Range<T>, to: Range<T>) -> T
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy + FromPrimitive,
{
    ((value - from.start) * (to.end - to.start)) / (from.end - from.start) + to.start
}
