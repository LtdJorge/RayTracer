mod vec3;
mod ray;
mod random;
mod util;

pub use {vec3::Vec3, ray::Ray, vec3::Point3, vec3::Color, random::random_double, random::random_double_range, util::clamp};

pub const INFINITY: f64 = f64::MAX;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0_f64;
}