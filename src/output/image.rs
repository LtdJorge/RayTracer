use crate::{clamp, Color};
use exr::prelude::*;
use rayon::prelude::*;
use std::path::PathBuf;

fn write_test_image() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;
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

fn write_pixel(red: f64, green: f64, blue: f64) {
    println!("{} {} {}", red, green, blue);
}

fn write_color(pixel_color: Color) {
    let red: i32 = (pixel_color.x * 255.999) as i32;
    let green: i32 = (pixel_color.y * 255.999) as i32;
    let blue: i32 = (pixel_color.z * 255.999) as i32;
    println!("{} {} {}", red, green, blue);
}

pub fn write_color_multisample(pixel_color: Color, samples_per_pixel: i32) {
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

pub fn write_color_multisample_batch(pixels: Vec<(usize, Color)>, samples_per_pixel: i32) {
    for (_pixel, pixel_color) in pixels {
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
}
//TODO: implement in parallel
#[cfg(feature = "parallel")]
pub fn write_color_multisample_iter(pixels: Vec<(i32, Color)>, samples_per_pixel: i32) {
    pixels.into_par_iter().chunks(4).for_each(|chunk| {
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
    });
}

pub fn write_header(image_width: usize, image_height: usize, max_color: i32) {
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("{}", max_color);
}

pub fn write_exr(
    path: PathBuf,
    (x_size, y_size): (usize, usize),
    pixels: Vec<(usize, Color)>,
    samples_per_pixel: i32,
) {
    let scale = 1.0 / samples_per_pixel as f64;

    let channel = SpecificChannels::rgb(|position: Vec2<usize>| {
        let x = position.0;
        let y = position.1;
        let pixel_position = y * x_size + x;
        let color: Color = pixels[pixel_position].1;
        (
            (color.x * scale).sqrt() as f32,
            (color.y * scale).sqrt() as f32,
            (color.z * scale).sqrt() as f32,
        )
    });
    // let channel = SpecificChannels::build()
    //     .with_channel("B")
    //     .with_channel("G")
    //     .with_channel("R")
    //     .with_pixel_fn(|position| {
    //         let x = position.0;
    //         let y = position.1;
    //         let pixel_position = y * x_size + x;
    //         let color: Color = pixels[pixel_position].1;
    //         (color.z as f32, color.y as f32, color.x as f32)
    //     });
    let image = Image::from_channels((x_size, y_size), channel);
    let mut current_progress_percentage = 0;

    image
        .write()
        .on_progress(|progress| {
            let new_progress = (progress * 100.0) as usize;
            if new_progress != current_progress_percentage {
                current_progress_percentage = new_progress;
                println!("progress: {}%", current_progress_percentage)
            }
        })
        .to_file(&path)
        .unwrap();

    eprintln!("created file {}", path.to_str().unwrap());
}
