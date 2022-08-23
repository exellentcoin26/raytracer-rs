mod camera;
mod color;
mod hitrecord;
pub mod hittables;
pub mod materials;
mod ray;
pub mod utils;
mod vec3;

pub use camera::Camera;
pub use color::Color;
pub use hitrecord::HitRecord;
pub use ray::Ray;
pub use vec3::{Point3, Vec3};

// trait definitions
pub mod traits;
