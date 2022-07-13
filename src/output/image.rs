use crate::{clamp, Color};

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

pub fn write_color_multisample(pixel_color: Color, samples_per_pixel: i32){
    let mut red = pixel_color.x;
    let mut green = pixel_color.y;
    let mut blue = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    red = (red * scale).sqrt();
    green = (green * scale).sqrt();
    blue = (blue * scale).sqrt();

    let clamped_red = (256.0 * clamp(red, 0.0, 0.999)) as i32;
    let clamped_green = (256.0 * clamp(green, 0.0, 0.999)) as i32;
    let clamped_blue = (256.0 * clamp(blue, 0.0, 0.999)) as i32;

    println!("{} {} {}", clamped_red, clamped_green, clamped_blue);
}

pub fn write_header(image_width: i32, image_height: i32, max_color: i32){
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("{}", max_color);
}
