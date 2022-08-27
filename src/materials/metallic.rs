use crate::{Color, Ray, Vec3};
use crate::rendering::{Material, ScatteringResult};
use crate::hittables::HitRecord;

#[derive(Clone, Copy)]
pub struct MetallicMaterial {
    pub albedo: Color,
    pub fuzz: f64
}

impl Material for MetallicMaterial {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<ScatteringResult>{
        let reflected: Vec3 = Vec3::reflect(ray_in.direction.unit_vector(), record.normal);
        let scattered = Ray::new(record.point, reflected);
        let attenuation = self.albedo;
        if Vec3::dot_product(&scattered.direction, &record.normal) > 0.0_f64 {
            Some(ScatteringResult{ray: scattered, attenuation})
        } else {
            None
        }
    }
}