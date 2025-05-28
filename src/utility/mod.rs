use std::f64::consts::PI;

use rand::Rng;

pub mod interval;

// Generate value between 0.0 and 1.0
pub fn random_double() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

pub fn random_double_clamp(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
