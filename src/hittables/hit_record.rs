use crate::math::{Vec3, Point3, Ray};
use crate::rendering::Material;
use std::sync::Arc;
use crate::{Color, DiffuseLambertianMaterial, MaterialPointer};

#[derive(Clone)]
pub struct HitRecord<T> {
    /// Coordinates of the hit.
    pub p: Point3,
    /// Surface normal of the hit.
    pub normal: Vec3,
    pub material: MaterialPointer,
    pub t: f64
}


impl<M> HitRecord<M> {
    pub fn set_face_normal(front_face: bool, outward_normal: Vec3) -> Vec3 {
        if front_face { outward_normal } else { -outward_normal }
    }
    pub fn set_front_face(ray: &Ray, outward_normal: Vec3) -> bool {
        Vec3::dot_product(&ray.direction, &outward_normal) < 0.0
    }
    pub fn set_material(material: Arc<dyn Material>){

    }
    pub fn create<T: Material + Clone>(p: Point3, normal: Vec3, t: f64, material: MaterialPointer<T>) -> HitRecord<T> {
        HitRecord { p, normal, t, material }
    }
    pub const EMPTY: HitRecord<M> = HitRecord {
        p: Vec3::ZERO,
        normal: Vec3::ZERO,
        material: MaterialPointer { material: Arc::new(DiffuseLambertianMaterial::new(Color::ZERO)) },
        t: 0.0
    };
}