use std::fmt::Debug;

use crate::ray::Ray;
use crate::vec3::Vec3;

pub mod plane;
pub mod sphere;

pub trait Shape: Debug + Send + Sync {
    /// Reflect a ray of the shape's surface
    fn reflect(&self, ray: &Ray) -> Option<(Ray, Vec3)>;

    fn center(&self) -> Vec3;
}
