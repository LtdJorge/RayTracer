#![allow(dead_code)]
/// Peter Shirley's "Raytracing in one Weekend" implemented in Rust
mod math;
mod hittables;
mod rendering;
mod output;

use output::image;
use rendering::sampling;
use crate::hittables::{Hittable, HittableList, Sphere};
use crate::math::{clamp, Color, Point3, random_double, Ray, Vec3};
use crate::rendering::Camera;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 40;

    // World
    let mut world: HittableList<Sphere>  = HittableList{ objects: vec![] };
    world.add(Sphere{ center: Point3::new(0.0, 0.0, -1.0), radius: 0.5 });
    world.add(Sphere{ center: Point3::new(0.0, -100.5, -1.0), radius: 100.0 });

    // Camera
    let camera = Camera::new();


    // Render
    image::write_header(image_width, image_height, 255);

    for j in (0..image_height).rev() {
        // eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::ZERO;

            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width as f64);
                let v = (j as f64 + random_double()) / (image_height as f64);
                let ray = camera.get_ray(u, v);
                pixel_color += sampling::ray_color(&ray, &world, max_depth);
            }
            image::write_color_multisample(pixel_color, samples_per_pixel);
        }
        eprintln!("Rendering: {}%", ((image_height - j) as f64 / image_height as f64) * 100.0);
    }
    eprintln!("Done");
}
