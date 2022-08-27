#![allow(dead_code)]

pub use camera::Camera;
pub use material::{Material, ScatteringResult};

mod camera;
pub(crate) mod sampling;
mod material;

