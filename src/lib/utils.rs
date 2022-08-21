use rand::prelude::*;
use std::ops::Range;

/// Returns a random `f64` between 0 and 1
/// Note: the range is exclusive for 1
pub fn random_double() -> f64 {
    rand::random()
}

/// Returns a random `f64` between range values
pub fn random_double_range(range: Range<f64>) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}
