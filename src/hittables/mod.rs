#![allow(dead_code)]

pub use hit_record::HitRecord;
pub use hittable::Hittable;
pub use hittable_list::HittableList;
pub use sphere::Sphere;

mod hit_record;
mod sphere;
mod hittable_list;
mod hittable;

