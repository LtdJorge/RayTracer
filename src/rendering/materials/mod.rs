mod diffuse;
mod metallic;
mod material;

pub use diffuse::{lambertian_render_function, hemispheric_scattering_render_function, DiffuseLambertianMaterial};
pub use metallic::MetallicMaterial;
pub use material::{Material, MaterialPointer};