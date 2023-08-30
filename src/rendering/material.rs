use crate::hittables::HitRecord;
use crate::math::Vec3;
use crate::{Color, Ray};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<ScatteringResult>;
}

pub struct ScatteringResult {
    pub ray: Ray,
    pub attenuation: Color,
}

#[derive(Default, Copy, Clone)]
pub struct UberShader {
    albedo: Color,
    metallic: bool,
    fuzz: f64,
}

impl Material for UberShader {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<ScatteringResult> {
        match self.metallic {
            // Metallic
            true => {
                let reflected: Vec3 = Vec3::reflect(ray_in.direction.unit_vector(), record.normal);
                let scattered = Ray::new(
                    record.point,
                    reflected + self.fuzz * Vec3::random_point_in_unit_sphere(),
                );
                let attenuation = self.albedo;
                if Vec3::dot_product(&scattered.direction, &record.normal) > 0.0_f64 {
                    eprintln!("Not absorbed: metallic");
                    Some(ScatteringResult {
                        ray: scattered,
                        attenuation,
                    })
                } else {
                    None
                }
            }
            // Lambertian
            false => {
                let mut scatter_direction: Vec3 =
                    record.normal + Vec3::random_point_in_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = record.normal;
                }

                Some(ScatteringResult {
                    ray: Ray::new(record.point, scatter_direction),
                    attenuation: self.albedo,
                })
            }
        }
    }
}

impl UberShader {
    pub fn new(albedo: Color, metallic: bool, fuzz: f64) -> UberShader {
        UberShader {
            albedo,
            metallic,
            fuzz,
        }
    }
}
