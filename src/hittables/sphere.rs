use crate::hittables::hit_record::HitRecord;
use crate::hittables::Hittable;
use crate::rendering::UberShader;
use crate::{Point3, Ray, Vec3};

#[derive(Clone)]
pub struct Sphere {
    pub(crate) center: Point3,
    pub(crate) radius: f64,
    pub(crate) material: UberShader,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_to_center: Vec3 = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let half_b = Vec3::dot_product(&origin_to_center, &ray.direction);
        let c = origin_to_center.squared_length() - self.radius * self.radius;

        // Value inside the square root of the quadratic equation
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            // Negative number = no square root, so no hit
            return None;
        }
        // Result of the square root
        let sqrt_discriminant = discriminant.sqrt();

        // Quadratic equation is (-half_b +- sqrtd) / a
        // The negative value of the sqrt value is used first because it would be closer
        let mut root = (-half_b - sqrt_discriminant) / a;
        // If root is outside range
        if root < t_min || t_max < root {
            // Use the positive sqrt value
            root = (-half_b + sqrt_discriminant) / a;
            // If root is outside range
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        let front_face = HitRecord::set_front_face(ray, outward_normal);
        let normal = HitRecord::set_face_normal(front_face, outward_normal);
        let hit_record = HitRecord::new(point, normal, root, self.material.clone());

        Some(hit_record)
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: UberShader) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}
