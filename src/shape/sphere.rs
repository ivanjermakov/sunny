use crate::ray::Ray;
use crate::shape::Shape;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        todo!()
    }
}
