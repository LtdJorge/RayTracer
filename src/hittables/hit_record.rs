use crate::math::{Vec3, Point3, Ray};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64
}