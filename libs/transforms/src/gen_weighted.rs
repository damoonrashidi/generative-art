use rand::{distributions::WeightedIndex, prelude::Distribution};
use rand::{rngs::ThreadRng, Rng};
use std::ops::Range;

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

/// A set of colors where one can be chosen randomly but biased by a given weight
#[derive(Debug)]
pub struct WeightedChoice<T, const N: usize> {
    choices: [T; N],
}

/// Pair of Item + Weigth
pub type WeightPair<T> = (T, usize);

impl<T, const N: usize> WeightedChoice<WeightPair<T>, N>
where
    T: Copy + Clone,
{
    /// This thing
    pub fn get_random_choice(&self) -> Option<T> {
        let mut rng = rand::thread_rng();
        let weights = self
            .choices
            .iter()
            .map(|color| color.1)
            .collect::<Vec<usize>>();

        let dist = if let Ok(dist) = WeightedIndex::new(&weights) {
            dist
        } else {
            return None;
        };

        let i = dist.sample(&mut rng);

        match self.choices.len() {
            0 => None,
            _ => Some(self.choices[i].0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::WeightedChoice;

    #[test]
    fn test_gen_random() {
        let choice = WeightedChoice {
            choices: [(1, 1), (100, 0), (200, 0)],
        };

        let chosen = choice.get_random_choice();
        assert_eq!(chosen.unwrap(), 1);
    }
}
