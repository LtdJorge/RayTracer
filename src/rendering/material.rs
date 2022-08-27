use crate::hittables::HitRecord;
use crate::{Color, Ray};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<ScatteringResult>;
}

pub struct ScatteringResult {
    pub ray: Ray,
    pub attenuation: Color
}