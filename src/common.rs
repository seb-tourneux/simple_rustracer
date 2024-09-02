// Constants
use crate::vec3::Scalar;

pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

//use std::rc::Rc;
use std::sync::Arc;

use rand::Rng;

pub type SP<T> = Arc<T>;


// Utility

pub fn degrees_to_radians(degrees: Scalar) -> Scalar {
    degrees * PI / 180.0
}

pub fn random_double() -> Scalar {
    rand::thread_rng().gen()
}

pub fn linear_step(t:Scalar, min: Scalar, max: Scalar) -> Scalar {
    min + (max - min) * t
}

pub fn random_double_range(min: Scalar, max: Scalar) -> Scalar {
    linear_step(random_double(), min, max)
}

pub fn checkerboard(t: Scalar, size: Scalar) -> bool {
    (((t / size).round() as i32) % 2) != 0
}

pub fn sigmoid(x: Scalar, lambda: Scalar) -> Scalar {
    1.0 / (1.0 + Scalar::exp(- lambda * x))
}