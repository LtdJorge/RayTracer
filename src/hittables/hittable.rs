use crate::hittables::HitRecord;
use crate::Ray;

pub trait Hittable {
    fn hit<T>(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult<T>;
}

#[derive(Clone)]
pub struct HitResult<T> {
    pub got_hit: bool,
    pub hit_record: HitRecord<T>
}

impl<T> HitResult<T> {
    pub fn create(hit_record: HitRecord<T>) -> HitResult<T> {
        HitResult{ got_hit: true, hit_record }
    }
    pub const FALSE: HitResult<T> = HitResult { got_hit: false, hit_record: HitRecord::EMPTY };
}