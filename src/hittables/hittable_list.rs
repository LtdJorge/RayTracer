use crate::hittables::{HitRecord, Hittable};
use crate::{Ray};

#[derive(Debug, Clone)]
pub struct HittableList<Geometry: Hittable> {
    pub objects: Vec<Geometry>,
}

impl<Geometry: Hittable> HittableList<Geometry> {
    pub fn add(&mut self, object: Geometry) {
        self.objects.push(object)
    }
}

impl<T: Hittable> HittableList<T> {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far: Option<HitRecord> = None;
        let mut t_max = t_max;

        for object in &self.objects {
            if let Some(record) = object.hit(ray, t_min, t_max){
                t_max = record.t;
                closest_so_far = Some(record);
            }
        }
        closest_so_far
    }
}