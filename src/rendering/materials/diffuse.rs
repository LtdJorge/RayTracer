use crate::hittables::HitRecord;
use crate::{Color, Material, Point3, Ray, Vec3};

pub fn hemispheric_scattering_render_function(record: &HitRecord) -> Point3 {
    record.p + random_in_hemisphere(&record.normal)
}

pub fn lambertian_render_function(record: HitRecord) -> Point3 {
    record.p + record.normal + Vec3::random_point_in_unit_vector()
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let point_in_unit_sphere = Vec3::random_point_in_unit_sphere();
    if Vec3::dot_product(&point_in_unit_sphere, normal) > 0.0 {
        point_in_unit_sphere
    }
    else {
        -point_in_unit_sphere
    }
}

#[derive(Debug, Clone)]
pub struct DiffuseLambertianMaterial {
    pub albedo: Color
}

impl DiffuseLambertianMaterial {
    pub fn new(color: Color) -> DiffuseLambertianMaterial {
        DiffuseLambertianMaterial{ albedo: color}
    }
}

impl Material for DiffuseLambertianMaterial{
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord, mut attenuation: Color, mut scattered_ray: Ray) -> bool {
        let scatter_direction =
            if (hit_record.normal + Vec3::random_point_in_unit_vector()).near_zero()
                { hit_record.normal }
            else
                { hit_record.normal + Vec3::random_point_in_unit_vector() };

        scattered_ray = Ray{ origin: hit_record.p, direction: scatter_direction };
        attenuation = self.albedo;
        true
    }
}