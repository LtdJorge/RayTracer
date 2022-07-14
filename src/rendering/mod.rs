#![allow(dead_code)]
mod camera;
pub(crate) mod sampling;
mod materials;

pub use camera::{Camera};
pub use materials::{Material, DiffuseLambertianMaterial, MetallicMaterial, MaterialPointer};