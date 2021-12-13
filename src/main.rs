/// Peter Shirley's "Raytracing in one Weekend" implemented in Rust
mod math;
mod hittables;

use crate::math::{Vec3, Ray, Color, Point3};

fn main() {
    let aspect_ratio = 21.0 / 9.0;
    let image_width = 4096.0;
    // eprintln!("Ratio {}", aspect_ratio);
    let image_height = image_width as f64 / aspect_ratio;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3{
        x: 0.0,
        y: 0.0,
        z: 0.0
    };
    let horizontal = Vec3{
        x: viewport_width,
        y: 0.0,
        z: 0.0
    };
    let vertical = Vec3{
        x: 0.0,
        y: viewport_height,
        z: 0.0
    };
    let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3{
        x: 0.0,
        y: 0.0,
        z: focal_length
    };

    write_header(image_width as i32, image_height as i32, 255);

    // eprintln!("Height: {}", image_height);
    // eprintln!("Width: {}", image_width);

    for j in (0..image_height as i32).rev() {
        // eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width as i32 {
            let u = i as f64 / (image_width - 1.0);
            let v = j as f64 / (image_height - 1.0);
            let ray: Ray = Ray{ origin, direction: lower_left_corner + u as f64 * horizontal + v as f64 * vertical - origin };
            let pixel_color = ray_color(&ray);
            write_color(pixel_color);
        }
    }
    eprintln!("Done");
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Vec3::BACK, 0.5, ray);
    if t > 0.0_f64 {
        let normal: Vec3 = Vec3::unit_vector(&(ray.at(t) - Vec3::BACK));
        return 0.5 * Color{ x: normal.x+1.0, y: normal.y + 1.0, z: normal.z + 1.0 };
    }
    let unit_direction = Vec3::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color{ x: 1.0, y: 1.0, z: 1.0 } + t * Color{x: 0.5, y: 0.7, z: 1.0 }
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.squared_length();
    let half_b = Vec3::dot_product(&oc, &ray.direction);
    let c = oc.squared_length() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    return if discriminant < 0.0_f64 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn write_test_image() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    const MAX_COLOR: i32 = 255;

    write_header(IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR);

    for vertical_index in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", vertical_index);
        for horizontal_index in 0..IMAGE_WIDTH {
            let r: f64 = horizontal_index as f64 / (IMAGE_WIDTH - 1) as f64;
            let g: f64 = vertical_index as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b: f64 = 0.25;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            write_pixel(ir as f64, ig as f64, ib as f64);
        }
    }

    eprintln!("Done");
}

fn write_pixel(red: f64, green: f64, blue: f64){
    println!("{} {} {}", red, green, blue);
}

fn write_color(pixel_color: Color){
    let red: i32 = (pixel_color.x * 255.999) as i32;
    let green: i32 = (pixel_color.y * 255.999) as i32;
    let blue: i32 = (pixel_color.z * 255.999) as i32;
    println!("{} {} {}", red, green, blue);
}

fn write_header(image_width: i32, image_height: i32, max_color: i32){
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("{}", max_color);
}
