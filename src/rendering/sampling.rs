use crate::rendering::Material;
use crate::{math, Color, Hittable, HittableList, Point3, Ray, Vec3};

pub fn ray_color<T: Hittable>(mut ray: Ray, world: &HittableList<T>, mut depth: i32) -> Color {
    let mut color: Color = Color::ONE;

    loop {
        if depth <= 0 {
            color = Color::ZERO;
            break;
        }

        if let Some(record) = world.hit(ray, 0.0001, math::INFINITY) {
            if let Some(scatter) = record.material.scatter(&ray, &record) {
                color *= scatter.attenuation;
                depth -= 1;
                ray = scatter.ray;
            } else {
                color = Color::ZERO;
                break;
            }
        } else {
            // Background color, kinda skyish
            let unit_direction = &ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            color *= (1.0 - t) * Color::ONE
                + t * Color {
                    x: 0.5,
                    y: 0.7,
                    z: 1.0,
                };
            break;
        }
    }
    color
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
