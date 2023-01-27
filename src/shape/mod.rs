use crate::ray::Ray;
use std::fmt::Debug;

pub mod sphere;
pub mod plane;

pub trait Shape: Debug {
    fn intersect(&self, ray: &Ray) -> bool;
}
