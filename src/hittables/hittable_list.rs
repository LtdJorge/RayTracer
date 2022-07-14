use crate::hittables::{HitRecord, HitResult, Hittable};
use crate::{Material, Ray};

#[derive(Debug, Clone)]
pub struct HittableList<Geometry: Hittable> {
    pub objects: Vec<Geometry>,
}

impl<Geometry: Hittable> HittableList<Geometry> {
    pub fn add(&mut self, object: Geometry) {
        self.objects.push(object)
    }
}

impl<Geometry: Hittable> HittableList<Geometry> {
    pub fn hit<Mat: Material>(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult<Mat> {
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        let mut record: HitRecord<Geometry> = HitRecord::EMPTY;

        for object in &self.objects {
            let hit: HitResult<Geometry> = object.hit(ray, t_min, closest_so_far);
            if hit.got_hit {
                hit_anything = true;
                // eprintln!("{}", hit.hit_record.t);
                closest_so_far = hit.hit_record.t;
                record = hit.hit_record;
            }
        }
        HitResult { got_hit: hit_anything, hit_record: record }
    }
}