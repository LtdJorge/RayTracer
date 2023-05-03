#![allow(dead_code)]

pub use camera::Camera;
pub use material::{Material, ScatteringResult, UberShader};

mod camera;
mod material;
pub(crate) mod sampling;
