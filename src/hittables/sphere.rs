use crate::{Point3, Ray};
use crate::hittables::hit_record::HitRecord;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn hit(ray: &Ray, t_min: f64, t_max: f64, record: &HitRecord){}
}