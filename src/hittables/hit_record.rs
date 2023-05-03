use crate::math::{Point3, Ray, Vec3};
use crate::rendering::UberShader;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: UberShader,
}

impl HitRecord {
    pub fn set_face_normal(front_face: bool, outward_normal: Vec3) -> Vec3 {
        if front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
    pub fn set_front_face(ray: &Ray, outward_normal: Vec3) -> bool {
        Vec3::dot_product(&ray.direction, &outward_normal) < 0.0
    }
    // #[inline]
    pub fn new(p: Point3, normal: Vec3, t: f64, material: UberShader) -> HitRecord {
        HitRecord {
            point: p,
            normal,
            t,
            material,
        }
    }
}
