use std::fmt::Debug;

use crate::ray::Ray;

pub mod plane;
pub mod sphere;

pub trait Shape: Debug + Send + Sync {
    /// Reflect a ray of the shape's surface
    fn reflect(&self, ray: &Ray) -> Option<Ray>;
}
