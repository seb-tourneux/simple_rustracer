// Constants
use crate::vec3::Scalar;

pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

use rand::Rng;

// Utility

pub fn degrees_to_radians(degrees: Scalar) -> Scalar {
    degrees * PI / 180.0
}

pub fn random_double() -> Scalar {
    rand::thread_rng().gen()
}

pub fn random_double_range(min: Scalar, max: Scalar) -> Scalar {
    min + (max - min) * random_double()
}