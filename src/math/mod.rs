#![allow(dead_code)]

pub use {random::random_double, random::random_double_in_range, ray::Ray, util::clamp, vec3::Color, vec3::Point3, vec3::Vec3};

mod vec3;
mod ray;
mod random;
mod util;
mod vec4;

pub const INFINITY: f64 = f64::MAX;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0_f64
}