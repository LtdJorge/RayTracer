use crate::{Color, Material, Ray, Vec3};
use crate::hittables::HitRecord;

#[derive(Debug, Clone)]
pub struct MetallicMaterial {
    pub albedo: Color,
}

impl Material for MetallicMaterial {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, mut attenuation: Color, mut scattered_ray: Ray) -> bool {
        let reflected: Vec3 = Vec3::reflect(Vec3::unit_vector(&ray_in.direction), hit_record.normal);
        scattered_ray = Ray { origin: hit_record.p, direction: reflected };
        attenuation = self.albedo;
        let normal = hit_record.normal;
        Vec3::dot_product(&scattered_ray.direction, &normal) > 0.0
    }
}
