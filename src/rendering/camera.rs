use crate::math::{Point3, Vec3, Ray};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    // pub u: Vec3,
    // pub v: Vec3,
    // pub w: Vec3,
    // pub lens_radius: f64,
    // pub time0: f64,
    // pub time1: f64,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::ZERO;
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

        Camera{
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray{
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        }
    }
}