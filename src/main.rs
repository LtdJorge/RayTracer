#![allow(dead_code)]
/// Peter Shirley's "Raytracing in one Weekend" implemented in Rust
mod math;
mod hittables;
mod rendering;
mod output;

use std::sync::Arc;
use output::image;
use rendering::sampling;
use rayon::prelude::*;
use crate::hittables::{Hittable, HittableList, Sphere};
use crate::image::write_color_multisample_batch;
use crate::math::{clamp, Color, Point3, random_double, Ray, Vec3};
use crate::rendering::{Camera, DiffuseLambertianMaterial, Material, MaterialPointer, MetallicMaterial};
// use std::time::Instant;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 60;
    let max_depth = 40;

    // World
    let mut world  = HittableList{ objects: vec![] };
    // let material1 = Material;
    // let material2 = Material;
    // world.add(Sphere{ center: Point3::new(0.0, 0.0, -1.0), radius: 0.5, material: Arc::new(material1) });
    // world.add(Sphere{ center: Point3::new(0.0, -100.5, -1.0), radius: 100.0, material: Arc::new(material2) });
    let material_ground: DiffuseLambertianMaterial = DiffuseLambertianMaterial{ albedo: Color::new(0.8, 0.8, 0.0) };
    let material_center: DiffuseLambertianMaterial = DiffuseLambertianMaterial{ albedo: Color::new(0.7, 0.3, 0.3) };
    let material_left: MetallicMaterial = MetallicMaterial{ albedo: Color::new(0.8, 0.8, 0.8) };
    let material_right: MetallicMaterial = MetallicMaterial{ albedo: Color::new(0.8, 0.6, 0.2) };

    world.add(Sphere::new(Point3{
        x: 0.0,
        y: -100.5,
        z: -1.0
    },  100.0, MaterialPointer{ material: Arc::new(material_ground) }));
    world.add(Sphere::new(Point3{
        x: 0.0,
        y: 0.0,
        z: -1.0
    },  0.5, MaterialPointer{ material: Arc::new(material_center) }));
    world.add(Sphere::new(Point3{
        x: -1.0,
        y: 0.0,
        z: -1.0
    },  0.5, MaterialPointer{ material: Arc::new(material_left) }));
    world.add(Sphere::new(Point3{
        x: 1.0,
        y: 0.0,
        z: -1.0
    },  0.5, MaterialPointer{ material: Arc::new(material_right) }));

    // Camera
    let camera = Camera::new();

    render_image_iterator(image_height, image_width, samples_per_pixel, camera, world, max_depth);
    // render_image(image_height, image_width, samples_per_pixel, camera, world, max_depth);
}

fn render_image<Mat: Material + Clone>(image_height: i32, image_width: i32, samples_per_pixel: i32, camera: Camera, world: HittableList<Sphere<Mat>>, max_depth: i32)
where Mat: Material
{
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
        // eprintln!("Rendering: {}%", ((image_height - j) as f64 / image_height as f64) * 100.0);
    }
    eprintln!("Done");
}

fn render_image_iterator<Mat: Material + Clone>(image_height: i32, image_width: i32, samples_per_pixel: i32, camera: Camera, world: HittableList<Sphere<Mat>>, max_depth: i32)
where Mat: Material
{
    let mut pixels = vec![(0, Vec3::ZERO)];

    // let now = Instant::now();
    (0..(image_height*image_width)).into_par_iter().map(|pixel| {
        let mut pixel_color = Color::ZERO;
        let x = pixel % image_width;
        let y = pixel / image_width;
        // eprintln!("Pixel: {} X: {} Y: {} Width: {}", pixel, x, y, image_width);

        for _s in 0..samples_per_pixel {
            let u = (x as f64 + random_double()) / (image_width as f64);
            let v = (y as f64 + random_double()) / (image_height as f64);
            let ray = camera.get_ray(u, v);
            pixel_color += sampling::ray_color(&ray, &world, max_depth);
            // eprintln!("Pixel: {} X: {} Y: {} U: {} V: {} Color: {:?}", pixel, x, y, u, v, pixel_color);
        }
        (pixel, pixel_color)
    }).collect_into_vec(&mut pixels);
    // let elapsed = now.elapsed();
    pixels.sort_by(|(pixel_a, _color_a), (pixel_b, _color_b)| pixel_b.cmp(pixel_a));
    // let elapsed2 = now.elapsed();

    image::write_header(image_width, image_height, 255);
    write_color_multisample_batch(pixels, samples_per_pixel);
    // eprintln!("{:?}\n\n", pixels);
    // eprintln!("First timer: {:.4?} Second timer: {:.4?} Difference: {:.4?}", elapsed, elapsed2, elapsed2 - elapsed);
    eprintln!("Done");
}
