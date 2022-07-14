use crate::{Color, Hittable, HittableList, math, Point3, Ray, Vec3, rendering::materials};
use crate::rendering::materials::{hemispheric_scattering_render_function, lambertian_render_function};

pub fn ray_color<T: Hittable>(ray: &Ray, world: &HittableList<T>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::ZERO;
    }

    let hit_result = world.hit(ray, 0.0001, math::INFINITY).clone();

    if hit_result.got_hit {
        let mut scattered: Ray = Ray::new(Point3::ZERO, Vec3::ZERO);
        let mut attenuation: Color = Color::ZERO;
        if hit_result.hit_record.material.scatter(ray, &hit_result.hit_record, attenuation, scattered){
            return attenuation * ray_color(&scattered, world, depth-1)
            //TODO: check if scattered and attenuation are being modified
        }
        Color::ZERO;
        // let target: Point3 = hemispheric_scattering_render_function(&hit_result.hit_record);
        // return 0.5 * ray_color(&Ray::new(hit_result.hit_record.p, target - hit_result.hit_record.p), world, depth - 1);
    }

    // Background color, kinda skyish
    let unit_direction = &ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::ONE + t * Color{x: 0.5, y: 0.7, z: 1.0 }
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.squared_length();
    let half_b = Vec3::dot_product(&oc, &ray.direction);
    let c = oc.squared_length() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0_f64 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}
