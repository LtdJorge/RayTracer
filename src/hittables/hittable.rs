use crate::hittables::HitRecord;
use crate::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult;
}

pub struct HitResult {
    pub got_hit: bool,
    pub hit_record: HitRecord
}

impl HitResult {
    pub fn create(hit_record: HitRecord) -> HitResult {
        HitResult{ got_hit: true, hit_record }
    }
    pub const FALSE: HitResult = HitResult { got_hit: false, hit_record: HitRecord::EMPTY };
}