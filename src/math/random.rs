extern crate rand;

use rand::Rng;

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_float() -> f32 {
    rand::thread_rng().gen_range(0.0f32..1.0f32)
}

pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn random_float_in_range(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}
