use crate::{Color, Ray, Vec3};
use crate::hittables::HitRecord;
use crate::rendering::{Material, ScatteringResult};

#[derive(Clone, Copy)]
pub struct LambertianMaterial {
    pub albedo: Color
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray_in: &Ray, record: &HitRecord) -> Option<ScatteringResult> {
        let mut scatter_direction: Vec3 = record.normal + Vec3::random_point_in_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        Some(ScatteringResult{ ray: Ray::new(record.point, scatter_direction), attenuation: self.albedo })
    }
}