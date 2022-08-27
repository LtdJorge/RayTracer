#![allow(dead_code)]
mod camera;
pub(crate) mod sampling;
mod material;

pub use camera::{Camera};
pub use material::{Material, ScatteringResult};
