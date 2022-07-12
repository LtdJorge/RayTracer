use crate::hittables::{HitRecord, HitResult, Hittable};
use crate::{Ray};

#[derive(Debug, Clone)]
pub struct HittableList<T: Hittable> {
    pub objects: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn add(&mut self, object: T) {
        self.objects.push(object)
    }
}

impl<T: Hittable> HittableList<T> {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult {
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        let mut record: HitRecord = HitRecord::EMPTY;

        for object in &self.objects {
            let hit: HitResult = object.hit(ray, t_min, closest_so_far);
            if hit.got_hit {
                hit_anything = true;
                // eprintln!("{}", hit.hit_record.t);
                closest_so_far = hit.hit_record.t;
                record = hit.hit_record;
            }
        }
        return HitResult { got_hit: hit_anything, hit_record: record };
    }
}