use crate::math::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    /**
    #Arguments
    * `distance` - 't' in the book. Distance in the ray where it is sampled.
    */
    pub fn at(&self, distance: f64) -> Point3 {
        self.origin + distance * self.direction
    }
}