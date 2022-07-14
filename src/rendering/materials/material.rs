use std::sync::Arc;
use crate::hittables::HitRecord;
use crate::{Color, Ray};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord<T>, attenuation: Color, scattered_ray: Ray) -> bool;
}

// pub type MaterialPointer = Arc<T>;

#[derive(Clone)]
pub struct MaterialPointer<Mat: Material + Clone> {
    pub material: Arc<Mat>
}
