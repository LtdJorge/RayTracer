use crate::{Material, Point3, Ray, Vec3};
use crate::hittables::hit_record::HitRecord;
use crate::hittables::{HitResult, Hittable};
use crate::rendering::{MaterialPointer};

#[derive(Clone)]
pub struct Sphere<Mat: Material + Clone> {
    pub center: Point3,
    pub radius: f64,
    pub material: MaterialPointer<Mat>
}

impl<Mat: Material + Clone> Sphere<Mat> {
    pub fn new(center: Point3, radius: f64, material: MaterialPointer<Mat>) -> Sphere<Mat> {
        Sphere{
            center,
            radius,
            material
        }
    }
}

impl<Mat: Material + Clone> Hittable for Sphere<Mat> {
    fn hit<Mat: Material + Clone>(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult<T> {
        // Was 'oc'
        let origin_to_center: Vec3 = ray.origin - self.center;
        // A param of quadratic equation
        let a = ray.direction.squared_length();
        // Half the b param of quadratic equation
        let half_b = Vec3::dot_product(&origin_to_center, &ray.direction);
        // C param of quadratic equation
        let c = origin_to_center.squared_length() - self.radius * self.radius;

        // Value inside the square root of the quadratic equation
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            // Negative number = no square root, so no hit
            return HitResult::FALSE;
        }
        // Result of the square root
        let sqrt_discriminant = discriminant.sqrt();

        // Quadratic equation is (-half_b +- sqrtd) / a
        // The negative value of the sqrt value is used first because it would be closer
        let mut  root = (-half_b - sqrt_discriminant) / a;
        // If root is outside range
        if root < t_min || t_max < root {
            // Use the positive sqrt value
            root = (-half_b + sqrt_discriminant) / a;
            // If root is outside range
            if root < t_min || t_max < root {
                return HitResult::FALSE;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        let front_face = HitRecord::set_front_face(ray, outward_normal);
        let normal = HitRecord::set_face_normal(front_face, outward_normal);
        let material = self.material.clone();
        let hit_record = HitRecord::create(point, normal, root, material);

        /*let outward_normal = (new_record.p - self.center) / self.radius;
        let front_face = HitRecord::set_front_face(ray, outward_normal);

        let temp_rec = &HitRecord {
            t: root,
            p: ray.at(root),
            normal: HitRecord::set_face_normal(front_face, outward_normal),
            front_face
        };

        //TODO: this is the ugliest thing I've done ever
        record.create(
            ray.at(root),
            HitRecord::set_face_normal(front_face, outward_normal),
            root,
            front_face
        );*/

        HitResult::create(hit_record)
    }
}