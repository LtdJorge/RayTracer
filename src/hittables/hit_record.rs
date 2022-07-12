use crate::math::{Vec3, Point3, Ray};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64
}

impl HitRecord {
    pub fn set_face_normal(front_face: bool, outward_normal: Vec3) -> Vec3 {
        return if front_face { outward_normal } else { -outward_normal };
    }
    pub fn set_front_face(ray: &Ray, outward_normal: Vec3) -> bool {
        return  Vec3::dot_product(&ray.direction, &outward_normal) < 0.0;
    }
    pub fn create(p: Point3, normal: Vec3, t: f64) -> HitRecord {
        HitRecord { p, normal, t }
    }
    pub const EMPTY: HitRecord = HitRecord {
        p: Vec3::ZERO,
        normal: Vec3::ZERO,
        t: 0.0
    };
}